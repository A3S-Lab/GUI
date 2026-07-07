use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{CollectionSectionKind, UseCollectionSectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiListBoxSectionProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_list_box_section(cx: &mut ComponentCx<UiListBoxSectionProps>) -> RSX {
    cx.use_collection_section(|props: &UiListBoxSectionProps| {
        UseCollectionSectionProps::new()
            .label(Some(props.label.clone()))
            .collection_kind(CollectionSectionKind::ListBox)
    });
    cx.use_prop("className", |props: &UiListBoxSectionProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiListBoxSectionProps| props.label.clone());

    crate::rsx!(
        <Section
            key="root"
            data-slot="list-box-section"
            {...props.collectionSectionProps}
            data-collection-kind="list-box"
            class="grid gap-1 px-1 py-1"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Section>
    )
}
