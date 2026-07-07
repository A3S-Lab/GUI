use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseToggleButtonGroupProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiToggleButtonGroupProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub orientation: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
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
            is_read_only: false,
            selection_mode: "single".to_string(),
        }
    }
}

pub fn ui_toggle_button_group(cx: &mut ComponentCx<UiToggleButtonGroupProps>) -> RSX {
    cx.use_toggle_button_group(|props: &UiToggleButtonGroupProps| {
        UseToggleButtonGroupProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .orientation(Some(props.orientation.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiToggleButtonGroupProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Toolbar
            key="root"
            {...props.toggleButtonGroupProps}
            data-slot="toggle-button-group"
            class="inline-flex items-center gap-1 rounded-md border border-hairline bg-surface-strong p-1"
            className={props.className}
        >
            <Slot key="content" />
        </Toolbar>
    )
}
