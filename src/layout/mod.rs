//! Deterministic GUI-owned layout records derived from
//! [`NativeElement`](crate::native::NativeElement).
//!
//! This module remains available in semantic-only builds. It does not depend
//! on Graphics, a window toolkit, or platform handles. The optional Graphics
//! adapter consumes the versioned records produced here.

mod engine;
mod resolve;
mod text;
mod types;

pub use engine::layout_native_tree;
pub use text::{
    LayoutOptions, LayoutText, ShapedGlyph, ShapedGlyphRun, ShapedText, ShapedTextLine,
    TextContentSource, TextFontFaceId, TextShapeRequest, TextShaper, MAX_TEXT_FACE_ID_BYTES,
    MAX_TEXT_GLYPHS_PER_NODE, MAX_TEXT_LINES_PER_NODE, MAX_TEXT_RUNS_PER_NODE,
    MAX_TEXT_SOURCE_BYTES,
};
pub use types::{
    LayoutChange, LayoutChangeKind, LayoutColor, LayoutCornerRadii, LayoutDiagnostic,
    LayoutDiagnosticCode, LayoutDiagnosticSeverity, LayoutDiff, LayoutEdgeColors, LayoutEdgeWidths,
    LayoutElementId, LayoutHitRegion, LayoutNodeRecord, LayoutPaint, LayoutSnapshot,
    LAYOUT_QUANTIZATION, LAYOUT_SCHEMA_VERSION,
};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod text_tests;
