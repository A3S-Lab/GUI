use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableHeaderProps {
    pub class_name: String,
}

pub fn ui_table_header(cx: &mut ComponentCx<UiTableHeaderProps>) -> RSX {
    cx.use_prop("className", |props: &UiTableHeaderProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableSection
            key="root"
            data-slot="table-header"
            class="[&_tr]:border-b"
            className={props.className}
        >
            <Slot key="content" />
        </TableSection>
    )
}
