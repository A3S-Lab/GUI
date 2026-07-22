use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UsePressProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiPressableProps {
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub on_press_up: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub class_name: String,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiPressableProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            on_press_up: None,
            is_disabled: false,
            is_pressed: false,
            class_name: String::new(),
            action_value: String::new(),
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_pressable(cx: &mut ComponentCx<UiPressableProps>) -> RSX {
    cx.use_press(|props: &UiPressableProps| {
        UsePressProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .on_press_up(props.on_press_up.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiPressableProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.pressProps}
            data-slot="pressable"
            data-pressed={props.isPressed}
            class="outline-none disabled:pointer-events-none disabled:text-muted-soft focus-visible:ring-[2px] focus-visible:ring-ink/40"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
