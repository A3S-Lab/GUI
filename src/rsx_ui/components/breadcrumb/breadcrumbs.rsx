use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseBreadcrumbsProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiBreadcrumbsProps {
    pub class_name: String,
    pub label: Option<String>,
}

pub fn ui_breadcrumbs(cx: &mut ComponentCx<UiBreadcrumbsProps>) -> RSX {
    cx.use_breadcrumbs(|props: &UiBreadcrumbsProps| {
        UseBreadcrumbsProps::new().label(props.label.clone())
    });
    cx.use_prop("className", |props: &UiBreadcrumbsProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Navigation
            key="root"
            {...props.breadcrumbsProps}
            data-slot="breadcrumbs"
            class="flex flex-wrap items-center gap-1 text-sm text-body"
            className={props.className}
        >
            <Slot key="content" />
        </Navigation>
    )
}
