use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTableCaptionProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableCaptionProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_table_caption(cx: &mut ComponentCx<UiTableCaptionProps>) -> RSX {
    cx.use_table_caption(|props: &UiTableCaptionProps| {
        UseTableCaptionProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiTableCaptionProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableCaption
            key="root"
            {...props.tableCaptionProps}
            data-slot="table-caption"
            class="mt-4 text-sm text-body"
            className={props.className}
        >
            <Slot key="content" />
        </TableCaption>
    )
}
