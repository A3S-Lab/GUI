use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLinkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiLinkProps {
    pub class_name: String,
    pub href: String,
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub on_press_up: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub action_value: String,
    pub action_payload: JsonValue,
}

pub fn ui_link(cx: &mut ComponentCx<UiLinkProps>) -> RSX {
    cx.use_link(|props: &UiLinkProps| {
        UseLinkProps::new()
            .href(Some(props.href.clone()))
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .on_press_up(props.on_press_up.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiLinkProps| props.class_name.clone());

    crate::rsx!(
        <a
            key="root"
            {...props.linkProps}
            data-slot="link"
            data-pressed={props.isPressed}
            class="inline-flex items-center gap-1 rounded-sm text-sm font-medium text-link underline-offset-4 outline-none hover:underline focus-visible:ring-[2px] focus-visible:ring-ink/40 disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </a>
    )
}
