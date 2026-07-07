use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDisclosureProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDisclosurePanelProps {
    pub class_name: String,
    pub label: String,
    pub is_expanded: bool,
}

pub fn ui_disclosure_panel(cx: &mut ComponentCx<UiDisclosurePanelProps>) -> RSX {
    cx.use_disclosure(|props: &UiDisclosurePanelProps| {
        UseDisclosureProps::new().expanded(props.is_expanded)
    });
    cx.use_prop("className", |props: &UiDisclosurePanelProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiDisclosurePanelProps| {
        props.label.clone()
    });

    crate::rsx!(
        <Section
            key="root"
            {...props.disclosurePanelProps}
            data-slot="disclosure-panel"
            class="grid gap-2 px-4 pb-4 text-sm text-body"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Section>
    )
}
