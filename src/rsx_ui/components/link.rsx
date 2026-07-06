use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UsePressProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiLinkProps {
    pub class_name: String,
    pub href: String,
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub action_value: String,
    pub action_payload: JsonValue,
}

pub fn ui_link(cx: &mut ComponentCx<UiLinkProps>) -> RSX {
    cx.use_press(|props: &UiLinkProps| {
        UsePressProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiLinkProps| props.class_name.clone());
    cx.use_prop("href", |props: &UiLinkProps| props.href.clone());
    cx.use_prop("actionValue", |props: &UiLinkProps| {
        props.action_value.clone()
    });
    cx.use_prop("actionPayload", |props: &UiLinkProps| {
        props.action_payload.clone()
    });

    crate::rsx!(
        <a
            key="root"
            {...props.pressProps}
            data-slot="link"
            data-pressed={props.isPressed}
            class="inline-flex items-center gap-1 rounded-sm text-sm font-medium text-foreground underline-offset-4 outline-none transition-colors hover:underline focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
            href={props.href}
            actionValue={props.actionValue}
            actionPayload={props.actionPayload}
        >
            <Slot key="content" />
        </a>
    )
}
