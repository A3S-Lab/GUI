use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLandmarkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSectionProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_section(cx: &mut ComponentCx<UiSectionProps>) -> RSX {
    cx.use_landmark(|props: &UiSectionProps| {
        UseLandmarkProps::new()
            .kind(Some("section"))
            .label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiSectionProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <section
            key="root"
            {...props.landmarkProps}
            data-slot="section"
            class="grid gap-3"
            className={props.className}
        >
            <Slot key="content" />
        </section>
    )
}
