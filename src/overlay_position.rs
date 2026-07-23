use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{GuiError, GuiResult};
use crate::geometry::{Rect, Size};
use crate::style::TextDirection;

pub const OVERLAY_POSITION_MARKER: &str = "data-overlay-position";
pub const OVERLAY_PLACEMENT_ATTRIBUTE: &str = "data-placement";
pub const OVERLAY_OFFSET_ATTRIBUTE: &str = "data-offset";
pub const OVERLAY_CROSS_OFFSET_ATTRIBUTE: &str = "data-cross-offset";
pub const OVERLAY_SHOULD_FLIP_ATTRIBUTE: &str = "data-should-flip";
pub const OVERLAY_SHOULD_UPDATE_POSITION_ATTRIBUTE: &str = "data-should-update-position";
pub const OVERLAY_CONTAINER_PADDING_ATTRIBUTE: &str = "data-container-padding";
pub const OVERLAY_ARROW_SIZE_ATTRIBUTE: &str = "data-arrow-size";
pub const OVERLAY_ARROW_BOUNDARY_OFFSET_ATTRIBUTE: &str = "data-arrow-boundary-offset";
pub const OVERLAY_MAX_HEIGHT_ATTRIBUTE: &str = "data-max-height";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OverlayPlacement {
    Bottom,
    BottomLeft,
    BottomRight,
    BottomStart,
    BottomEnd,
    Top,
    TopLeft,
    TopRight,
    TopStart,
    TopEnd,
    Left,
    LeftTop,
    LeftBottom,
    Start,
    StartTop,
    StartBottom,
    Right,
    RightTop,
    RightBottom,
    End,
    EndTop,
    EndBottom,
}

impl OverlayPlacement {
    pub const ALL: [Self; 22] = [
        Self::Bottom,
        Self::BottomLeft,
        Self::BottomRight,
        Self::BottomStart,
        Self::BottomEnd,
        Self::Top,
        Self::TopLeft,
        Self::TopRight,
        Self::TopStart,
        Self::TopEnd,
        Self::Left,
        Self::LeftTop,
        Self::LeftBottom,
        Self::Start,
        Self::StartTop,
        Self::StartBottom,
        Self::Right,
        Self::RightTop,
        Self::RightBottom,
        Self::End,
        Self::EndTop,
        Self::EndBottom,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::BottomLeft => "bottom left",
            Self::BottomRight => "bottom right",
            Self::BottomStart => "bottom start",
            Self::BottomEnd => "bottom end",
            Self::Top => "top",
            Self::TopLeft => "top left",
            Self::TopRight => "top right",
            Self::TopStart => "top start",
            Self::TopEnd => "top end",
            Self::Left => "left",
            Self::LeftTop => "left top",
            Self::LeftBottom => "left bottom",
            Self::Start => "start",
            Self::StartTop => "start top",
            Self::StartBottom => "start bottom",
            Self::Right => "right",
            Self::RightTop => "right top",
            Self::RightBottom => "right bottom",
            Self::End => "end",
            Self::EndTop => "end top",
            Self::EndBottom => "end bottom",
        }
    }

    pub const fn resolve(self, direction: TextDirection) -> ResolvedOverlayPlacement {
        use OverlayCrossAlignment::{Center, Far, Near};
        use OverlayPlacementAxis::{Bottom, Left, Right, Top};

        match self {
            Self::Bottom => ResolvedOverlayPlacement::new(Bottom, Center),
            Self::BottomLeft => ResolvedOverlayPlacement::new(Bottom, Near),
            Self::BottomRight => ResolvedOverlayPlacement::new(Bottom, Far),
            Self::BottomStart => ResolvedOverlayPlacement::new(
                Bottom,
                if matches!(direction, TextDirection::Rtl) {
                    Far
                } else {
                    Near
                },
            ),
            Self::BottomEnd => ResolvedOverlayPlacement::new(
                Bottom,
                if matches!(direction, TextDirection::Rtl) {
                    Near
                } else {
                    Far
                },
            ),
            Self::Top => ResolvedOverlayPlacement::new(Top, Center),
            Self::TopLeft => ResolvedOverlayPlacement::new(Top, Near),
            Self::TopRight => ResolvedOverlayPlacement::new(Top, Far),
            Self::TopStart => ResolvedOverlayPlacement::new(
                Top,
                if matches!(direction, TextDirection::Rtl) {
                    Far
                } else {
                    Near
                },
            ),
            Self::TopEnd => ResolvedOverlayPlacement::new(
                Top,
                if matches!(direction, TextDirection::Rtl) {
                    Near
                } else {
                    Far
                },
            ),
            Self::Left => ResolvedOverlayPlacement::new(Left, Center),
            Self::LeftTop => ResolvedOverlayPlacement::new(Left, Near),
            Self::LeftBottom => ResolvedOverlayPlacement::new(Left, Far),
            Self::Start => ResolvedOverlayPlacement::new(
                if matches!(direction, TextDirection::Rtl) {
                    Right
                } else {
                    Left
                },
                Center,
            ),
            Self::StartTop => ResolvedOverlayPlacement::new(
                if matches!(direction, TextDirection::Rtl) {
                    Right
                } else {
                    Left
                },
                Near,
            ),
            Self::StartBottom => ResolvedOverlayPlacement::new(
                if matches!(direction, TextDirection::Rtl) {
                    Right
                } else {
                    Left
                },
                Far,
            ),
            Self::Right => ResolvedOverlayPlacement::new(Right, Center),
            Self::RightTop => ResolvedOverlayPlacement::new(Right, Near),
            Self::RightBottom => ResolvedOverlayPlacement::new(Right, Far),
            Self::End => ResolvedOverlayPlacement::new(
                if matches!(direction, TextDirection::Rtl) {
                    Left
                } else {
                    Right
                },
                Center,
            ),
            Self::EndTop => ResolvedOverlayPlacement::new(
                if matches!(direction, TextDirection::Rtl) {
                    Left
                } else {
                    Right
                },
                Near,
            ),
            Self::EndBottom => ResolvedOverlayPlacement::new(
                if matches!(direction, TextDirection::Rtl) {
                    Left
                } else {
                    Right
                },
                Far,
            ),
        }
    }
}

