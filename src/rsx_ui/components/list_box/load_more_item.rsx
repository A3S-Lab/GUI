use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLoadMoreItemProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiListBoxLoadMoreItemProps {
    pub class_name: String,
    pub on_press: String,
    pub label: String,
    pub is_loading: bool,
    pub is_disabled: bool,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiListBoxLoadMoreItemProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            on_press: String::new(),
            label: String::new(),
            is_loading: false,
            is_disabled: false,
            action_value: String::new(),
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_list_box_load_more_item(cx: &mut ComponentCx<UiListBoxLoadMoreItemProps>) -> RSX {
    cx.use_load_more_item(|props: &UiListBoxLoadMoreItemProps| {
        UseLoadMoreItemProps::new()
            .label(Some(props.label.clone()))
            .on_press(Some(props.on_press.clone()))
            .loading(props.is_loading)
            .disabled(props.is_disabled)
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
    });
    cx.use_prop("className", |props: &UiListBoxLoadMoreItemProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <ListBoxItem
            key="root"
            {...props.loadMoreItemProps}
            data-slot="list-box-load-more-item"
            class="relative flex w-full cursor-default select-none items-center justify-center rounded-sm px-2 py-1.5 text-sm text-body outline-none focus:bg-surface-strong focus:text-ink disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
        >
            <Slot key="content" />
        </ListBoxItem>
    )
}
