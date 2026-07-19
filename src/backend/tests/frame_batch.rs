use crate::backend::{
    CommandExecutingHost, PlatformBatchAck, PlatformCommandBatch, PlatformCommandExecutor,
    RecordingBackend, DEFAULT_RECORDING_COMMAND_HISTORY_LIMIT,
};
use crate::error::{GuiError, GuiResult};
use crate::host::NativeHost;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformCommand};
use crate::renderer::Renderer;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug, Default)]
struct BatchProbeExecutor {
    fail_at: Option<usize>,
    reject_prepare: bool,
    prepared: Vec<PlatformCommandBatch>,
    applied: Vec<PlatformCommand>,
}

impl PlatformCommandExecutor for BatchProbeExecutor {
    fn prepare_batch(&mut self, batch: &PlatformCommandBatch) -> GuiResult<()> {
        self.prepared.push(batch.clone());
        if self.reject_prepare {
            Err(GuiError::host("forced batch prepare failure"))
        } else {
            Ok(())
        }
    }

    fn execute(&mut self, command: &PlatformCommand) -> GuiResult<()> {
        if self.fail_at == Some(self.applied.len()) {
            return Err(GuiError::host("forced batch commit failure"));
        }
        self.applied.push(command.clone());
        Ok(())
    }
}

#[derive(Debug, Default)]
struct InvalidAckExecutor {
    applied: Vec<PlatformCommand>,
}

impl PlatformCommandExecutor for InvalidAckExecutor {
    fn execute(&mut self, command: &PlatformCommand) -> GuiResult<()> {
        self.applied.push(command.clone());
        Ok(())
    }

    fn commit_batch(
        &mut self,
        batch: &PlatformCommandBatch,
    ) -> Result<PlatformBatchAck, crate::backend::PlatformBatchFailure> {
        self.applied.extend(batch.commands.iter().cloned());
        Ok(PlatformBatchAck {
            batch_id: batch.id,
            prepared_commands: batch.commands.len(),
            applied_commands: batch.commands.len().saturating_sub(1),
        })
    }
}

#[derive(Debug)]
struct DropProbeExecutor {
    old_dropped: Rc<Cell<bool>>,
    replacement_saw_drop: Rc<Cell<bool>>,
    fail_execute: bool,
    require_old_dropped_on_prepare: bool,
}

impl Drop for DropProbeExecutor {
    fn drop(&mut self) {
        self.old_dropped.set(true);
    }
}

impl PlatformCommandExecutor for DropProbeExecutor {
    fn prepare_batch(&mut self, _batch: &PlatformCommandBatch) -> GuiResult<()> {
        if !self.require_old_dropped_on_prepare {
            return Ok(());
        }
        self.replacement_saw_drop.set(self.old_dropped.get());
        if self.old_dropped.get() {
            Ok(())
        } else {
            Err(GuiError::host(
                "replacement prepared before old executor was dropped",
            ))
        }
    }

    fn execute(&mut self, _command: &PlatformCommand) -> GuiResult<()> {
        if self.fail_execute {
            Err(GuiError::host("forced old executor failure"))
        } else {
            Ok(())
        }
    }
}

#[test]
fn failed_commit_preserves_queue_rolls_back_planning_and_requires_replay() {
    let mut host = CommandExecutingHost::new(
        Gtk4Adapter,
        BatchProbeExecutor {
            fail_at: Some(1),
            ..BatchProbeExecutor::default()
        },
    );

    host.begin_frame().unwrap();
    let id = host
        .create(&NativeElement::new("save", NativeRole::Button))
        .unwrap();
    host.set_root(id).unwrap();
    let queued = host.planning().commands().to_vec();

    let error = host.commit_frame().unwrap_err();

    assert!(error.to_string().contains("partially committed"));
    assert_eq!(host.planning().commands(), queued);
    assert_eq!(host.planning().root(), Some(id));
    let degraded = host.degraded_state().unwrap();
    assert_eq!(degraded.batch.commands, queued);
    assert_eq!(degraded.applied_commands, 1);
    assert_eq!(degraded.failed_command, 1);

    let error = host
        .create(&NativeElement::new("blocked", NativeRole::Button))
        .unwrap_err();
    assert!(error.to_string().contains("degraded"));

    host.rollback_frame().unwrap();
    assert!(host.planning().commands().is_empty());
    assert!(host.planning().nodes().is_empty());
    assert!(host.planning().root().is_none());
    assert!(host.is_degraded());

    let recovery = host
        .recover_with_executor(BatchProbeExecutor::default())
        .unwrap();
    assert_eq!(
        recovery,
        PlatformBatchAck {
            batch_id: recovery.batch_id,
            prepared_commands: 0,
            applied_commands: 0,
        }
    );
    assert!(!host.is_degraded());

    host.begin_frame().unwrap();
    let recovered = host
        .create(&NativeElement::new("save", NativeRole::Button))
        .unwrap();
    host.set_root(recovered).unwrap();
    let ack = host.commit_frame().unwrap();
    assert_eq!(ack.applied_operations, 2);
    assert!(host.planning().commands().is_empty());
}

