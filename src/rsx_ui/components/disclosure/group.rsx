use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDisclosureGroupProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDisclosureGroupProps {
    pub class_name: String,
    pub label: String,
    pub expanded_keys: String,
    pub on_expanded_change: Option<String>,
    pub allows_multiple_expanded: bool,
    pub is_disabled: bool,
}

pub fn ui_disclosure_group(cx: &mut ComponentCx<UiDisclosureGroupProps>) -> RSX {
    cx.use_disclosure_group(|props: &UiDisclosureGroupProps| {
        UseDisclosureGroupProps::new()
            .label(Some(props.label.clone()))
            .expanded_keys(Some(props.expanded_keys.clone()))
            .on_expanded_change(props.on_expanded_change.clone())
            .allows_multiple_expanded(props.allows_multiple_expanded)
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiDisclosureGroupProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.disclosureGroupProps}
            data-slot="disclosure-group"
            data-disabled={props.isDisabled}
            data-allows-multiple-expanded={props.allowsMultipleExpanded}
            class="grid gap-2 rounded-md data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
