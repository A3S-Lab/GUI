mod core;
mod html_forms;
mod html_structure;
mod html_text;
mod intrinsic;

mod support {
    pub(super) use std::collections::BTreeMap;

    pub(super) use crate::compiler::{CompiledJsxNode, CompiledProps, ReactCompilerBridge};
    pub(super) use crate::geometry::Orientation;
    pub(super) use crate::html::{HTML_CONFORMING_ELEMENTS, HTML_ELEMENTS, HTML_TAG_METADATA_KEY};
    pub(super) use crate::native::NativeRole;
    pub(super) use crate::svg::{SVG_ELEMENTS, SVG_TAG_METADATA_KEY};
}
