use crate::error::GuiResult;
use crate::rsx_app::ComponentRegistry;

/// Facade between the RSX application runtime and the optional default
/// design-system definitions.
#[cfg(feature = "design-system")]
pub(crate) fn registry() -> GuiResult<ComponentRegistry> {
    crate::rsx_ui::builtin_component_registry()
}

/// Authoring-only builds deliberately start with an empty registry. Applications
/// can inject their own compiled definitions through the explicit registry API.
#[cfg(not(feature = "design-system"))]
pub(crate) fn registry() -> GuiResult<ComponentRegistry> {
    Ok(ComponentRegistry::new())
}
