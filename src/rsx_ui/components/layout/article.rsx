use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseLandmarkProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiArticleProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_article(cx: &mut ComponentCx<UiArticleProps>) -> RSX {
    cx.use_landmark(|props: &UiArticleProps| {
        UseLandmarkProps::new()
            .kind(Some("article"))
            .label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiArticleProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <article
            key="root"
            {...props.landmarkProps}
            data-slot="article"
            class="grid gap-3"
            className={props.className}
        >
            <Slot key="content" />
        </article>
    )
}
