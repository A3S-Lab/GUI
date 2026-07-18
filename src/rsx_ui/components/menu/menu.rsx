use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseMenuProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiMenuProps {
    pub class_name: String,
    pub label: String,
    pub is_disabled: bool,
}

pub fn ui_menu(cx: &mut ComponentCx<UiMenuProps>) -> RSX {
    cx.use_menu(|props: &UiMenuProps| {
        UseMenuProps::new()
            .label(Some(props.label.clone()))
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiMenuProps| props.class_name.clone());

    crate::rsx!(
        <Menu
            key="root"
            {...props.menuProps}
            data-slot="menu"
            class="min-w-32 overflow-hidden rounded-md border border-hairline-strong bg-surface-card p-1 text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Menu>
    )
}
