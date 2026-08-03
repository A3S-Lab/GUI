//! Raw Win32 top-level host for the first H2 self-drawn window slice.
//!
//! This module owns only HWND lifecycle, DPI-aware client geometry, the
//! message pump, raw surface identity, presentation scheduling, and normalized
//! legacy mouse, keyboard, and wheel translation. Graphics owns all
//! application-content drawing and GPU presentation; WM_POINTER touch/pen,
//! TSF, UI Automation, and system services remain later H2 work.

use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;

use windows_sys::Win32::Foundation::HINSTANCE;

use crate::error::{GuiError, GuiResult};

use super::{
    PlatformAccessibilitySnapshot, PlatformHost, PlatformHostCommand, PlatformHostCommitAck,
    PlatformHostEvent, PlatformHostRevision, PlatformHostTransaction, PlatformPresentationAck,
    PlatformPresentationStatus, PlatformWindowEvent, PlatformWindowId, PlatformWindowSpec,
    DEFAULT_PLATFORM_HOST_EVENT_QUEUE_LIMIT,
};

mod commit;
mod events;
mod input;
mod keyboard;
mod native;
mod plan;
mod surface;

use events::WindowsEventQueue;
use native::{pump_messages, register_window_class, system_scale_factor, NativeWindow};
use plan::{plan_transaction, PreparedWindowsTransaction, WindowsWindowState};
pub use surface::WindowsSurfaceHandle;

struct WindowRecord {
    native: NativeWindow,
    state: WindowsWindowState,
}

/// Thread-affine, zero-widget Win32 host.
///
/// `prepare` validates the complete transaction and stages newly opened HWNDs
/// while they are hidden, so Graphics can prepare pixels against a real target.
/// `commit` reconciles existing native state while suppressing re-entrant
/// events and publishes visibility only after all fallible updates succeed.
pub struct WindowsPlatformHost {
    hinstance: HINSTANCE,
    windows: BTreeMap<PlatformWindowId, WindowRecord>,
    staged: BTreeMap<PlatformWindowId, NativeWindow>,
    pending: Option<PreparedWindowsTransaction>,
    events: WindowsEventQueue,
    last_committed_revision: Option<PlatformHostRevision>,
    shutdown: bool,
    _thread_affine: PhantomData<Rc<()>>,
}

impl std::fmt::Debug for WindowsPlatformHost {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WindowsPlatformHost")
            .field("window_count", &self.windows.len())
            .field("staged_window_count", &self.staged.len())
            .field(
                "pending_revision",
                &self
                    .pending
                    .as_ref()
                    .map(|pending| pending.transaction.revision),
            )
            .field("last_committed_revision", &self.last_committed_revision)
            .field("event_queue_limit", &self.events.limit())
            .field("shutdown", &self.shutdown)
            .finish_non_exhaustive()
    }
}

impl WindowsPlatformHost {
    /// Creates a host with the default bounded event queue.
    pub fn new() -> GuiResult<Self> {
        Self::with_event_queue_limit(DEFAULT_PLATFORM_HOST_EVENT_QUEUE_LIMIT)
    }

    /// Creates a host with an explicit nonzero event-queue limit.
    pub fn with_event_queue_limit(event_queue_limit: usize) -> GuiResult<Self> {
        if event_queue_limit == 0 {
            return Err(GuiError::host(
                "Windows host event queue limit must be greater than zero",
            ));
        }
        Ok(Self {
            hinstance: register_window_class()?,
            windows: BTreeMap::new(),
            staged: BTreeMap::new(),
            pending: None,
            events: WindowsEventQueue::new(event_queue_limit),
            last_committed_revision: None,
            shutdown: false,
            _thread_affine: PhantomData,
        })
    }

    /// Returns the number of committed native top-level windows.
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Returns the number of hidden HWNDs retained by a pending transaction.
    pub fn staged_window_count(&self) -> usize {
        self.staged.len()
    }

