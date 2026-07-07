use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseButtonProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiColumnResizerProps {
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_resizing: bool,
    pub class_name: String,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiColumnResizerProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            is_disabled: false,
            is_pressed: false,
            is_resizing: false,
            class_name: String::new(),
            value_number: 0.0,
            min_value: 0.0,
            max_value: 0.0,
            action_value: String::new(),
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_column_resizer(cx: &mut ComponentCx<UiColumnResizerProps>) -> RSX {
    cx.use_button(|props: &UiColumnResizerProps| {
        UseButtonProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiColumnResizerProps| {
        props.class_name.clone()
    });
    cx.use_prop("valueNumber", |props: &UiColumnResizerProps| {
        props.value_number
    });
    cx.use_prop("minValue", |props: &UiColumnResizerProps| props.min_value);
    cx.use_prop("maxValue", |props: &UiColumnResizerProps| props.max_value);
    cx.use_prop("isResizing", |props: &UiColumnResizerProps| {
        props.is_resizing
    });

    crate::rsx!(
        <button
            key="root"
            {...props.buttonProps}
            data-slot="column-resizer"
            data-resizing={props.isResizing}
            data-value-number={props.valueNumber}
            data-min-value={props.minValue}
            data-max-value={props.maxValue}
            data-pressed={props.isPressed}
            class="absolute right-0 top-0 h-full w-2 cursor-col-resize touch-none bg-transparent outline-none transition-colors hover:bg-hairline focus-visible:bg-ring disabled:pointer-events-none disabled:opacity-50 data-[resizing=true]:bg-ring"
            className={props.className}
        >
            <Slot key="content" />
        </button>
    )
}
