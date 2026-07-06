use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableCaptionProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_table_caption(cx: &mut ComponentCx<UiTableCaptionProps>) -> RSX {
    cx.use_prop("className", |props: &UiTableCaptionProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTableCaptionProps| props.label.clone());
    cx.use_prop("textValue", |props: &UiTableCaptionProps| {
        props.text_value.clone()
    });

    crate::rsx!(
        <TableCaption
            key="root"
            data-slot="table-caption"
            class="mt-4 text-sm text-muted-foreground"
            className={props.className}
            label={props.label}
            textValue={props.textValue}
        >
            <Slot key="content" />
        </TableCaption>
    )
}
