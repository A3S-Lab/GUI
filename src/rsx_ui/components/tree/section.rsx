use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{CollectionSectionKind, UseCollectionSectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTreeSectionProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_tree_section(cx: &mut ComponentCx<UiTreeSectionProps>) -> RSX {
    cx.use_collection_section(|props: &UiTreeSectionProps| {
        UseCollectionSectionProps::new()
            .label(Some(props.label.clone()))
            .collection_kind(CollectionSectionKind::Tree)
    });
    cx.use_prop("className", |props: &UiTreeSectionProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTreeSectionProps| props.label.clone());

    crate::rsx!(
        <Section
            key="root"
            data-slot="tree-section"
            {...props.collectionSectionProps}
            data-collection-kind="tree"
            class="grid gap-1"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Section>
    )
}
