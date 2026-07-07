use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{CollectionSectionKind, UseCollectionSectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiMenuSectionProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_menu_section(cx: &mut ComponentCx<UiMenuSectionProps>) -> RSX {
    cx.use_collection_section(|props: &UiMenuSectionProps| {
        UseCollectionSectionProps::new()
            .label(Some(props.label.clone()))
            .collection_kind(CollectionSectionKind::Menu)
    });
    cx.use_prop("className", |props: &UiMenuSectionProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiMenuSectionProps| props.label.clone());

    crate::rsx!(
        <Section
            key="root"
            data-slot="menu-section"
            {...props.collectionSectionProps}
            data-collection-kind="menu"
            class="grid gap-1 px-1 py-1"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Section>
    )
}
