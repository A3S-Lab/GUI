use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseButtonProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiNavigateButtonProps {
    pub class_name: String,
    pub to: String,
    pub on_navigate: Option<String>,
    pub is_active: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub action_payload: JsonValue,
}

impl Default for UiNavigateButtonProps {
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

pub fn ui_navigate_button(cx: &mut ComponentCx<UiNavigateButtonProps>) -> RSX {
    cx.use_button(|props: &UiNavigateButtonProps| {
        UseButtonProps::new()
            .on_press(props.on_navigate.clone())
            .action_value(Some(props.to.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiNavigateButtonProps| {
        props.class_name.clone()
    });
    cx.use_prop("isActive", |props: &UiNavigateButtonProps| props.is_active);
    cx.use_prop("to", |props: &UiNavigateButtonProps| props.to.clone());
    cx.use_prop("onNavigate", |props: &UiNavigateButtonProps| {
        props.on_navigate.clone()
    });

    crate::rsx!(
        <button
            key="root"
            {...props.buttonProps}
            onPress={props.onNavigate}
            data-slot="navigate-button"
            data-active={props.isActive}
            data-route-to={props.to}
            actionValue={props.to}
            class="inline-flex h-9 items-center justify-center gap-2 rounded-md border border-transparent px-3 text-sm font-medium text-body outline-none active:bg-surface-strong focus-visible:ring-[2px] focus-visible:ring-ink/40 disabled:pointer-events-none disabled:text-muted-soft data-[active=true]:border-primary data-[active=true]:bg-primary data-[active=true]:text-on-primary"
            className={props.className}
        >
            <Slot key="content" />
        </button>
    )
}
