use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiGridListHeaderProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_grid_list_header(cx: &mut ComponentCx<UiGridListHeaderProps>) -> RSX {
    cx.use_grid_list_header(|props: &UiGridListHeaderProps| {
        UseTextProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiGridListHeaderProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Header
            key="root"
            {...props.gridListHeaderProps}
            data-slot="grid-list-header"
            class="px-1 text-xs font-medium text-body"
            className={props.className}
        >
            <Slot key="content" />
        </Header>
    )
}
