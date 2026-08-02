use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::geometry::{Rect, Size};
use crate::native::{NativeRole, ValueSensitivity};
use crate::style::{PortableStyle, TextDirection, WritingMode};

use super::types::quantize;
use super::LayoutColor;

mod content;
pub(super) use content::{shape_node_text, MeasuredText};

pub const MAX_TEXT_SOURCE_BYTES: usize = 1024 * 1024;
pub const MAX_TEXT_FACE_ID_BYTES: usize = 1024;
pub const MAX_TEXT_LINES_PER_NODE: usize = 65_536;
pub const MAX_TEXT_RUNS_PER_NODE: usize = 262_144;
pub const MAX_TEXT_GLYPHS_PER_NODE: usize = 1024 * 1024;

/// Explicit layout configuration for one native tree.
///
/// Box-only layout is the M3 diagnostic path. A visible text-capable product
/// must supply a shaper rather than relying on estimated character widths.
#[derive(Clone, Copy)]
pub struct LayoutOptions<'a> {
    pub logical_size: Size,
    text_shaper: Option<&'a dyn TextShaper>,
}

impl Debug for LayoutOptions<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LayoutOptions")
            .field("logical_size", &self.logical_size)
            .field("text_shaper", &self.text_shaper.map(|_| "configured"))
            .finish()
    }
}

impl<'a> LayoutOptions<'a> {
    pub const fn boxes_only(logical_size: Size) -> Self {
        Self {
            logical_size,
            text_shaper: None,
        }
    }

    pub const fn with_text(logical_size: Size, text_shaper: &'a dyn TextShaper) -> Self {
        Self {
            logical_size,
            text_shaper: Some(text_shaper),
        }
    }

    pub const fn text_shaper(self) -> Option<&'a dyn TextShaper> {
        self.text_shaper
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextContentSource {
    Label,
    Value,
    Placeholder,
}

/// Stable identifier for one resolved font face in a shaper-owned font set.
///
/// It is an opaque resource key, never a platform handle. Implementations must
/// return the same identifier for the same font database and face.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TextFontFaceId(String);

impl TextFontFaceId {
    pub fn new(value: impl Into<String>) -> GuiResult<Self> {
        let value = value.into();
        validate_face_id(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One transient shaping request. Source text is deliberately absent from its
/// debug representation so values cannot leak through backend diagnostics.
pub struct TextShapeRequest<'a> {
    pub text: &'a str,
    pub source: TextContentSource,
    pub sensitivity: ValueSensitivity,
    pub role: NativeRole,
    pub language: Option<&'a str>,
    pub direction: TextDirection,
    pub writing_mode: WritingMode,
    pub available: Size,
    /// Portable node style input for font discovery, features, paragraph
    /// layout, and decorations. A backend must reject relevant text fields it
    /// cannot honor; unrelated box and paint fields remain owned by layout.
    pub style: &'a PortableStyle,
}

impl Debug for TextShapeRequest<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TextShapeRequest")
            .field("text_bytes", &self.text.len())
            .field("source", &self.source)
            .field("sensitivity", &self.sensitivity)
            .field("role", &self.role)
            .field("language", &self.language)
            .field("direction", &self.direction)
            .field("writing_mode", &self.writing_mode)
            .field("available", &self.available)
            .finish_non_exhaustive()
    }
}

impl TextShapeRequest<'_> {
    fn validate(&self) -> GuiResult<()> {
        validate_source_text(self.text)?;
        validate_size(self.available, "text shaping available size")
    }
}

