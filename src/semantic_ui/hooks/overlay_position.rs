use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};
use crate::overlay_position::{OverlayPlacement, OverlayPositionOptions};

#[derive(Debug, Clone, PartialEq)]
pub struct UseOverlayPositionProps {
    placement: String,
    anchor: Option<String>,
    offset: f64,
    cross_offset: f64,
    should_flip: bool,
    should_update_position: bool,
    container_padding: f64,
    arrow_size: f64,
    arrow_boundary_offset: f64,
    max_height: Option<f64>,
}

impl Default for UseOverlayPositionProps {
    fn default() -> Self {
        let defaults = OverlayPositionOptions::default();
        Self {
            placement: defaults.placement.to_string(),
            anchor: None,
            offset: defaults.offset,
            cross_offset: defaults.cross_offset,
            should_flip: defaults.should_flip,
            should_update_position: defaults.should_update_position,
            container_padding: defaults.container_padding,
            arrow_size: defaults.arrow_size,
            arrow_boundary_offset: defaults.arrow_boundary_offset,
            max_height: defaults.max_height,
        }
    }
}

impl UseOverlayPositionProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn placement(mut self, placement: impl Into<String>) -> Self {
        self.placement = placement.into();
        self
    }

    pub fn anchor(mut self, anchor: Option<impl Into<String>>) -> Self {
        self.anchor = anchor
            .map(Into::into)
            .map(|anchor| anchor.trim().to_string())
            .filter(|anchor| !anchor.is_empty());
        self
    }

    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = offset;
        self
    }

    pub fn cross_offset(mut self, cross_offset: f64) -> Self {
        self.cross_offset = cross_offset;
        self
    }

    pub fn should_flip(mut self, should_flip: bool) -> Self {
        self.should_flip = should_flip;
        self
    }

    pub fn should_update_position(mut self, should_update_position: bool) -> Self {
        self.should_update_position = should_update_position;
        self
    }

    pub fn container_padding(mut self, container_padding: f64) -> Self {
        self.container_padding = container_padding;
        self
    }

    pub fn arrow_size(mut self, arrow_size: f64) -> Self {
        self.arrow_size = arrow_size;
        self
    }

    pub fn arrow_boundary_offset(mut self, arrow_boundary_offset: f64) -> Self {
        self.arrow_boundary_offset = arrow_boundary_offset;
        self
    }

    pub fn max_height(mut self, max_height: Option<f64>) -> Self {
        self.max_height = max_height;
        self
    }

    fn options(&self) -> GuiResult<OverlayPositionOptions> {
        OverlayPositionOptions {
            placement: self.placement.parse::<OverlayPlacement>()?,
            offset: self.offset,
            cross_offset: self.cross_offset,
            should_flip: self.should_flip,
            should_update_position: self.should_update_position,
            container_padding: self.container_padding,
            arrow_size: self.arrow_size,
            arrow_boundary_offset: self.arrow_boundary_offset,
            max_height: self.max_height,
        }
        .validate()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseOverlayPositionResult {
    pub overlay_position_props: OverlayPositionProps,
    pub arrow_props: OverlayArrowProps,
    pub placement: OverlayPlacement,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayPositionProps {
    #[serde(rename = "data-overlay-position")]
    pub data_overlay_position: bool,
    #[serde(rename = "data-placement")]
    pub data_placement: String,
    #[serde(rename = "data-offset")]
    pub data_offset: f64,
    #[serde(rename = "data-cross-offset")]
    pub data_cross_offset: f64,
    #[serde(rename = "data-should-flip")]
    pub data_should_flip: bool,
    #[serde(rename = "data-should-update-position")]
    pub data_should_update_position: bool,
    #[serde(rename = "data-container-padding")]
    pub data_container_padding: f64,
    #[serde(rename = "data-arrow-size")]
    pub data_arrow_size: f64,
    #[serde(rename = "data-arrow-boundary-offset")]
    pub data_arrow_boundary_offset: f64,
    #[serde(rename = "data-max-height", skip_serializing_if = "Option::is_none")]
    pub data_max_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayArrowProps {
    pub role: &'static str,
    #[serde(rename = "aria-hidden")]
    pub aria_hidden: bool,
    #[serde(rename = "data-placement")]
    pub data_placement: String,
}

pub fn use_overlay_position(props: UseOverlayPositionProps) -> GuiResult<UseOverlayPositionResult> {
    let options = props.options()?;
    let placement = options.placement.to_string();
    Ok(UseOverlayPositionResult {
        overlay_position_props: OverlayPositionProps {
            data_overlay_position: true,
            data_placement: placement.clone(),
            data_offset: options.offset,
            data_cross_offset: options.cross_offset,
            data_should_flip: options.should_flip,
            data_should_update_position: options.should_update_position,
            data_container_padding: options.container_padding,
            data_arrow_size: options.arrow_size,
            data_arrow_boundary_offset: options.arrow_boundary_offset,
            data_max_height: options.max_height,
            anchor: props.anchor,
        },
        arrow_props: OverlayArrowProps {
            role: "presentation",
            aria_hidden: true,
            data_placement: placement,
        },
        placement: options.placement,
    })
}

pub fn use_overlay_position_value(props: UseOverlayPositionProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_overlay_position(props)?).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_overlay_position hook did not serialize: {error}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_hook_serializes_the_typed_native_contract() {
        let result = use_overlay_position(
            UseOverlayPositionProps::new()
                .placement("bottom start")
                .anchor(Some("trigger"))
                .offset(8.0)
                .cross_offset(3.0)
                .should_flip(false)
                .container_padding(16.0)
                .arrow_size(10.0)
                .arrow_boundary_offset(4.0)
                .max_height(Some(240.0)),
        )
        .unwrap();

        assert_eq!(result.placement, OverlayPlacement::BottomStart);
        assert_eq!(
            result.overlay_position_props.anchor.as_deref(),
            Some("trigger")
        );
        assert!(!result.overlay_position_props.data_should_flip);
        assert_eq!(result.arrow_props.data_placement, "bottom start");
        let value = use_overlay_position_value(UseOverlayPositionProps::new()).unwrap();
        assert!(value.get("overlayPositionProps").is_some());
        assert!(value.get("arrowProps").is_some());
    }

    #[test]
    fn position_hook_rejects_invalid_placement_and_geometry() {
        assert!(
            use_overlay_position(UseOverlayPositionProps::new().placement("diagonal")).is_err()
        );
        assert!(
            use_overlay_position(UseOverlayPositionProps::new().container_padding(-1.0)).is_err()
        );
    }
}
