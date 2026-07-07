use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{CollectionSectionKind, UseCollectionSectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiGridListSectionProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_grid_list_section(cx: &mut ComponentCx<UiGridListSectionProps>) -> RSX {
    cx.use_collection_section(|props: &UiGridListSectionProps| {
        UseCollectionSectionProps::new()
            .label(Some(props.label.clone()))
            .collection_kind(CollectionSectionKind::GridList)
    });
    cx.use_prop("className", |props: &UiGridListSectionProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiGridListSectionProps| {
        props.label.clone()
    });

    crate::rsx!(
        <Section
            key="root"
            data-slot="grid-list-section"
            {...props.collectionSectionProps}
            data-collection-kind="grid-list"
            class="grid gap-2"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Section>
    )
}
