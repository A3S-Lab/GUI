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
            class="inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 gap-1 transition-[color,box-shadow] overflow-hidden"
            className={props.className}
        >
            <Slot key="content" />
        </span>
    )
}
