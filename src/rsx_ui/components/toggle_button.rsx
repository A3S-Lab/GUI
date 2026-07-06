use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UsePressProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiToggleButtonProps {
    pub class_name: String,
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub action_value: String,
    pub action_payload: JsonValue,
}

pub fn ui_toggle_button(cx: &mut ComponentCx<UiToggleButtonProps>) -> RSX {
    cx.use_button(|props: &UiToggleButtonProps| {
        UsePressProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiToggleButtonProps| {
        props.class_name.clone()
    });
    cx.use_prop("isSelected", |props: &UiToggleButtonProps| {
        props.is_selected
    });
    cx.use_prop("actionValue", |props: &UiToggleButtonProps| {
        props.action_value.clone()
    });
    cx.use_prop("actionPayload", |props: &UiToggleButtonProps| {
        props.action_payload.clone()
    });

    crate::rsx!(
        <button
            key="root"
            {...props.pressProps}
            data-slot="toggle-button"
            data-selected={props.isSelected}
            data-pressed={props.isPressed}
            class="inline-flex h-9 items-center justify-center gap-2 rounded-md border border-input bg-background px-3 text-sm font-medium shadow-xs outline-none transition-[color,box-shadow] hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground"
            className={props.className}
            selected={props.isSelected}
            aria-pressed={props.isSelected}
            actionValue={props.actionValue}
            actionPayload={props.actionPayload}
        >
            <Slot key="content" />
        </button>
    )
}
