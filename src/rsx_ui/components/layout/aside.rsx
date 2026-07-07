use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLandmarkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiAsideProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_aside(cx: &mut ComponentCx<UiAsideProps>) -> RSX {
    cx.use_landmark(|props: &UiAsideProps| {
        UseLandmarkProps::new()
            .kind(Some("aside"))
            .label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiAsideProps| props.class_name.clone());

    crate::rsx!(
        <aside
            key="root"
            {...props.landmarkProps}
            data-slot="aside"
            class="grid gap-3"
            className={props.className}
        >
            <Slot key="content" />
        </aside>
    )
}
