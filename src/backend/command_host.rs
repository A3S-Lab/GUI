use crate::accessibility::{AccessibilityNode, AccessibilityTreeHost};
use crate::capability::{CapabilityHost, NativeCapabilities};
use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::{
    HostFrameAck, HostNodeId, NativeHost, OverlayPositionHost, ProgrammaticFocusHost,
};
use crate::native::{NativeElement, NativeProps};
use crate::overlay_position::OverlayPositionRequest;
use crate::platform::{
    BlueprintHost, NativeWidgetBlueprint, PlatformAdapter, PlatformPlanningHost,
};
use crate::selection::{CollectionKey, CollectionLayoutSnapshot};
use crate::style::PortableStyle;

use super::traits::{
    NativeEventHost, NativeEventSource, PlatformBatchAck, PlatformCommandBatch,
    PlatformCommandExecutor,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DegradedNativeState {
    pub batch: PlatformCommandBatch,
    pub applied_commands: usize,
    pub failed_command: usize,
    pub reason: String,
}

#[derive(Debug)]
pub struct CommandExecutingHost<A: PlatformAdapter, E: PlatformCommandExecutor> {
    planning: PlatformPlanningHost<A>,
    executor: E,
    next_batch_id: u64,
    active_frame: Option<crate::platform::PlatformPlanningCheckpoint>,
    last_ack: Option<PlatformBatchAck>,
    degraded: Option<DegradedNativeState>,
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> CommandExecutingHost<A, E> {
    pub fn new(adapter: A, executor: E) -> Self {
        Self {
            planning: PlatformPlanningHost::new(adapter),
            executor,
            next_batch_id: 0,
            active_frame: None,
            last_ack: None,
            degraded: None,
        }
    }

    pub fn planning(&self) -> &PlatformPlanningHost<A> {
        &self.planning
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn executor_mut(&mut self) -> &mut E {
        &mut self.executor
    }

    pub fn last_ack(&self) -> Option<PlatformBatchAck> {
        self.last_ack
    }

    pub fn degraded_state(&self) -> Option<&DegradedNativeState> {
        self.degraded.as_ref()
    }

    pub fn is_degraded(&self) -> bool {
        self.degraded.is_some()
    }

    pub fn into_parts(self) -> (PlatformPlanningHost<A>, E) {
        (self.planning, self.executor)
    }

    /// Replace a degraded executor with a fresh native executor and replay the
    /// complete acknowledged planning snapshot into it.
    ///
    /// Dropping the old executor is intentional: an OS command can partially
    /// mutate native objects before reporting failure, so incremental recovery
    /// against that surface is unsafe.
    pub fn recover_with_executor(&mut self, executor: E) -> GuiResult<PlatformBatchAck> {
        if self.active_frame.is_some() {
            return Err(GuiError::host(
                "cannot recover native execution while a render frame is active",
            ));
        }
        if self.degraded.is_none() {
            return Err(GuiError::host(
                "native execution is healthy; recovery is not required",
            ));
        }

        let commands = self.planning.replay_commands();
        let batch = PlatformCommandBatch::new(self.allocate_batch_id()?, commands);
        // Tear down the partially committed surface before the replacement can
        // create any windows/widgets. PlatformCommandExecutor's Drop contract
        // requires it to release all native resources it owns.
        let old_executor = std::mem::replace(&mut self.executor, executor);
        drop(old_executor);

        if let Err(error) = self.executor.prepare_batch(&batch) {
            self.degraded = Some(DegradedNativeState {
                batch: redacted_batch(&batch),
                applied_commands: 0,
                failed_command: 0,
                reason: format!("recovery prepare failed: {error}"),
            });
            return Err(GuiError::host(format!(
                "failed to prepare native recovery batch {}: {error}",
                batch.id
            )));
        }
        let ack = match self.executor.commit_batch(&batch) {
            Ok(ack) => ack,
            Err(failure) => {
                self.degraded = Some(DegradedNativeState {
                    batch: redacted_batch(&batch),
                    applied_commands: failure.applied_commands,
                    failed_command: failure.failed_command,
                    reason: format!("recovery commit failed: {}", failure.error),
                });
                return Err(GuiError::host(format!(
                    "native recovery batch {} failed at command {} after {} acknowledged commands: {}",
                    failure.batch_id,
                    failure.failed_command,
                    failure.applied_commands,
                    failure.error
                )));
            }
        };
        if let Err(error) = validate_ack(&batch, ack) {
            self.degraded = Some(DegradedNativeState {
                batch: redacted_batch(&batch),
                applied_commands: ack.applied_commands,
                failed_command: ack.applied_commands.min(batch.commands.len()),
                reason: format!("recovery acknowledgement failed: {error}"),
            });
            return Err(error);
        }

        self.planning.clear_commands();
        self.last_ack = Some(ack);
        self.degraded = None;
        Ok(ack)
    }

    fn ensure_healthy(&self) -> GuiResult<()> {
        if let Some(degraded) = &self.degraded {
            Err(GuiError::host(format!(
                "native execution is degraded after batch {} failed at command {}; replace the executor and replay before continuing",
                degraded.batch.id, degraded.failed_command
            )))
        } else {
            Ok(())
        }
    }

    fn allocate_batch_id(&mut self) -> GuiResult<u64> {
        self.next_batch_id = self
            .next_batch_id
            .checked_add(1)
            .ok_or_else(|| GuiError::host("native command batch id overflow"))?;
        Ok(self.next_batch_id)
    }

    fn flush_commands(&mut self) -> GuiResult<Option<PlatformBatchAck>> {
        self.ensure_healthy()?;
        let commands = self.planning.commands().to_vec();
        if commands.is_empty() {
            return Ok(None);
        }
        let batch = PlatformCommandBatch::new(self.allocate_batch_id()?, commands);

        self.executor.prepare_batch(&batch).map_err(|error| {
            GuiError::host(format!(
                "failed to prepare native command batch {}: {error}",
                batch.id
            ))
        })?;

        let ack = match self.executor.commit_batch(&batch) {
            Ok(ack) => ack,
            Err(failure) => {
                if failure.native_state_may_be_partial {
                    self.degraded = Some(DegradedNativeState {
                        batch: redacted_batch(&batch),
                        applied_commands: failure.applied_commands,
                        failed_command: failure.failed_command,
                        reason: failure.error.to_string(),
                    });
                }
                return Err(GuiError::host(format!(
                    "native command batch {} may be partially committed; command {} failed after {} acknowledged commands: {}",
                    failure.batch_id,
                    failure.failed_command,
                    failure.applied_commands,
                    failure.error
                )));
            }
        };

        if let Err(error) = validate_ack(&batch, ack) {
            self.degraded = Some(DegradedNativeState {
                batch: redacted_batch(&batch),
                applied_commands: ack.applied_commands,
                failed_command: ack.applied_commands.min(ack.prepared_commands),
                reason: error.to_string(),
            });
            return Err(error);
        }
        if let Err(error) = self.planning.acknowledge_commands(&batch.commands) {
            self.degraded = Some(DegradedNativeState {
                batch: redacted_batch(&batch),
                applied_commands: ack.applied_commands,
                failed_command: ack.applied_commands,
                reason: error.to_string(),
            });
            return Err(error);
        }
        self.last_ack = Some(ack);
        Ok(Some(ack))
    }

    fn commit_planning<T>(
        &mut self,
        apply: impl FnOnce(&mut PlatformPlanningHost<A>) -> GuiResult<T>,
    ) -> GuiResult<T> {
        self.ensure_healthy()?;
        if self.active_frame.is_some() {
            return apply(&mut self.planning);
        }

        let checkpoint = self.planning.checkpoint();
        let value = match apply(&mut self.planning) {
            Ok(value) => value,
            Err(error) => {
                self.planning.restore(checkpoint);
                return Err(error);
            }
        };
        if let Err(error) = self.flush_commands() {
            self.planning.restore(checkpoint);
            return Err(error);
        }
        Ok(value)
    }
}

fn validate_ack(batch: &PlatformCommandBatch, ack: PlatformBatchAck) -> GuiResult<()> {
    if ack.batch_id != batch.id
        || ack.prepared_commands != batch.commands.len()
        || ack.applied_commands != batch.commands.len()
    {
        return Err(GuiError::host(format!(
            "native executor returned invalid acknowledgement for batch {} (prepared {}, applied {}, expected {})",
            ack.batch_id,
            ack.prepared_commands,
            ack.applied_commands,
            batch.commands.len()
        )));
    }
    Ok(())
}

fn redacted_batch(batch: &PlatformCommandBatch) -> PlatformCommandBatch {
    PlatformCommandBatch::new(
        batch.id,
        batch
            .commands
            .iter()
            .map(crate::platform::PlatformCommand::redacted_for_diagnostics)
            .collect(),
    )
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor + NativeEventSource> NativeEventHost
    for CommandExecutingHost<A, E>
{
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.executor.take_native_events()
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> AccessibilityTreeHost
    for CommandExecutingHost<A, E>
{
    fn accessibility_tree(&self) -> Option<AccessibilityNode> {
        self.planning.accessibility_tree()
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> CapabilityHost for CommandExecutingHost<A, E> {
    fn native_capabilities(&self) -> NativeCapabilities {
        self.planning.capabilities()
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> BlueprintHost for CommandExecutingHost<A, E> {
    fn blueprint(&self, id: HostNodeId) -> Option<&NativeWidgetBlueprint> {
        self.planning.blueprint(id)
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> NativeHost for CommandExecutingHost<A, E> {
    fn begin_frame(&mut self) -> GuiResult<()> {
        self.ensure_healthy()?;
        if self.active_frame.is_some() {
            return Err(GuiError::host("a native render frame is already active"));
        }
        self.active_frame = Some(self.planning.checkpoint());
        Ok(())
    }

    fn commit_frame(&mut self) -> GuiResult<HostFrameAck> {
        if self.active_frame.is_none() {
            return Err(GuiError::host("no native render frame is active"));
        }
        match self.flush_commands() {
            Ok(ack) => {
                self.active_frame.take();
                Ok(match ack {
                    Some(ack) => HostFrameAck {
                        batch_id: Some(ack.batch_id),
                        applied_operations: ack.applied_commands,
                    },
                    None => HostFrameAck::default(),
                })
            }
            // Keep the checkpoint and queued commands intact until the caller
            // explicitly rolls the failed frame back. This makes command loss
            // impossible and lets diagnostics inspect the exact failed batch.
            Err(error) => Err(error),
        }
    }

    fn rollback_frame(&mut self) -> GuiResult<()> {
        if let Some(checkpoint) = self.active_frame.take() {
            self.planning.restore(checkpoint);
        }
        Ok(())
    }

    fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        self.commit_planning(|planning| planning.create(element))
    }

    fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()> {
        self.commit_planning(|planning| planning.update(id, props))
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        self.commit_planning(|planning| planning.insert_child(parent, child, index))
    }

    fn remove(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.commit_planning(|planning| planning.remove(id))
    }

    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.commit_planning(|planning| planning.set_root(id))
    }

    fn update_portable_style(&mut self, id: HostNodeId, style: &PortableStyle) -> GuiResult<()> {
        self.commit_planning(|planning| planning.project_portable_style(id, style))
    }

    fn programmatic_focus_host(&mut self) -> Option<&mut dyn ProgrammaticFocusHost> {
        Some(self)
    }

    fn overlay_position_host(&mut self) -> Option<&mut dyn OverlayPositionHost> {
        Some(self)
    }

    fn measure_collection_layout(
        &mut self,
        collection: HostNodeId,
        items: &[(HostNodeId, CollectionKey)],
    ) -> GuiResult<Option<CollectionLayoutSnapshot>> {
        self.ensure_healthy()?;
        self.executor.measure_collection_layout(collection, items)
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> ProgrammaticFocusHost
    for CommandExecutingHost<A, E>
{
    fn request_focus(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.commit_planning(|planning| planning.request_focus(id))
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> OverlayPositionHost
    for CommandExecutingHost<A, E>
{
    fn position_overlay(
        &mut self,
        overlay: HostNodeId,
        anchor: HostNodeId,
        request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        self.commit_planning(|planning| planning.position_overlay(overlay, anchor, request))
    }
}
