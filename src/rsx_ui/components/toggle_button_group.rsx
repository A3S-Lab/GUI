use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq)]
pub struct UiToggleButtonGroupProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub orientation: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
}

impl Default for UiToggleButtonGroupProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            label: String::new(),
            value: String::new(),
            orientation: "horizontal".to_string(),
            on_selection_change: String::new(),
            is_disabled: false,
        }
    }
}

pub fn ui_toggle_button_group(cx: &mut ComponentCx<UiToggleButtonGroupProps>) -> RSX {
    cx.use_prop("className", |props: &UiToggleButtonGroupProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiToggleButtonGroupProps| {
        props.label.clone()
    });
    cx.use_prop("value", |props: &UiToggleButtonGroupProps| {
        props.value.clone()
    });
    cx.use_prop("orientation", |props: &UiToggleButtonGroupProps| {
        props.orientation.clone()
    });
    cx.use_prop("onSelectionChange", |props: &UiToggleButtonGroupProps| {
        props.on_selection_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiToggleButtonGroupProps| {
        props.is_disabled
    });

    crate::rsx!(
        <Toolbar
            key="root"
            data-slot="toggle-button-group"
            class="inline-flex items-center gap-1 rounded-md border border-border bg-background p-1"
            className={props.className}
            label={props.label}
            value={props.value}
            orientation={props.orientation}
            onSelectionChange={props.onSelectionChange}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </Toolbar>
    )
}
