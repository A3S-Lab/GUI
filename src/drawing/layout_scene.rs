use crate::error::{GuiError, GuiResult};
use crate::layout::{
    LayoutColor, LayoutCornerRadii, LayoutEdgeWidths, LayoutNodeRecord, LayoutPaint,
    LayoutSnapshot, LAYOUT_SCHEMA_VERSION,
};

use super::{
    Color, CornerRadii, DrawCommand, DrawId, EdgeWidths, FillRect, FillRoundedRect, Primitive,
    Rect, Scene, SceneBuilder, Size, StrokeRect,
};

const DRAW_NAMESPACE: &str = "a3s-gui.layout.v1";
const BACKGROUND_SLOT: u32 = 0;
const BORDER_SLOT: u32 = 1;
const BORDER_TOP_SLOT: u32 = 2;
const BORDER_RIGHT_SLOT: u32 = 3;
const BORDER_BOTTOM_SLOT: u32 = 4;
const BORDER_LEFT_SLOT: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutSceneOptions {
    pub scale_factor: f32,
    pub clear_color: Color,
}

impl Default for LayoutSceneOptions {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            clear_color: Color::TRANSPARENT,
        }
    }
}

/// Lowers one validated GUI layout snapshot into the Graphics scene schema.
///
/// Error-level M3 diagnostics are rejected before any command is emitted, so
/// required layout or paint fields cannot disappear silently.
pub fn scene_from_layout(layout: &LayoutSnapshot, options: LayoutSceneOptions) -> GuiResult<Scene> {
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
        push_node(&mut builder, node)?;
    }
    Ok(builder.finish()?)
}

fn push_node(builder: &mut SceneBuilder, node: &LayoutNodeRecord) -> GuiResult<()> {
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

#[cfg(test)]
mod tests {
    use crate::geometry::Size as GuiSize;
    use crate::layout::layout_native_tree;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::web::WebProps;

    use super::*;

    fn styled(key: &str, class_name: &str) -> NativeElement {
        NativeElement::new(key, NativeRole::View)
            .with_props(NativeProps::new().web(WebProps::new().class_name(class_name)))
    }

    #[test]
    fn scene_rejects_error_level_layout_diagnostics() {
        let root = NativeElement::new("root", NativeRole::View).with_props(
            NativeProps::new().web(WebProps::new().style("width", "calc(100% - 2rem)")),
        );
        let layout = layout_native_tree(&root, GuiSize::new(10.0, 10.0)).unwrap();

        let error = scene_from_layout(&layout, LayoutSceneOptions::default()).unwrap_err();

        assert!(error
            .to_string()
            .contains("layout field support is incomplete"));
    }

    #[test]
    fn stable_element_paths_produce_stable_draw_ids() {
        let tree = |reversed: bool| {
            let mut children = vec![
                styled("a", "absolute left-[0px] top-[0px] h-2 w-2 bg-black"),
                styled("b", "absolute left-[2px] top-[0px] h-2 w-2 bg-white"),
            ];
            if reversed {
                children.reverse();
            }
            styled("root", "relative h-2 w-4").children(children)
        };
        let first = layout_native_tree(&tree(false), GuiSize::new(4.0, 2.0)).unwrap();
        let second = layout_native_tree(&tree(true), GuiSize::new(4.0, 2.0)).unwrap();
        let first = scene_from_layout(&first, LayoutSceneOptions::default()).unwrap();
        let second = scene_from_layout(&second, LayoutSceneOptions::default()).unwrap();
        let mut first_ids = first
            .commands
            .iter()
            .map(|command| command.id)
            .collect::<Vec<_>>();
        let mut second_ids = second
            .commands
            .iter()
            .map(|command| command.id)
            .collect::<Vec<_>>();
        first_ids.sort();
        second_ids.sort();

        assert_eq!(first_ids, second_ids);
    }

    #[cfg(feature = "software-reference")]
    #[test]
    fn sibling_z_index_controls_scene_paint_order() {
        let root = styled("root", "relative h-2 w-2")
            .child(styled(
                "front",
                "absolute left-[0px] top-[0px] z-10 h-2 w-2 bg-black",
            ))
            .child(styled(
                "back",
                "absolute left-[0px] top-[0px] z-0 h-2 w-2 bg-white",
            ));
        let layout = layout_native_tree(&root, GuiSize::new(2.0, 2.0)).unwrap();
        let scene = scene_from_layout(&layout, LayoutSceneOptions::default()).unwrap();
        let mut renderer = crate::drawing::ReferenceRenderer::new();

        let frame = renderer.render(scene).unwrap();

        assert_eq!(&frame.rgba8()[0..4], &[0, 0, 0, 255]);
    }

    #[cfg(feature = "software-reference")]
    #[test]
    fn rectangle_layout_lowers_through_the_reference_renderer() {
        let root = styled("root", "relative h-3 w-4 bg-white").child(styled(
            "pixel",
            "absolute left-[1px] top-[1px] h-1 w-2 bg-black",
        ));
        let layout = layout_native_tree(&root, GuiSize::new(4.0, 3.0)).unwrap();
        let scene = scene_from_layout(&layout, LayoutSceneOptions::default()).unwrap();
        let mut renderer = crate::drawing::ReferenceRenderer::new();

        let frame = renderer.render(scene).unwrap();

        assert_eq!((frame.width(), frame.height()), (4, 3));
        assert_eq!(&frame.rgba8()[0..4], &[255, 255, 255, 255]);
        let black = ((4 + 1) * 4) as usize;
        assert_eq!(&frame.rgba8()[black..black + 4], &[0, 0, 0, 255]);
    }
}
