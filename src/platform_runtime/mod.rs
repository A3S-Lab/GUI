//! Shared self-drawn window runtime above the zero-widget platform boundary.

mod accessibility;
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

pub use drag_drop::{SelfDrawnDragContext, SelfDrawnDropItem, SelfDrawnDropOperation};
pub use drag_drop_collection::{SelfDrawnCollectionDropTarget, SelfDrawnDropPosition};
pub use drop_policy::{
    SelfDrawnDropPolicyDecision, SelfDrawnDropPolicyQuery, SelfDrawnDropPolicyRequest,
    SelfDrawnDropPolicyResolution, SelfDrawnDropPolicyResolver, SelfDrawnDropPolicyResponse,
    SelfDrawnDropPolicyTarget,
};
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