/// Production font discovery, fallback, bidi, shaping, and line-layout edge.
///
/// Implementations may cache immutable font data internally, but returned
/// records must be deterministic, owned, platform-neutral, and handle-free.
pub trait TextShaper: Send + Sync {
    fn shape(&self, request: &TextShapeRequest<'_>) -> GuiResult<ShapedText>;
}

/// One positioned glyph. Coordinates are logical points relative to the
/// shaped paragraph origin retained by [`LayoutText`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapedGlyph {
    pub glyph_id: u32,
    pub cluster_start: u32,
    pub cluster_end: u32,
    pub x: f64,
    pub y: f64,
    pub advance_x: f64,
    pub advance_y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapedGlyphRun {
    pub font_face: TextFontFaceId,
    pub font_size: f64,
    pub direction: TextDirection,
    pub bidi_level: u8,
    pub glyphs: Vec<ShapedGlyph>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapedTextLine {
    pub byte_start: u32,
    pub byte_end: u32,
    pub baseline: f64,
    pub ascent: f64,
    pub descent: f64,
    pub advance: f64,
    pub runs: Vec<ShapedGlyphRun>,
}

/// Owned output shared by intrinsic measurement and scene encoding.
///
/// `logical_size` is the intrinsic paragraph size and `ink_bounds` is relative
/// to its origin. The source string is intentionally not retained. UTF-8
/// cluster boundaries are sufficient for later display-selection geometry
/// without copying user values into layout snapshots.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapedText {
    pub logical_size: Size,
    pub ink_bounds: Rect,
    pub lines: Vec<ShapedTextLine>,
}

impl ShapedText {
    pub fn validate(&self, shaped_text: &str) -> GuiResult<()> {
        self.validate_structure(shaped_text.len())?;
        for line in &self.lines {
            for boundary in [line.byte_start, line.byte_end] {
                if !shaped_text.is_char_boundary(boundary as usize) {
                    return Err(GuiError::text(
                        "text line must use UTF-8 character boundaries",
                    ));
                }
            }
            for run in &line.runs {
                for glyph in &run.glyphs {
                    for boundary in [glyph.cluster_start, glyph.cluster_end] {
                        if !shaped_text.is_char_boundary(boundary as usize) {
                            return Err(GuiError::text(
                                "glyph cluster must use UTF-8 character boundaries",
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_structure(&self, source_len: usize) -> GuiResult<()> {
        if source_len > MAX_TEXT_SOURCE_BYTES {
            return Err(GuiError::text(format!(
                "shaped text source exceeds its {MAX_TEXT_SOURCE_BYTES}-byte limit"
            )));
        }
        validate_size(self.logical_size, "shaped text logical size")?;
        validate_rect(self.ink_bounds, "shaped text ink bounds")?;
        if self.lines.len() > MAX_TEXT_LINES_PER_NODE {
            return Err(GuiError::text(format!(
                "shaped text exceeds its {MAX_TEXT_LINES_PER_NODE}-line limit"
            )));
        }
        if source_len > 0 && self.lines.is_empty() {
            return Err(GuiError::text(
                "non-empty shaped text must contain at least one line",
            ));
        }

        let mut previous_line_end = 0_usize;
        let mut run_count = 0_usize;
        let mut glyph_count = 0_usize;
        for line in &self.lines {
            let start = line.byte_start as usize;
            let end = line.byte_end as usize;
            validate_range(start, end, previous_line_end, source_len, true, "line")?;
            previous_line_end = end;
            validate_finite(line.baseline, "text line baseline")?;
            validate_non_negative(line.ascent, "text line ascent")?;
            validate_non_negative(line.descent, "text line descent")?;
            validate_non_negative(line.advance, "text line advance")?;

            run_count = run_count
                .checked_add(line.runs.len())
                .ok_or_else(|| GuiError::text("shaped text run count exceeds platform capacity"))?;
            if run_count > MAX_TEXT_RUNS_PER_NODE {
                return Err(GuiError::text(format!(
                    "shaped text exceeds its {MAX_TEXT_RUNS_PER_NODE}-run limit"
                )));
            }
            for run in &line.runs {
                validate_face_id(run.font_face.as_str())?;
                validate_non_negative(run.font_size, "shaped glyph run font size")?;
                if run.font_size == 0.0 {
                    return Err(GuiError::text(
                        "shaped glyph run font size must be greater than zero",
                    ));
                }
                let expected_rtl = run.bidi_level % 2 == 1;
                if expected_rtl != (run.direction == TextDirection::Rtl) {
                    return Err(GuiError::text(
                        "shaped text run direction disagrees with its bidi level",
                    ));
                }
                glyph_count = glyph_count.checked_add(run.glyphs.len()).ok_or_else(|| {
                    GuiError::text("shaped glyph count exceeds platform capacity")
                })?;
                if glyph_count > MAX_TEXT_GLYPHS_PER_NODE {
                    return Err(GuiError::text(format!(
                        "shaped text exceeds its {MAX_TEXT_GLYPHS_PER_NODE}-glyph limit"
                    )));
                }
                for glyph in &run.glyphs {
                    validate_range(
                        glyph.cluster_start as usize,
                        glyph.cluster_end as usize,
                        start,
                        end,
                        false,
                        "glyph cluster",
                    )?;
                    validate_finite(glyph.x, "shaped glyph x")?;
                    validate_finite(glyph.y, "shaped glyph y")?;
                    validate_finite(glyph.advance_x, "shaped glyph horizontal advance")?;
                    validate_finite(glyph.advance_y, "shaped glyph vertical advance")?;
                }
            }
        }
        if previous_line_end != source_len {
            return Err(GuiError::text(
                "shaped text lines must cover the complete source byte range",
            ));
        }
        Ok(())
    }

    pub(super) fn quantized(mut self) -> Self {
        self.logical_size.width = quantize(self.logical_size.width);
        self.logical_size.height = quantize(self.logical_size.height);
        self.ink_bounds.x = quantize(self.ink_bounds.x);
        self.ink_bounds.y = quantize(self.ink_bounds.y);
        self.ink_bounds.width = quantize(self.ink_bounds.width);
        self.ink_bounds.height = quantize(self.ink_bounds.height);
        for line in &mut self.lines {
            line.baseline = quantize(line.baseline);
            line.ascent = quantize(line.ascent);
            line.descent = quantize(line.descent);
            line.advance = quantize(line.advance);
            for run in &mut line.runs {
                run.font_size = quantize(run.font_size);
                for glyph in &mut run.glyphs {
                    glyph.x = quantize(glyph.x);
                    glyph.y = quantize(glyph.y);
                    glyph.advance_x = quantize(glyph.advance_x);
                    glyph.advance_y = quantize(glyph.advance_y);
                }
            }
        }
        self
    }
}

/// Text geometry retained on a layout node and consumed by scene extraction.
/// It contains glyph identities and metrics, never the source string.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutText {
    pub source: TextContentSource,
    pub sensitivity: ValueSensitivity,
    pub shaped_text_bytes: u32,
    pub origin_x: f64,
    pub origin_y: f64,
    pub clip: Option<Rect>,
    pub color: LayoutColor,
    pub shape: ShapedText,
}

impl LayoutText {
    pub fn validate(&self) -> GuiResult<()> {
        validate_finite(self.origin_x, "layout text origin x")?;
        validate_finite(self.origin_y, "layout text origin y")?;
        if let Some(clip) = self.clip {
            validate_rect(clip, "layout text clip")?;
        }
        self.shape
            .validate_structure(self.shaped_text_bytes as usize)?;
        validate_rect(
            self.unclipped_ink_bounds(),
            "absolute layout text ink bounds",
        )?;
        Ok(())
    }

    pub fn ink_bounds(&self) -> Option<Rect> {
        if self.shape.ink_bounds.width == 0.0 || self.shape.ink_bounds.height == 0.0 {
            return None;
        }
        let bounds = self.unclipped_ink_bounds();
        self.clip
            .map_or(Some(bounds), |clip| intersect(bounds, clip))
    }

    fn unclipped_ink_bounds(&self) -> Rect {
        Rect::new(
            self.origin_x + self.shape.ink_bounds.x,
            self.origin_y + self.shape.ink_bounds.y,
            self.shape.ink_bounds.width,
            self.shape.ink_bounds.height,
        )
    }
}

fn validate_face_id(value: &str) -> GuiResult<()> {
    if value.is_empty()
        || value.len() > MAX_TEXT_FACE_ID_BYTES
        || value.chars().any(char::is_control)
    {
        return Err(GuiError::text(format!(
            "text font face id must contain 1 to {MAX_TEXT_FACE_ID_BYTES} non-control UTF-8 bytes"
        )));
    }
    Ok(())
}

fn validate_source_text(value: &str) -> GuiResult<()> {
    if value.len() > MAX_TEXT_SOURCE_BYTES {
        return Err(GuiError::text(format!(
            "text source exceeds its {MAX_TEXT_SOURCE_BYTES}-byte limit"
        )));
    }
    Ok(())
}

fn validate_range(
    start: usize,
    end: usize,
    minimum: usize,
    maximum: usize,
    contiguous: bool,
    field: &str,
) -> GuiResult<()> {
    if (contiguous && start != minimum) || start < minimum || end < start || end > maximum {
        return Err(GuiError::text(format!(
            "{field} must use an ordered byte range inside its parent"
        )));
    }
    Ok(())
}

fn intersect(left: Rect, right: Rect) -> Option<Rect> {
    let x = left.x.max(right.x);
    let y = left.y.max(right.y);
    let right_edge = (left.x + left.width).min(right.x + right.width);
    let bottom_edge = (left.y + left.height).min(right.y + right.height);
    (right_edge > x && bottom_edge > y).then(|| Rect::new(x, y, right_edge - x, bottom_edge - y))
}

fn validate_size(size: Size, field: &str) -> GuiResult<()> {
    validate_non_negative(size.width, &format!("{field} width"))?;
    validate_non_negative(size.height, &format!("{field} height"))
}

fn validate_rect(rect: Rect, field: &str) -> GuiResult<()> {
    validate_finite(rect.x, &format!("{field} x"))?;
    validate_finite(rect.y, &format!("{field} y"))?;
    validate_non_negative(rect.width, &format!("{field} width"))?;
    validate_non_negative(rect.height, &format!("{field} height"))
}

fn validate_non_negative(value: f64, field: &str) -> GuiResult<()> {
    validate_finite(value, field)?;
    if value < 0.0 {
        return Err(GuiError::text(format!("{field} cannot be negative")));
    }
    Ok(())
}

fn validate_finite(value: f64, field: &str) -> GuiResult<()> {
    if !value.is_finite() {
        return Err(GuiError::text(format!("{field} must be finite")));
    }
    Ok(())
}
