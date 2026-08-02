use crate::error::{GuiError, GuiResult};
use crate::layout::{
    LayoutColor, LayoutCornerRadii, LayoutEdgeWidths, LayoutNodeRecord, LayoutPaint,
    LayoutSnapshot, LAYOUT_SCHEMA_VERSION,
};

use super::{
    Color, CornerRadii, DrawCommand, DrawId, EdgeWidths, FillRect, FillRoundedRect, Primitive,
    Rect, Scene, SceneBuilder, Size, StrokeRect, TextSceneEncoder, TextSceneRequest,
    MAX_TEXT_SCENE_PRIMITIVES_PER_NODE,
};

const DRAW_NAMESPACE: &str = "a3s-gui.layout.v1";
const BACKGROUND_SLOT: u32 = 0;
const BORDER_SLOT: u32 = 1;
const BORDER_TOP_SLOT: u32 = 2;
const BORDER_RIGHT_SLOT: u32 = 3;
const BORDER_BOTTOM_SLOT: u32 = 4;
const BORDER_LEFT_SLOT: u32 = 5;
const TEXT_SLOT_BASE: u32 = 65_536;

pub struct LayoutSceneOptions<'a> {
    pub scale_factor: f32,
    pub clear_color: Color,
    pub text_encoder: Option<&'a mut dyn TextSceneEncoder>,
}

impl std::fmt::Debug for LayoutSceneOptions<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LayoutSceneOptions")
            .field("scale_factor", &self.scale_factor)
            .field("clear_color", &self.clear_color)
            .field(
                "text_encoder",
                &self.text_encoder.as_ref().map(|_| "configured"),
            )
            .finish()
    }
}

impl Default for LayoutSceneOptions<'_> {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            clear_color: Color::TRANSPARENT,
            text_encoder: None,
        }
    }
}

impl<'a> LayoutSceneOptions<'a> {
    pub fn with_text_encoder(mut self, text_encoder: &'a mut dyn TextSceneEncoder) -> Self {
        self.text_encoder = Some(text_encoder);
        self
    }
}

/// Lowers one validated GUI layout snapshot into the Graphics scene schema.
///
/// Error-level M3 diagnostics are rejected before any command is emitted, so
/// required layout or paint fields cannot disappear silently.
pub fn scene_from_layout(
    layout: &LayoutSnapshot,
    mut options: LayoutSceneOptions<'_>,
) -> GuiResult<Scene> {
    if layout.schema_version != LAYOUT_SCHEMA_VERSION {
        return Err(GuiError::graphics(format!(
            "unsupported GUI layout schema {}; expected {}",
            layout.schema_version, LAYOUT_SCHEMA_VERSION
        )));
    }
    layout.require_supported()?;
    if !options.scale_factor.is_finite() || options.scale_factor <= 0.0 {
        return Err(GuiError::graphics(
            "layout scene scale factor must be finite and greater than zero",
        ));
    }
    let logical_size = Size::new(
        finite_f32(layout.logical_size.width, "layout logical width")?,
        finite_f32(layout.logical_size.height, "layout logical height")?,
    );
    let mut builder = SceneBuilder::new(logical_size, options.scale_factor, options.clear_color);
    let mut nodes = layout.nodes.iter().collect::<Vec<_>>();
    nodes.sort_by_key(|node| node.paint_order);
    for node in nodes {
        push_node(
            &mut builder,
            node,
            options.scale_factor,
            &mut options.text_encoder,
        )?;
    }
    Ok(builder.finish()?)
}

fn push_node(
    builder: &mut SceneBuilder,
    node: &LayoutNodeRecord,
    scale_factor: f32,
    text_encoder: &mut Option<&mut dyn TextSceneEncoder>,
) -> GuiResult<()> {
    push_box(builder, node)?;
    push_text(builder, node, scale_factor, text_encoder)
}

