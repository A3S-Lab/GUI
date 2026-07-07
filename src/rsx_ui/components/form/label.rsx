use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiLabelProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_label(cx: &mut ComponentCx<UiLabelProps>) -> RSX {
    cx.use_label(|props: &UiLabelProps| UseTextProps::new().label(Some(props.label.clone())));
    cx.use_prop("className", |props: &UiLabelProps| props.class_name.clone());

    crate::rsx!(
        <Label
            key="root"
            {...props.labelProps}
            data-slot="label"
            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            className={props.className}
        >
            <Slot key="content" />
        </Label>
    )
}
