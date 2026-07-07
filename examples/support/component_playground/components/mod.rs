#[path = "app.rsx"]
mod app;
#[path = "collections_panel.rsx"]
mod collections_panel;
#[path = "controls_panel.rsx"]
mod controls_panel;
#[path = "data_panel.rsx"]
mod data_panel;
#[path = "date_color_range_panel.rsx"]
mod date_color_range_panel;
#[path = "foundation_panel.rsx"]
mod foundation_panel;
#[path = "overlays_feedback_panel.rsx"]
mod overlays_feedback_panel;
#[path = "section_frame.rsx"]
mod section_frame;
#[path = "shell.rsx"]
mod shell;

use a3s_gui::{ComponentCx, GuiResult, RsxComponent, RsxComponentContract, RSX};

pub use app::component_playground;

#[cfg(test)]
pub(super) const PLAYGROUND_RSX_SOURCES: &[&str] = &[
    include_str!("app.rsx"),
    include_str!("shell.rsx"),
    include_str!("section_frame.rsx"),
    include_str!("foundation_panel.rsx"),
    include_str!("controls_panel.rsx"),
    include_str!("collections_panel.rsx"),
    include_str!("data_panel.rsx"),
    include_str!("date_color_range_panel.rsx"),
    include_str!("overlays_feedback_panel.rsx"),
];

pub fn with_component_playground_components<S: 'static>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_template(
        component,
        "PlaygroundShell",
        shell::playground_shell,
        RsxComponentContract::new().required([
            "interactionCount",
            "lastEvent",
            "query",
            "selectedValue",
            "activeSection",
            "foundationActive",
            "controlsActive",
            "collectionsActive",
            "dataActive",
            "dateColorRangeActive",
            "overlaysFeedbackActive",
            "overlayOpen",
            "record",
            "setValue",
            "setSection",
            "openOverlay",
            "closeOverlay",
        ]),
    )?;
    let component = with_template(
        component,
        "PlaygroundSection",
        section_frame::playground_section,
        RsxComponentContract::new()
            .required(["title", "description"])
            .default_prop("className", "")?,
    )?;
    let component = with_template(
        component,
        "FoundationPanel",
        foundation_panel::foundation_panel,
        RsxComponentContract::new().required(["record", "setValue"]),
    )?;
    let component = with_template(
        component,
        "ControlsPanel",
        controls_panel::controls_panel,
        RsxComponentContract::new().required(["record", "setValue", "selectedValue"]),
    )?;
    let component = with_template(
        component,
        "CollectionsPanel",
        collections_panel::collections_panel,
        RsxComponentContract::new().required(["record", "setValue", "selectedValue"]),
    )?;
    let component = with_template(
        component,
        "DataPanel",
        data_panel::data_panel,
        RsxComponentContract::new().required(["record", "setValue", "selectedValue"]),
    )?;
    let component = with_template(
        component,
        "DateColorRangePanel",
        date_color_range_panel::date_color_range_panel,
        RsxComponentContract::new().required(["record", "setValue"]),
    )?;
    with_template(
        component,
        "OverlaysFeedbackPanel",
        overlays_feedback_panel::overlays_feedback_panel,
        RsxComponentContract::new().required([
            "record",
            "setValue",
            "openOverlay",
            "closeOverlay",
            "overlayOpen",
        ]),
    )
}

fn with_template<S, P, F>(
    component: RsxComponent<S>,
    name: &str,
    render: F,
    contract: RsxComponentContract,
) -> GuiResult<RsxComponent<S>>
where
    P: 'static,
    F: FnOnce(&mut ComponentCx<P>) -> RSX,
{
    let template = ComponentCx::compile(name, render)?;
    component.use_template_component_with_contract(name, template.template().clone(), contract)
}
