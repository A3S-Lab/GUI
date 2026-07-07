use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDisclosureProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDisclosureProps {
    pub class_name: String,
    pub label: String,
    pub is_expanded: bool,
    pub on_expanded_change: String,
    pub is_disabled: bool,
}

pub fn ui_disclosure(cx: &mut ComponentCx<UiDisclosureProps>) -> RSX {
    cx.use_disclosure(|props: &UiDisclosureProps| {
        UseDisclosureProps::new()
            .expanded(props.is_expanded)
            .on_expanded_change(Some(props.on_expanded_change.clone()))
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiDisclosureProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiDisclosureProps| props.label.clone());

    crate::rsx!(
        <Disclosure
            key="root"
            {...props.disclosureProps}
            data-slot="disclosure"
            class="rounded-md border border-hairline-strong bg-canvas text-ink"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Disclosure>
    )
}
