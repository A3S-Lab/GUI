use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLandmarkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiMainProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_main(cx: &mut ComponentCx<UiMainProps>) -> RSX {
    cx.use_landmark(|props: &UiMainProps| {
        UseLandmarkProps::new()
            .kind(Some("main"))
            .label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiMainProps| props.class_name.clone());

    crate::rsx!(
        <main
            key="root"
            {...props.landmarkProps}
            data-slot="main"
            class="grid gap-4"
            className={props.className}
        >
            <Slot key="content" />
        </main>
    )
}
