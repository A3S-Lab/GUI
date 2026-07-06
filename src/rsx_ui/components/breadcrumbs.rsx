use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiBreadcrumbsProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_breadcrumbs(cx: &mut ComponentCx<UiBreadcrumbsProps>) -> RSX {
    cx.use_prop("className", |props: &UiBreadcrumbsProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiBreadcrumbsProps| props.label.clone());

    crate::rsx!(
        <Navigation
            key="root"
            data-slot="breadcrumbs"
            class="flex flex-wrap items-center gap-1 text-sm text-muted-foreground"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Navigation>
    )
}
