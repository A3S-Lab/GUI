use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLandmarkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSearchProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_search(cx: &mut ComponentCx<UiSearchProps>) -> RSX {
    cx.use_landmark(|props: &UiSearchProps| {
        UseLandmarkProps::new()
            .kind(Some("search"))
            .label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiSearchProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <search
            key="root"
            {...props.landmarkProps}
            data-slot="search"
            class="grid gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </search>
    )
}
