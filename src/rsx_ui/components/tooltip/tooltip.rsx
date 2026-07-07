use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseOverlayProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTooltipProps {
    pub class_name: String,
    pub label: String,
    pub is_open: bool,
}

pub fn ui_tooltip(cx: &mut ComponentCx<UiTooltipProps>) -> RSX {
    cx.use_overlay(|props: &UiTooltipProps| UseOverlayProps::new().open(props.is_open));
    cx.use_prop("className", |props: &UiTooltipProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTooltipProps| props.label.clone());

    crate::rsx!(
        <Popover
            key="root"
            {...props.overlayProps}
            data-slot="tooltip"
            class="z-50 max-w-xs rounded-md border border-hairline-strong bg-canvas px-3 py-1.5 text-xs text-ink"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Popover>
    )
}
