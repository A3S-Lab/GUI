use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLinkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiBreadcrumbProps {
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

pub fn ui_breadcrumb(cx: &mut ComponentCx<UiBreadcrumbProps>) -> RSX {
    cx.use_link(|props: &UiBreadcrumbProps| {
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
    cx.use_prop("className", |props: &UiBreadcrumbProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <a
            key="root"
            {...props.linkProps}
            data-slot="breadcrumb"
            data-pressed={props.isPressed}
            class="inline-flex items-center rounded-sm text-body outline-none transition-colors hover:text-ink focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
        >
            <Slot key="content" />
        </a>
    )
}
