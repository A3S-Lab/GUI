use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseColorThumbProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiColorThumbProps {
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_dragging: bool,
    pub class_name: String,
    pub value: String,
    pub x_value: f64,
    pub y_value: f64,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiColorThumbProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            is_disabled: false,
            is_pressed: false,
            is_dragging: false,
            class_name: String::new(),
            value: String::new(),
            x_value: 0.0,
            y_value: 0.0,
            action_value: String::new(),
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_color_thumb(cx: &mut ComponentCx<UiColorThumbProps>) -> RSX {
    cx.use_color_thumb(|props: &UiColorThumbProps| {
        UseColorThumbProps::new()
            .value(Some(props.value.clone()))
            .x_value(props.x_value)
            .y_value(props.y_value)
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
            .dragging(props.is_dragging)
    });
    cx.use_prop("className", |props: &UiColorThumbProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <button
            key="root"
            {...props.colorThumbProps}
            data-slot="color-thumb"
            class="absolute size-4 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-canvas bg-canvas ring-1 ring-hairline-strong transition-colors disabled:pointer-events-none disabled:opacity-50 focus-visible:ring-[3px] focus-visible:ring-ring/50"
            className={props.className}
        >
            <Slot key="content" />
        </button>
    )
}
