use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseOverlayProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiPopoverProps {
    pub class_name: String,
    pub is_open: bool,
    pub on_close: String,
    pub is_non_modal: bool,
    pub is_keyboard_dismiss_disabled: bool,
}

pub fn ui_popover(cx: &mut ComponentCx<UiPopoverProps>) -> RSX {
    cx.use_overlay(|props: &UiPopoverProps| {
        UseOverlayProps::new()
            .open(props.is_open)
            .on_close(Some(props.on_close.clone()))
            .modal(!props.is_non_modal)
            .dismissable(!props.is_non_modal)
            .keyboard_dismiss_disabled(props.is_keyboard_dismiss_disabled)
            .close_on_blur(true)
            .contain_focus(!props.is_non_modal)
            .restore_focus(true)
            .auto_focus(true)
    });
    cx.use_prop("className", |props: &UiPopoverProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Popover
            key="root"
            {...props.overlayProps}
            data-slot="popover"
            class="z-50 min-w-[8rem] overflow-hidden rounded-md border border-hairline bg-canvas text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Popover>
    )
}