fn push_box(builder: &mut SceneBuilder, node: &LayoutNodeRecord) -> GuiResult<()> {
    let rect = graphics_rect(node.border_box, "layout border box")?;
    if rect.is_empty() {
        return Ok(());
    }
    let clip = node
        .clip
        .map(|clip| graphics_rect(clip, "layout clip"))
        .transpose()?;
    let opacity = finite_f32(node.paint.opacity, "layout paint opacity")?;
    if !(0.0..=1.0).contains(&opacity) {
        return Err(GuiError::graphics(format!(
            "layout node {} has opacity outside zero to one",
            node.id
        )));
    }
    let widths = graphics_widths(node.paint.border_widths)?;
    let radii = graphics_radii(node.paint.corner_radii)?;
    let border_color = uniform_visible_border_color(&node.paint)?;
    let has_border = widths != EdgeWidths::ZERO;
    let has_radii = radii != CornerRadii::ZERO;

    if has_border && has_radii {
        let border_color = border_color.ok_or_else(|| {
            GuiError::graphics(format!(
                "layout node {} has a rounded non-uniform border",
                node.id
            ))
        })?;
        let background = node.paint.background.ok_or_else(|| {
            GuiError::graphics(format!(
                "layout node {} has a rounded border without a background",
                node.id
            ))
        })?;
        push_command(
            builder,
            node,
            BORDER_SLOT,
            Primitive::FillRoundedRect(FillRoundedRect {
                rect,
                radii,
                color: graphics_color(border_color),
            }),
            clip,
            opacity,
        )?;
        let inner = rect.inset(widths);
        if !inner.is_empty() {
            push_command(
                builder,
                node,
                BACKGROUND_SLOT,
                Primitive::FillRoundedRect(FillRoundedRect {
                    rect: inner,
                    radii: inner_radii(radii, widths),
                    color: graphics_color(background),
                }),
                clip,
                opacity,
            )?;
        }
        return Ok(());
    }

    if let Some(background) = node.paint.background {
        let primitive = if has_radii {
            Primitive::FillRoundedRect(FillRoundedRect {
                rect,
                radii,
                color: graphics_color(background),
            })
        } else {
            Primitive::FillRect(FillRect {
                rect,
                color: graphics_color(background),
            })
        };
        push_command(builder, node, BACKGROUND_SLOT, primitive, clip, opacity)?;
    }

    if !has_border {
        return Ok(());
    }
    if let Some(color) = border_color {
        push_command(
            builder,
            node,
            BORDER_SLOT,
            Primitive::StrokeRect(StrokeRect {
                rect,
                widths,
                color: graphics_color(color),
            }),
            clip,
            opacity,
        )?;
    } else {
        push_edge_borders(builder, node, rect, widths, clip, opacity)?;
    }
    Ok(())
}

fn push_text(
    builder: &mut SceneBuilder,
    node: &LayoutNodeRecord,
    scale_factor: f32,
    text_encoder: &mut Option<&mut dyn TextSceneEncoder>,
) -> GuiResult<()> {
    let Some(text) = node.text.as_ref() else {
        return Ok(());
    };
    text.validate().map_err(|error| {
        GuiError::graphics(format!(
            "layout node {} contains invalid shaped text: {error}",
            node.id
        ))
    })?;
    let encoder = text_encoder.as_deref_mut().ok_or_else(|| {
        GuiError::graphics(format!(
            "layout node {} contains shaped text but no text scene encoder was configured",
            node.id
        ))
    })?;
    let primitives = encoder.encode(TextSceneRequest { text, scale_factor })?;
    if primitives.len() > MAX_TEXT_SCENE_PRIMITIVES_PER_NODE {
        return Err(GuiError::graphics(format!(
            "layout node {} text encoder exceeds its {}-primitive limit",
            node.id, MAX_TEXT_SCENE_PRIMITIVES_PER_NODE
        )));
    }
    let opacity = finite_f32(node.paint.opacity, "layout text opacity")?;
    let clip = text
        .clip
        .map(|clip| graphics_rect(clip, "layout text clip"))
        .transpose()?;
    let ink_bounds = text
        .ink_bounds()
        .map(|bounds| graphics_rect(bounds, "layout text ink bounds"))
        .transpose()?;
    for (index, primitive) in primitives.into_iter().enumerate() {
        let primitive_bounds = primitive.local_bounds();
        if !primitive_bounds.is_empty()
            && !ink_bounds.is_some_and(|ink| contains_rect(ink, primitive_bounds))
        {
            return Err(GuiError::graphics(format!(
                "layout node {} text primitive lies outside its shaped ink bounds",
                node.id
            )));
        }
        let slot = TEXT_SLOT_BASE
            .checked_add(index as u32)
            .ok_or_else(|| GuiError::graphics("layout text draw slot exceeds u32 capacity"))?;
        push_command(builder, node, slot, primitive, clip, opacity)?;
    }
    Ok(())
}

fn contains_rect(outer: Rect, inner: Rect) -> bool {
    const TOLERANCE: f32 = 1.0 / 64.0;
    inner.x + TOLERANCE >= outer.x
        && inner.y + TOLERANCE >= outer.y
        && inner.right() <= outer.right() + TOLERANCE
        && inner.bottom() <= outer.bottom() + TOLERANCE
}

