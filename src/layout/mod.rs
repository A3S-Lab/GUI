//! Deterministic GUI-owned layout records derived from
//! [`NativeElement`](crate::native::NativeElement).
//!
//! This module remains available in semantic-only builds. It does not depend
//! on Graphics, a window toolkit, or platform handles. The optional Graphics
//! adapter consumes the versioned records produced here.

mod engine;
mod resolve;
mod types;

pub use engine::layout_native_tree;
pub use types::{
    LayoutChange, LayoutChangeKind, LayoutColor, LayoutCornerRadii, LayoutDiagnostic,
    LayoutDiagnosticCode, LayoutDiagnosticSeverity, LayoutDiff, LayoutEdgeColors, LayoutEdgeWidths,
    LayoutElementId, LayoutHitRegion, LayoutNodeRecord, LayoutPaint, LayoutSnapshot,
    LAYOUT_QUANTIZATION, LAYOUT_SCHEMA_VERSION,
};

#[cfg(test)]
mod tests;
