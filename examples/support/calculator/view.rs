use a3s_gui::{ComponentCx, GuiResult, RsxComponent, WindowOptions};

use super::{components, model::CalculatorState};

pub fn calculator_component(
    frame_id: &str,
    title: &str,
) -> GuiResult<RsxComponent<CalculatorState>> {
    Ok(ComponentCx::compile(frame_id, components::calculator)?
        .try_register(components::with_calculator_components)?
        .with_window(WindowOptions {
            title: title.to_string(),
            on_close: None,
            width: Some(410.0),
            height: Some(620.0),
            min_width: Some(360.0),
            min_height: Some(560.0),
            max_width: None,
            max_height: None,
            resizable: true,
        }))
}
