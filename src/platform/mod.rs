mod adapters;
mod config;
mod planning;
#[cfg(test)]
mod tests;
mod types;
mod widget_names;

pub use adapters::{AppKitAdapter, BlueprintHost, Gtk4Adapter, PlatformAdapter, WinUiAdapter};
pub use config::{
    apply_widget_setter, apply_widget_setters, push_widget_setter_history, NativeConfigValueChange,
    NativeWidgetConfig, NativeWidgetConfigPatch, NativeWidgetReplacement, NativeWidgetSetter,
    NativeWidgetSetterBatch, DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
};
pub(crate) use planning::PlatformPlanningCheckpoint;
pub use planning::{PlatformCommand, PlatformPlannedNode, PlatformPlanningHost};
pub use types::{
    NativeBackendKind, NativeContainerKind, NativeControlState, NativeTextInputHints,
    NativeTextInputKind, NativeTextInputPurpose, NativeWidgetBlueprint, NativeWidgetKind,
};
pub use widget_names::{native_widget_kind, native_widget_name, widget_blueprint};
