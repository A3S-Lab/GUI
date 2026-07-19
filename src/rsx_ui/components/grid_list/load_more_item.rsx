use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLoadMoreItemProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiGridListLoadMoreItemProps {
    pub class_name: String,
    pub on_press: String,
    pub label: String,
    pub is_loading: bool,
    pub is_disabled: bool,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiGridListLoadMoreItemProps {
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

pub fn ui_grid_list_load_more_item(cx: &mut ComponentCx<UiGridListLoadMoreItemProps>) -> RSX {
    cx.use_load_more_item(|props: &UiGridListLoadMoreItemProps| {
        UseLoadMoreItemProps::new()
            .label(Some(props.label.clone()))
            .on_press(Some(props.on_press.clone()))
            .loading(props.is_loading)
            .disabled(props.is_disabled)
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
    });
    cx.use_prop("className", |props: &UiGridListLoadMoreItemProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <ListBoxItem
            key="root"
            {...props.loadMoreItemProps}
            data-slot="grid-list-load-more-item"
            class="flex min-h-10 w-full cursor-default select-none items-center justify-center rounded-md border border-dashed border-hairline-strong px-3 py-1.5 text-sm text-body outline-none focus:bg-surface-strong focus:text-ink disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
        >
            <Slot key="content" />
        </ListBoxItem>
    )
}