    /// Returns the DPI scale used to plan a default-located first window.
    pub fn initial_scale_factor(&self) -> GuiResult<f64> {
        self.ensure_running()?;
        system_scale_factor()
    }

    /// Returns the committed logical specification for `window`.
    pub fn window_spec(&self, window: PlatformWindowId) -> Option<&PlatformWindowSpec> {
        self.windows.get(&window).map(|record| &record.state.spec)
    }

    /// Returns the latest committed semantic snapshot retained for `window`.
    ///
    /// H2 does not publish this snapshot through UI Automation yet.
    pub fn accessibility_snapshot(
        &self,
        window: PlatformWindowId,
    ) -> Option<&PlatformAccessibilitySnapshot> {
        self.windows
            .get(&window)
            .and_then(|record| record.state.accessibility.as_ref())
    }

    /// Returns the latest successfully committed host revision.
    pub const fn last_committed_revision(&self) -> Option<PlatformHostRevision> {
        self.last_committed_revision
    }

    /// Returns the revision currently prepared for commit, if any.
    pub fn pending_revision(&self) -> Option<PlatformHostRevision> {
        self.pending
            .as_ref()
            .map(|pending| pending.transaction.revision)
    }

    /// Reports whether all native resources were shut down successfully.
    pub const fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    /// Leases the committed HWND/HINSTANCE pair for Graphics surface
    /// attachment. A committed close is rejected while the lease remains live.
    pub fn surface(&self, window: PlatformWindowId) -> GuiResult<WindowsSurfaceHandle> {
        self.ensure_running()?;
        let record = self.windows.get(&window).ok_or_else(|| {
            GuiError::host(format!(
                "Windows host window {} has no committed surface",
                window.get()
            ))
        })?;
        Self::surface_handle(window, &record.native)
    }

    fn surface_handle(
        window: PlatformWindowId,
        native: &NativeWindow,
    ) -> GuiResult<WindowsSurfaceHandle> {
        Ok(WindowsSurfaceHandle::new(
            window,
            native.surface_state(),
            native.physical_size()?,
            native.scale_factor()?,
        ))
    }

    fn ensure_running(&self) -> GuiResult<()> {
        if self.shutdown {
            Err(GuiError::host("Windows platform host is shut down"))
        } else {
            Ok(())
        }
    }

    fn pop_event(&mut self) -> GuiResult<Option<PlatformHostEvent>> {
        let event = self.events.pop()?;
        if let Some(PlatformHostEvent::Window {
            event:
                PlatformWindowEvent::Resized {
                    window,
                    logical_size,
                },
        }) = &event
        {
            if let Some(record) = self.windows.get_mut(window) {
                record.state.spec.logical_size = *logical_size;
                record.native.observe_logical_size(*logical_size);
            }
        }
        Ok(event)
    }

    fn current_state(&self) -> BTreeMap<PlatformWindowId, WindowsWindowState> {
        self.windows
            .iter()
            .map(|(id, record)| (*id, record.state.clone()))
            .collect()
    }

