use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiListBoxProps {
    pub class_name: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
}

pub fn ui_list_box(cx: &mut ComponentCx<UiListBoxProps>) -> RSX {
    cx.use_prop("className", |props: &UiListBoxProps| {
        props.class_name.clone()
    });
    cx.use_prop("onSelectionChange", |props: &UiListBoxProps| {
        props.on_selection_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiListBoxProps| props.is_disabled);

    crate::rsx!(
        <ListBox
            key="root"
            data-slot="list-box"
            class="max-h-72 min-w-32 overflow-auto rounded-md border bg-popover p-1 text-popover-foreground shadow-md outline-none"
            className={props.className}
            onSelectionChange={props.onSelectionChange}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </ListBox>
    )
}
