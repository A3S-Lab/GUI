use crate::error::{GuiError, GuiResult};
use crate::platform_host::{
    PlatformHostCommitAck, PlatformHostRevision, PlatformPresentationRequest,
    PlatformPresentationStatus, PlatformWindowId,
};

use super::SelfDrawnFrameSnapshot;

pub(super) fn next_revision(
    previous: Option<&SelfDrawnFrameSnapshot>,
) -> GuiResult<PlatformHostRevision> {
    let value = previous
        .map(SelfDrawnFrameSnapshot::revision)
        .map(PlatformHostRevision::get)
        .unwrap_or_default()
        .checked_add(1)
        .ok_or_else(|| GuiError::host("self-drawn frame revision overflowed"))?;
    Ok(PlatformHostRevision::new(value))
}

pub(super) fn presentation_request(
    snapshot: &SelfDrawnFrameSnapshot,
) -> PlatformPresentationRequest {
    PlatformPresentationRequest {
        window: snapshot.window(),
        logical_size: snapshot.logical_size(),
        scale_factor: snapshot.scale_factor(),
        scene_fingerprint: snapshot.scene_fingerprint(),
        damage: snapshot.damage().to_vec(),
    }
}

pub(super) fn validate_ack(
    ack: &PlatformHostCommitAck,
    revision: PlatformHostRevision,
    command_count: usize,
    presentation_window: Option<PlatformWindowId>,
) -> GuiResult<()> {
    ack.validate()?;
    if ack.revision != revision || ack.applied_commands != command_count {
        return Err(GuiError::host(
            "platform host commit acknowledgement does not match the prepared frame",
        ));
    }
    match (presentation_window, ack.presentations.as_slice()) {
        (None, []) => {}
        (Some(window), [presentation])
            if presentation.window == window
                && presentation.status == PlatformPresentationStatus::Queued => {}
        _ => {
            return Err(GuiError::host(
                "platform host presentation acknowledgement does not match the prepared frame",
            ));
        }
    }
    Ok(())
}

pub(super) fn rollback_staged_surface(
    surface_cleanup: Option<GuiResult<()>>,
    rollback: GuiResult<()>,
) -> GuiResult<()> {
    let mut failures = Vec::new();
    if let Some(Err(error)) = surface_cleanup {
        failures.push(format!("scene presenter cleanup failed: {error}"));
    }
    if let Err(error) = rollback {
        failures.push(format!("platform host rollback failed: {error}"));
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(GuiError::host(failures.join("; ")))
    }
}

pub(super) fn rollback_after_staged_surface_error<T>(
    error: GuiError,
    surface_cleanup: Option<GuiResult<()>>,
    rollback: GuiResult<()>,
) -> GuiResult<T> {
    match rollback_staged_surface(surface_cleanup, rollback) {
        Ok(()) => Err(error),
        Err(cleanup_error) => Err(GuiError::host(format!("{error}; {cleanup_error}"))),
    }
}

pub(super) fn rollback_after_commit_error<T>(
    error: GuiError,
    rollback: GuiResult<()>,
) -> GuiResult<T> {
    match rollback {
        Ok(()) => Err(error),
        Err(rollback_error) => Err(GuiError::host(format!(
            "{error}; platform host rollback also failed: {rollback_error}"
        ))),
    }
}