    fn apply_prepared(&mut self, prepared: &PreparedWindowsTransaction) -> GuiResult<()> {
        let mut originals = BTreeMap::<PlatformWindowId, PlatformWindowSpec>::new();

        for window in &prepared.opened {
            if !self.staged.contains_key(window) {
                return Err(GuiError::host("Windows host lost a staged native window"));
            }
        }

        for (window, desired) in &prepared.desired {
            if prepared.opened.contains(window) {
                continue;
            }
            let Some(record) = self.windows.get_mut(window) else {
                return Err(self.restore_after_failure(
                    GuiError::host("Windows host lost a prepared native window"),
                    &originals,
                ));
            };
            if record.state.spec == desired.spec {
                continue;
            }
            originals.insert(*window, record.state.spec.clone());
            if native_properties_differ(&record.state.spec, &desired.spec) {
                if let Err(error) = record.native.apply_spec_properties(&desired.spec) {
                    return Err(self.restore_after_failure(error, &originals));
                }
            }
        }

        for window in &prepared.redraw {
            let result = if let Some(native) = self.staged.get(window) {
                native.invalidate()
            } else if let Some(record) = self.windows.get(window) {
                record.native.invalidate()
            } else {
                Err(GuiError::host(
                    "Windows host lost a prepared redraw surface",
                ))
            };
            if let Err(error) = result {
                return Err(self.restore_after_failure(error, &originals));
            }
        }

        if let Some(window) = prepared.closed.first() {
            let destroy_result = self
                .windows
                .get_mut(window)
                .ok_or_else(|| GuiError::host("Windows host lost a prepared close window"))
                .and_then(|record| record.native.destroy());
            if let Err(error) = destroy_result {
                return Err(self.restore_after_failure(error, &originals));
            }
        }

        for (window, desired) in &prepared.desired {
            if let Some(native) = self.staged.get_mut(window) {
                native.set_visible(desired.spec.visible);
            } else if let Some(record) = self.windows.get_mut(window) {
                if record.state.spec.visible != desired.spec.visible {
                    record.native.set_visible(desired.spec.visible);
                }
            }
        }

        for window in &prepared.closed {
            self.windows.remove(window);
        }
        for (window, desired) in &prepared.desired {
            if prepared.opened.contains(window) {
                if let Some(native) = self.staged.remove(window) {
                    self.windows.insert(
                        *window,
                        WindowRecord {
                            native,
                            state: desired.clone(),
                        },
                    );
                }
            } else if let Some(record) = self.windows.get_mut(window) {
                record.state = desired.clone();
            }
        }
        Ok(())
    }

    fn restore_after_failure(
        &mut self,
        primary: GuiError,
        originals: &BTreeMap<PlatformWindowId, PlatformWindowSpec>,
    ) -> GuiError {
        let mut failures = Vec::new();
        for (window, original) in originals {
            if let Some(record) = self.windows.get_mut(window) {
                if let Err(error) = record.native.restore_spec(original) {
                    failures.push(format!("window {}: {error}", window.get()));
                }
            }
        }
        for native in self.staged.values_mut() {
            native.set_visible(false);
        }
        if failures.is_empty() {
            primary
        } else {
            GuiError::host(format!(
                "{primary}; Windows host rollback also failed: {}",
                failures.join("; ")
            ))
        }
    }

    fn commit_ack(prepared: &PreparedWindowsTransaction) -> GuiResult<PlatformHostCommitAck> {
        let presentations = prepared
            .transaction
            .commands
            .iter()
            .filter_map(|command| match command {
                PlatformHostCommand::Present { request } => Some(PlatformPresentationAck {
                    revision: prepared.transaction.revision,
                    window: request.window,
                    status: PlatformPresentationStatus::Queued,
                }),
                _ => None,
            })
            .collect();
        let ack = PlatformHostCommitAck {
            revision: prepared.transaction.revision,
            applied_commands: prepared.transaction.commands.len(),
            presentations,
        };
        ack.validate()?;
        Ok(ack)
    }
}

impl PlatformHost for WindowsPlatformHost {
    type PresentationTarget = WindowsSurfaceHandle;

    fn prepare(&mut self, transaction: PlatformHostTransaction) -> GuiResult<()> {
        self.ensure_running()?;
        if self.pending.is_some() {
            return Err(GuiError::host(
                "Windows host already has a pending transaction",
            ));
        }
        transaction.validate()?;
        if self
            .last_committed_revision
            .is_some_and(|revision| transaction.revision <= revision)
        {
            return Err(GuiError::host(format!(
                "Windows host revision {} must be newer than committed revision {}",
                transaction.revision.get(),
                self.last_committed_revision
                    .map(PlatformHostRevision::get)
                    .unwrap_or_default()
            )));
        }
        if !self.staged.is_empty() {
            return Err(GuiError::host(
                "Windows host retained staged windows without a pending transaction",
            ));
        }
        let prepared = plan_transaction(transaction, self.current_state())?;
        self.stage_opened(&prepared)?;
        self.pending = Some(prepared);
        Ok(())
    }

