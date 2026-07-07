use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseOverlayProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiPopoverProps {
    pub class_name: String,
    pub is_open: bool,
}

pub fn ui_popover(cx: &mut ComponentCx<UiPopoverProps>) -> RSX {
    cx.use_overlay(|props: &UiPopoverProps| UseOverlayProps::new().open(props.is_open));
    cx.use_prop("className", |props: &UiPopoverProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Popover
            key="root"
            {...props.overlayProps}
            data-slot="popover"
            class="z-50 min-w-[8rem] overflow-hidden rounded-lg border border-hairline-strong bg-canvas text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Popover>
    )
}
