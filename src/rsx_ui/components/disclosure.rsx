use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDisclosureProps {
    pub class_name: String,
    pub label: String,
    pub is_expanded: bool,
    pub on_expanded_change: String,
    pub is_disabled: bool,
}

pub fn ui_disclosure(cx: &mut ComponentCx<UiDisclosureProps>) -> RSX {
    cx.use_prop("className", |props: &UiDisclosureProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiDisclosureProps| props.label.clone());
    cx.use_prop("isExpanded", |props: &UiDisclosureProps| props.is_expanded);
    cx.use_prop("onExpandedChange", |props: &UiDisclosureProps| {
        props.on_expanded_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiDisclosureProps| props.is_disabled);

    crate::rsx!(
        <Disclosure
            key="root"
            data-slot="disclosure"
            class="rounded-md border border-border bg-card text-card-foreground"
            className={props.className}
            label={props.label}
            expanded={props.isExpanded}
            onExpandedChange={props.onExpandedChange}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </Disclosure>
    )
}
