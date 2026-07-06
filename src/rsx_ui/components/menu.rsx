use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiMenuProps {
    pub class_name: String,
}

pub fn ui_menu(cx: &mut ComponentCx<UiMenuProps>) -> RSX {
    cx.use_prop("className", |props: &UiMenuProps| props.class_name.clone());

    crate::rsx!(
        <Menu
            key="root"
            data-slot="menu"
            class="min-w-32 overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md"
            className={props.className}
        >
            <Slot key="content" />
        </Menu>
    )
}
