use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseButtonProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiSliderThumbProps {
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub on_press_up: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_dragging: bool,
    pub class_name: String,
    pub value_number: f64,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiSliderThumbProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            on_press_up: None,
            is_disabled: false,
            is_pressed: false,
            is_dragging: false,
            class_name: String::new(),
            value_number: 0.0,
            action_value: String::new(),
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_slider_thumb(cx: &mut ComponentCx<UiSliderThumbProps>) -> RSX {
    cx.use_button(|props: &UiSliderThumbProps| {
        UseButtonProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .on_press_up(props.on_press_up.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiSliderThumbProps| {
        props.class_name.clone()
    });
    cx.use_prop("valueNumber", |props: &UiSliderThumbProps| {
        props.value_number
    });
    cx.use_prop("isDragging", |props: &UiSliderThumbProps| props.is_dragging);

    crate::rsx!(
        <button
            key="root"
            {...props.buttonProps}
            data-slot="slider-thumb"
            data-value-number={props.valueNumber}
            data-dragging={props.isDragging}
            data-pressed={props.isPressed}
            class="block size-4 rounded-full border border-primary bg-canvas transition-colors disabled:pointer-events-none disabled:opacity-50 focus-visible:ring-[3px] focus-visible:ring-ring/50"
            className={props.className}
        >
            <Slot key="content" />
        </button>
    )
}
