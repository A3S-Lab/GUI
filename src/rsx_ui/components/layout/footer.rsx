use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLandmarkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFooterProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_footer(cx: &mut ComponentCx<UiFooterProps>) -> RSX {
    cx.use_landmark(|props: &UiFooterProps| {
        UseLandmarkProps::new()
            .kind(Some("footer"))
            .label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiFooterProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <footer
            key="root"
            {...props.landmarkProps}
            data-slot="footer"
            class="flex items-center gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </footer>
    )
}
