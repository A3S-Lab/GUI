use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseToggleButtonProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiToggleButtonProps {
    pub class_name: String,
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub on_press_up: Option<String>,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub action_value: String,
    pub action_payload: JsonValue,
}

pub fn ui_toggle_button(cx: &mut ComponentCx<UiToggleButtonProps>) -> RSX {
    cx.use_toggle_button(|props: &UiToggleButtonProps| {
        UseToggleButtonProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .on_press_up(props.on_press_up.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .selected(props.is_selected)
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiToggleButtonProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <button
            key="root"
            {...props.toggleButtonProps}
            data-slot="toggle-button"
            class="inline-flex h-9 items-center justify-center gap-2 rounded-md border border-transparent bg-transparent px-3 text-sm font-medium text-ink outline-none active:bg-surface-strong focus-visible:border-ink focus-visible:ring-[2px] focus-visible:ring-ink/40 disabled:pointer-events-none disabled:text-muted-soft data-[selected=true]:bg-surface-card data-[selected=true]:text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </button>
    )
}
