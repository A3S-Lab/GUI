//! Shared self-drawn window runtime above the zero-widget platform boundary.

mod accessibility;
mod events;
mod frame;
mod input;
mod interaction;
mod interaction_tree;
mod keyboard_input;
mod long_press_input;
#[cfg(test)]
mod long_press_tests;
mod presenter;
#[cfg(feature = "software-reference")]
mod reference_presenter;
mod runtime;

pub use frame::{PlatformRenderFrame, SelfDrawnFrameSnapshot};
pub use interaction::{
    SelfDrawnActionInvocation, SelfDrawnActionPropagation, SelfDrawnElementInteraction,
    SelfDrawnEventContext, SelfDrawnInputDispatch, SelfDrawnInteractionChange,
};
pub use presenter::{
    PlatformScenePresenter, RecordingPreparedFrame, RecordingScenePresenter,
    DEFAULT_RECORDING_SCENE_HISTORY_LIMIT,
};
#[cfg(feature = "software-reference")]
pub use reference_presenter::{
    ReferencePreparedFrame, ReferencePresentedFrame, ReferenceScenePresenter,
};
pub use runtime::{
    SelfDrawnFrameCommit, SelfDrawnFrameCommitStatus, SelfDrawnHostEventOutcome,
    SelfDrawnRuntimeStats, SelfDrawnWindowRuntime,
};

#[cfg(test)]
mod tests;
