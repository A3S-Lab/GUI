use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTooltipProps {
    pub class_name: String,
    pub label: String,
    pub is_open: bool,
}

pub fn ui_tooltip(cx: &mut ComponentCx<UiTooltipProps>) -> RSX {
    cx.use_prop("className", |props: &UiTooltipProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTooltipProps| props.label.clone());
    cx.use_prop("isOpen", |props: &UiTooltipProps| props.is_open);

    crate::rsx!(
        <Popover
            key="root"
            data-slot="tooltip"
            class="z-50 max-w-xs rounded-md border border-border bg-popover px-3 py-1.5 text-xs text-popover-foreground shadow-md"
            className={props.className}
            label={props.label}
            open={props.isOpen}
        >
            <Slot key="content" />
        </Popover>
    )
}
