use a3s_gui::{ComponentCx, GuiResult, RsxComponent, WindowOptions};

use super::{components, model::ComponentPlaygroundState};

pub fn component_playground_component(
    frame_id: &str,
    title: &str,
) -> GuiResult<RsxComponent<ComponentPlaygroundState>> {
    Ok(
        ComponentCx::compile(frame_id, components::component_playground)?
            .try_register(components::with_component_playground_components)?
            .with_window(WindowOptions {
                title: title.to_string(),
                on_close: None,
                width: Some(1180.0),
                height: Some(860.0),
                min_width: Some(920.0),
                min_height: Some(680.0),
                max_width: None,
                max_height: None,
                resizable: true,
            }),
    )
}
