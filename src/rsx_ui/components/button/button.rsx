use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseButtonProps;

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
        UseButtonProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiButtonProps| {
        props.class_name.clone()
    });
    crate::rsx!(
        <button
            key="root"
            {...props.buttonProps}
            data-slot="button"
            data-pressed={props.isPressed}
            class="inline-flex h-9 items-center justify-center gap-2 whitespace-nowrap rounded-md px-3 py-1.5 text-sm font-medium leading-none disabled:pointer-events-none disabled:text-muted-soft [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:ring-[2px] focus-visible:ring-ink/40 aria-invalid:border-semantic-error"
            className={props.className}
        >
            <Slot key="content" />
        </button>
    )
}
