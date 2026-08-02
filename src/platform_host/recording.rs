use std::collections::VecDeque;

use crate::error::{GuiError, GuiResult};

use super::{
    PlatformHost, PlatformHostCommand, PlatformHostCommitAck, PlatformHostEvent,
    PlatformHostRevision, PlatformHostTransaction, PlatformPresentationAck,
    PlatformPresentationStatus,
};

pub const DEFAULT_PLATFORM_HOST_HISTORY_LIMIT: usize = 256;
pub const DEFAULT_PLATFORM_HOST_EVENT_QUEUE_LIMIT: usize = 1024;

/// Deterministic zero-widget host used to prove transaction and service
/// semantics before an OS shell exists.
///
/// Committed history is diagnostic state, so sensitive text is replaced while
/// retaining command structure and byte lengths. A pending transaction remains
/// complete until commit or rollback.
pub struct RecordingPlatformHost {
    pending: Option<PlatformHostTransaction>,
    committed: Vec<PlatformHostTransaction>,
    events: VecDeque<PlatformHostEvent>,
    last_committed_revision: Option<PlatformHostRevision>,
    history_limit: usize,
    event_queue_limit: usize,
    fail_next_commit: Option<String>,
    shutdown: bool,
}

impl std::fmt::Debug for RecordingPlatformHost {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RecordingPlatformHost")
            .field(
                "pending_revision",
                &self.pending.as_ref().map(|item| item.revision),
            )
            .field("committed_transactions", &self.committed.len())
            .field("queued_events", &self.events.len())
            .field("last_committed_revision", &self.last_committed_revision)
            .field("history_limit", &self.history_limit)
            .field("event_queue_limit", &self.event_queue_limit)
            .field("shutdown", &self.shutdown)
            .finish()
    }
}

impl Default for RecordingPlatformHost {
    fn default() -> Self {
        Self::with_limits(
            DEFAULT_PLATFORM_HOST_HISTORY_LIMIT,
            DEFAULT_PLATFORM_HOST_EVENT_QUEUE_LIMIT,
        )
    }
}

impl RecordingPlatformHost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_limits(history_limit: usize, event_queue_limit: usize) -> Self {
        Self {
            pending: None,
            committed: Vec::new(),
            events: VecDeque::new(),
            last_committed_revision: None,
            history_limit,
            event_queue_limit,
            fail_next_commit: None,
            shutdown: false,
        }
    }

    pub fn pending(&self) -> Option<&PlatformHostTransaction> {
        self.pending.as_ref()
    }

    pub fn committed(&self) -> &[PlatformHostTransaction] {
        &self.committed
    }

    pub const fn last_committed_revision(&self) -> Option<PlatformHostRevision> {
        self.last_committed_revision
    }

    pub fn queued_event_count(&self) -> usize {
        self.events.len()
    }

    pub const fn history_limit(&self) -> usize {
        self.history_limit
    }

    pub const fn event_queue_limit(&self) -> usize {
        self.event_queue_limit
    }

    pub const fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    pub fn queue_event(&mut self, event: PlatformHostEvent) -> GuiResult<()> {
        if self.shutdown {
            return Err(GuiError::host(
                "cannot queue a platform host event after shutdown",
            ));
        }
        event.validate()?;
        if self.events.len() >= self.event_queue_limit {
            return Err(GuiError::host(format!(
                "platform host event queue reached its {}-event limit",
                self.event_queue_limit
            )));
        }
        self.events.push_back(event);
        Ok(())
    }

    pub fn fail_next_commit(&mut self, message: impl Into<String>) {
        self.fail_next_commit = Some(message.into());
    }

    fn ensure_running(&self) -> GuiResult<()> {
        if self.shutdown {
            Err(GuiError::host("platform host is shut down"))
        } else {
            Ok(())
        }
    }

    fn record_committed(&mut self, transaction: PlatformHostTransaction) {
        if self.history_limit == 0 {
            return;
        }
        if self.committed.len() == self.history_limit {
            self.committed.remove(0);
        }
        self.committed.push(transaction.redacted_for_diagnostics());
    }
}

impl PlatformHost for RecordingPlatformHost {
    type PresentationTarget = ();

    fn prepare(&mut self, transaction: PlatformHostTransaction) -> GuiResult<()> {
        self.ensure_running()?;
        if self.pending.is_some() {
            return Err(GuiError::host(
                "platform host already has a pending transaction",
            ));
        }
        transaction.validate()?;
        if self
            .last_committed_revision
            .is_some_and(|revision| transaction.revision <= revision)
        {
            return Err(GuiError::host(format!(
                "platform host revision {} must be newer than committed revision {}",
                transaction.revision.get(),
                self.last_committed_revision
                    .map(PlatformHostRevision::get)
                    .unwrap_or_default()
            )));
        }
        self.pending = Some(transaction);
        Ok(())
    }

    fn presentation_target(
        &self,
        window: super::PlatformWindowId,
    ) -> GuiResult<Self::PresentationTarget> {
        self.ensure_running()?;
        window.validate()?;
        let pending = self.pending.as_ref().ok_or_else(|| {
            GuiError::host("platform host has no pending presentation transaction")
        })?;
        let requested = pending.commands.iter().any(|command| {
            matches!(
                command,
                PlatformHostCommand::Present { request } if request.window == window
            )
        });
        if !requested {
            return Err(GuiError::host(format!(
                "platform host transaction has no presentation for window {}",
                window.get()
            )));
        }
        Ok(())
    }

    fn commit(&mut self) -> GuiResult<PlatformHostCommitAck> {
        self.ensure_running()?;
        if self.pending.is_none() {
            return Err(GuiError::host(
                "platform host has no pending transaction to commit",
            ));
        }
        if let Some(message) = self.fail_next_commit.take() {
            return Err(GuiError::host(format!(
                "platform host commit failed: {message}"
            )));
        }
        let Some(transaction) = self.pending.take() else {
            return Err(GuiError::host("platform host transaction disappeared"));
        };
        let presentations = transaction
            .commands
            .iter()
            .filter_map(|command| match command {
                PlatformHostCommand::Present { request } => Some(PlatformPresentationAck {
                    revision: transaction.revision,
                    window: request.window,
                    status: PlatformPresentationStatus::Queued,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();
        let ack = PlatformHostCommitAck {
            revision: transaction.revision,
            applied_commands: transaction.commands.len(),
            presentations,
        };
        ack.validate()?;
        self.last_committed_revision = Some(transaction.revision);
        self.record_committed(transaction);
        Ok(ack)
    }

    fn rollback(&mut self) -> GuiResult<()> {
        self.ensure_running()?;
        self.pending = None;
        Ok(())
    }

    fn poll_event(&mut self) -> GuiResult<Option<PlatformHostEvent>> {
        self.ensure_running()?;
        Ok(self.events.pop_front())
    }

    fn shutdown(&mut self) -> GuiResult<()> {
        if self.shutdown {
            return Ok(());
        }
        if self.pending.is_some() {
            return Err(GuiError::host(
                "platform host cannot shut down with a pending transaction",
            ));
        }
        self.events.clear();
        self.shutdown = true;
        Ok(())
    }
}