#[test]
fn rejected_prepare_keeps_exact_queue_without_degrading_native_state() {
    let mut host = CommandExecutingHost::new(
        Gtk4Adapter,
        BatchProbeExecutor {
            reject_prepare: true,
            ..BatchProbeExecutor::default()
        },
    );

    host.begin_frame().unwrap();
    let id = host
        .create(&NativeElement::new("save", NativeRole::Button))
        .unwrap();
    host.set_root(id).unwrap();
    let queued = host.planning().commands().to_vec();

    let error = host.commit_frame().unwrap_err();

    assert!(error.to_string().contains("prepare"));
    assert_eq!(host.planning().commands(), queued);
    assert!(host.executor().applied.is_empty());
    assert!(!host.is_degraded());
    host.rollback_frame().unwrap();
    assert!(host.planning().commands().is_empty());
    assert!(host.planning().nodes().is_empty());
}

#[test]
fn invalid_ack_preserves_failed_batch_and_degrades_execution() {
    let mut host = CommandExecutingHost::new(Gtk4Adapter, InvalidAckExecutor::default());
    host.begin_frame().unwrap();
    let id = host
        .create(&NativeElement::new("save", NativeRole::Button))
        .unwrap();
    host.set_root(id).unwrap();
    let queued = host.planning().commands().to_vec();

    let error = host.commit_frame().unwrap_err();

    assert!(error.to_string().contains("invalid acknowledgement"));
    assert_eq!(host.planning().commands(), queued);
    assert_eq!(host.executor().applied, queued);
    let degraded = host.degraded_state().unwrap();
    assert_eq!(degraded.batch.commands, queued);
    assert_eq!(degraded.applied_commands, 1);
    host.rollback_frame().unwrap();
    assert!(host.planning().commands().is_empty());
    assert!(host.is_degraded());
}

#[test]
fn fresh_executor_replays_nonempty_snapshot_before_resuming_incremental_frames() {
    let mut renderer = Renderer::new();
    let mut host = CommandExecutingHost::new(Gtk4Adapter, BatchProbeExecutor::default());
    let old = NativeElement::new("status", NativeRole::Button)
        .with_props(NativeProps::new().label("Old"));
    let new = NativeElement::new("status", NativeRole::Button)
        .with_props(NativeProps::new().label("New"));
    let root = renderer.render(&old, &mut host).unwrap();

    let applied_before_failure = host.executor().applied.len();
    host.executor_mut().fail_at = Some(applied_before_failure + 1);
    let error = renderer.render(&new, &mut host).unwrap_err();

    assert!(error.to_string().contains("partially committed"));
    assert!(host.is_degraded());
    assert_eq!(
        host.planning()
            .node(root)
            .unwrap()
            .blueprint
            .label
            .as_deref(),
        Some("Old")
    );
    assert!(host.planning().commands().is_empty());

    let recovery = host
        .recover_with_executor(BatchProbeExecutor::default())
        .unwrap();

    assert_eq!(recovery.prepared_commands, 2);
    assert_eq!(recovery.applied_commands, 2);
    assert!(!host.is_degraded());
    assert!(matches!(
        &host.executor().applied[0],
        PlatformCommand::Create { id, blueprint }
            if *id == root && blueprint.label.as_deref() == Some("Old")
    ));
    assert_eq!(
        host.executor().applied[1],
        PlatformCommand::SetRoot { id: root }
    );

    let rerendered = renderer.render(&new, &mut host).unwrap();
    assert_eq!(rerendered, root);
    assert_eq!(
        host.planning()
            .node(root)
            .unwrap()
            .blueprint
            .label
            .as_deref(),
        Some("New")
    );
    assert!(host.planning().commands().is_empty());
}

#[test]
fn recovery_drops_partial_executor_before_preparing_replacement() {
    let old_dropped = Rc::new(Cell::new(false));
    let replacement_saw_drop = Rc::new(Cell::new(false));
    let old = DropProbeExecutor {
        old_dropped: old_dropped.clone(),
        replacement_saw_drop: replacement_saw_drop.clone(),
        fail_execute: true,
        require_old_dropped_on_prepare: false,
    };
    let mut renderer = Renderer::new();
    let mut host = CommandExecutingHost::new(Gtk4Adapter, old);
    renderer
        .render(&NativeElement::new("root", NativeRole::View), &mut host)
        .unwrap_err();
    assert!(host.is_degraded());

    let replacement = DropProbeExecutor {
        old_dropped: old_dropped.clone(),
        replacement_saw_drop: replacement_saw_drop.clone(),
        fail_execute: false,
        require_old_dropped_on_prepare: true,
    };
    host.recover_with_executor(replacement).unwrap();

    assert!(old_dropped.get());
    assert!(replacement_saw_drop.get());
    assert!(!host.is_degraded());
}

#[test]
fn long_running_frames_acknowledge_planning_and_bound_diagnostic_history() {
    let mut renderer = Renderer::new();
    let mut host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());

    for revision in 0..2_000 {
        renderer
            .render(
                &NativeElement::new("status", NativeRole::Button)
                    .with_props(NativeProps::new().label(format!("Revision {revision}"))),
                &mut host,
            )
            .unwrap();
        assert!(host.planning().commands().is_empty());
    }

    assert!(host.last_ack().is_some());
    assert_eq!(
        host.executor().commands().len(),
        DEFAULT_RECORDING_COMMAND_HISTORY_LIMIT
    );
    assert_eq!(host.planning().nodes().len(), 1);
}
