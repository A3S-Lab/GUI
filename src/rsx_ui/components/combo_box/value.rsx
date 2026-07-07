use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseComboBoxDisplayProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiComboBoxValueProps {
    pub class_name: String,
    pub value: String,
    pub placeholder: String,
}

pub fn ui_combo_box_value(cx: &mut ComponentCx<UiComboBoxValueProps>) -> RSX {
    cx.use_combo_box_display(|props: &UiComboBoxValueProps| {
        UseComboBoxDisplayProps::new()
            .value(Some(props.value.clone()))
            .placeholder(Some(props.placeholder.clone()))
    });
    cx.use_prop("className", |props: &UiComboBoxValueProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <SelectValue
            key="root"
            {...props.comboBoxValueProps}
            data-slot="combo-box-value"
            class="line-clamp-1 flex-1 text-left text-sm text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </SelectValue>
    )
}
