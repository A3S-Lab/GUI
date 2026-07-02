use std::collections::BTreeMap;

use crate::geometry::Orientation;

mod apply;
mod color_parsing;
mod composition;
mod declarations;
mod parsing;
mod portable;
mod shorthands;
mod tailwind;
mod tailwind_apply;
mod tailwind_utilities;
#[cfg(test)]
mod tests;
mod types;
mod value_parsing;

pub use portable::PortableStyle;
pub use types::*;

use color_parsing::{parse_background_shorthand_color, parse_color};
use parsing::*;
use tailwind::{
    arbitrary_or_custom_var as tailwind_arbitrary_or_custom_var, custom_var as tailwind_custom_var,
    decode_arbitrary_content_value as tailwind_arbitrary_content_value,
    decode_arbitrary_value as tailwind_arbitrary_value,
    typed_custom_var as tailwind_typed_custom_var,
};
#[cfg(test)]
use tailwind_utilities::tailwind_filter_pipeline;
use tailwind_utilities::{
    compose_tailwind_ring_shadow, compose_tailwind_shadow, tailwind_box_shadow_pipeline,
    tailwind_color, tailwind_edge_utility, tailwind_length, tailwind_opacity,
    tailwind_scrollbar_color_pipeline, tailwind_utility_declarations,
};
use types::{CornerSelection, EdgeSelection, LogicalCornerSelection, LogicalEdgeSelection};
use value_parsing::{parse_length, parse_time};
