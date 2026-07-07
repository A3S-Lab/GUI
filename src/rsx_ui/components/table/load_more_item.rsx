use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLoadMoreItemProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiTableLoadMoreItemProps {
    pub class_name: String,
    pub on_press: String,
    pub label: String,
    pub is_loading: bool,
    pub is_disabled: bool,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiTableLoadMoreItemProps {
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

pub fn ui_table_load_more_item(cx: &mut ComponentCx<UiTableLoadMoreItemProps>) -> RSX {
    cx.use_load_more_item(|props: &UiTableLoadMoreItemProps| {
        UseLoadMoreItemProps::new()
            .label(Some(props.label.clone()))
            .on_press(Some(props.on_press.clone()))
            .loading(props.is_loading)
            .disabled(props.is_disabled)
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
    });
    cx.use_prop("className", |props: &UiTableLoadMoreItemProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableRow
            key="root"
            {...props.loadMoreItemProps}
            data-slot="table-load-more-item"
            class="border-b border-hairline text-center text-sm text-body hover:bg-surface-strong/50 disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
        >
            <Slot key="content" />
        </TableRow>
    )
}
