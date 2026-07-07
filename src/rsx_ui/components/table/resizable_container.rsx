use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseGroupProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiResizableTableContainerProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_resizable_table_container(cx: &mut ComponentCx<UiResizableTableContainerProps>) -> RSX {
    cx.use_group(|props: &UiResizableTableContainerProps| {
        UseGroupProps::new().label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiResizableTableContainerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.groupProps}
            data-slot="resizable-table-container"
            class="relative w-full overflow-auto rounded-md border border-hairline-strong"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
