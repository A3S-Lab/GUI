use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseToolbarProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiToolbarProps {
    pub class_name: String,
    pub label: String,
    pub orientation: String,
    pub is_disabled: bool,
}

impl Default for UiToolbarProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            label: String::new(),
            orientation: "horizontal".to_string(),
            is_disabled: false,
        }
    }
}

pub fn ui_toolbar(cx: &mut ComponentCx<UiToolbarProps>) -> RSX {
    cx.use_toolbar(|props: &UiToolbarProps| {
        UseToolbarProps::new()
            .label(Some(props.label.clone()))
            .orientation(Some(props.orientation.clone()))
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiToolbarProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Toolbar
            key="root"
            {...props.toolbarProps}
            data-slot="toolbar"
            class="flex items-center gap-1 rounded-md border border-hairline bg-surface-strong p-1"
            className={props.className}
        >
            <Slot key="content" />
        </Toolbar>
    )
}
