use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiLabelProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_label(cx: &mut ComponentCx<UiLabelProps>) -> RSX {
    cx.use_prop("className", |props: &UiLabelProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiLabelProps| props.label.clone());

    crate::rsx!(
        <Label
            key="root"
            data-slot="label"
            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Label>
    )
}
