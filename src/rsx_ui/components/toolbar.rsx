use crate::rsx_app::{ComponentCx, RSX};

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
    cx.use_prop("className", |props: &UiToolbarProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiToolbarProps| props.label.clone());
    cx.use_prop("orientation", |props: &UiToolbarProps| {
        props.orientation.clone()
    });
    cx.use_prop("isDisabled", |props: &UiToolbarProps| props.is_disabled);

    crate::rsx!(
        <Toolbar
            key="root"
            data-slot="toolbar"
            class="flex items-center gap-1 rounded-md border border-border bg-background p-1"
            className={props.className}
            label={props.label}
            orientation={props.orientation}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </Toolbar>
    )
}
