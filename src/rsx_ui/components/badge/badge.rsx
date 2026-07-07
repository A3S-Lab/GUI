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
            class="inline-flex min-h-6 w-fit shrink-0 items-center justify-center gap-1 overflow-hidden whitespace-nowrap rounded-full border border-transparent bg-surface-strong px-2.5 py-1 text-[11px] font-semibold uppercase tracking-[0.08em] text-ink transition-colors"
            className={props.className}
        >
            <Slot key="content" />
        </span>
    )
}
