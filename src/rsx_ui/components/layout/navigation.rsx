use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLandmarkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiNavigationProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_navigation(cx: &mut ComponentCx<UiNavigationProps>) -> RSX {
    cx.use_landmark(|props: &UiNavigationProps| {
        UseLandmarkProps::new()
            .kind(Some("navigation"))
            .label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiNavigationProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <nav
            key="root"
            {...props.landmarkProps}
            data-slot="navigation"
            class="flex items-center gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </nav>
    )
}
