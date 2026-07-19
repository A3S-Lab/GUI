mod collections;
mod color;
mod commands;
mod controls;
mod data;
mod date_time;
mod drag_drop;
mod feedback;
mod foundation;
mod layout;
mod overlays;
mod range;
mod router;

use std::sync::LazyLock;

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::compiler::ComponentClassVariants;
use crate::error::GuiResult;
use crate::rsx_app::{ComponentCx, ComponentRegistry, RsxComponent, RsxComponentContract, RSX};

static BUILTIN_COMPONENT_REGISTRY: LazyLock<GuiResult<ComponentRegistry>> =
    LazyLock::new(build_builtin_component_registry);

#[cfg(test)]
static BUILTIN_COMPONENT_REGISTRY_INITIALIZATIONS: AtomicUsize = AtomicUsize::new(0);

/// Returns the process-wide registry of compiled built-in UI components.
///
/// The returned value is a cheap clone backed by the same immutable maps.
pub fn builtin_component_registry() -> GuiResult<ComponentRegistry> {
    match &*BUILTIN_COMPONENT_REGISTRY {
        Ok(registry) => Ok(registry.clone()),
        Err(error) => Err(error.clone()),
    }
}

fn build_builtin_component_registry() -> GuiResult<ComponentRegistry> {
    #[cfg(test)]
    BUILTIN_COMPONENT_REGISTRY_INITIALIZATIONS.fetch_add(1, Ordering::SeqCst);

    let component = RsxComponent::<()>::new_bare("a3s-builtin-component-registry", "<Fragment />")?;
    compile_builtin_components(component).map(RsxComponent::into_component_registry)
}

fn compile_builtin_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = controls::with_input_components(component)?;
    let component = color::with_color_components(component)?;
    let component = foundation::with_foundation_components(component)?;
    let component = controls::with_form_components(component)?;
    let component = layout::with_separator_component(component)?;
    let component = overlays::with_dialog_disclosure_components(component)?;
    let component = date_time::with_date_time_components(component)?;
    let component = collections::with_selection_input_components(component)?;
    let component = overlays::with_popover_tooltip_components(component)?;
    let component = drag_drop::with_drag_drop_components(component)?;
    let component = collections::with_collection_components(component)?;
    let component = collections::with_menu_components(component)?;
    let component = range::with_range_components(component)?;
    let component = router::with_router_components(component)?;
    let component = commands::with_command_components(component)?;
    let component = layout::with_landmark_components(component)?;
    let component = overlays::with_overlay_trigger_components(component)?;
    let component = layout::with_group_component(component)?;
    let component = feedback::with_feedback_components(component)?;
    data::with_data_components(component)
}

fn with_builtin_template<S, P, F>(
    component: RsxComponent<S>,
    name: &str,
    render: F,
    contract: RsxComponentContract,
    variants: Option<ComponentClassVariants>,
) -> GuiResult<RsxComponent<S>>
where
    P: 'static,
    F: FnOnce(&mut ComponentCx<P>) -> RSX,
{
    let template = ComponentCx::compile_bare(name, render)?;
    let component = component.use_template_component_with_contract(
        name,
        template.template().clone(),
        contract,
    )?;
    match variants {
        Some(variants) => component.use_component_class_variants(name, variants),
        None => Ok(component),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_component_registry_is_initialized_once_and_shared() {
        let first = RsxComponent::<()>::new("first", "<Fragment />").unwrap();
        let after_first = BUILTIN_COMPONENT_REGISTRY_INITIALIZATIONS.load(Ordering::SeqCst);
        let second = RsxComponent::<()>::new("second", "<Fragment />").unwrap();
        let after_second = BUILTIN_COMPONENT_REGISTRY_INITIALIZATIONS.load(Ordering::SeqCst);

        assert_eq!(after_first, 1);
        assert_eq!(after_second, after_first);
        assert!(!first.component_registry().is_empty());
        assert!(first
            .component_registry()
            .shares_compiled_definitions_with(second.component_registry()));
    }
}
