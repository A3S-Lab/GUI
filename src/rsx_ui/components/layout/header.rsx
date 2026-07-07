use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLandmarkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiHeaderProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_header(cx: &mut ComponentCx<UiHeaderProps>) -> RSX {
    cx.use_landmark(|props: &UiHeaderProps| {
        UseLandmarkProps::new()
            .kind(Some("header"))
            .label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiHeaderProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <header
            key="root"
            {...props.landmarkProps}
            data-slot="header"
            class="flex items-center gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </header>
    )
}
