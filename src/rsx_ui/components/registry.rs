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

use crate::compiler::ComponentClassVariants;
use crate::error::GuiResult;
use crate::rsx_app::{ComponentCx, RsxComponent, RsxComponentContract, RSX};

pub(crate) fn with_builtin_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
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
