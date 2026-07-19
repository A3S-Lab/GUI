use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiBadgeProps {
    pub class_name: String,
}

pub fn ui_badge(cx: &mut ComponentCx<UiBadgeProps>) -> RSX {
    cx.use_prop("className", |props: &UiBadgeProps| props.class_name.clone());

    crate::rsx!(
        <span
            key="root"
            data-slot="badge"
            class="inline-flex min-h-5 w-fit shrink-0 items-center justify-center gap-1 overflow-hidden whitespace-nowrap rounded-md border border-transparent bg-canvas-soft px-2 py-0.5 text-xs font-medium text-body"
            className={props.className}
        >
            <Slot key="content" />
        </span>
    )
}
