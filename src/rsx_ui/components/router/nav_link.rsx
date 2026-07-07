use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLinkProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiNavLinkProps {
    pub class_name: String,
    pub to: String,
    pub on_navigate: Option<String>,
    pub is_active: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub action_payload: JsonValue,
}

impl Default for UiNavLinkProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            to: String::new(),
            on_navigate: None,
            is_active: false,
            is_disabled: false,
            is_pressed: false,
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_nav_link(cx: &mut ComponentCx<UiNavLinkProps>) -> RSX {
    cx.use_link(|props: &UiNavLinkProps| {
        UseLinkProps::new()
            .href(Some(props.to.clone()))
            .on_press(props.on_navigate.clone())
            .action_value(Some(props.to.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiNavLinkProps| {
        props.class_name.clone()
    });
    cx.use_prop("isActive", |props: &UiNavLinkProps| props.is_active);
    cx.use_prop("to", |props: &UiNavLinkProps| props.to.clone());

    crate::rsx!(
        <a
            key="root"
            {...props.linkProps}
            data-slot="nav-link"
            data-active={props.isActive}
            data-route-to={props.to}
            actionValue={props.to}
            class="inline-flex h-9 items-center gap-2 rounded-md px-3 text-sm font-medium text-body outline-none transition-colors hover:bg-surface-strong hover:text-ink focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:text-muted-soft data-[active=true]:bg-surface-strong data-[active=true]:text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </a>
    )
}
