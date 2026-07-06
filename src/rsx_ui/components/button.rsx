use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UsePressProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiButtonProps {
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub class_name: String,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiButtonProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            is_disabled: false,
            is_pressed: false,
            class_name: String::new(),
            action_value: String::new(),
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_button(cx: &mut ComponentCx<UiButtonProps>) -> RSX {
    cx.use_button(|props: &UiButtonProps| {
        UsePressProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiButtonProps| {
        props.class_name.clone()
    });
    cx.use_prop("actionValue", |props: &UiButtonProps| {
        props.action_value.clone()
    });
    cx.use_prop("actionPayload", |props: &UiButtonProps| {
        props.action_payload.clone()
    });

    crate::rsx!(
        <button
            key="root"
            {...props.pressProps}
            data-slot="button"
            data-pressed={props.isPressed}
            class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-[color,box-shadow] disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive"
            className={props.className}
            actionValue={props.actionValue}
            actionPayload={props.actionPayload}
        >
            <Slot key="content" />
        </button>
    )
}
