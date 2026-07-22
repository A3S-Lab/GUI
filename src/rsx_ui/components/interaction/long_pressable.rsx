use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLongPressProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiLongPressableProps {
    pub class_name: String,
    pub on_long_press_start: Option<String>,
    pub on_long_press_end: Option<String>,
    pub on_long_press: Option<String>,
    pub action_value: String,
    pub action_payload: JsonValue,
    pub accessibility_description: String,
    pub threshold: u64,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_long_pressed: bool,
}

impl Default for UiLongPressableProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            on_long_press_start: None,
            on_long_press_end: None,
            on_long_press: None,
            action_value: String::new(),
            action_payload: JsonValue::Null,
            accessibility_description: String::new(),
            threshold: 500,
            is_disabled: false,
            is_pressed: false,
            is_long_pressed: false,
        }
    }
}

pub fn ui_long_pressable(cx: &mut ComponentCx<UiLongPressableProps>) -> RSX {
    cx.use_long_press(|props: &UiLongPressableProps| {
        UseLongPressProps::new()
            .on_long_press_start(props.on_long_press_start.clone())
            .on_long_press_end(props.on_long_press_end.clone())
            .on_long_press(props.on_long_press.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .accessibility_description(Some(props.accessibility_description.clone()))
            .threshold(props.threshold)
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
            .long_pressed(props.is_long_pressed)
    });
    cx.use_prop("className", |props: &UiLongPressableProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.longPressProps}
            data-slot="long-pressable"
            data-pressed={props.isPressed}
            data-long-pressed={props.isLongPressed}
            class="outline-none data-[long-pressed=true]:bg-canvas-soft disabled:pointer-events-none disabled:text-muted-soft focus-visible:ring-[2px] focus-visible:ring-ink/40"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
