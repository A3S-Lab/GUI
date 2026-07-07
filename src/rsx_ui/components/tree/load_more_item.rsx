use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLoadMoreItemProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiTreeLoadMoreItemProps {
    pub class_name: String,
    pub label: String,
    pub on_press: Option<String>,
    pub is_loading: bool,
    pub is_disabled: bool,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiTreeLoadMoreItemProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            label: String::new(),
            on_press: None,
            is_loading: false,
            is_disabled: false,
            action_value: String::new(),
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_tree_load_more_item(cx: &mut ComponentCx<UiTreeLoadMoreItemProps>) -> RSX {
    cx.use_load_more_item(|props: &UiTreeLoadMoreItemProps| {
        UseLoadMoreItemProps::new()
            .label(Some(props.label.clone()))
            .on_press(props.on_press.clone())
            .loading(props.is_loading)
            .disabled(props.is_disabled)
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
    });
    cx.use_prop("className", |props: &UiTreeLoadMoreItemProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TreeItem
            key="root"
            {...props.loadMoreItemProps}
            data-slot="tree-load-more-item"
            class="flex min-h-10 w-full cursor-default select-none items-center justify-center rounded-md border border-dashed border-hairline-strong bg-canvas px-4 py-2 text-sm font-medium text-body outline-none transition-colors focus:bg-primary focus:text-on-primary disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </TreeItem>
    )
}
