use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{UseOverlayPositionProps, UseOverlayProps};

#[derive(Debug, Clone, PartialEq)]
pub struct UiPopoverProps {
    pub class_name: String,
    pub is_open: bool,
    pub on_close: String,
    pub is_non_modal: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub placement: String,
    pub anchor: Option<String>,
    pub offset: f64,
    pub cross_offset: f64,
    pub should_flip: bool,
    pub should_update_position: bool,
    pub container_padding: f64,
    pub arrow_size: f64,
    pub arrow_boundary_offset: f64,
    pub max_height: Option<f64>,
}

impl Default for UiPopoverProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            is_open: false,
            on_close: String::new(),
            is_non_modal: false,
            is_keyboard_dismiss_disabled: false,
            placement: "bottom".to_string(),
            anchor: None,
            offset: 0.0,
            cross_offset: 0.0,
            should_flip: true,
            should_update_position: true,
            container_padding: 12.0,
            arrow_size: 0.0,
            arrow_boundary_offset: 0.0,
            max_height: None,
        }
    }
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
    cx.use_overlay_position(|props: &UiPopoverProps| {
        UseOverlayPositionProps::new()
            .placement(props.placement.clone())
            .anchor(props.anchor.clone())
            .offset(props.offset)
            .cross_offset(props.cross_offset)
            .should_flip(props.should_flip)
            .should_update_position(props.should_update_position)
            .container_padding(props.container_padding)
            .arrow_size(props.arrow_size)
            .arrow_boundary_offset(props.arrow_boundary_offset)
            .max_height(props.max_height)
    });
    cx.use_prop("className", |props: &UiPopoverProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Popover
            key="root"
            {...props.overlayProps}
            {...props.overlayPositionProps}
            data-slot="popover"
            class="z-50 min-w-[8rem] overflow-hidden rounded-md border border-hairline bg-canvas text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Popover>
    )
}
