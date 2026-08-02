//! Shared self-drawn window runtime above the zero-widget platform boundary.

mod accessibility;
#[cfg(test)]
mod deferred_tests;
mod drag_drop;
mod drag_drop_activation_input;
#[cfg(test)]
mod drag_drop_activation_tests;
mod drag_drop_collection;
#[cfg(test)]
mod drag_drop_collection_policy_tests;
#[cfg(test)]
mod drag_drop_collection_tests;
mod drag_drop_input;
#[cfg(test)]
mod drag_drop_items_tests;
mod drag_drop_keyboard_input;
#[cfg(test)]
mod drag_drop_keyboard_tests;
#[cfg(test)]
mod drag_drop_model_tests;
mod drag_drop_source_input;
#[cfg(test)]
mod drag_drop_tests;
mod drop_policy;
mod events;
mod frame;
#[cfg(feature = "gpu")]
mod gpu_presenter;
mod input;
mod interaction;
mod interaction_tree;
mod interaction_tree_collection_policy;
mod interaction_tree_drag_drop;
mod interaction_tree_drag_source;
mod keyboard_input;
mod long_press_input;
#[cfg(test)]
mod long_press_tests;
mod move_input;
#[cfg(test)]
mod move_tests;
mod presenter;
#[cfg(feature = "software-reference")]
mod reference_presenter;
mod runtime;
mod transaction;
#[cfg(test)]
mod transaction_tests;

pub use drag_drop::{SelfDrawnDragContext, SelfDrawnDropItem, SelfDrawnDropOperation};
pub use drag_drop_collection::{SelfDrawnCollectionDropTarget, SelfDrawnDropPosition};
pub use drop_policy::{
    SelfDrawnDropPolicyDecision, SelfDrawnDropPolicyQuery, SelfDrawnDropPolicyRequest,
    SelfDrawnDropPolicyResolution, SelfDrawnDropPolicyResolver, SelfDrawnDropPolicyResponse,
    SelfDrawnDropPolicyTarget,
};
pub use frame::{PlatformRenderFrame, SelfDrawnFrameSnapshot};
#[cfg(feature = "gpu")]
pub use gpu_presenter::{GpuPreparedSceneFrame, GpuPresentedSceneFrame, GpuScenePresenter};
pub use interaction::{
    SelfDrawnActionInvocation, SelfDrawnActionPropagation, SelfDrawnElementInteraction,
    SelfDrawnEventContext, SelfDrawnInputDispatch, SelfDrawnInteractionChange,
};
pub use presenter::{
    PlatformSceneDeferral, PlatformScenePreparation, PlatformScenePresenter,
    PlatformScenePublishStatus, RecordingPreparedFrame, RecordingScenePresenter,
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