    fn presentation_target(&self, window: PlatformWindowId) -> GuiResult<Self::PresentationTarget> {
        self.ensure_running()?;
        let pending = self.pending.as_ref().ok_or_else(|| {
            GuiError::host("Windows host has no pending presentation transaction")
        })?;
        let requested = pending.transaction.commands.iter().any(|command| {
            matches!(
                command,
                PlatformHostCommand::Present { request } if request.window == window
            )
        });
        if !requested {
            return Err(GuiError::host(format!(
                "Windows host transaction has no presentation for window {}",
                window.get()
            )));
        }
        if let Some(native) = self.staged.get(&window) {
            return Self::surface_handle(window, native);
        }
        let record = self.windows.get(&window).ok_or_else(|| {
            GuiError::host(format!(
                "Windows host window {} has no presentation surface",
                window.get()
            ))
        })?;
        Self::surface_handle(window, &record.native)
    }

    fn commit(&mut self) -> GuiResult<PlatformHostCommitAck> {
        self.ensure_running()?;
        let prepared =
            self.pending.as_ref().cloned().ok_or_else(|| {
                GuiError::host("Windows host has no pending transaction to commit")
            })?;
        let ack = Self::commit_ack(&prepared)?;
        self.events.set_suppressed(true);
        let result = self.apply_prepared(&prepared);
        self.events.set_suppressed(false);
        result?;
        self.pending = None;
        self.last_committed_revision = Some(prepared.transaction.revision);
        for window in &prepared.closed {
            self.events.push(PlatformHostEvent::Window {
                event: PlatformWindowEvent::Closed { window: *window },
            });
        }
        Ok(ack)
    }

    fn rollback(&mut self) -> GuiResult<()> {
        self.ensure_running()?;
        self.destroy_staged()?;
        self.pending = None;
        Ok(())
    }

    fn poll_event(&mut self) -> GuiResult<Option<PlatformHostEvent>> {
        self.ensure_running()?;
        if let Some(event) = self.pop_event()? {
            return Ok(Some(event));
        }
        pump_messages(&self.events);
        self.pop_event()
    }

    fn shutdown(&mut self) -> GuiResult<()> {
        if self.shutdown {
            return Ok(());
        }
        if self.pending.is_some() {
            return Err(GuiError::host(
                "Windows host cannot shut down with a pending transaction",
            ));
        }
        self.events.set_suppressed(true);
        let windows = self.windows.keys().copied().collect::<Vec<_>>();
        let mut failures = Vec::new();
        for window in windows {
            let result = self
                .windows
                .get_mut(&window)
                .map_or(Ok(()), |record| record.native.destroy());
            match result {
                Ok(()) => {
                    self.windows.remove(&window);
                }
                Err(error) => failures.push(format!("window {}: {error}", window.get())),
            }
        }
        self.events.set_suppressed(false);
        if !failures.is_empty() {
            return Err(GuiError::host(format!(
                "Windows host shutdown failed: {}",
                failures.join("; ")
            )));
        }
        self.events.clear();
        self.shutdown = true;
        Ok(())
    }
}

impl Drop for WindowsPlatformHost {
    fn drop(&mut self) {
        self.events.set_suppressed(true);
        for native in self.staged.values_mut() {
            let _ = native.destroy();
        }
        self.staged.clear();
        for record in self.windows.values_mut() {
            let _ = record.native.destroy();
        }
        self.windows.clear();
        self.events.clear();
    }
}

fn native_properties_differ(left: &PlatformWindowSpec, right: &PlatformWindowSpec) -> bool {
    left.title != right.title
        || left.logical_size != right.logical_size
        || left.min_size != right.min_size
        || left.max_size != right.max_size
        || left.resizable != right.resizable
}