fn push_edge_borders(
    builder: &mut SceneBuilder,
    node: &LayoutNodeRecord,
    rect: Rect,
    widths: EdgeWidths,
    clip: Option<Rect>,
    opacity: f32,
) -> GuiResult<()> {
    let edges = [
        (
            BORDER_TOP_SLOT,
            widths.top,
            node.paint.border_colors.top,
            Rect::new(rect.x, rect.y, rect.width, widths.top),
        ),
        (
            BORDER_RIGHT_SLOT,
            widths.right,
            node.paint.border_colors.right,
            Rect::new(
                rect.right() - widths.right,
                rect.y,
                widths.right,
                rect.height,
            ),
        ),
        (
            BORDER_BOTTOM_SLOT,
            widths.bottom,
            node.paint.border_colors.bottom,
            Rect::new(
                rect.x,
                rect.bottom() - widths.bottom,
                rect.width,
                widths.bottom,
            ),
        ),
        (
            BORDER_LEFT_SLOT,
            widths.left,
            node.paint.border_colors.left,
            Rect::new(rect.x, rect.y, widths.left, rect.height),
        ),
    ];
    for (slot, width, color, edge_rect) in edges {
        if width == 0.0 {
            continue;
        }
        let color = color.ok_or_else(|| {
            GuiError::graphics(format!(
                "layout node {} has a border width without an edge color",
                node.id
            ))
        })?;
        push_command(
            builder,
            node,
            slot,
            Primitive::FillRect(FillRect {
                rect: edge_rect,
                color: graphics_color(color),
            }),
            clip,
            opacity,
        )?;
    }
    Ok(())
}

fn push_command(
    builder: &mut SceneBuilder,
    node: &LayoutNodeRecord,
    slot: u32,
    primitive: Primitive,
    clip: Option<Rect>,
    opacity: f32,
) -> GuiResult<()> {
    let mut command = DrawCommand::new(
        DrawId::from_stable_key(DRAW_NAMESPACE, node.id.as_str(), slot),
        primitive,
    )
    .with_opacity(opacity);
    if let Some(clip) = clip {
        command = command.with_clip(clip);
    }
    builder.push(command)?;
    Ok(())
}

fn uniform_visible_border_color(paint: &LayoutPaint) -> GuiResult<Option<LayoutColor>> {
    let edges = [
        (paint.border_widths.top, paint.border_colors.top),
        (paint.border_widths.right, paint.border_colors.right),
        (paint.border_widths.bottom, paint.border_colors.bottom),
        (paint.border_widths.left, paint.border_colors.left),
    ];
    let mut uniform = None;
    for (width, color) in edges {
        if width == 0.0 {
            continue;
        }
        let Some(color) = color else {
            return Ok(None);
        };
        match uniform {
            None => uniform = Some(color),
            Some(existing) if existing == color => {}
            Some(_) => return Ok(None),
        }
    }
    Ok(uniform)
}

fn graphics_rect(rect: crate::geometry::Rect, field: &str) -> GuiResult<Rect> {
    let rect = Rect::new(
        finite_f32(rect.x, field)?,
        finite_f32(rect.y, field)?,
        finite_f32(rect.width, field)?,
        finite_f32(rect.height, field)?,
    );
    if rect.width < 0.0 || rect.height < 0.0 {
        return Err(GuiError::graphics(format!(
            "{field} width and height cannot be negative"
        )));
    }
    Ok(rect)
}

fn graphics_widths(widths: LayoutEdgeWidths) -> GuiResult<EdgeWidths> {
    let widths = EdgeWidths {
        top: finite_f32(widths.top, "top border width")?,
        right: finite_f32(widths.right, "right border width")?,
        bottom: finite_f32(widths.bottom, "bottom border width")?,
        left: finite_f32(widths.left, "left border width")?,
    };
    if !widths.is_finite_and_non_negative() {
        return Err(GuiError::graphics(
            "layout border widths must be finite and non-negative",
        ));
    }
    Ok(widths)
}

fn graphics_radii(radii: LayoutCornerRadii) -> GuiResult<CornerRadii> {
    let radii = CornerRadii {
        top_left: finite_f32(radii.top_left, "top-left corner radius")?,
        top_right: finite_f32(radii.top_right, "top-right corner radius")?,
        bottom_right: finite_f32(radii.bottom_right, "bottom-right corner radius")?,
        bottom_left: finite_f32(radii.bottom_left, "bottom-left corner radius")?,
    };
    if !radii.is_finite_and_non_negative() {
        return Err(GuiError::graphics(
            "layout corner radii must be finite and non-negative",
        ));
    }
    Ok(radii)
}

fn inner_radii(radii: CornerRadii, widths: EdgeWidths) -> CornerRadii {
    CornerRadii {
        top_left: (radii.top_left - widths.top.max(widths.left)).max(0.0),
        top_right: (radii.top_right - widths.top.max(widths.right)).max(0.0),
        bottom_right: (radii.bottom_right - widths.bottom.max(widths.right)).max(0.0),
        bottom_left: (radii.bottom_left - widths.bottom.max(widths.left)).max(0.0),
    }
}

fn graphics_color(color: LayoutColor) -> Color {
    Color::rgba(color.red, color.green, color.blue, color.alpha)
}

fn finite_f32(value: f64, field: &str) -> GuiResult<f32> {
    let narrowed = value as f32;
    if !value.is_finite() || !narrowed.is_finite() {
        return Err(GuiError::graphics(format!(
            "{field} must fit in a finite 32-bit float"
        )));
    }
    Ok(narrowed)
}
