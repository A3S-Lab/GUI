use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{UseOverlayPositionProps, UseOverlayProps};

#[derive(Debug, Clone, PartialEq)]
pub struct UiTooltipProps {
    pub class_name: String,
    pub label: String,
    pub is_open: bool,
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

impl Default for UiTooltipProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            label: String::new(),
            is_open: false,
            placement: "top".to_string(),
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

pub fn ui_tooltip(cx: &mut ComponentCx<UiTooltipProps>) -> RSX {
    cx.use_overlay(|props: &UiTooltipProps| {
        UseOverlayProps::new().open(props.is_open).managed(false)
    });
    cx.use_overlay_position(|props: &UiTooltipProps| {
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
    cx.use_prop("className", |props: &UiTooltipProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTooltipProps| props.label.clone());

    crate::rsx!(
        <Popover
            key="root"
            {...props.overlayProps}
            {...props.overlayPositionProps}
            data-slot="tooltip"
            class="z-50 max-w-xs rounded-md border border-hairline-strong bg-surface-card px-3 py-1.5 text-xs text-ink"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Popover>
    )
}