impl Default for OverlayPlacement {
    fn default() -> Self {
        Self::Bottom
    }
}

impl Display for OverlayPlacement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OverlayPlacement {
    type Err = GuiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = value
            .trim()
            .to_ascii_lowercase()
            .replace(['-', '_'], " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        Self::ALL
            .into_iter()
            .find(|placement| placement.as_str() == normalized)
            .ok_or_else(|| {
                GuiError::invalid_tree(format!(
                    "unsupported overlay placement {value:?}; expected a React Aria placement"
                ))
            })
    }
}

impl Serialize for OverlayPlacement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OverlayPlacement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_str(&value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OverlayPlacementAxis {
    Top,
    Bottom,
    Left,
    Right,
}

impl OverlayPlacementAxis {
    pub const fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OverlayCrossAlignment {
    Near,
    Center,
    Far,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedOverlayPlacement {
    pub axis: OverlayPlacementAxis,
    pub alignment: OverlayCrossAlignment,
}

impl ResolvedOverlayPlacement {
    pub const fn new(axis: OverlayPlacementAxis, alignment: OverlayCrossAlignment) -> Self {
        Self { axis, alignment }
    }

    pub const fn flipped(self) -> Self {
        Self::new(self.axis.opposite(), self.alignment)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayPositionOptions {
    pub placement: OverlayPlacement,
    pub offset: f64,
    pub cross_offset: f64,
    pub should_flip: bool,
    pub should_update_position: bool,
    pub container_padding: f64,
    pub arrow_size: f64,
    pub arrow_boundary_offset: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_height: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayPositionRequest {
    pub options: OverlayPositionOptions,
    pub direction: TextDirection,
}

impl OverlayPositionRequest {
    pub fn new(options: OverlayPositionOptions, direction: TextDirection) -> GuiResult<Self> {
        Ok(Self {
            options: options.validate()?,
            direction,
        })
    }

    pub const fn resolved_placement(self) -> ResolvedOverlayPlacement {
        self.options.placement.resolve(self.direction)
    }
}

impl Default for OverlayPositionOptions {
    fn default() -> Self {
        Self {
            placement: OverlayPlacement::Bottom,
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

impl OverlayPositionOptions {
    pub fn validate(self) -> GuiResult<Self> {
        finite("offset", self.offset)?;
        finite("crossOffset", self.cross_offset)?;
        non_negative("containerPadding", self.container_padding)?;
        non_negative("arrowSize", self.arrow_size)?;
        non_negative("arrowBoundaryOffset", self.arrow_boundary_offset)?;
        if let Some(max_height) = self.max_height {
            non_negative("maxHeight", max_height)?;
        }
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayPoint {
    pub x: f64,
    pub y: f64,
}

impl OverlayPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayArrowPosition {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculatedOverlayPosition {
    pub rect: Rect,
    pub max_height: f64,
    pub placement: ResolvedOverlayPlacement,
    pub arrow: OverlayArrowPosition,
    pub trigger_anchor_point: OverlayPoint,
    pub flipped: bool,
}

pub fn calculate_overlay_position(
    anchor: Rect,
    overlay_size: Size,
    boundary: Rect,
    direction: TextDirection,
    options: OverlayPositionOptions,
) -> GuiResult<CalculatedOverlayPosition> {
    validate_rect("anchor", anchor)?;
    validate_size("overlay", overlay_size)?;
    validate_rect("boundary", boundary)?;
    let options = options.validate()?;

    let requested = options.placement.resolve(direction);
    let requested_space = available_main_space(anchor, boundary, requested.axis, options);
    let opposite = requested.flipped();
    let opposite_space = available_main_space(anchor, boundary, opposite.axis, options);
    let requested_size = main_size(overlay_size, requested.axis);
    let placement = if options.should_flip
        && requested_size > requested_space
        && opposite_space > requested_space
    {
        opposite
    } else {
        requested
    };
    let flipped = placement.axis != requested.axis;

    let available_height = if placement.axis.is_vertical() {
        available_main_space(anchor, boundary, placement.axis, options)
    } else {
        (boundary.height - options.container_padding * 2.0).max(0.0)
    };
    let max_height = options
        .max_height
        .map(|height| height.min(available_height))
        .unwrap_or(available_height);
    let size = Size::new(overlay_size.width, overlay_size.height.min(max_height));
    let mut rect = initial_rect(anchor, size, placement, options);
    clamp_cross_axis(
        &mut rect,
        boundary,
        placement.axis,
        options.container_padding,
    );

    let arrow_offset = arrow_offset(anchor, rect, placement.axis, options);
    let arrow = if placement.axis.is_vertical() {
        OverlayArrowPosition {
            x: Some(arrow_offset),
            y: None,
        }
    } else {
        OverlayArrowPosition {
            x: None,
            y: Some(arrow_offset),
        }
    };
    let trigger_anchor_point = if placement.axis.is_vertical() {
        OverlayPoint::new(
            arrow_offset,
            if matches!(placement.axis, OverlayPlacementAxis::Top) {
                rect.height
            } else {
                0.0
            },
        )
    } else {
        OverlayPoint::new(
            if matches!(placement.axis, OverlayPlacementAxis::Left) {
                rect.width
            } else {
                0.0
            },
            arrow_offset,
        )
    };

    Ok(CalculatedOverlayPosition {
        rect,
        max_height,
        placement,
        arrow,
        trigger_anchor_point,
        flipped,
    })
}

fn initial_rect(
    anchor: Rect,
    size: Size,
    placement: ResolvedOverlayPlacement,
    options: OverlayPositionOptions,
) -> Rect {
    let (x, y) = match placement.axis {
        OverlayPlacementAxis::Top => (
            aligned_cross_start(anchor.x, anchor.width, size.width, placement.alignment)
                + options.cross_offset,
            anchor.y - size.height - options.offset,
        ),
        OverlayPlacementAxis::Bottom => (
            aligned_cross_start(anchor.x, anchor.width, size.width, placement.alignment)
                + options.cross_offset,
            anchor.y + anchor.height + options.offset,
        ),
        OverlayPlacementAxis::Left => (
            anchor.x - size.width - options.offset,
            aligned_cross_start(anchor.y, anchor.height, size.height, placement.alignment)
                + options.cross_offset,
        ),
        OverlayPlacementAxis::Right => (
            anchor.x + anchor.width + options.offset,
            aligned_cross_start(anchor.y, anchor.height, size.height, placement.alignment)
                + options.cross_offset,
        ),
    };
    Rect::new(x, y, size.width, size.height)
}

fn aligned_cross_start(
    anchor_start: f64,
    anchor_size: f64,
    overlay_size: f64,
    alignment: OverlayCrossAlignment,
) -> f64 {
    match alignment {
        OverlayCrossAlignment::Near => anchor_start,
        OverlayCrossAlignment::Center => anchor_start + (anchor_size - overlay_size) / 2.0,
        OverlayCrossAlignment::Far => anchor_start + anchor_size - overlay_size,
    }
}

fn clamp_cross_axis(rect: &mut Rect, boundary: Rect, axis: OverlayPlacementAxis, padding: f64) {
    if axis.is_vertical() {
        rect.x = clamp_to_boundary(rect.x, rect.width, boundary.x, boundary.width, padding);
    } else {
        rect.y = clamp_to_boundary(rect.y, rect.height, boundary.y, boundary.height, padding);
    }
}

fn clamp_to_boundary(
    value: f64,
    size: f64,
    boundary_start: f64,
    boundary_size: f64,
    padding: f64,
) -> f64 {
    let minimum = boundary_start + padding;
    let maximum = boundary_start + boundary_size - padding - size;
    if maximum < minimum {
        minimum
    } else {
        value.clamp(minimum, maximum)
    }
}

fn arrow_offset(
    anchor: Rect,
    overlay: Rect,
    axis: OverlayPlacementAxis,
    options: OverlayPositionOptions,
) -> f64 {
    let (anchor_start, anchor_size, overlay_start, overlay_size) = if axis.is_vertical() {
        (anchor.x, anchor.width, overlay.x, overlay.width)
    } else {
        (anchor.y, anchor.height, overlay.y, overlay.height)
    };
    let preferred = anchor_start + anchor_size / 2.0 - overlay_start;
    let minimum = options.arrow_size / 2.0 + options.arrow_boundary_offset;
    let maximum = overlay_size - minimum;
    if maximum < minimum {
        overlay_size / 2.0
    } else {
        preferred.clamp(minimum, maximum)
    }
}

fn available_main_space(
    anchor: Rect,
    boundary: Rect,
    axis: OverlayPlacementAxis,
    options: OverlayPositionOptions,
) -> f64 {
    let padding = options.container_padding + options.offset;
    match axis {
        OverlayPlacementAxis::Top => (anchor.y - boundary.y - padding).max(0.0),
        OverlayPlacementAxis::Bottom => {
            (boundary.y + boundary.height - anchor.y - anchor.height - padding).max(0.0)
        }
        OverlayPlacementAxis::Left => (anchor.x - boundary.x - padding).max(0.0),
        OverlayPlacementAxis::Right => {
            (boundary.x + boundary.width - anchor.x - anchor.width - padding).max(0.0)
        }
    }
}

fn main_size(size: Size, axis: OverlayPlacementAxis) -> f64 {
    if axis.is_vertical() {
        size.height
    } else {
        size.width
    }
}

fn validate_rect(name: &str, rect: Rect) -> GuiResult<()> {
    finite(&format!("{name}.x"), rect.x)?;
    finite(&format!("{name}.y"), rect.y)?;
    non_negative(&format!("{name}.width"), rect.width)?;
    non_negative(&format!("{name}.height"), rect.height)
}

fn validate_size(name: &str, size: Size) -> GuiResult<()> {
    non_negative(&format!("{name}.width"), size.width)?;
    non_negative(&format!("{name}.height"), size.height)
}

fn finite(name: &str, value: f64) -> GuiResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(GuiError::invalid_tree(format!(
            "overlay position {name} must be finite"
        )))
    }
}

fn non_negative(name: &str, value: f64) -> GuiResult<()> {
    finite(name, value)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(GuiError::invalid_tree(format!(
            "overlay position {name} must be non-negative"
        )))
    }
}

mod mounted;
pub use mounted::{mounted_overlay_positions, MountedOverlayPosition};

#[cfg(test)]
mod tests;
