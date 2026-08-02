use crate::error::GuiResult;
use crate::layout::LayoutText;

use super::Primitive;

pub const MAX_TEXT_SCENE_PRIMITIVES_PER_NODE: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy)]
pub struct TextSceneRequest<'a> {
    /// Source-free shaped record in logical coordinates.
    pub text: &'a LayoutText,
    /// Device pixels per logical point for glyph rasterization and caching.
    pub scale_factor: f32,
}

/// Converts one already-shaped layout record into self-drawn Graphics
/// primitives.
///
/// The encoder must rasterize or reference GUI-owned glyph resources. It is
/// paired with the same font-face namespace as the shaper, is not a platform
/// text widget, and must not shape or measure the source again. Returned
/// primitives use logical scene coordinates; raster caches may use the request
/// scale factor. Output order must be deterministic because stable draw slots
/// are assigned by primitive index.
pub trait TextSceneEncoder: Send {
    fn encode(&mut self, request: TextSceneRequest<'_>) -> GuiResult<Vec<Primitive>>;
}
