use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTreeProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTreeProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_tree(cx: &mut ComponentCx<UiTreeProps>) -> RSX {
    cx.use_tree(|props: &UiTreeProps| {
        UseTreeProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiTreeProps| props.class_name.clone());

    crate::rsx!(
        <Tree
            key="root"
            {...props.treeProps}
            data-slot="tree"
            class="grid gap-1"
            className={props.className}
        >
            <Slot key="content" />
        </Tree>
    )
}
