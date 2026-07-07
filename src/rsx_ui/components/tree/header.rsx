use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTreeHeaderProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_tree_header(cx: &mut ComponentCx<UiTreeHeaderProps>) -> RSX {
    cx.use_tree_header(|props: &UiTreeHeaderProps| {
        UseTextProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiTreeHeaderProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Header
            key="root"
            {...props.treeHeaderProps}
            data-slot="tree-header"
            class="px-1 text-xs font-medium text-body"
            className={props.className}
        >
            <Slot key="content" />
        </Header>
    )
}
