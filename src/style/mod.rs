use std::collections::BTreeMap;

use crate::geometry::Orientation;
use crate::web::WebProps;
use serde::{Deserialize, Serialize};

mod color_parsing;
mod tailwind;
mod tailwind_utilities;
#[cfg(test)]
mod tests;
mod value_parsing;

use color_parsing::{parse_background_shorthand_color, parse_color};
use tailwind::{
    arbitrary_or_custom_var as tailwind_arbitrary_or_custom_var, custom_var as tailwind_custom_var,
    decode_arbitrary_content_value as tailwind_arbitrary_content_value,
    decode_arbitrary_value as tailwind_arbitrary_value,
    typed_custom_var as tailwind_typed_custom_var,
};
#[cfg(test)]
use tailwind_utilities::tailwind_filter_pipeline;
use tailwind_utilities::{
    compose_tailwind_ring_shadow, tailwind_box_shadow_pipeline, tailwind_color,
    tailwind_edge_utility, tailwind_length, tailwind_opacity, tailwind_scrollbar_color_pipeline,
    tailwind_utility_declarations,
};
use value_parsing::{parse_length, parse_time};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableStyle {
    pub declarations: BTreeMap<String, String>,
    pub custom_properties: BTreeMap<String, String>,
    pub variant_declarations: BTreeMap<String, BTreeMap<String, String>>,
    pub all: Option<String>,
    pub display: Option<DisplayMode>,
    pub box_sizing: Option<BoxSizing>,
    pub box_decoration_break: Option<BoxDecorationBreak>,
    pub position: Option<PositionMode>,
    pub anchor_name: Option<String>,
    pub anchor_scope: Option<String>,
    pub position_anchor: Option<String>,
    pub position_area: Option<String>,
    pub position_try: Option<String>,
    pub position_try_fallbacks: Option<String>,
    pub position_try_order: Option<String>,
    pub position_try_options: Option<String>,
    pub position_visibility: Option<String>,
    pub flex_direction: Option<Orientation>,
    pub flex_wrap: Option<FlexWrap>,
    pub flex: Option<String>,
    pub flex_basis: Option<StyleLength>,
    pub flex_grow: Option<String>,
    pub flex_shrink: Option<String>,
    pub order: Option<String>,
    pub reading_flow: Option<String>,
    pub reading_order: Option<String>,
    pub align_items: Option<AlignItems>,
    pub align_content: Option<JustifyContent>,
    pub align_self: Option<SelfAlignment>,
    pub justify_content: Option<JustifyContent>,
    pub justify_items: Option<AlignItems>,
    pub justify_self: Option<SelfAlignment>,
    pub place_content: Option<String>,
    pub place_items: Option<String>,
    pub place_self: Option<String>,
    pub width: Option<StyleLength>,
    pub height: Option<StyleLength>,
    pub min_width: Option<StyleLength>,
    pub min_height: Option<StyleLength>,
    pub max_width: Option<StyleLength>,
    pub max_height: Option<StyleLength>,
    pub inline_size: Option<StyleLength>,
    pub block_size: Option<StyleLength>,
    pub min_inline_size: Option<StyleLength>,
    pub min_block_size: Option<StyleLength>,
    pub max_inline_size: Option<StyleLength>,
    pub max_block_size: Option<StyleLength>,
    pub interpolate_size: Option<String>,
    pub gap: Option<StyleLength>,
    pub row_gap: Option<StyleLength>,
    pub column_gap: Option<StyleLength>,
    pub grid: Option<String>,
    pub grid_template: Option<String>,
    pub grid_template_columns: Option<String>,
    pub grid_template_rows: Option<String>,
    pub grid_template_areas: Option<String>,
    pub grid_auto_columns: Option<String>,
    pub grid_auto_rows: Option<String>,
    pub grid_auto_flow: Option<GridAutoFlow>,
    pub grid_column: Option<String>,
    pub grid_column_start: Option<String>,
    pub grid_column_end: Option<String>,
    pub grid_row: Option<String>,
    pub grid_row_start: Option<String>,
    pub grid_row_end: Option<String>,
    pub grid_area: Option<String>,
    pub contain: Option<String>,
    pub container: Option<String>,
    pub container_type: Option<ContainerType>,
    pub container_name: Option<String>,
    pub content: Option<String>,
    pub counter_reset: Option<String>,
    pub counter_increment: Option<String>,
    pub counter_set: Option<String>,
    pub quotes: Option<String>,
    pub string_set: Option<String>,
    pub content_visibility: Option<ContentVisibility>,
    pub contain_intrinsic_size: Option<String>,
    pub contain_intrinsic_width: Option<String>,
    pub contain_intrinsic_height: Option<String>,
    pub contain_intrinsic_inline_size: Option<String>,
    pub contain_intrinsic_block_size: Option<String>,
    pub inset: EdgeInsets,
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub margin_trim: Option<String>,
    pub scroll_margin: EdgeInsets,
    pub scroll_padding: EdgeInsets,
    pub space_x: Option<StyleLength>,
    pub space_y: Option<StyleLength>,
    pub space_x_reverse: Option<String>,
    pub space_y_reverse: Option<String>,
    pub logical_inset: LogicalEdgeInsets,
    pub logical_padding: LogicalEdgeInsets,
    pub logical_margin: LogicalEdgeInsets,
    pub logical_scroll_margin: LogicalEdgeInsets,
    pub logical_scroll_padding: LogicalEdgeInsets,
    pub border_width: EdgeInsets,
    pub logical_border_width: LogicalEdgeInsets,
    pub border_color: Option<StyleColor>,
    pub border_colors: EdgeColors,
    pub logical_border_colors: LogicalEdgeColors,
    pub border_style: Option<BorderStyle>,
    pub border_styles: EdgeBorderStyles,
    pub logical_border_styles: LogicalBorderStyles,
    pub border_image: Option<String>,
    pub border_image_source: Option<String>,
    pub border_image_slice: Option<String>,
    pub border_image_width: Option<String>,
    pub border_image_outset: Option<String>,
    pub border_image_repeat: Option<String>,
    pub box_shadow: Option<String>,
    pub outline_width: Option<StyleLength>,
    pub outline_color: Option<StyleColor>,
    pub outline_style: Option<BorderStyle>,
    pub outline_offset: Option<StyleLength>,
    pub ring_shadow: Option<String>,
    pub ring_color: Option<String>,
    pub inset_ring_shadow: Option<String>,
    pub inset_ring_color: Option<String>,
    pub divide_x_width: Option<StyleLength>,
    pub divide_y_width: Option<StyleLength>,
    pub divide_x_reverse: Option<String>,
    pub divide_y_reverse: Option<String>,
    pub divide_color: Option<StyleColor>,
    pub divide_style: Option<BorderStyle>,
    pub color: Option<StyleColor>,
    pub accent_color: Option<StyleColor>,
    pub caret_color: Option<StyleColor>,
    pub background: Option<String>,
    pub background_color: Option<StyleColor>,
    pub background_image: Option<String>,
    pub background_position: Option<String>,
    pub background_size: Option<String>,
    pub background_repeat: Option<String>,
    pub background_attachment: Option<BackgroundAttachment>,
    pub background_origin: Option<BackgroundBox>,
    pub background_clip: Option<BackgroundBox>,
    pub background_blend_mode: Option<String>,
    pub clip: Option<String>,
    pub clip_path: Option<String>,
    pub mask: Option<String>,
    pub mask_image: Option<String>,
    pub mask_mode: Option<String>,
    pub mask_repeat: Option<String>,
    pub mask_position: Option<String>,
    pub mask_size: Option<String>,
    pub mask_origin: Option<String>,
    pub mask_clip: Option<String>,
    pub mask_composite: Option<String>,
    pub mask_type: Option<String>,
    pub mask_border: Option<String>,
    pub mask_border_source: Option<String>,
    pub mask_border_mode: Option<String>,
    pub mask_border_slice: Option<String>,
    pub mask_border_width: Option<String>,
    pub mask_border_outset: Option<String>,
    pub mask_border_repeat: Option<String>,
    pub border_radius: Option<StyleLength>,
    pub border_radii: CornerRadii,
    pub logical_border_radii: LogicalCornerRadii,
    pub image_rendering: Option<String>,
    pub image_orientation: Option<String>,
    pub image_resolution: Option<String>,
    pub object_fit: Option<ObjectFit>,
    pub object_position: Option<String>,
    pub shape_outside: Option<String>,
    pub shape_inside: Option<String>,
    pub shape_margin: Option<StyleLength>,
    pub shape_padding: Option<StyleLength>,
    pub shape_image_threshold: Option<f64>,
    pub list_style_type: Option<String>,
    pub list_style_position: Option<ListStylePosition>,
    pub list_style_image: Option<String>,
    pub marker_side: Option<String>,
    pub columns: Option<String>,
    pub column_count: Option<String>,
    pub column_width: Option<StyleLength>,
    pub column_rule: Option<String>,
    pub column_rule_width: Option<StyleLength>,
    pub column_rule_style: Option<BorderStyle>,
    pub column_rule_color: Option<StyleColor>,
    pub column_span: Option<String>,
    pub column_fill: Option<String>,
    pub page_size: Option<String>,
    pub page: Option<String>,
    pub page_orientation: Option<String>,
    pub bleed: Option<String>,
    pub marks: Option<String>,
    pub orphans: Option<String>,
    pub widows: Option<String>,
    pub bookmark_label: Option<String>,
    pub bookmark_level: Option<String>,
    pub bookmark_state: Option<String>,
    pub footnote_display: Option<String>,
    pub footnote_policy: Option<String>,
    pub break_before: Option<String>,
    pub break_after: Option<String>,
    pub break_inside: Option<String>,
    pub font: Option<String>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<StyleLength>,
    pub font_size_adjust: Option<String>,
    pub font_weight: Option<FontWeight>,
    pub font_stretch: Option<String>,
    pub font_width: Option<String>,
    pub font_palette: Option<String>,
    pub font_language_override: Option<String>,
    pub font_kerning: Option<String>,
    pub font_optical_sizing: Option<String>,
    pub webkit_font_smoothing: Option<String>,
    pub moz_osx_font_smoothing: Option<String>,
    pub font_feature_settings: Option<String>,
    pub font_variation_settings: Option<String>,
    pub font_variant: Option<String>,
    pub font_variant_alternates: Option<String>,
    pub font_variant_caps: Option<String>,
    pub font_variant_east_asian: Option<String>,
    pub font_variant_emoji: Option<String>,
    pub font_variant_ligatures: Option<String>,
    pub font_variant_numeric: Option<String>,
    pub font_variant_position: Option<String>,
    pub font_synthesis: Option<String>,
    pub font_synthesis_weight: Option<String>,
    pub font_synthesis_style: Option<String>,
    pub font_synthesis_small_caps: Option<String>,
    pub font_synthesis_position: Option<String>,
    pub line_height: Option<StyleLength>,
    pub line_height_step: Option<String>,
    pub block_step: Option<String>,
    pub block_step_size: Option<String>,
    pub block_step_insert: Option<String>,
    pub block_step_align: Option<String>,
    pub block_step_round: Option<String>,
    pub line_grid: Option<String>,
    pub line_snap: Option<String>,
    pub box_snap: Option<String>,
    pub math_depth: Option<String>,
    pub math_shift: Option<String>,
    pub math_style: Option<String>,
    pub dominant_baseline: Option<String>,
    pub baseline_source: Option<String>,
    pub alignment_baseline: Option<String>,
    pub baseline_shift: Option<String>,
    pub line_fit_edge: Option<String>,
    pub inline_sizing: Option<String>,
    pub initial_letter: Option<String>,
    pub initial_letter_align: Option<String>,
    pub initial_letter_wrap: Option<String>,
    pub letter_spacing: Option<StyleLength>,
    pub word_spacing: Option<StyleLength>,
    pub tab_size: Option<String>,
    pub text_align: Option<TextAlign>,
    pub text_align_all: Option<String>,
    pub text_align_last: Option<String>,
    pub text_group_align: Option<String>,
    pub text_justify: Option<String>,
    pub word_space_transform: Option<String>,
    pub text_size_adjust: Option<String>,
    pub webkit_text_size_adjust: Option<String>,
    pub moz_text_size_adjust: Option<String>,
    pub ms_text_size_adjust: Option<String>,
    pub direction: Option<TextDirection>,
    pub unicode_bidi: Option<UnicodeBidi>,
    pub writing_mode: Option<WritingMode>,
    pub text_orientation: Option<TextOrientation>,
    pub text_combine_upright: Option<String>,
    pub text_transform: Option<TextTransform>,
    pub text_indent: Option<StyleLength>,
    pub text_wrap: Option<TextWrapMode>,
    pub text_wrap_mode: Option<String>,
    pub text_wrap_style: Option<String>,
    pub wrap_before: Option<String>,
    pub wrap_after: Option<String>,
    pub wrap_inside: Option<String>,
    pub line_padding: Option<String>,
    pub text_spacing: Option<String>,
    pub text_spacing_trim: Option<String>,
    pub text_autospace: Option<String>,
    pub text_box: Option<String>,
    pub text_box_trim: Option<String>,
    pub text_box_edge: Option<String>,
    pub hanging_punctuation: Option<String>,
    pub line_clamp: Option<String>,
    pub block_ellipsis: Option<String>,
    #[serde(rename = "continue")]
    pub continue_mode: Option<String>,
    pub max_lines: Option<String>,
    pub box_orient: Option<String>,
    pub speak: Option<String>,
    pub speak_as: Option<String>,
    pub pause: Option<String>,
    pub pause_before: Option<String>,
    pub pause_after: Option<String>,
    pub rest: Option<String>,
    pub rest_before: Option<String>,
    pub rest_after: Option<String>,
    pub cue: Option<String>,
    pub cue_before: Option<String>,
    pub cue_after: Option<String>,
    pub voice_family: Option<String>,
    pub voice_balance: Option<String>,
    pub voice_duration: Option<String>,
    pub voice_pitch: Option<String>,
    pub voice_range: Option<String>,
    pub voice_rate: Option<String>,
    pub voice_stress: Option<String>,
    pub voice_volume: Option<String>,
    pub fill: Option<StyleColor>,
    pub fill_opacity: Option<f64>,
    pub fill_rule: Option<FillRule>,
    pub clip_rule: Option<FillRule>,
    pub stroke: Option<StyleColor>,
    pub stroke_width: Option<StyleLength>,
    pub stroke_linecap: Option<StrokeLineCap>,
    pub stroke_linejoin: Option<StrokeLineJoin>,
    pub stroke_miterlimit: Option<String>,
    pub stroke_dasharray: Option<String>,
    pub stroke_dashoffset: Option<StyleLength>,
    pub stroke_opacity: Option<f64>,
    pub vector_effect: Option<String>,
    pub paint_order: Option<String>,
    pub shape_rendering: Option<String>,
    pub text_rendering: Option<String>,
    pub color_rendering: Option<String>,
    pub color_interpolation: Option<String>,
    pub color_interpolation_filters: Option<String>,
    pub marker: Option<String>,
    pub marker_start: Option<String>,
    pub marker_mid: Option<String>,
    pub marker_end: Option<String>,
    pub stop_color: Option<StyleColor>,
    pub stop_opacity: Option<f64>,
    pub flood_color: Option<StyleColor>,
    pub flood_opacity: Option<f64>,
    pub lighting_color: Option<StyleColor>,
    pub text_decoration_line: Option<String>,
    pub text_decoration_color: Option<StyleColor>,
    pub text_decoration_style: Option<TextDecorationStyle>,
    pub text_decoration_thickness: Option<StyleLength>,
    pub text_decoration_skip: Option<String>,
    pub text_decoration_skip_box: Option<String>,
    pub text_decoration_skip_ink: Option<String>,
    pub text_decoration_skip_inset: Option<String>,
    pub text_decoration_skip_self: Option<String>,
    pub text_decoration_skip_spaces: Option<String>,
    pub text_underline_offset: Option<StyleLength>,
    pub text_underline_position: Option<String>,
    pub text_emphasis_style: Option<String>,
    pub text_emphasis_color: Option<StyleColor>,
    pub text_emphasis_position: Option<String>,
    pub text_emphasis_skip: Option<String>,
    pub ruby_align: Option<String>,
    pub ruby_position: Option<String>,
    pub ruby_merge: Option<String>,
    pub ruby_overhang: Option<String>,
    pub text_shadow: Option<String>,
    pub text_overflow: Option<TextOverflow>,
    pub line_break: Option<String>,
    pub white_space: Option<WhiteSpaceMode>,
    pub white_space_collapse: Option<String>,
    pub white_space_trim: Option<String>,
    pub word_break: Option<WordBreakMode>,
    pub overflow_wrap: Option<OverflowWrapMode>,
    pub hyphens: Option<HyphensMode>,
    pub hyphenate_character: Option<String>,
    pub hyphenate_limit_zone: Option<String>,
    pub hyphenate_limit_chars: Option<String>,
    pub hyphenate_limit_lines: Option<String>,
    pub hyphenate_limit_last: Option<String>,
    pub overflow_x: Option<OverflowMode>,
    pub overflow_y: Option<OverflowMode>,
    pub overflow_block: Option<OverflowMode>,
    pub overflow_inline: Option<OverflowMode>,
    pub overflow_clip_margin: Option<String>,
    pub overflow_anchor: Option<String>,
    pub visibility: Option<VisibilityMode>,
    pub z_index: Option<i32>,
    pub isolation: Option<IsolationMode>,
    pub mix_blend_mode: Option<BlendMode>,
    pub float: Option<FloatMode>,
    pub clear: Option<ClearMode>,
    pub vertical_align: Option<String>,
    pub table_layout: Option<TableLayout>,
    pub border_collapse: Option<BorderCollapse>,
    pub border_spacing: Option<String>,
    pub caption_side: Option<CaptionSide>,
    pub empty_cells: Option<String>,
    pub aspect_ratio: Option<String>,
    pub transform: Option<String>,
    pub translate: Option<String>,
    pub rotate: Option<String>,
    pub scale: Option<String>,
    pub transform_origin: Option<String>,
    pub transform_style: Option<String>,
    pub transform_box: Option<String>,
    pub offset: Option<String>,
    pub offset_path: Option<String>,
    pub offset_distance: Option<String>,
    pub offset_rotate: Option<String>,
    pub offset_anchor: Option<String>,
    pub offset_position: Option<String>,
    pub backface_visibility: Option<BackfaceVisibility>,
    pub perspective: Option<StyleLength>,
    pub perspective_origin: Option<String>,
    pub filter: Option<String>,
    pub filter_blur: Option<String>,
    pub filter_brightness: Option<String>,
    pub filter_contrast: Option<String>,
    pub filter_drop_shadow: Option<String>,
    pub filter_grayscale: Option<String>,
    pub filter_hue_rotate: Option<String>,
    pub filter_invert: Option<String>,
    pub filter_saturate: Option<String>,
    pub filter_sepia: Option<String>,
    pub backdrop_filter: Option<String>,
    pub backdrop_filter_blur: Option<String>,
    pub backdrop_filter_brightness: Option<String>,
    pub backdrop_filter_contrast: Option<String>,
    pub backdrop_filter_grayscale: Option<String>,
    pub backdrop_filter_hue_rotate: Option<String>,
    pub backdrop_filter_invert: Option<String>,
    pub backdrop_filter_opacity: Option<String>,
    pub backdrop_filter_saturate: Option<String>,
    pub backdrop_filter_sepia: Option<String>,
    pub transition: Option<String>,
    pub transition_property: Option<String>,
    pub transition_duration: Option<StyleTime>,
    pub transition_timing_function: Option<String>,
    pub transition_delay: Option<StyleTime>,
    pub transition_behavior: Option<String>,
    pub overlay: Option<String>,
    pub animation: Option<String>,
    pub animation_name: Option<String>,
    pub animation_duration: Option<StyleTime>,
    pub animation_timing_function: Option<String>,
    pub animation_delay: Option<StyleTime>,
    pub animation_iteration_count: Option<String>,
    pub animation_direction: Option<String>,
    pub animation_fill_mode: Option<String>,
    pub animation_play_state: Option<String>,
    pub animation_composition: Option<String>,
    pub animation_timeline: Option<String>,
    pub animation_range: Option<String>,
    pub animation_range_start: Option<String>,
    pub animation_range_end: Option<String>,
    pub view_transition_name: Option<String>,
    pub view_transition_class: Option<String>,
    pub view_transition_group: Option<String>,
    pub view_transition_scope: Option<String>,
    pub will_change: Option<String>,
    pub color_scheme: Option<String>,
    pub forced_color_adjust: Option<String>,
    pub print_color_adjust: Option<String>,
    pub color_adjust: Option<String>,
    pub field_sizing: Option<String>,
    pub appearance: Option<String>,
    pub caret: Option<String>,
    pub caret_animation: Option<String>,
    pub caret_shape: Option<String>,
    pub resize: Option<ResizeMode>,
    pub scroll_behavior: Option<ScrollBehavior>,
    pub scroll_timeline: Option<String>,
    pub scroll_timeline_name: Option<String>,
    pub scroll_timeline_axis: Option<String>,
    pub view_timeline: Option<String>,
    pub view_timeline_name: Option<String>,
    pub view_timeline_axis: Option<String>,
    pub view_timeline_inset: Option<String>,
    pub timeline_scope: Option<String>,
    pub scroll_snap_type: Option<String>,
    pub scroll_snap_align: Option<String>,
    pub scroll_snap_stop: Option<String>,
    pub scroll_initial_target: Option<String>,
    pub scroll_target_group: Option<String>,
    pub scroll_marker_group: Option<String>,
    pub scrollbar_gutter: Option<String>,
    pub scrollbar_width: Option<String>,
    pub scrollbar_color: Option<String>,
    pub scrollbar_thumb_color: Option<String>,
    pub scrollbar_track_color: Option<String>,
    pub overscroll_behavior_x: Option<OverscrollBehavior>,
    pub overscroll_behavior_y: Option<OverscrollBehavior>,
    pub overscroll_behavior_block: Option<OverscrollBehavior>,
    pub overscroll_behavior_inline: Option<OverscrollBehavior>,
    pub touch_action: Option<String>,
    pub nav_up: Option<String>,
    pub nav_right: Option<String>,
    pub nav_down: Option<String>,
    pub nav_left: Option<String>,
    pub spatial_navigation_action: Option<String>,
    pub spatial_navigation_contain: Option<String>,
    pub spatial_navigation_function: Option<String>,
    pub interactivity: Option<String>,
    pub cursor: Option<String>,
    pub pointer_events: Option<PointerEvents>,
    pub user_select: Option<UserSelect>,
    pub opacity: Option<f64>,
    pub unsupported: BTreeMap<String, String>,
}

impl PortableStyle {
    pub fn from_web(web: &WebProps) -> Self {
        let mut style = PortableStyle::default();
        if let Some(class_name) = &web.class_name {
            for class in tailwind::ordered_class_tokens(class_name) {
                style.apply_tailwind_utility(class);
            }
        }
        for (property, value) in &web.style {
            style.apply(property, value);
        }
        style
    }

    fn apply(&mut self, property: &str, value: &str) {
        let property = normalize_css_property_name(property);
        let value = normalize_css_value(value);
        let value_ref = value.as_str();
        self.record_declaration(&property, value_ref);
        if property.starts_with("--") {
            self.apply_tailwind_custom_property(&property, value_ref);
            return;
        }
        match property.as_str() {
            "all" => self.all = parse_css_string_token(value_ref),
            "display" => self.display = parse_display(value_ref),
            "box-sizing" => self.box_sizing = parse_box_sizing(value_ref),
            "box-decoration-break" => {
                self.box_decoration_break = parse_box_decoration_break(value_ref);
            }
            "position" => self.position = parse_position(value_ref),
            "anchor-name" => self.anchor_name = parse_css_string_token(value_ref),
            "anchor-scope" => self.anchor_scope = parse_css_string_token(value_ref),
            "position-anchor" => self.position_anchor = parse_css_string_token(value_ref),
            "position-area" => self.position_area = parse_css_string_token(value_ref),
            "position-try" => self.position_try = parse_css_string_token(value_ref),
            "position-try-fallbacks" => {
                self.position_try_fallbacks = parse_css_string_token(value_ref);
            }
            "position-try-order" => self.position_try_order = parse_css_string_token(value_ref),
            "position-try-options" => self.position_try_options = parse_css_string_token(value_ref),
            "position-visibility" => self.position_visibility = parse_css_string_token(value_ref),
            "flex-direction" => self.flex_direction = parse_flex_direction(value_ref),
            "flex-wrap" => self.flex_wrap = parse_flex_wrap(value_ref),
            "flex" => self.flex = parse_css_string_token(value_ref),
            "flex-basis" => self.flex_basis = parse_length(value_ref),
            "flex-grow" => self.flex_grow = parse_css_string_token(value_ref),
            "flex-shrink" => self.flex_shrink = parse_css_string_token(value_ref),
            "order" => self.order = parse_css_string_token(value_ref),
            "reading-flow" => self.reading_flow = parse_css_string_token(value_ref),
            "reading-order" => self.reading_order = parse_css_string_token(value_ref),
            "align-items" => self.align_items = parse_align_items(value_ref),
            "align-content" => self.align_content = parse_justify_content(value_ref),
            "align-self" => self.align_self = parse_self_alignment(value_ref),
            "justify-content" => self.justify_content = parse_justify_content(value_ref),
            "justify-items" => self.justify_items = parse_align_items(value_ref),
            "justify-self" => self.justify_self = parse_self_alignment(value_ref),
            "place-content" => self.place_content = parse_css_string_token(value_ref),
            "place-items" => self.place_items = parse_css_string_token(value_ref),
            "place-self" => self.place_self = parse_css_string_token(value_ref),
            "width" => self.width = parse_length(value_ref),
            "height" => self.height = parse_length(value_ref),
            "min-width" => self.min_width = parse_length(value_ref),
            "min-height" => self.min_height = parse_length(value_ref),
            "max-width" => self.max_width = parse_length(value_ref),
            "max-height" => self.max_height = parse_length(value_ref),
            "inline-size" => self.inline_size = parse_length(value_ref),
            "block-size" => self.block_size = parse_length(value_ref),
            "min-inline-size" => self.min_inline_size = parse_length(value_ref),
            "min-block-size" => self.min_block_size = parse_length(value_ref),
            "max-inline-size" => self.max_inline_size = parse_length(value_ref),
            "max-block-size" => self.max_block_size = parse_length(value_ref),
            "interpolate-size" => self.interpolate_size = parse_css_string_token(value_ref),
            "gap" => self.gap = parse_length(value_ref),
            "row-gap" => self.row_gap = parse_length(value_ref),
            "column-gap" => self.column_gap = parse_length(value_ref),
            "grid" => self.grid = parse_css_string_token(value_ref),
            "grid-template" => self.grid_template = parse_css_string_token(value_ref),
            "grid-template-columns" => {
                self.grid_template_columns = parse_css_string_token(value_ref);
            }
            "grid-template-rows" => self.grid_template_rows = parse_css_string_token(value_ref),
            "grid-template-areas" => self.grid_template_areas = parse_css_string_token(value_ref),
            "grid-auto-columns" => self.grid_auto_columns = parse_css_string_token(value_ref),
            "grid-auto-rows" => self.grid_auto_rows = parse_css_string_token(value_ref),
            "grid-auto-flow" => self.grid_auto_flow = parse_grid_auto_flow(value_ref),
            "grid-column" => self.grid_column = parse_css_string_token(value_ref),
            "grid-column-start" => self.grid_column_start = parse_css_string_token(value_ref),
            "grid-column-end" => self.grid_column_end = parse_css_string_token(value_ref),
            "grid-row" => self.grid_row = parse_css_string_token(value_ref),
            "grid-row-start" => self.grid_row_start = parse_css_string_token(value_ref),
            "grid-row-end" => self.grid_row_end = parse_css_string_token(value_ref),
            "grid-area" => self.grid_area = parse_css_string_token(value_ref),
            "contain" => self.contain = parse_css_string_token(value_ref),
            "container" => self.apply_container_shorthand(value_ref),
            "container-type" => self.container_type = parse_container_type(value_ref),
            "container-name" => self.container_name = parse_css_string_token(value_ref),
            "content" => self.content = parse_css_string_token(value_ref),
            "counter-reset" => self.counter_reset = parse_css_string_token(value_ref),
            "counter-increment" => self.counter_increment = parse_css_string_token(value_ref),
            "counter-set" => self.counter_set = parse_css_string_token(value_ref),
            "quotes" => self.quotes = parse_css_string_token(value_ref),
            "string-set" => self.string_set = parse_css_string_token(value_ref),
            "content-visibility" => {
                self.content_visibility = parse_content_visibility(value_ref);
            }
            "contain-intrinsic-size" => {
                self.contain_intrinsic_size = parse_css_string_token(value_ref);
            }
            "contain-intrinsic-width" => {
                self.contain_intrinsic_width = parse_css_string_token(value_ref);
            }
            "contain-intrinsic-height" => {
                self.contain_intrinsic_height = parse_css_string_token(value_ref);
            }
            "contain-intrinsic-inline-size" => {
                self.contain_intrinsic_inline_size = parse_css_string_token(value_ref);
            }
            "contain-intrinsic-block-size" => {
                self.contain_intrinsic_block_size = parse_css_string_token(value_ref);
            }
            "inset" => self.inset = parse_edge_insets(value_ref),
            "inset-block" => {
                self.inset
                    .apply_edges_opt(EdgeSelection::Y, parse_length(value_ref));
                self.logical_inset
                    .apply_axis_values(LogicalEdgeSelection::Block, value_ref);
            }
            "inset-inline" => {
                self.inset
                    .apply_edges_opt(EdgeSelection::X, parse_length(value_ref));
                self.logical_inset
                    .apply_axis_values(LogicalEdgeSelection::Inline, value_ref);
            }
            "inset-block-start" => {
                self.logical_inset.block_start = parse_length(value_ref);
            }
            "inset-block-end" => {
                self.logical_inset.block_end = parse_length(value_ref);
            }
            "inset-inline-start" | "start" => {
                self.logical_inset.inline_start = parse_length(value_ref);
            }
            "inset-inline-end" | "end" => {
                self.logical_inset.inline_end = parse_length(value_ref);
            }
            "top" => self.inset.top = parse_length(value_ref),
            "right" => self.inset.right = parse_length(value_ref),
            "bottom" => self.inset.bottom = parse_length(value_ref),
            "left" => self.inset.left = parse_length(value_ref),
            "padding" => self.padding = parse_edge_insets(value_ref),
            "padding-block" => {
                self.padding
                    .apply_edges_opt(EdgeSelection::Y, parse_length(value_ref));
                self.logical_padding
                    .apply_axis_values(LogicalEdgeSelection::Block, value_ref);
            }
            "padding-inline" => {
                self.padding
                    .apply_edges_opt(EdgeSelection::X, parse_length(value_ref));
                self.logical_padding
                    .apply_axis_values(LogicalEdgeSelection::Inline, value_ref);
            }
            "padding-block-start" => {
                self.logical_padding.block_start = parse_length(value_ref);
            }
            "padding-block-end" => {
                self.logical_padding.block_end = parse_length(value_ref);
            }
            "padding-inline-start" => {
                self.logical_padding.inline_start = parse_length(value_ref);
            }
            "padding-inline-end" => {
                self.logical_padding.inline_end = parse_length(value_ref);
            }
            "padding-top" => self.padding.top = parse_length(value_ref),
            "padding-right" => self.padding.right = parse_length(value_ref),
            "padding-bottom" => self.padding.bottom = parse_length(value_ref),
            "padding-left" => self.padding.left = parse_length(value_ref),
            "margin" => self.margin = parse_edge_insets(value_ref),
            "margin-block" => {
                self.margin
                    .apply_edges_opt(EdgeSelection::Y, parse_length(value_ref));
                self.logical_margin
                    .apply_axis_values(LogicalEdgeSelection::Block, value_ref);
            }
            "margin-inline" => {
                self.margin
                    .apply_edges_opt(EdgeSelection::X, parse_length(value_ref));
                self.logical_margin
                    .apply_axis_values(LogicalEdgeSelection::Inline, value_ref);
            }
            "margin-block-start" => {
                self.logical_margin.block_start = parse_length(value_ref);
            }
            "margin-block-end" => {
                self.logical_margin.block_end = parse_length(value_ref);
            }
            "margin-inline-start" => {
                self.logical_margin.inline_start = parse_length(value_ref);
            }
            "margin-inline-end" => {
                self.logical_margin.inline_end = parse_length(value_ref);
            }
            "margin-top" => self.margin.top = parse_length(value_ref),
            "margin-right" => self.margin.right = parse_length(value_ref),
            "margin-bottom" => self.margin.bottom = parse_length(value_ref),
            "margin-left" => self.margin.left = parse_length(value_ref),
            "margin-trim" => self.margin_trim = parse_css_string_token(value_ref),
            "space-x" => self.space_x = parse_length(value_ref),
            "space-y" => self.space_y = parse_length(value_ref),
            "scroll-margin" => self.scroll_margin = parse_edge_insets(value_ref),
            "scroll-margin-block" => {
                self.scroll_margin
                    .apply_edges_opt(EdgeSelection::Y, parse_length(value_ref));
                self.logical_scroll_margin
                    .apply_axis_values(LogicalEdgeSelection::Block, value_ref);
            }
            "scroll-margin-inline" => {
                self.scroll_margin
                    .apply_edges_opt(EdgeSelection::X, parse_length(value_ref));
                self.logical_scroll_margin
                    .apply_axis_values(LogicalEdgeSelection::Inline, value_ref);
            }
            "scroll-margin-block-start" => {
                self.logical_scroll_margin.block_start = parse_length(value_ref);
            }
            "scroll-margin-block-end" => {
                self.logical_scroll_margin.block_end = parse_length(value_ref);
            }
            "scroll-margin-inline-start" => {
                self.logical_scroll_margin.inline_start = parse_length(value_ref);
            }
            "scroll-margin-inline-end" => {
                self.logical_scroll_margin.inline_end = parse_length(value_ref);
            }
            "scroll-margin-top" => self.scroll_margin.top = parse_length(value_ref),
            "scroll-margin-right" => self.scroll_margin.right = parse_length(value_ref),
            "scroll-margin-bottom" => self.scroll_margin.bottom = parse_length(value_ref),
            "scroll-margin-left" => self.scroll_margin.left = parse_length(value_ref),
            "scroll-padding" => self.scroll_padding = parse_edge_insets(value_ref),
            "scroll-padding-block" => {
                self.scroll_padding
                    .apply_edges_opt(EdgeSelection::Y, parse_length(value_ref));
                self.logical_scroll_padding
                    .apply_axis_values(LogicalEdgeSelection::Block, value_ref);
            }
            "scroll-padding-inline" => {
                self.scroll_padding
                    .apply_edges_opt(EdgeSelection::X, parse_length(value_ref));
                self.logical_scroll_padding
                    .apply_axis_values(LogicalEdgeSelection::Inline, value_ref);
            }
            "scroll-padding-block-start" => {
                self.logical_scroll_padding.block_start = parse_length(value_ref);
            }
            "scroll-padding-block-end" => {
                self.logical_scroll_padding.block_end = parse_length(value_ref);
            }
            "scroll-padding-inline-start" => {
                self.logical_scroll_padding.inline_start = parse_length(value_ref);
            }
            "scroll-padding-inline-end" => {
                self.logical_scroll_padding.inline_end = parse_length(value_ref);
            }
            "scroll-padding-top" => self.scroll_padding.top = parse_length(value_ref),
            "scroll-padding-right" => self.scroll_padding.right = parse_length(value_ref),
            "scroll-padding-bottom" => self.scroll_padding.bottom = parse_length(value_ref),
            "scroll-padding-left" => self.scroll_padding.left = parse_length(value_ref),
            "border" => self.apply_border_shorthand(value_ref),
            "border-width" => self.border_width = parse_edge_insets(value_ref),
            "border-block-width" => {
                self.border_width
                    .apply_edges_opt(EdgeSelection::Y, parse_length(value_ref));
                self.logical_border_width
                    .apply_axis_values(LogicalEdgeSelection::Block, value_ref);
            }
            "border-inline-width" => {
                self.border_width
                    .apply_edges_opt(EdgeSelection::X, parse_length(value_ref));
                self.logical_border_width
                    .apply_axis_values(LogicalEdgeSelection::Inline, value_ref);
            }
            "border-block-start-width" => {
                self.logical_border_width.block_start = parse_length(value_ref);
            }
            "border-block-end-width" => {
                self.logical_border_width.block_end = parse_length(value_ref);
            }
            "border-inline-start-width" => {
                self.logical_border_width.inline_start = parse_length(value_ref);
            }
            "border-inline-end-width" => {
                self.logical_border_width.inline_end = parse_length(value_ref);
            }
            "border-top-width" => self.border_width.top = parse_length(value_ref),
            "border-right-width" => self.border_width.right = parse_length(value_ref),
            "border-bottom-width" => self.border_width.bottom = parse_length(value_ref),
            "border-left-width" => self.border_width.left = parse_length(value_ref),
            "border-top" => self.apply_border_side_shorthand(EdgeSelection::Top, value_ref),
            "border-right" => self.apply_border_side_shorthand(EdgeSelection::Right, value_ref),
            "border-bottom" => self.apply_border_side_shorthand(EdgeSelection::Bottom, value_ref),
            "border-left" => self.apply_border_side_shorthand(EdgeSelection::Left, value_ref),
            "border-block" => {
                self.apply_border_side_shorthand(EdgeSelection::Y, value_ref);
                self.apply_logical_border_side_shorthand(LogicalEdgeSelection::Block, value_ref);
            }
            "border-inline" => {
                self.apply_border_side_shorthand(EdgeSelection::X, value_ref);
                self.apply_logical_border_side_shorthand(LogicalEdgeSelection::Inline, value_ref);
            }
            "border-block-start" => self
                .apply_logical_border_side_shorthand(LogicalEdgeSelection::BlockStart, value_ref),
            "border-block-end" => {
                self.apply_logical_border_side_shorthand(LogicalEdgeSelection::BlockEnd, value_ref);
            }
            "border-inline-start" => self
                .apply_logical_border_side_shorthand(LogicalEdgeSelection::InlineStart, value_ref),
            "border-inline-end" => {
                self.apply_logical_border_side_shorthand(
                    LogicalEdgeSelection::InlineEnd,
                    value_ref,
                );
            }
            "border-color" => {
                self.border_color = parse_color(value_ref);
                self.border_colors = parse_edge_colors(value_ref);
            }
            "border-block-color" => {
                if let Some(color) = parse_color(value_ref) {
                    self.border_colors.apply_edges(EdgeSelection::Y, color);
                }
                self.logical_border_colors
                    .apply_axis_values(LogicalEdgeSelection::Block, value_ref);
            }
            "border-inline-color" => {
                if let Some(color) = parse_color(value_ref) {
                    self.border_colors.apply_edges(EdgeSelection::X, color);
                }
                self.logical_border_colors
                    .apply_axis_values(LogicalEdgeSelection::Inline, value_ref);
            }
            "border-block-start-color" => {
                self.logical_border_colors.block_start = parse_color(value_ref);
            }
            "border-block-end-color" => {
                self.logical_border_colors.block_end = parse_color(value_ref);
            }
            "border-inline-start-color" => {
                self.logical_border_colors.inline_start = parse_color(value_ref);
            }
            "border-inline-end-color" => {
                self.logical_border_colors.inline_end = parse_color(value_ref);
            }
            "border-top-color" => self.border_colors.top = parse_color(value_ref),
            "border-right-color" => self.border_colors.right = parse_color(value_ref),
            "border-bottom-color" => self.border_colors.bottom = parse_color(value_ref),
            "border-left-color" => self.border_colors.left = parse_color(value_ref),
            "border-style" => {
                self.border_style = parse_border_style(value_ref);
                self.border_styles = parse_edge_border_styles(value_ref);
            }
            "border-block-style" => {
                if let Some(style) = parse_border_style(value_ref) {
                    self.border_styles.apply_edges(EdgeSelection::Y, style);
                }
                self.logical_border_styles
                    .apply_axis_values(LogicalEdgeSelection::Block, value_ref);
            }
            "border-inline-style" => {
                if let Some(style) = parse_border_style(value_ref) {
                    self.border_styles.apply_edges(EdgeSelection::X, style);
                }
                self.logical_border_styles
                    .apply_axis_values(LogicalEdgeSelection::Inline, value_ref);
            }
            "border-block-start-style" => {
                self.logical_border_styles.block_start = parse_border_style(value_ref);
            }
            "border-block-end-style" => {
                self.logical_border_styles.block_end = parse_border_style(value_ref);
            }
            "border-inline-start-style" => {
                self.logical_border_styles.inline_start = parse_border_style(value_ref);
            }
            "border-inline-end-style" => {
                self.logical_border_styles.inline_end = parse_border_style(value_ref);
            }
            "border-top-style" => self.border_styles.top = parse_border_style(value_ref),
            "border-right-style" => self.border_styles.right = parse_border_style(value_ref),
            "border-bottom-style" => self.border_styles.bottom = parse_border_style(value_ref),
            "border-left-style" => self.border_styles.left = parse_border_style(value_ref),
            "border-image" => self.border_image = parse_css_string_token(value_ref),
            "border-image-source" => {
                self.border_image_source = parse_css_string_token(value_ref);
            }
            "border-image-slice" => {
                self.border_image_slice = parse_css_string_token(value_ref);
            }
            "border-image-width" => {
                self.border_image_width = parse_css_string_token(value_ref);
            }
            "border-image-outset" => {
                self.border_image_outset = parse_css_string_token(value_ref);
            }
            "border-image-repeat" => {
                self.border_image_repeat = parse_css_string_token(value_ref);
            }
            "box-shadow" => self.apply_box_shadow_property(value_ref),
            "outline" => self.apply_outline_shorthand(value_ref),
            "outline-width" => self.outline_width = parse_length(value_ref),
            "outline-color" => self.outline_color = parse_color(value_ref),
            "outline-style" => self.outline_style = parse_border_style(value_ref),
            "outline-offset" => self.outline_offset = parse_length(value_ref),
            "divide-x-width" => self.divide_x_width = parse_length(value_ref),
            "divide-y-width" => self.divide_y_width = parse_length(value_ref),
            "divide-color" => self.divide_color = parse_color(value_ref),
            "divide-style" => self.divide_style = parse_border_style(value_ref),
            "color" => self.color = parse_color(value_ref),
            "accent-color" => self.accent_color = parse_color(value_ref),
            "caret-color" => self.caret_color = parse_color(value_ref),
            "background" => self.apply_background_shorthand(value_ref),
            "background-color" => self.background_color = parse_color(value_ref),
            "background-image" => self.background_image = parse_css_string_token(value_ref),
            "background-position" => self.background_position = parse_css_string_token(value_ref),
            "background-size" => self.background_size = parse_css_string_token(value_ref),
            "background-repeat" => self.background_repeat = parse_css_string_token(value_ref),
            "background-attachment" => {
                self.background_attachment = parse_background_attachment(value_ref);
            }
            "background-origin" => self.background_origin = parse_background_box(value_ref),
            "background-clip" => self.background_clip = parse_background_box(value_ref),
            "background-blend-mode" => {
                self.background_blend_mode = parse_css_string_token(value_ref);
            }
            "clip" => self.clip = parse_css_string_token(value_ref),
            "clip-path" => self.clip_path = parse_css_string_token(value_ref),
            "mask" | "-webkit-mask" => self.mask = parse_css_string_token(value_ref),
            "mask-image" | "-webkit-mask-image" => {
                self.mask_image = parse_css_string_token(value_ref);
            }
            "mask-mode" | "-webkit-mask-mode" => {
                self.mask_mode = parse_css_string_token(value_ref);
            }
            "mask-repeat" | "-webkit-mask-repeat" => {
                self.mask_repeat = parse_css_string_token(value_ref);
            }
            "mask-position" | "-webkit-mask-position" => {
                self.mask_position = parse_css_string_token(value_ref);
            }
            "mask-size" | "-webkit-mask-size" => {
                self.mask_size = parse_css_string_token(value_ref);
            }
            "mask-origin" | "-webkit-mask-origin" => {
                self.mask_origin = parse_css_string_token(value_ref);
            }
            "mask-clip" | "-webkit-mask-clip" => {
                self.mask_clip = parse_css_string_token(value_ref);
            }
            "mask-composite" | "-webkit-mask-composite" => {
                self.mask_composite = parse_css_string_token(value_ref);
            }
            "mask-type" => self.mask_type = parse_css_string_token(value_ref),
            "mask-border" => self.mask_border = parse_css_string_token(value_ref),
            "mask-border-source" => {
                self.mask_border_source = parse_css_string_token(value_ref);
            }
            "mask-border-mode" => self.mask_border_mode = parse_css_string_token(value_ref),
            "mask-border-slice" => {
                self.mask_border_slice = parse_css_string_token(value_ref);
            }
            "mask-border-width" => {
                self.mask_border_width = parse_css_string_token(value_ref);
            }
            "mask-border-outset" => {
                self.mask_border_outset = parse_css_string_token(value_ref);
            }
            "mask-border-repeat" => {
                self.mask_border_repeat = parse_css_string_token(value_ref);
            }
            "border-radius" => {
                self.border_radius = parse_length(value_ref);
                self.border_radii = parse_corner_radii(value_ref);
            }
            "border-top-left-radius" => {
                self.border_radii.top_left = parse_corner_radius(value_ref);
            }
            "border-top-right-radius" => {
                self.border_radii.top_right = parse_corner_radius(value_ref);
            }
            "border-bottom-right-radius" => {
                self.border_radii.bottom_right = parse_corner_radius(value_ref);
            }
            "border-bottom-left-radius" => {
                self.border_radii.bottom_left = parse_corner_radius(value_ref);
            }
            "border-start-start-radius" => {
                self.logical_border_radii.start_start = parse_corner_radius(value_ref);
            }
            "border-start-end-radius" => {
                self.logical_border_radii.start_end = parse_corner_radius(value_ref);
            }
            "border-end-end-radius" => {
                self.logical_border_radii.end_end = parse_corner_radius(value_ref);
            }
            "border-end-start-radius" => {
                self.logical_border_radii.end_start = parse_corner_radius(value_ref);
            }
            "image-rendering" => self.image_rendering = parse_css_string_token(value_ref),
            "image-orientation" => self.image_orientation = parse_css_string_token(value_ref),
            "image-resolution" => self.image_resolution = parse_css_string_token(value_ref),
            "object-fit" => self.object_fit = parse_object_fit(value_ref),
            "object-position" => self.object_position = parse_css_string_token(value_ref),
            "shape-outside" => self.shape_outside = parse_css_string_token(value_ref),
            "shape-inside" => self.shape_inside = parse_css_string_token(value_ref),
            "shape-margin" => self.shape_margin = parse_length(value_ref),
            "shape-padding" => self.shape_padding = parse_length(value_ref),
            "shape-image-threshold" => self.shape_image_threshold = parse_opacity(value_ref),
            "list-style" => self.list_style_type = parse_css_string_token(value_ref),
            "list-style-type" => self.list_style_type = parse_css_string_token(value_ref),
            "list-style-position" => {
                self.list_style_position = parse_list_style_position(value_ref);
            }
            "list-style-image" => self.list_style_image = parse_css_string_token(value_ref),
            "marker-side" => self.marker_side = parse_css_string_token(value_ref),
            "columns" => self.columns = parse_css_string_token(value_ref),
            "column-count" => self.column_count = parse_css_string_token(value_ref),
            "column-width" => self.column_width = parse_length(value_ref),
            "column-rule" => self.apply_column_rule_shorthand(value_ref),
            "column-rule-width" => self.column_rule_width = parse_length(value_ref),
            "column-rule-style" => self.column_rule_style = parse_border_style(value_ref),
            "column-rule-color" => self.column_rule_color = parse_color(value_ref),
            "column-span" => self.column_span = parse_css_string_token(value_ref),
            "column-fill" => self.column_fill = parse_css_string_token(value_ref),
            "size" => self.page_size = parse_css_string_token(value_ref),
            "page" => self.page = parse_css_string_token(value_ref),
            "page-orientation" => self.page_orientation = parse_css_string_token(value_ref),
            "bleed" => self.bleed = parse_css_string_token(value_ref),
            "marks" => self.marks = parse_css_string_token(value_ref),
            "orphans" => self.orphans = parse_css_string_token(value_ref),
            "widows" => self.widows = parse_css_string_token(value_ref),
            "bookmark-label" => self.bookmark_label = parse_css_string_token(value_ref),
            "bookmark-level" => self.bookmark_level = parse_css_string_token(value_ref),
            "bookmark-state" => self.bookmark_state = parse_css_string_token(value_ref),
            "footnote-display" => self.footnote_display = parse_css_string_token(value_ref),
            "footnote-policy" => self.footnote_policy = parse_css_string_token(value_ref),
            "break-before" | "page-break-before" => {
                self.break_before = parse_css_string_token(value_ref);
            }
            "break-after" | "page-break-after" => {
                self.break_after = parse_css_string_token(value_ref);
            }
            "break-inside" | "page-break-inside" => {
                self.break_inside = parse_css_string_token(value_ref);
            }
            "font" => self.font = parse_css_string_token(value_ref),
            "font-family" => self.font_family = parse_css_string_token(value_ref),
            "font-style" => self.font_style = parse_font_style(value_ref),
            "font-size" => self.font_size = parse_length(value_ref),
            "font-size-adjust" => self.font_size_adjust = parse_css_string_token(value_ref),
            "font-weight" => self.font_weight = parse_font_weight(value_ref),
            "font-stretch" => self.font_stretch = parse_css_string_token(value_ref),
            "font-width" => self.font_width = parse_css_string_token(value_ref),
            "font-palette" => self.font_palette = parse_css_string_token(value_ref),
            "font-language-override"
            | "-moz-font-language-override"
            | "moz-font-language-override" => {
                self.font_language_override = parse_css_string_token(value_ref);
            }
            "font-kerning" => self.font_kerning = parse_css_string_token(value_ref),
            "font-optical-sizing" => {
                self.font_optical_sizing = parse_css_string_token(value_ref);
            }
            "-webkit-font-smoothing" | "webkit-font-smoothing" => {
                self.webkit_font_smoothing = parse_css_string_token(value_ref);
            }
            "-moz-osx-font-smoothing" | "moz-osx-font-smoothing" => {
                self.moz_osx_font_smoothing = parse_css_string_token(value_ref);
            }
            "font-feature-settings" => {
                self.font_feature_settings = parse_css_string_token(value_ref);
            }
            "font-variation-settings" => {
                self.font_variation_settings = parse_css_string_token(value_ref);
            }
            "font-variant" => self.font_variant = parse_css_string_token(value_ref),
            "font-variant-alternates" => {
                self.font_variant_alternates = parse_css_string_token(value_ref);
            }
            "font-variant-caps" => self.font_variant_caps = parse_css_string_token(value_ref),
            "font-variant-east-asian" => {
                self.font_variant_east_asian = parse_css_string_token(value_ref);
            }
            "font-variant-emoji" => {
                self.font_variant_emoji = parse_css_string_token(value_ref);
            }
            "font-variant-ligatures" => {
                self.font_variant_ligatures = parse_css_string_token(value_ref);
            }
            "font-variant-numeric" => self.apply_font_variant_numeric(value_ref),
            "font-variant-position" => {
                self.font_variant_position = parse_css_string_token(value_ref);
            }
            "font-synthesis" => self.font_synthesis = parse_css_string_token(value_ref),
            "font-synthesis-weight" => {
                self.font_synthesis_weight = parse_css_string_token(value_ref);
            }
            "font-synthesis-style" => {
                self.font_synthesis_style = parse_css_string_token(value_ref);
            }
            "font-synthesis-small-caps" => {
                self.font_synthesis_small_caps = parse_css_string_token(value_ref);
            }
            "font-synthesis-position" => {
                self.font_synthesis_position = parse_css_string_token(value_ref);
            }
            "line-height" => self.line_height = parse_length(value_ref),
            "line-height-step" => self.line_height_step = parse_css_string_token(value_ref),
            "block-step" => self.block_step = parse_css_string_token(value_ref),
            "block-step-size" => self.block_step_size = parse_css_string_token(value_ref),
            "block-step-insert" => self.block_step_insert = parse_css_string_token(value_ref),
            "block-step-align" => self.block_step_align = parse_css_string_token(value_ref),
            "block-step-round" => self.block_step_round = parse_css_string_token(value_ref),
            "line-grid" => self.line_grid = parse_css_string_token(value_ref),
            "line-snap" => self.line_snap = parse_css_string_token(value_ref),
            "box-snap" => self.box_snap = parse_css_string_token(value_ref),
            "math-depth" => self.math_depth = parse_css_string_token(value_ref),
            "math-shift" => self.math_shift = parse_css_string_token(value_ref),
            "math-style" => self.math_style = parse_css_string_token(value_ref),
            "dominant-baseline" => self.dominant_baseline = parse_css_string_token(value_ref),
            "baseline-source" => self.baseline_source = parse_css_string_token(value_ref),
            "alignment-baseline" => self.alignment_baseline = parse_css_string_token(value_ref),
            "baseline-shift" => self.baseline_shift = parse_css_string_token(value_ref),
            "line-fit-edge" => self.line_fit_edge = parse_css_string_token(value_ref),
            "inline-sizing" => self.inline_sizing = parse_css_string_token(value_ref),
            "initial-letter" => self.initial_letter = parse_css_string_token(value_ref),
            "initial-letter-align" => {
                self.initial_letter_align = parse_css_string_token(value_ref);
            }
            "initial-letter-wrap" => self.initial_letter_wrap = parse_css_string_token(value_ref),
            "letter-spacing" => self.letter_spacing = parse_length(value_ref),
            "word-spacing" => self.word_spacing = parse_length(value_ref),
            "tab-size" | "-moz-tab-size" => self.tab_size = parse_css_string_token(value_ref),
            "text-align" => self.text_align = parse_text_align(value_ref),
            "text-align-all" => self.text_align_all = parse_css_string_token(value_ref),
            "text-align-last" => self.text_align_last = parse_css_string_token(value_ref),
            "text-group-align" => self.text_group_align = parse_css_string_token(value_ref),
            "text-justify" => self.text_justify = parse_css_string_token(value_ref),
            "word-space-transform" => self.word_space_transform = parse_css_string_token(value_ref),
            "text-size-adjust" => self.text_size_adjust = parse_css_string_token(value_ref),
            "-webkit-text-size-adjust" | "webkit-text-size-adjust" => {
                self.webkit_text_size_adjust = parse_css_string_token(value_ref);
            }
            "-moz-text-size-adjust" | "moz-text-size-adjust" => {
                self.moz_text_size_adjust = parse_css_string_token(value_ref);
            }
            "-ms-text-size-adjust" | "ms-text-size-adjust" => {
                self.ms_text_size_adjust = parse_css_string_token(value_ref);
            }
            "direction" => self.direction = parse_text_direction(value_ref),
            "unicode-bidi" => self.unicode_bidi = parse_unicode_bidi(value_ref),
            "writing-mode" | "-webkit-writing-mode" => {
                self.writing_mode = parse_writing_mode(value_ref);
            }
            "text-orientation" => self.text_orientation = parse_text_orientation(value_ref),
            "text-combine-upright"
            | "-webkit-text-combine"
            | "webkit-text-combine"
            | "-ms-text-combine-horizontal"
            | "ms-text-combine-horizontal" => {
                self.text_combine_upright = parse_css_string_token(value_ref);
            }
            "text-transform" => self.text_transform = parse_text_transform(value_ref),
            "text-indent" => self.text_indent = parse_length(value_ref),
            "text-wrap" => {
                self.text_wrap = parse_text_wrap(value_ref);
                self.text_wrap_mode = parse_css_string_token(value_ref);
            }
            "text-wrap-mode" => {
                self.text_wrap = parse_text_wrap(value_ref);
                self.text_wrap_mode = parse_css_string_token(value_ref);
            }
            "text-wrap-style" => self.text_wrap_style = parse_css_string_token(value_ref),
            "wrap-before" => self.wrap_before = parse_css_string_token(value_ref),
            "wrap-after" => self.wrap_after = parse_css_string_token(value_ref),
            "wrap-inside" => self.wrap_inside = parse_css_string_token(value_ref),
            "line-padding" => self.line_padding = parse_css_string_token(value_ref),
            "text-spacing" => self.text_spacing = parse_css_string_token(value_ref),
            "text-spacing-trim" => self.text_spacing_trim = parse_css_string_token(value_ref),
            "text-autospace" | "-ms-text-autospace" | "ms-text-autospace" => {
                self.text_autospace = parse_css_string_token(value_ref);
            }
            "text-box" => self.text_box = parse_css_string_token(value_ref),
            "text-box-trim" => self.text_box_trim = parse_css_string_token(value_ref),
            "text-box-edge" => self.text_box_edge = parse_css_string_token(value_ref),
            "hanging-punctuation" => self.hanging_punctuation = parse_css_string_token(value_ref),
            "line-clamp" | "-webkit-line-clamp" => {
                self.line_clamp = parse_css_string_token(value_ref);
            }
            "block-ellipsis" => self.block_ellipsis = parse_css_string_token(value_ref),
            "continue" => self.continue_mode = parse_css_string_token(value_ref),
            "max-lines" => self.max_lines = parse_css_string_token(value_ref),
            "box-orient" | "-webkit-box-orient" => {
                self.box_orient = parse_css_string_token(value_ref);
            }
            "speak" => self.speak = parse_css_string_token(value_ref),
            "speak-as" => self.speak_as = parse_css_string_token(value_ref),
            "pause" => self.pause = parse_css_string_token(value_ref),
            "pause-before" => self.pause_before = parse_css_string_token(value_ref),
            "pause-after" => self.pause_after = parse_css_string_token(value_ref),
            "rest" => self.rest = parse_css_string_token(value_ref),
            "rest-before" => self.rest_before = parse_css_string_token(value_ref),
            "rest-after" => self.rest_after = parse_css_string_token(value_ref),
            "cue" => self.cue = parse_css_string_token(value_ref),
            "cue-before" => self.cue_before = parse_css_string_token(value_ref),
            "cue-after" => self.cue_after = parse_css_string_token(value_ref),
            "voice-family" => self.voice_family = parse_css_string_token(value_ref),
            "voice-balance" => self.voice_balance = parse_css_string_token(value_ref),
            "voice-duration" => self.voice_duration = parse_css_string_token(value_ref),
            "voice-pitch" => self.voice_pitch = parse_css_string_token(value_ref),
            "voice-range" => self.voice_range = parse_css_string_token(value_ref),
            "voice-rate" => self.voice_rate = parse_css_string_token(value_ref),
            "voice-stress" => self.voice_stress = parse_css_string_token(value_ref),
            "voice-volume" => self.voice_volume = parse_css_string_token(value_ref),
            "fill" => self.fill = parse_color(value_ref),
            "fill-opacity" => self.fill_opacity = parse_opacity(value_ref),
            "fill-rule" => self.fill_rule = parse_fill_rule(value_ref),
            "clip-rule" => self.clip_rule = parse_fill_rule(value_ref),
            "stroke" => self.stroke = parse_color(value_ref),
            "stroke-width" => self.stroke_width = parse_length(value_ref),
            "stroke-linecap" => self.stroke_linecap = parse_stroke_linecap(value_ref),
            "stroke-linejoin" => self.stroke_linejoin = parse_stroke_linejoin(value_ref),
            "stroke-miterlimit" => self.stroke_miterlimit = parse_css_string_token(value_ref),
            "stroke-dasharray" => self.stroke_dasharray = parse_css_string_token(value_ref),
            "stroke-dashoffset" => self.stroke_dashoffset = parse_length(value_ref),
            "stroke-opacity" => self.stroke_opacity = parse_opacity(value_ref),
            "vector-effect" => self.vector_effect = parse_css_string_token(value_ref),
            "paint-order" => self.paint_order = parse_css_string_token(value_ref),
            "shape-rendering" => self.shape_rendering = parse_css_string_token(value_ref),
            "text-rendering" => self.text_rendering = parse_css_string_token(value_ref),
            "color-rendering" => self.color_rendering = parse_css_string_token(value_ref),
            "color-interpolation" => self.color_interpolation = parse_css_string_token(value_ref),
            "color-interpolation-filters" => {
                self.color_interpolation_filters = parse_css_string_token(value_ref);
            }
            "marker" => self.marker = parse_css_string_token(value_ref),
            "marker-start" => self.marker_start = parse_css_string_token(value_ref),
            "marker-mid" => self.marker_mid = parse_css_string_token(value_ref),
            "marker-end" => self.marker_end = parse_css_string_token(value_ref),
            "stop-color" => self.stop_color = parse_color(value_ref),
            "stop-opacity" => self.stop_opacity = parse_opacity(value_ref),
            "flood-color" => self.flood_color = parse_color(value_ref),
            "flood-opacity" => self.flood_opacity = parse_opacity(value_ref),
            "lighting-color" => self.lighting_color = parse_color(value_ref),
            "text-decoration" => self.apply_text_decoration_shorthand(value_ref),
            "text-decoration-line" => self.text_decoration_line = parse_css_string_token(value_ref),
            "text-decoration-color" => self.text_decoration_color = parse_color(value_ref),
            "text-decoration-style" => {
                self.text_decoration_style = parse_text_decoration_style(value_ref);
            }
            "text-decoration-thickness" => self.text_decoration_thickness = parse_length(value_ref),
            "text-decoration-skip" => {
                self.text_decoration_skip = parse_css_string_token(value_ref);
            }
            "text-decoration-skip-box" => {
                self.text_decoration_skip_box = parse_css_string_token(value_ref);
            }
            "text-decoration-skip-ink" => {
                self.text_decoration_skip_ink = parse_css_string_token(value_ref);
            }
            "text-decoration-skip-inset" => {
                self.text_decoration_skip_inset = parse_css_string_token(value_ref);
            }
            "text-decoration-skip-self" => {
                self.text_decoration_skip_self = parse_css_string_token(value_ref);
            }
            "text-decoration-skip-spaces" => {
                self.text_decoration_skip_spaces = parse_css_string_token(value_ref);
            }
            "text-underline-offset" => self.text_underline_offset = parse_length(value_ref),
            "text-underline-position" => {
                self.text_underline_position = parse_css_string_token(value_ref);
            }
            "text-emphasis" | "-webkit-text-emphasis" | "webkit-text-emphasis" => {
                self.apply_text_emphasis_shorthand(value_ref);
            }
            "text-emphasis-style"
            | "-webkit-text-emphasis-style"
            | "webkit-text-emphasis-style" => {
                self.text_emphasis_style = parse_css_string_token(value_ref);
            }
            "text-emphasis-color"
            | "-webkit-text-emphasis-color"
            | "webkit-text-emphasis-color" => {
                self.text_emphasis_color = parse_color(value_ref);
            }
            "text-emphasis-position"
            | "-webkit-text-emphasis-position"
            | "webkit-text-emphasis-position" => {
                self.text_emphasis_position = parse_css_string_token(value_ref);
            }
            "text-emphasis-skip" | "-webkit-text-emphasis-skip" | "webkit-text-emphasis-skip" => {
                self.text_emphasis_skip = parse_css_string_token(value_ref);
            }
            "ruby-align" => self.ruby_align = parse_css_string_token(value_ref),
            "ruby-position" => self.ruby_position = parse_css_string_token(value_ref),
            "ruby-merge" => self.ruby_merge = parse_css_string_token(value_ref),
            "ruby-overhang" => self.ruby_overhang = parse_css_string_token(value_ref),
            "text-shadow" => self.text_shadow = parse_css_string_token(value_ref),
            "text-overflow" => self.text_overflow = parse_text_overflow(value_ref),
            "line-break" => self.line_break = parse_css_string_token(value_ref),
            "white-space" => self.white_space = parse_white_space(value_ref),
            "white-space-collapse" => {
                self.white_space_collapse = parse_css_string_token(value_ref);
            }
            "white-space-trim" => self.white_space_trim = parse_css_string_token(value_ref),
            "word-break" => self.word_break = parse_word_break(value_ref),
            "overflow-wrap" => self.overflow_wrap = parse_overflow_wrap(value_ref),
            "hyphens" => self.hyphens = parse_hyphens(value_ref),
            "hyphenate-character" => self.hyphenate_character = parse_css_string_token(value_ref),
            "hyphenate-limit-zone" => self.hyphenate_limit_zone = parse_css_string_token(value_ref),
            "hyphenate-limit-chars" => {
                self.hyphenate_limit_chars = parse_css_string_token(value_ref);
            }
            "hyphenate-limit-lines" => {
                self.hyphenate_limit_lines = parse_css_string_token(value_ref);
            }
            "hyphenate-limit-last" => {
                self.hyphenate_limit_last = parse_css_string_token(value_ref);
            }
            "overflow" => {
                let overflow = parse_overflow(value_ref);
                self.overflow_x = overflow;
                self.overflow_y = overflow;
            }
            "overflow-x" => self.overflow_x = parse_overflow(value_ref),
            "overflow-y" => self.overflow_y = parse_overflow(value_ref),
            "overflow-block" => self.overflow_block = parse_overflow(value_ref),
            "overflow-inline" => self.overflow_inline = parse_overflow(value_ref),
            "overflow-clip-margin" => {
                self.overflow_clip_margin = parse_css_string_token(value_ref);
            }
            "overflow-anchor" => self.overflow_anchor = parse_css_string_token(value_ref),
            "visibility" => self.visibility = parse_visibility(value_ref),
            "z-index" => self.z_index = parse_z_index(value_ref),
            "isolation" => self.isolation = parse_isolation(value_ref),
            "mix-blend-mode" => self.mix_blend_mode = parse_blend_mode(value_ref),
            "float" => self.float = parse_float(value_ref),
            "clear" => self.clear = parse_clear(value_ref),
            "vertical-align" => self.vertical_align = parse_css_string_token(value_ref),
            "table-layout" => self.table_layout = parse_table_layout(value_ref),
            "border-collapse" => self.border_collapse = parse_border_collapse(value_ref),
            "border-spacing" => {
                self.border_spacing = self.resolve_tailwind_border_spacing(value_ref)
            }
            "caption-side" => self.caption_side = parse_caption_side(value_ref),
            "empty-cells" => self.empty_cells = parse_css_string_token(value_ref),
            "aspect-ratio" => self.aspect_ratio = parse_css_string_token(value_ref),
            "transform" => self.apply_transform_property(value_ref),
            "translate" => self.translate = self.resolve_tailwind_translate(value_ref),
            "rotate" => self.rotate = self.resolve_tailwind_rotate(value_ref),
            "scale" => self.scale = self.resolve_tailwind_scale(value_ref),
            "transform-origin" => self.transform_origin = parse_css_string_token(value_ref),
            "transform-style" => self.transform_style = parse_css_string_token(value_ref),
            "transform-box" => self.transform_box = parse_css_string_token(value_ref),
            "offset" => self.offset = parse_css_string_token(value_ref),
            "offset-path" => self.offset_path = parse_css_string_token(value_ref),
            "offset-distance" => self.offset_distance = parse_css_string_token(value_ref),
            "offset-rotate" => self.offset_rotate = parse_css_string_token(value_ref),
            "offset-anchor" => self.offset_anchor = parse_css_string_token(value_ref),
            "offset-position" => self.offset_position = parse_css_string_token(value_ref),
            "backface-visibility" => {
                self.backface_visibility = parse_backface_visibility(value_ref)
            }
            "perspective" => self.perspective = parse_length(value_ref),
            "perspective-origin" => self.perspective_origin = parse_css_string_token(value_ref),
            "filter" => self.apply_filter_property(value_ref),
            "backdrop-filter" => self.apply_backdrop_filter_property(value_ref),
            "transition" => self.transition = parse_css_string_token(value_ref),
            "transition-property" => {
                self.transition_property = parse_css_string_token(value_ref);
            }
            "transition-duration" => self.transition_duration = parse_time(value_ref),
            "transition-timing-function" => {
                self.transition_timing_function = parse_css_string_token(value_ref);
            }
            "transition-delay" => self.transition_delay = parse_time(value_ref),
            "transition-behavior" => self.transition_behavior = parse_css_string_token(value_ref),
            "overlay" => self.overlay = parse_css_string_token(value_ref),
            "animation" => self.animation = parse_css_string_token(value_ref),
            "animation-name" => self.animation_name = parse_css_string_token(value_ref),
            "animation-duration" => self.animation_duration = parse_time(value_ref),
            "animation-timing-function" => {
                self.animation_timing_function = parse_css_string_token(value_ref);
            }
            "animation-delay" => self.animation_delay = parse_time(value_ref),
            "animation-iteration-count" => {
                self.animation_iteration_count = parse_css_string_token(value_ref);
            }
            "animation-direction" => self.animation_direction = parse_css_string_token(value_ref),
            "animation-fill-mode" => self.animation_fill_mode = parse_css_string_token(value_ref),
            "animation-play-state" => self.animation_play_state = parse_css_string_token(value_ref),
            "animation-composition" => {
                self.animation_composition = parse_css_string_token(value_ref);
            }
            "animation-timeline" => self.animation_timeline = parse_css_string_token(value_ref),
            "animation-range" => self.animation_range = parse_css_string_token(value_ref),
            "animation-range-start" => {
                self.animation_range_start = parse_css_string_token(value_ref);
            }
            "animation-range-end" => self.animation_range_end = parse_css_string_token(value_ref),
            "view-transition-name" => {
                self.view_transition_name = parse_css_string_token(value_ref);
            }
            "view-transition-class" => {
                self.view_transition_class = parse_css_string_token(value_ref);
            }
            "view-transition-group" => {
                self.view_transition_group = parse_css_string_token(value_ref);
            }
            "view-transition-scope" => {
                self.view_transition_scope = parse_css_string_token(value_ref);
            }
            "will-change" => self.will_change = parse_css_string_token(value_ref),
            "color-scheme" => self.color_scheme = parse_css_string_token(value_ref),
            "forced-color-adjust" => self.forced_color_adjust = parse_css_string_token(value_ref),
            "print-color-adjust" | "-webkit-print-color-adjust" | "webkit-print-color-adjust" => {
                self.print_color_adjust = parse_css_string_token(value_ref);
            }
            "color-adjust" => self.color_adjust = parse_css_string_token(value_ref),
            "field-sizing" => self.field_sizing = parse_css_string_token(value_ref),
            "appearance" => self.appearance = parse_css_string_token(value_ref),
            "caret" => self.caret = parse_css_string_token(value_ref),
            "resize" => self.resize = parse_resize(value_ref),
            "caret-animation" => self.caret_animation = parse_css_string_token(value_ref),
            "scroll-behavior" => self.scroll_behavior = parse_scroll_behavior(value_ref),
            "scroll-timeline" => self.scroll_timeline = parse_css_string_token(value_ref),
            "scroll-timeline-name" => {
                self.scroll_timeline_name = parse_css_string_token(value_ref);
            }
            "scroll-timeline-axis" => {
                self.scroll_timeline_axis = parse_css_string_token(value_ref);
            }
            "view-timeline" => self.view_timeline = parse_css_string_token(value_ref),
            "view-timeline-name" => self.view_timeline_name = parse_css_string_token(value_ref),
            "view-timeline-axis" => self.view_timeline_axis = parse_css_string_token(value_ref),
            "view-timeline-inset" => self.view_timeline_inset = parse_css_string_token(value_ref),
            "timeline-scope" => self.timeline_scope = parse_css_string_token(value_ref),
            "scroll-snap-type" => self.scroll_snap_type = parse_css_string_token(value_ref),
            "scroll-snap-align" => self.scroll_snap_align = parse_css_string_token(value_ref),
            "scroll-snap-stop" => self.scroll_snap_stop = parse_css_string_token(value_ref),
            "scroll-initial-target" | "scroll-start-target" => {
                self.scroll_initial_target = parse_css_string_token(value_ref);
            }
            "scroll-target-group" => self.scroll_target_group = parse_css_string_token(value_ref),
            "scroll-marker-group" => self.scroll_marker_group = parse_css_string_token(value_ref),
            "scrollbar-gutter" => self.scrollbar_gutter = parse_css_string_token(value_ref),
            "scrollbar-width" => self.scrollbar_width = parse_css_string_token(value_ref),
            "scrollbar-color" => self.apply_scrollbar_color_property(value_ref),
            "overscroll-behavior" => {
                let overscroll = parse_overscroll_behavior(value_ref);
                self.overscroll_behavior_x = overscroll;
                self.overscroll_behavior_y = overscroll;
            }
            "overscroll-behavior-x" => {
                self.overscroll_behavior_x = parse_overscroll_behavior(value_ref);
            }
            "overscroll-behavior-y" => {
                self.overscroll_behavior_y = parse_overscroll_behavior(value_ref);
            }
            "overscroll-behavior-block" => {
                self.overscroll_behavior_block = parse_overscroll_behavior(value_ref);
            }
            "overscroll-behavior-inline" => {
                self.overscroll_behavior_inline = parse_overscroll_behavior(value_ref);
            }
            "touch-action" => self.touch_action = parse_css_string_token(value_ref),
            "nav-up" => self.nav_up = parse_css_string_token(value_ref),
            "nav-right" => self.nav_right = parse_css_string_token(value_ref),
            "nav-down" => self.nav_down = parse_css_string_token(value_ref),
            "nav-left" => self.nav_left = parse_css_string_token(value_ref),
            "spatial-navigation-action" => {
                self.spatial_navigation_action = parse_css_string_token(value_ref);
            }
            "spatial-navigation-contain" => {
                self.spatial_navigation_contain = parse_css_string_token(value_ref);
            }
            "spatial-navigation-function" => {
                self.spatial_navigation_function = parse_css_string_token(value_ref);
            }
            "interactivity" => self.interactivity = parse_css_string_token(value_ref),
            "cursor" => self.cursor = parse_css_string_token(value_ref),
            "caret-shape" => self.caret_shape = parse_css_string_token(value_ref),
            "pointer-events" => self.pointer_events = parse_pointer_events(value_ref),
            "user-select" => self.user_select = parse_user_select(value_ref),
            "opacity" => self.opacity = parse_opacity(value_ref),
            other => {
                self.unsupported.insert(other.to_string(), value);
            }
        }
    }

    fn record_declaration(&mut self, property: &str, value: &str) {
        if property.starts_with("--") {
            self.custom_properties
                .insert(property.to_string(), value.to_string());
        } else {
            self.declarations
                .insert(property.to_string(), value.to_string());
        }
    }

    fn record_variant_declaration(&mut self, variant: &str, property: String, value: String) {
        self.variant_declarations
            .entry(variant.to_string())
            .or_default()
            .insert(property, value);
    }

    fn apply_container_shorthand(&mut self, value: &str) {
        self.container = parse_css_string_token(value);
        let Some(value) = self.container.as_deref() else {
            return;
        };
        if let Some((name, container_type)) = value.split_once('/') {
            self.container_name = parse_css_string_token(name);
            self.container_type = parse_container_type(container_type);
            return;
        }
        if let Some(container_type) = parse_container_type(value) {
            self.container_type = Some(container_type);
        } else {
            self.container_name = parse_css_string_token(value);
        }
    }

    fn apply_border_shorthand(&mut self, value: &str) {
        for part in value.split_whitespace() {
            if let Some(width) = parse_length(part) {
                self.border_width.set_all(Some(width));
            } else if let Some(style) = parse_border_style(part) {
                self.border_style = Some(style);
                self.border_styles.set_all(Some(style));
            } else if let Some(color) = parse_color(part) {
                self.border_color = Some(color.clone());
                self.border_colors.set_all(Some(color));
            }
        }
    }

    fn apply_border_side_shorthand(&mut self, edges: EdgeSelection, value: &str) {
        for part in value.split_whitespace() {
            if let Some(width) = parse_length(part) {
                self.border_width.apply_edges(edges, width);
            } else if let Some(style) = parse_border_style(part) {
                self.border_styles.apply_edges(edges, style);
            } else if let Some(color) = parse_color(part) {
                self.border_colors.apply_edges(edges, color);
            }
        }
    }

    fn apply_logical_border_side_shorthand(&mut self, edges: LogicalEdgeSelection, value: &str) {
        for part in value.split_whitespace() {
            if let Some(width) = parse_length(part) {
                self.logical_border_width.apply_edges(edges, width);
            } else if let Some(style) = parse_border_style(part) {
                self.logical_border_styles.apply_edges(edges, style);
            } else if let Some(color) = parse_color(part) {
                self.logical_border_colors.apply_edges(edges, color);
            }
        }
    }

    fn apply_outline_shorthand(&mut self, value: &str) {
        for part in value.split_whitespace() {
            if let Some(width) = parse_length(part) {
                self.outline_width = Some(width);
            } else if let Some(style) = parse_border_style(part) {
                self.outline_style = Some(style);
            } else if let Some(color) = parse_color(part) {
                self.outline_color = Some(color);
            }
        }
    }

    fn apply_background_shorthand(&mut self, value: &str) {
        self.background = parse_css_string_token(value);
        if let Some(color) = parse_background_shorthand_color(value) {
            self.background_color = Some(color);
        }
    }

    fn apply_box_shadow_property(&mut self, value: &str) {
        if value.trim() == tailwind_box_shadow_pipeline() {
            self.box_shadow = self
                .compose_tailwind_box_shadow()
                .or_else(|| parse_css_string_token(value));
            return;
        }
        self.box_shadow = parse_css_string_token(value);
    }

    fn apply_scrollbar_color_property(&mut self, value: &str) {
        if value.trim() == tailwind_scrollbar_color_pipeline() {
            self.scrollbar_color = self
                .compose_tailwind_scrollbar_color()
                .or_else(|| parse_css_string_token(value));
            return;
        }
        self.scrollbar_color = parse_css_string_token(value);
    }

    fn apply_column_rule_shorthand(&mut self, value: &str) {
        self.column_rule = parse_css_string_token(value);
        for part in value.split_whitespace() {
            if let Some(width) = parse_length(part) {
                self.column_rule_width = Some(width);
            } else if let Some(style) = parse_border_style(part) {
                self.column_rule_style = Some(style);
            } else if let Some(color) = parse_color(part) {
                self.column_rule_color = Some(color);
            }
        }
    }

    fn apply_text_decoration_shorthand(&mut self, value: &str) {
        for part in value.split_whitespace() {
            if let Some(style) = parse_text_decoration_style(part) {
                self.text_decoration_style = Some(style);
            } else if let Some(thickness) = parse_length(part) {
                self.text_decoration_thickness = Some(thickness);
            } else if parse_text_decoration_line(part).is_some() {
                self.text_decoration_line = Some(part.to_string());
            } else if let Some(color) = parse_color(part) {
                self.text_decoration_color = Some(color);
            }
        }
    }

    fn apply_text_emphasis_shorthand(&mut self, value: &str) {
        let mut style_parts = Vec::new();
        for part in value.split_whitespace() {
            if is_text_emphasis_style_token(part) {
                style_parts.push(part);
            } else if let Some(color) = parse_color(part) {
                self.text_emphasis_color = Some(color);
            }
        }
        if !style_parts.is_empty() {
            self.text_emphasis_style = Some(style_parts.join(" "));
        }
    }

    fn apply_tailwind_custom_property(&mut self, property: &str, value: &str) {
        match property {
            "--tw-blur" => self.filter_blur = parse_non_empty_css_string(value),
            "--tw-brightness" => self.filter_brightness = parse_non_empty_css_string(value),
            "--tw-contrast" => self.filter_contrast = parse_non_empty_css_string(value),
            "--tw-drop-shadow" => self.filter_drop_shadow = parse_non_empty_css_string(value),
            "--tw-grayscale" => self.filter_grayscale = parse_non_empty_css_string(value),
            "--tw-hue-rotate" => self.filter_hue_rotate = parse_non_empty_css_string(value),
            "--tw-invert" => self.filter_invert = parse_non_empty_css_string(value),
            "--tw-saturate" => self.filter_saturate = parse_non_empty_css_string(value),
            "--tw-sepia" => self.filter_sepia = parse_non_empty_css_string(value),
            "--tw-backdrop-blur" => {
                self.backdrop_filter_blur = parse_non_empty_css_string(value);
            }
            "--tw-backdrop-brightness" => {
                self.backdrop_filter_brightness = parse_non_empty_css_string(value);
            }
            "--tw-backdrop-contrast" => {
                self.backdrop_filter_contrast = parse_non_empty_css_string(value);
            }
            "--tw-backdrop-grayscale" => {
                self.backdrop_filter_grayscale = parse_non_empty_css_string(value);
            }
            "--tw-backdrop-hue-rotate" => {
                self.backdrop_filter_hue_rotate = parse_non_empty_css_string(value);
            }
            "--tw-backdrop-invert" => {
                self.backdrop_filter_invert = parse_non_empty_css_string(value);
            }
            "--tw-backdrop-opacity" => {
                self.backdrop_filter_opacity = parse_non_empty_css_string(value);
            }
            "--tw-backdrop-saturate" => {
                self.backdrop_filter_saturate = parse_non_empty_css_string(value);
            }
            "--tw-backdrop-sepia" => {
                self.backdrop_filter_sepia = parse_non_empty_css_string(value);
            }
            "--tw-ring-shadow" => {
                self.ring_shadow = parse_non_empty_css_string(value);
                self.box_shadow = self.compose_tailwind_box_shadow();
            }
            "--tw-ring-color" => {
                self.ring_color = parse_non_empty_css_string(value);
                self.box_shadow = self.compose_tailwind_box_shadow();
            }
            "--tw-ring-inset" => {
                self.box_shadow = self.compose_tailwind_box_shadow();
            }
            "--tw-inset-ring-shadow" => {
                self.inset_ring_shadow = parse_non_empty_css_string(value);
                self.box_shadow = self.compose_tailwind_box_shadow();
            }
            "--tw-inset-ring-color" => {
                self.inset_ring_color = parse_non_empty_css_string(value);
                self.box_shadow = self.compose_tailwind_box_shadow();
            }
            "--tw-divide-x-reverse" => {
                self.divide_x_reverse = parse_non_empty_css_string(value);
            }
            "--tw-divide-y-reverse" => {
                self.divide_y_reverse = parse_non_empty_css_string(value);
            }
            "--tw-space-x-reverse" => {
                self.space_x_reverse = parse_non_empty_css_string(value);
            }
            "--tw-space-y-reverse" => {
                self.space_y_reverse = parse_non_empty_css_string(value);
            }
            "--tw-scrollbar-thumb" => {
                self.scrollbar_thumb_color = parse_non_empty_css_string(value);
                self.scrollbar_color = self.compose_tailwind_scrollbar_color();
            }
            "--tw-scrollbar-track" => {
                self.scrollbar_track_color = parse_non_empty_css_string(value);
                self.scrollbar_color = self.compose_tailwind_scrollbar_color();
            }
            "--tw-ordinal"
            | "--tw-slashed-zero"
            | "--tw-numeric-figure"
            | "--tw-numeric-spacing"
            | "--tw-numeric-fraction" => {
                self.font_variant_numeric = self.compose_font_variant_numeric();
            }
            _ => {}
        }
    }

    fn apply_font_variant_numeric(&mut self, value: &str) {
        if is_tailwind_font_variant_numeric_pipeline(value) {
            self.font_variant_numeric = self
                .compose_font_variant_numeric()
                .or_else(|| parse_css_string_token(value));
            return;
        }
        self.font_variant_numeric = parse_css_string_token(value);
    }

    fn compose_font_variant_numeric(&self) -> Option<String> {
        join_css_functions([
            self.custom_properties
                .get("--tw-ordinal")
                .map(String::as_str),
            self.custom_properties
                .get("--tw-slashed-zero")
                .map(String::as_str),
            self.custom_properties
                .get("--tw-numeric-figure")
                .map(String::as_str),
            self.custom_properties
                .get("--tw-numeric-spacing")
                .map(String::as_str),
            self.custom_properties
                .get("--tw-numeric-fraction")
                .map(String::as_str),
        ])
    }

    fn apply_transform_property(&mut self, value: &str) {
        if value.trim() == "none" {
            self.transform = Some("none".to_string());
            return;
        }
        if is_tailwind_transform_pipeline(value) {
            self.transform = self
                .compose_tailwind_transform(value.contains("translateZ(0)"))
                .or_else(|| parse_css_string_token(value));
            return;
        }
        self.transform = parse_css_string_token(value);
    }

    fn resolve_tailwind_translate(&self, value: &str) -> Option<String> {
        if !is_tailwind_translate_pipeline(value) {
            return parse_css_string_token(value);
        }
        let x = self
            .custom_properties
            .get("--tw-translate-x")
            .map(String::as_str)
            .unwrap_or("0");
        let y = self
            .custom_properties
            .get("--tw-translate-y")
            .map(String::as_str)
            .unwrap_or("0");
        let z = self.custom_properties.get("--tw-translate-z");
        Some(match z {
            Some(z) => format!("{x} {y} {z}"),
            None => format!("{x} {y}"),
        })
    }

    fn resolve_tailwind_rotate(&self, value: &str) -> Option<String> {
        if value == "var(--tw-rotate)" {
            return self.custom_properties.get("--tw-rotate").cloned();
        }
        parse_css_string_token(value)
    }

    fn resolve_tailwind_scale(&self, value: &str) -> Option<String> {
        if !is_tailwind_scale_pipeline(value) {
            return parse_css_string_token(value);
        }
        let x = self
            .custom_properties
            .get("--tw-scale-x")
            .map(String::as_str)
            .unwrap_or("100%");
        let y = self
            .custom_properties
            .get("--tw-scale-y")
            .map(String::as_str)
            .unwrap_or("100%");
        let z = self.custom_properties.get("--tw-scale-z");
        Some(match z {
            Some(z) => format!("{x} {y} {z}"),
            None => format!("{x} {y}"),
        })
    }

    fn resolve_tailwind_border_spacing(&self, value: &str) -> Option<String> {
        if !is_tailwind_border_spacing_pipeline(value) {
            return parse_css_string_token(value);
        }
        let x = self
            .custom_properties
            .get("--tw-border-spacing-x")
            .map(String::as_str)
            .unwrap_or("0");
        let y = self
            .custom_properties
            .get("--tw-border-spacing-y")
            .map(String::as_str)
            .unwrap_or("0");
        Some(format!("{x} {y}"))
    }

    fn compose_tailwind_transform(&self, gpu: bool) -> Option<String> {
        let mut parts = Vec::new();
        if gpu {
            parts.push("translateZ(0)".to_string());
        }
        for property in [
            "--tw-rotate-x",
            "--tw-rotate-y",
            "--tw-rotate-z",
            "--tw-skew-x",
            "--tw-skew-y",
        ] {
            if let Some(value) = self.custom_properties.get(property) {
                if !value.trim().is_empty() {
                    parts.push(value.clone());
                }
            }
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }

    fn apply_filter_property(&mut self, value: &str) {
        if value.trim() == "none" {
            self.clear_filter_components();
            self.filter = Some("none".to_string());
            return;
        }
        if is_tailwind_filter_pipeline(value) {
            self.filter = self
                .compose_filter()
                .or_else(|| parse_css_string_token(value));
            return;
        }
        self.filter = parse_css_string_token(value);
    }

    fn apply_backdrop_filter_property(&mut self, value: &str) {
        if value.trim() == "none" {
            self.clear_backdrop_filter_components();
            self.backdrop_filter = Some("none".to_string());
            return;
        }
        if is_tailwind_backdrop_filter_pipeline(value) {
            self.backdrop_filter = self
                .compose_backdrop_filter()
                .or_else(|| parse_css_string_token(value));
            return;
        }
        self.backdrop_filter = parse_css_string_token(value);
    }

    fn clear_filter_components(&mut self) {
        self.filter_blur = None;
        self.filter_brightness = None;
        self.filter_contrast = None;
        self.filter_drop_shadow = None;
        self.filter_grayscale = None;
        self.filter_hue_rotate = None;
        self.filter_invert = None;
        self.filter_saturate = None;
        self.filter_sepia = None;
    }

    fn clear_backdrop_filter_components(&mut self) {
        self.backdrop_filter_blur = None;
        self.backdrop_filter_brightness = None;
        self.backdrop_filter_contrast = None;
        self.backdrop_filter_grayscale = None;
        self.backdrop_filter_hue_rotate = None;
        self.backdrop_filter_invert = None;
        self.backdrop_filter_opacity = None;
        self.backdrop_filter_saturate = None;
        self.backdrop_filter_sepia = None;
    }

    fn compose_filter(&self) -> Option<String> {
        join_css_functions([
            self.filter_blur.as_deref(),
            self.filter_brightness.as_deref(),
            self.filter_contrast.as_deref(),
            self.filter_drop_shadow.as_deref(),
            self.filter_grayscale.as_deref(),
            self.filter_hue_rotate.as_deref(),
            self.filter_invert.as_deref(),
            self.filter_saturate.as_deref(),
            self.filter_sepia.as_deref(),
        ])
    }

    fn compose_backdrop_filter(&self) -> Option<String> {
        join_css_functions([
            self.backdrop_filter_blur.as_deref(),
            self.backdrop_filter_brightness.as_deref(),
            self.backdrop_filter_contrast.as_deref(),
            self.backdrop_filter_grayscale.as_deref(),
            self.backdrop_filter_hue_rotate.as_deref(),
            self.backdrop_filter_invert.as_deref(),
            self.backdrop_filter_opacity.as_deref(),
            self.backdrop_filter_saturate.as_deref(),
            self.backdrop_filter_sepia.as_deref(),
        ])
    }

    fn compose_tailwind_box_shadow(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(shadow) = self.inset_ring_shadow.as_deref() {
            parts.push(compose_tailwind_ring_shadow(
                shadow,
                self.inset_ring_color.as_deref(),
                false,
            ));
        }
        if let Some(shadow) = self.ring_shadow.as_deref() {
            let inset = self
                .custom_properties
                .get("--tw-ring-inset")
                .is_some_and(|value| value.trim() == "inset");
            parts.push(compose_tailwind_ring_shadow(
                shadow,
                self.ring_color.as_deref(),
                inset,
            ));
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(", "))
        }
    }

    fn compose_tailwind_scrollbar_color(&self) -> Option<String> {
        if self.scrollbar_thumb_color.is_none() && self.scrollbar_track_color.is_none() {
            return None;
        }
        let thumb = self
            .scrollbar_thumb_color
            .as_deref()
            .unwrap_or("var(--tw-scrollbar-thumb)");
        let track = self
            .scrollbar_track_color
            .as_deref()
            .unwrap_or("var(--tw-scrollbar-track)");
        Some(format!("{thumb} {track}"))
    }

    fn apply_tailwind_utility(&mut self, class: &str) {
        let Some(class) = tailwind::parse_class(class) else {
            return;
        };
        let variants = class.variants;
        let class = class.utility;
        if class.is_empty() {
            return;
        }
        let declarations = tailwind_utility_declarations(class);
        if !variants.is_empty() {
            let variant_key = tailwind::variant_key(&variants);
            for (property, value) in declarations {
                self.record_variant_declaration(variant_key.as_str(), property, value);
            }
            return;
        }
        for (property, value) in declarations {
            self.apply(&property, &value);
        }
        if let Some(arbitrary) = class
            .strip_prefix('[')
            .and_then(|value| value.strip_suffix(']'))
        {
            if let Some((property, value)) = arbitrary.split_once(':') {
                self.apply(property.trim(), &tailwind_arbitrary_value(value.trim()));
            }
            return;
        }
        match class {
            "flex" => self.display = Some(DisplayMode::Flex),
            "inline-flex" => self.display = Some(DisplayMode::InlineFlex),
            "block" => self.display = Some(DisplayMode::Block),
            "inline-block" => self.display = Some(DisplayMode::InlineBlock),
            "inline" => self.display = Some(DisplayMode::Inline),
            "grid" => self.display = Some(DisplayMode::Grid),
            "inline-grid" => self.display = Some(DisplayMode::InlineGrid),
            "flow-root" => self.display = Some(DisplayMode::FlowRoot),
            "contents" => self.display = Some(DisplayMode::Contents),
            "list-item" => self.display = Some(DisplayMode::ListItem),
            "table" => self.display = Some(DisplayMode::Table),
            "inline-table" => self.display = Some(DisplayMode::InlineTable),
            "table-caption" => self.display = Some(DisplayMode::TableCaption),
            "table-cell" => self.display = Some(DisplayMode::TableCell),
            "table-column" => self.display = Some(DisplayMode::TableColumn),
            "table-column-group" => self.display = Some(DisplayMode::TableColumnGroup),
            "table-footer-group" => self.display = Some(DisplayMode::TableFooterGroup),
            "table-header-group" => self.display = Some(DisplayMode::TableHeaderGroup),
            "table-row-group" => self.display = Some(DisplayMode::TableRowGroup),
            "table-row" => self.display = Some(DisplayMode::TableRow),
            "line-clamp-1" | "line-clamp-2" | "line-clamp-3" | "line-clamp-4" | "line-clamp-5"
            | "line-clamp-6" => self.display = Some(DisplayMode::WebkitBox),
            "line-clamp-none" => self.display = Some(DisplayMode::Block),
            "hidden" => self.display = Some(DisplayMode::None),
            "static" => self.position = Some(PositionMode::Static),
            "fixed" => self.position = Some(PositionMode::Fixed),
            "absolute" => self.position = Some(PositionMode::Absolute),
            "relative" => self.position = Some(PositionMode::Relative),
            "sticky" => self.position = Some(PositionMode::Sticky),
            "flex-row" | "flex-row-reverse" => self.flex_direction = Some(Orientation::Horizontal),
            "flex-col" | "flex-col-reverse" => self.flex_direction = Some(Orientation::Vertical),
            "flex-wrap" => self.flex_wrap = Some(FlexWrap::Wrap),
            "flex-nowrap" => self.flex_wrap = Some(FlexWrap::NoWrap),
            "flex-wrap-reverse" => self.flex_wrap = Some(FlexWrap::WrapReverse),
            "items-start" => self.align_items = Some(AlignItems::Start),
            "items-center" => self.align_items = Some(AlignItems::Center),
            "items-end" => self.align_items = Some(AlignItems::End),
            "items-stretch" => self.align_items = Some(AlignItems::Stretch),
            "items-baseline" => self.align_items = Some(AlignItems::Baseline),
            "justify-start" => self.justify_content = Some(JustifyContent::Start),
            "justify-center" => self.justify_content = Some(JustifyContent::Center),
            "justify-end" => self.justify_content = Some(JustifyContent::End),
            "justify-between" => self.justify_content = Some(JustifyContent::SpaceBetween),
            "justify-around" => self.justify_content = Some(JustifyContent::SpaceAround),
            "justify-evenly" => self.justify_content = Some(JustifyContent::SpaceEvenly),
            "rounded" => self.border_radius = Some(StyleLength::Points(4.0)),
            "rounded-none" => self.border_radius = Some(StyleLength::Points(0.0)),
            "rounded-xs" => self.border_radius = Some(StyleLength::Points(2.0)),
            "rounded-sm" => self.border_radius = Some(StyleLength::Points(4.0)),
            "rounded-md" => self.border_radius = Some(StyleLength::Points(6.0)),
            "rounded-lg" => self.border_radius = Some(StyleLength::Points(8.0)),
            "rounded-xl" => self.border_radius = Some(StyleLength::Points(12.0)),
            "rounded-2xl" => self.border_radius = Some(StyleLength::Points(16.0)),
            "rounded-3xl" => self.border_radius = Some(StyleLength::Points(24.0)),
            "rounded-4xl" => self.border_radius = Some(StyleLength::Points(32.0)),
            "rounded-full" => {
                self.border_radius = Some(StyleLength::Css("calc(infinity * 1px)".to_string()));
            }
            _ => self.apply_tailwind_prefixed_utility(class),
        }
    }

    fn apply_tailwind_prefixed_utility(&mut self, class: &str) {
        if let Some(value) = class.strip_prefix("w-").and_then(tailwind_length) {
            self.width = Some(value);
        } else if let Some(value) = class.strip_prefix("h-").and_then(tailwind_length) {
            self.height = Some(value);
        } else if let Some(value) = class.strip_prefix("size-").and_then(tailwind_length) {
            self.width = Some(value.clone());
            self.height = Some(value);
        } else if let Some(value) = class.strip_prefix("min-w-").and_then(tailwind_length) {
            self.min_width = Some(value);
        } else if let Some(value) = class.strip_prefix("min-h-").and_then(tailwind_length) {
            self.min_height = Some(value);
        } else if let Some(value) = class.strip_prefix("max-w-").and_then(tailwind_length) {
            self.max_width = Some(value);
        } else if let Some(value) = class.strip_prefix("max-h-").and_then(tailwind_length) {
            self.max_height = Some(value);
        } else if let Some(value) = class.strip_prefix("gap-").and_then(tailwind_length) {
            self.gap = Some(value);
        } else if let Some(value) = class.strip_prefix("gap-x-").and_then(tailwind_length) {
            self.column_gap = Some(value);
        } else if let Some(value) = class.strip_prefix("gap-y-").and_then(tailwind_length) {
            self.row_gap = Some(value);
        } else if let Some(opacity) = class.strip_prefix("opacity-").and_then(tailwind_opacity) {
            self.opacity = Some(opacity);
        } else if let Some(color) = class.strip_prefix("bg-").and_then(tailwind_color) {
            self.background_color = Some(color);
        } else if let Some(color) = class.strip_prefix("text-").and_then(tailwind_color) {
            self.color = Some(color);
        } else if let Some((edges, value)) = tailwind_edge_utility(class, "p") {
            self.padding.apply_edges(edges, value);
        } else if let Some((edges, value)) = tailwind_edge_utility(class, "m") {
            self.margin.apply_edges(edges, value);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DisplayMode {
    Inline,
    InlineBlock,
    Flex,
    InlineFlex,
    Block,
    Grid,
    InlineGrid,
    FlowRoot,
    Contents,
    ListItem,
    Table,
    InlineTable,
    TableCaption,
    TableCell,
    TableColumn,
    TableColumnGroup,
    TableFooterGroup,
    TableHeaderGroup,
    TableRowGroup,
    TableRow,
    Ruby,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
    WebkitBox,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BoxSizing {
    BorderBox,
    ContentBox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BoxDecorationBreak {
    Slice,
    Clone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PositionMode {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GridAutoFlow {
    Row,
    Column,
    Dense,
    RowDense,
    ColumnDense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContainerType {
    Normal,
    Size,
    InlineSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentVisibility {
    Visible,
    Auto,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AlignItems {
    Normal,
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JustifyContent {
    Normal,
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SelfAlignment {
    Auto,
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BorderStyle {
    None,
    Hidden,
    Solid,
    Dashed,
    Dotted,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundAttachment {
    Fixed,
    Local,
    Scroll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundBox {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ListStylePosition {
    Inside,
    Outside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum FontWeight {
    Number(u16),
    Keyword(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextAlign {
    Start,
    Center,
    End,
    Justify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextDirection {
    Ltr,
    Rtl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnicodeBidi {
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WritingMode {
    HorizontalTb,
    VerticalRl,
    VerticalLr,
    SidewaysRl,
    SidewaysLr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextOrientation {
    Mixed,
    Upright,
    Sideways,
    SidewaysRight,
    UseGlyphOrientation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StrokeLineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StrokeLineJoin {
    Arcs,
    Bevel,
    Miter,
    MiterClip,
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WhiteSpaceMode {
    Normal,
    NoWrap,
    Pre,
    PreLine,
    PreWrap,
    BreakSpaces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextWrapMode {
    Wrap,
    NoWrap,
    Balance,
    Pretty,
    Stable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WordBreakMode {
    Normal,
    BreakAll,
    KeepAll,
    BreakWord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OverflowWrapMode {
    Normal,
    BreakWord,
    Anywhere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HyphensMode {
    None,
    Manual,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OverflowMode {
    Visible,
    Hidden,
    Scroll,
    Auto,
    Clip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VisibilityMode {
    Visible,
    Hidden,
    Collapse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IsolationMode {
    Auto,
    Isolate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    PlusDarker,
    PlusLighter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FloatMode {
    Left,
    Right,
    InlineStart,
    InlineEnd,
    Footnote,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClearMode {
    Left,
    Right,
    Both,
    InlineStart,
    InlineEnd,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TableLayout {
    Auto,
    Fixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BorderCollapse {
    Collapse,
    Separate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CaptionSide {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PointerEvents {
    Auto,
    None,
    VisiblePainted,
    VisibleFill,
    VisibleStroke,
    Visible,
    Painted,
    Fill,
    Stroke,
    BoundingBox,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserSelect {
    Auto,
    Text,
    None,
    All,
    Contain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResizeMode {
    None,
    Both,
    Horizontal,
    Vertical,
    Block,
    Inline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OverscrollBehavior {
    Auto,
    Contain,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum StyleLength {
    Points(f64),
    Percent(f64),
    Auto,
    Css(String),
}

impl StyleLength {
    pub fn points(&self) -> Option<f64> {
        match self {
            StyleLength::Points(value) => Some(*value),
            StyleLength::Percent(_) | StyleLength::Auto | StyleLength::Css(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum StyleTime {
    Milliseconds(f64),
    Css(String),
}

impl StyleTime {
    pub fn milliseconds(&self) -> Option<f64> {
        match self {
            StyleTime::Milliseconds(value) => Some(*value),
            StyleTime::Css(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum StyleColor {
    Rgba {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    },
    Function(String),
    Keyword(String),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeInsets {
    pub top: Option<StyleLength>,
    pub right: Option<StyleLength>,
    pub bottom: Option<StyleLength>,
    pub left: Option<StyleLength>,
}

impl EdgeInsets {
    fn set_all(&mut self, value: Option<StyleLength>) {
        self.top = value.clone();
        self.right = value.clone();
        self.bottom = value.clone();
        self.left = value;
    }

    fn apply_edges(&mut self, edges: EdgeSelection, value: StyleLength) {
        match edges {
            EdgeSelection::All => self.set_all(Some(value)),
            EdgeSelection::X => {
                self.left = Some(value.clone());
                self.right = Some(value);
            }
            EdgeSelection::Y => {
                self.top = Some(value.clone());
                self.bottom = Some(value);
            }
            EdgeSelection::Top => self.top = Some(value),
            EdgeSelection::Right => self.right = Some(value),
            EdgeSelection::Bottom => self.bottom = Some(value),
            EdgeSelection::Left => self.left = Some(value),
        }
    }

    fn apply_edges_opt(&mut self, edges: EdgeSelection, value: Option<StyleLength>) {
        if let Some(value) = value {
            self.apply_edges(edges, value);
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum EdgeSelection {
    All,
    X,
    Y,
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicalEdgeInsets {
    pub block_start: Option<StyleLength>,
    pub block_end: Option<StyleLength>,
    pub inline_start: Option<StyleLength>,
    pub inline_end: Option<StyleLength>,
}

impl LogicalEdgeInsets {
    fn apply_edges(&mut self, edges: LogicalEdgeSelection, value: StyleLength) {
        match edges {
            LogicalEdgeSelection::Block => {
                self.block_start = Some(value.clone());
                self.block_end = Some(value);
            }
            LogicalEdgeSelection::Inline => {
                self.inline_start = Some(value.clone());
                self.inline_end = Some(value);
            }
            LogicalEdgeSelection::BlockStart => self.block_start = Some(value),
            LogicalEdgeSelection::BlockEnd => self.block_end = Some(value),
            LogicalEdgeSelection::InlineStart => self.inline_start = Some(value),
            LogicalEdgeSelection::InlineEnd => self.inline_end = Some(value),
        }
    }

    fn apply_axis_values(&mut self, axis: LogicalEdgeSelection, value: &str) {
        if let Some(value) = parse_length(value) {
            self.apply_edges(axis, value);
            return;
        }
        let values = value
            .split_whitespace()
            .filter_map(parse_length)
            .collect::<Vec<_>>();
        match (axis, values.as_slice()) {
            (_, []) => {}
            (LogicalEdgeSelection::Block, [both]) => {
                self.block_start = Some(both.clone());
                self.block_end = Some(both.clone());
            }
            (LogicalEdgeSelection::Block, [start, end, ..]) => {
                self.block_start = Some(start.clone());
                self.block_end = Some(end.clone());
            }
            (LogicalEdgeSelection::Inline, [both]) => {
                self.inline_start = Some(both.clone());
                self.inline_end = Some(both.clone());
            }
            (LogicalEdgeSelection::Inline, [start, end, ..]) => {
                self.inline_start = Some(start.clone());
                self.inline_end = Some(end.clone());
            }
            (_, [single, ..]) => self.apply_edges(axis, single.clone()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum LogicalEdgeSelection {
    Block,
    Inline,
    BlockStart,
    BlockEnd,
    InlineStart,
    InlineEnd,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeColors {
    pub top: Option<StyleColor>,
    pub right: Option<StyleColor>,
    pub bottom: Option<StyleColor>,
    pub left: Option<StyleColor>,
}

impl EdgeColors {
    fn set_all(&mut self, value: Option<StyleColor>) {
        self.top = value.clone();
        self.right = value.clone();
        self.bottom = value.clone();
        self.left = value;
    }

    fn apply_edges(&mut self, edges: EdgeSelection, value: StyleColor) {
        match edges {
            EdgeSelection::All => self.set_all(Some(value)),
            EdgeSelection::X => {
                self.left = Some(value.clone());
                self.right = Some(value);
            }
            EdgeSelection::Y => {
                self.top = Some(value.clone());
                self.bottom = Some(value);
            }
            EdgeSelection::Top => self.top = Some(value),
            EdgeSelection::Right => self.right = Some(value),
            EdgeSelection::Bottom => self.bottom = Some(value),
            EdgeSelection::Left => self.left = Some(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicalEdgeColors {
    pub block_start: Option<StyleColor>,
    pub block_end: Option<StyleColor>,
    pub inline_start: Option<StyleColor>,
    pub inline_end: Option<StyleColor>,
}

impl LogicalEdgeColors {
    fn apply_edges(&mut self, edges: LogicalEdgeSelection, value: StyleColor) {
        match edges {
            LogicalEdgeSelection::Block => {
                self.block_start = Some(value.clone());
                self.block_end = Some(value);
            }
            LogicalEdgeSelection::Inline => {
                self.inline_start = Some(value.clone());
                self.inline_end = Some(value);
            }
            LogicalEdgeSelection::BlockStart => self.block_start = Some(value),
            LogicalEdgeSelection::BlockEnd => self.block_end = Some(value),
            LogicalEdgeSelection::InlineStart => self.inline_start = Some(value),
            LogicalEdgeSelection::InlineEnd => self.inline_end = Some(value),
        }
    }

    fn apply_axis_values(&mut self, axis: LogicalEdgeSelection, value: &str) {
        if let Some(value) = parse_color(value) {
            self.apply_edges(axis, value);
            return;
        }
        let values = value
            .split_whitespace()
            .filter_map(parse_color)
            .collect::<Vec<_>>();
        match (axis, values.as_slice()) {
            (_, []) => {}
            (LogicalEdgeSelection::Block, [both]) => {
                self.block_start = Some(both.clone());
                self.block_end = Some(both.clone());
            }
            (LogicalEdgeSelection::Block, [start, end, ..]) => {
                self.block_start = Some(start.clone());
                self.block_end = Some(end.clone());
            }
            (LogicalEdgeSelection::Inline, [both]) => {
                self.inline_start = Some(both.clone());
                self.inline_end = Some(both.clone());
            }
            (LogicalEdgeSelection::Inline, [start, end, ..]) => {
                self.inline_start = Some(start.clone());
                self.inline_end = Some(end.clone());
            }
            (_, [single, ..]) => self.apply_edges(axis, single.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeBorderStyles {
    pub top: Option<BorderStyle>,
    pub right: Option<BorderStyle>,
    pub bottom: Option<BorderStyle>,
    pub left: Option<BorderStyle>,
}

impl EdgeBorderStyles {
    fn set_all(&mut self, value: Option<BorderStyle>) {
        self.top = value;
        self.right = value;
        self.bottom = value;
        self.left = value;
    }

    fn apply_edges(&mut self, edges: EdgeSelection, value: BorderStyle) {
        match edges {
            EdgeSelection::All => self.set_all(Some(value)),
            EdgeSelection::X => {
                self.left = Some(value);
                self.right = Some(value);
            }
            EdgeSelection::Y => {
                self.top = Some(value);
                self.bottom = Some(value);
            }
            EdgeSelection::Top => self.top = Some(value),
            EdgeSelection::Right => self.right = Some(value),
            EdgeSelection::Bottom => self.bottom = Some(value),
            EdgeSelection::Left => self.left = Some(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicalBorderStyles {
    pub block_start: Option<BorderStyle>,
    pub block_end: Option<BorderStyle>,
    pub inline_start: Option<BorderStyle>,
    pub inline_end: Option<BorderStyle>,
}

impl LogicalBorderStyles {
    fn apply_edges(&mut self, edges: LogicalEdgeSelection, value: BorderStyle) {
        match edges {
            LogicalEdgeSelection::Block => {
                self.block_start = Some(value);
                self.block_end = Some(value);
            }
            LogicalEdgeSelection::Inline => {
                self.inline_start = Some(value);
                self.inline_end = Some(value);
            }
            LogicalEdgeSelection::BlockStart => self.block_start = Some(value),
            LogicalEdgeSelection::BlockEnd => self.block_end = Some(value),
            LogicalEdgeSelection::InlineStart => self.inline_start = Some(value),
            LogicalEdgeSelection::InlineEnd => self.inline_end = Some(value),
        }
    }

    fn apply_axis_values(&mut self, axis: LogicalEdgeSelection, value: &str) {
        if let Some(value) = parse_border_style(value) {
            self.apply_edges(axis, value);
            return;
        }
        let values = value
            .split_whitespace()
            .filter_map(parse_border_style)
            .collect::<Vec<_>>();
        match (axis, values.as_slice()) {
            (_, []) => {}
            (LogicalEdgeSelection::Block, [both]) => {
                self.block_start = Some(*both);
                self.block_end = Some(*both);
            }
            (LogicalEdgeSelection::Block, [start, end, ..]) => {
                self.block_start = Some(*start);
                self.block_end = Some(*end);
            }
            (LogicalEdgeSelection::Inline, [both]) => {
                self.inline_start = Some(*both);
                self.inline_end = Some(*both);
            }
            (LogicalEdgeSelection::Inline, [start, end, ..]) => {
                self.inline_start = Some(*start);
                self.inline_end = Some(*end);
            }
            (_, [single, ..]) => self.apply_edges(axis, *single),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CornerRadius {
    pub horizontal: StyleLength,
    pub vertical: Option<StyleLength>,
}

impl CornerRadius {
    fn circular(value: StyleLength) -> Self {
        Self {
            horizontal: value,
            vertical: None,
        }
    }

    fn elliptical(horizontal: StyleLength, vertical: StyleLength) -> Self {
        Self {
            horizontal,
            vertical: Some(vertical),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CornerRadii {
    pub top_left: Option<CornerRadius>,
    pub top_right: Option<CornerRadius>,
    pub bottom_right: Option<CornerRadius>,
    pub bottom_left: Option<CornerRadius>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicalCornerRadii {
    pub start_start: Option<CornerRadius>,
    pub start_end: Option<CornerRadius>,
    pub end_end: Option<CornerRadius>,
    pub end_start: Option<CornerRadius>,
}

#[derive(Debug, Clone, Copy)]
enum CornerSelection {
    All,
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Clone, Copy)]
enum LogicalCornerSelection {
    Start,
    End,
    StartStart,
    StartEnd,
    EndEnd,
    EndStart,
}

pub fn normalize_css_property_name(property: &str) -> String {
    let property = property.trim();
    if property.starts_with("--") {
        return property.to_string();
    }
    let mut normalized = String::with_capacity(property.len());
    for (index, ch) in property.chars().enumerate() {
        if ch == '_' {
            normalized.push('-');
        } else if ch.is_ascii_uppercase() {
            if index > 0 {
                normalized.push('-');
            }
            normalized.push(ch.to_ascii_lowercase());
        } else {
            normalized.push(ch.to_ascii_lowercase());
        }
    }
    normalized
}

fn normalize_css_value(value: &str) -> String {
    value.trim().to_string()
}

fn parse_display(value: &str) -> Option<DisplayMode> {
    let value = value.trim().to_ascii_lowercase();
    match value.as_str() {
        "inline" => Some(DisplayMode::Inline),
        "inline-block" => Some(DisplayMode::InlineBlock),
        "flow" => Some(DisplayMode::Block),
        "flex" => Some(DisplayMode::Flex),
        "inline-flex" => Some(DisplayMode::InlineFlex),
        "block" => Some(DisplayMode::Block),
        "grid" => Some(DisplayMode::Grid),
        "inline-grid" => Some(DisplayMode::InlineGrid),
        "flow-root" => Some(DisplayMode::FlowRoot),
        "contents" => Some(DisplayMode::Contents),
        "list-item" => Some(DisplayMode::ListItem),
        "table" => Some(DisplayMode::Table),
        "inline-table" => Some(DisplayMode::InlineTable),
        "table-caption" => Some(DisplayMode::TableCaption),
        "table-cell" => Some(DisplayMode::TableCell),
        "table-column" => Some(DisplayMode::TableColumn),
        "table-column-group" => Some(DisplayMode::TableColumnGroup),
        "table-footer-group" => Some(DisplayMode::TableFooterGroup),
        "table-header-group" => Some(DisplayMode::TableHeaderGroup),
        "table-row-group" => Some(DisplayMode::TableRowGroup),
        "table-row" => Some(DisplayMode::TableRow),
        "ruby" => Some(DisplayMode::Ruby),
        "ruby-base" => Some(DisplayMode::RubyBase),
        "ruby-text" => Some(DisplayMode::RubyText),
        "ruby-base-container" => Some(DisplayMode::RubyBaseContainer),
        "ruby-text-container" => Some(DisplayMode::RubyTextContainer),
        "-webkit-box" => Some(DisplayMode::WebkitBox),
        "none" => Some(DisplayMode::None),
        _ => parse_multi_keyword_display(&value),
    }
}

fn parse_multi_keyword_display(value: &str) -> Option<DisplayMode> {
    let tokens = value.split_ascii_whitespace();
    let mut outside = None;
    let mut inside = None;
    let mut list_item = false;
    let mut count = 0usize;

    for token in tokens {
        count += 1;
        match token {
            "block" | "inline" => {
                if outside.is_some() {
                    return None;
                }
                outside = Some(token);
            }
            "flow" | "flow-root" | "table" | "flex" | "grid" | "ruby" => {
                if inside.is_some() {
                    return None;
                }
                inside = Some(token);
            }
            "list-item" => {
                if list_item {
                    return None;
                }
                list_item = true;
            }
            _ => return None,
        }
    }

    if !(2..=3).contains(&count) {
        return None;
    }

    if list_item {
        return match (outside.unwrap_or("block"), inside.unwrap_or("flow")) {
            ("block", "flow") => Some(DisplayMode::ListItem),
            _ => None,
        };
    }

    let outside = outside?;
    let inside = inside.unwrap_or("flow");
    match (outside, inside) {
        ("block", "flow") => Some(DisplayMode::Block),
        ("inline", "flow") => Some(DisplayMode::Inline),
        ("block", "flow-root") => Some(DisplayMode::FlowRoot),
        ("inline", "flow-root") => Some(DisplayMode::InlineBlock),
        ("block", "table") => Some(DisplayMode::Table),
        ("inline", "table") => Some(DisplayMode::InlineTable),
        ("block", "flex") => Some(DisplayMode::Flex),
        ("inline", "flex") => Some(DisplayMode::InlineFlex),
        ("block", "grid") => Some(DisplayMode::Grid),
        ("inline", "grid") => Some(DisplayMode::InlineGrid),
        ("inline", "ruby") => Some(DisplayMode::Ruby),
        _ => None,
    }
}

fn parse_box_sizing(value: &str) -> Option<BoxSizing> {
    match value.trim() {
        "border-box" => Some(BoxSizing::BorderBox),
        "content-box" => Some(BoxSizing::ContentBox),
        _ => None,
    }
}

fn parse_box_decoration_break(value: &str) -> Option<BoxDecorationBreak> {
    match value.trim() {
        "slice" => Some(BoxDecorationBreak::Slice),
        "clone" => Some(BoxDecorationBreak::Clone),
        _ => None,
    }
}

fn parse_position(value: &str) -> Option<PositionMode> {
    match value.trim() {
        "static" => Some(PositionMode::Static),
        "relative" => Some(PositionMode::Relative),
        "absolute" => Some(PositionMode::Absolute),
        "fixed" => Some(PositionMode::Fixed),
        "sticky" => Some(PositionMode::Sticky),
        _ => None,
    }
}

fn parse_flex_direction(value: &str) -> Option<Orientation> {
    match value.trim() {
        "row" | "row-reverse" => Some(Orientation::Horizontal),
        "column" | "column-reverse" => Some(Orientation::Vertical),
        _ => None,
    }
}

fn parse_flex_wrap(value: &str) -> Option<FlexWrap> {
    match value.trim() {
        "nowrap" => Some(FlexWrap::NoWrap),
        "wrap" => Some(FlexWrap::Wrap),
        "wrap-reverse" => Some(FlexWrap::WrapReverse),
        _ => None,
    }
}

fn parse_grid_auto_flow(value: &str) -> Option<GridAutoFlow> {
    match value.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["row"] => Some(GridAutoFlow::Row),
        ["column"] => Some(GridAutoFlow::Column),
        ["dense"] => Some(GridAutoFlow::Dense),
        ["row", "dense"] | ["dense", "row"] => Some(GridAutoFlow::RowDense),
        ["column", "dense"] | ["dense", "column"] => Some(GridAutoFlow::ColumnDense),
        _ => None,
    }
}

fn parse_container_type(value: &str) -> Option<ContainerType> {
    match value.trim() {
        "normal" => Some(ContainerType::Normal),
        "size" => Some(ContainerType::Size),
        "inline-size" => Some(ContainerType::InlineSize),
        _ => None,
    }
}

fn parse_content_visibility(value: &str) -> Option<ContentVisibility> {
    match value.trim() {
        "visible" => Some(ContentVisibility::Visible),
        "auto" => Some(ContentVisibility::Auto),
        "hidden" => Some(ContentVisibility::Hidden),
        _ => None,
    }
}

fn parse_align_items(value: &str) -> Option<AlignItems> {
    match value.trim() {
        "normal" => Some(AlignItems::Normal),
        "flex-start" | "start" => Some(AlignItems::Start),
        "center" => Some(AlignItems::Center),
        "flex-end" | "end" => Some(AlignItems::End),
        "stretch" => Some(AlignItems::Stretch),
        "baseline" => Some(AlignItems::Baseline),
        _ => None,
    }
}

fn parse_justify_content(value: &str) -> Option<JustifyContent> {
    match value.trim() {
        "normal" => Some(JustifyContent::Normal),
        "flex-start" | "start" => Some(JustifyContent::Start),
        "center" => Some(JustifyContent::Center),
        "flex-end" | "end" => Some(JustifyContent::End),
        "space-between" => Some(JustifyContent::SpaceBetween),
        "space-around" => Some(JustifyContent::SpaceAround),
        "space-evenly" => Some(JustifyContent::SpaceEvenly),
        "stretch" => Some(JustifyContent::Stretch),
        "baseline" => Some(JustifyContent::Baseline),
        _ => None,
    }
}

fn parse_self_alignment(value: &str) -> Option<SelfAlignment> {
    match value.trim() {
        "auto" => Some(SelfAlignment::Auto),
        "flex-start" | "start" => Some(SelfAlignment::Start),
        "center" => Some(SelfAlignment::Center),
        "flex-end" | "end" => Some(SelfAlignment::End),
        "stretch" => Some(SelfAlignment::Stretch),
        "baseline" => Some(SelfAlignment::Baseline),
        _ => None,
    }
}

fn parse_border_style(value: &str) -> Option<BorderStyle> {
    match value.trim() {
        "none" => Some(BorderStyle::None),
        "hidden" => Some(BorderStyle::Hidden),
        "solid" => Some(BorderStyle::Solid),
        "dashed" => Some(BorderStyle::Dashed),
        "dotted" => Some(BorderStyle::Dotted),
        "double" => Some(BorderStyle::Double),
        "groove" => Some(BorderStyle::Groove),
        "ridge" => Some(BorderStyle::Ridge),
        "inset" => Some(BorderStyle::Inset),
        "outset" => Some(BorderStyle::Outset),
        _ => None,
    }
}

fn parse_background_attachment(value: &str) -> Option<BackgroundAttachment> {
    match value.trim() {
        "fixed" => Some(BackgroundAttachment::Fixed),
        "local" => Some(BackgroundAttachment::Local),
        "scroll" => Some(BackgroundAttachment::Scroll),
        _ => None,
    }
}

fn parse_background_box(value: &str) -> Option<BackgroundBox> {
    match value.trim() {
        "border-box" => Some(BackgroundBox::BorderBox),
        "padding-box" => Some(BackgroundBox::PaddingBox),
        "content-box" => Some(BackgroundBox::ContentBox),
        "text" => Some(BackgroundBox::Text),
        _ => None,
    }
}

fn parse_object_fit(value: &str) -> Option<ObjectFit> {
    match value.trim() {
        "fill" => Some(ObjectFit::Fill),
        "contain" => Some(ObjectFit::Contain),
        "cover" => Some(ObjectFit::Cover),
        "none" => Some(ObjectFit::None),
        "scale-down" => Some(ObjectFit::ScaleDown),
        _ => None,
    }
}

fn parse_list_style_position(value: &str) -> Option<ListStylePosition> {
    match value.trim() {
        "inside" => Some(ListStylePosition::Inside),
        "outside" => Some(ListStylePosition::Outside),
        _ => None,
    }
}

fn parse_font_style(value: &str) -> Option<FontStyle> {
    match value.trim() {
        "normal" => Some(FontStyle::Normal),
        "italic" => Some(FontStyle::Italic),
        "oblique" => Some(FontStyle::Oblique),
        _ => None,
    }
}

fn parse_font_weight(value: &str) -> Option<FontWeight> {
    let value = value.trim();
    if let Ok(number) = value.parse::<u16>() {
        return Some(FontWeight::Number(number));
    }
    if matches!(
        value,
        "normal" | "bold" | "bolder" | "lighter" | "inherit" | "initial" | "unset"
    ) {
        Some(FontWeight::Keyword(value.to_string()))
    } else {
        None
    }
}

fn parse_text_align(value: &str) -> Option<TextAlign> {
    match value.trim() {
        "left" | "start" => Some(TextAlign::Start),
        "center" => Some(TextAlign::Center),
        "right" | "end" => Some(TextAlign::End),
        "justify" => Some(TextAlign::Justify),
        _ => None,
    }
}

fn parse_text_direction(value: &str) -> Option<TextDirection> {
    match value.trim() {
        "ltr" => Some(TextDirection::Ltr),
        "rtl" => Some(TextDirection::Rtl),
        _ => None,
    }
}

fn parse_unicode_bidi(value: &str) -> Option<UnicodeBidi> {
    match value.trim() {
        "normal" => Some(UnicodeBidi::Normal),
        "embed" => Some(UnicodeBidi::Embed),
        "isolate" => Some(UnicodeBidi::Isolate),
        "bidi-override" => Some(UnicodeBidi::BidiOverride),
        "isolate-override" => Some(UnicodeBidi::IsolateOverride),
        "plaintext" => Some(UnicodeBidi::Plaintext),
        _ => None,
    }
}

fn parse_writing_mode(value: &str) -> Option<WritingMode> {
    match value.trim() {
        "horizontal-tb" => Some(WritingMode::HorizontalTb),
        "vertical-rl" => Some(WritingMode::VerticalRl),
        "vertical-lr" => Some(WritingMode::VerticalLr),
        "sideways-rl" => Some(WritingMode::SidewaysRl),
        "sideways-lr" => Some(WritingMode::SidewaysLr),
        _ => None,
    }
}

fn parse_text_orientation(value: &str) -> Option<TextOrientation> {
    match value.trim() {
        "mixed" => Some(TextOrientation::Mixed),
        "upright" => Some(TextOrientation::Upright),
        "sideways" => Some(TextOrientation::Sideways),
        "sideways-right" => Some(TextOrientation::SidewaysRight),
        "use-glyph-orientation" => Some(TextOrientation::UseGlyphOrientation),
        _ => None,
    }
}

fn parse_text_transform(value: &str) -> Option<TextTransform> {
    match value.trim() {
        "none" => Some(TextTransform::None),
        "uppercase" => Some(TextTransform::Uppercase),
        "lowercase" => Some(TextTransform::Lowercase),
        "capitalize" => Some(TextTransform::Capitalize),
        _ => None,
    }
}

fn parse_fill_rule(value: &str) -> Option<FillRule> {
    match value.trim() {
        "nonzero" => Some(FillRule::Nonzero),
        "evenodd" => Some(FillRule::Evenodd),
        _ => None,
    }
}

fn parse_stroke_linecap(value: &str) -> Option<StrokeLineCap> {
    match value.trim() {
        "butt" => Some(StrokeLineCap::Butt),
        "round" => Some(StrokeLineCap::Round),
        "square" => Some(StrokeLineCap::Square),
        _ => None,
    }
}

fn parse_stroke_linejoin(value: &str) -> Option<StrokeLineJoin> {
    match value.trim() {
        "arcs" => Some(StrokeLineJoin::Arcs),
        "bevel" => Some(StrokeLineJoin::Bevel),
        "miter" => Some(StrokeLineJoin::Miter),
        "miter-clip" => Some(StrokeLineJoin::MiterClip),
        "round" => Some(StrokeLineJoin::Round),
        _ => None,
    }
}

fn parse_text_decoration_line(value: &str) -> Option<&str> {
    match value.trim() {
        "none" | "underline" | "overline" | "line-through" | "blink" => Some(value.trim()),
        _ => None,
    }
}

fn parse_text_decoration_style(value: &str) -> Option<TextDecorationStyle> {
    match value.trim() {
        "solid" => Some(TextDecorationStyle::Solid),
        "double" => Some(TextDecorationStyle::Double),
        "dotted" => Some(TextDecorationStyle::Dotted),
        "dashed" => Some(TextDecorationStyle::Dashed),
        "wavy" => Some(TextDecorationStyle::Wavy),
        _ => None,
    }
}

fn is_text_emphasis_style_token(value: &str) -> bool {
    matches!(
        value.trim(),
        "none" | "filled" | "open" | "dot" | "circle" | "double-circle" | "triangle" | "sesame"
    ) || value.trim().starts_with('"')
        || value.trim().starts_with('\'')
}

fn parse_text_overflow(value: &str) -> Option<TextOverflow> {
    match value.trim() {
        "clip" => Some(TextOverflow::Clip),
        "ellipsis" => Some(TextOverflow::Ellipsis),
        _ => None,
    }
}

fn parse_white_space(value: &str) -> Option<WhiteSpaceMode> {
    match value.trim() {
        "normal" => Some(WhiteSpaceMode::Normal),
        "nowrap" => Some(WhiteSpaceMode::NoWrap),
        "pre" => Some(WhiteSpaceMode::Pre),
        "pre-line" => Some(WhiteSpaceMode::PreLine),
        "pre-wrap" => Some(WhiteSpaceMode::PreWrap),
        "break-spaces" => Some(WhiteSpaceMode::BreakSpaces),
        _ => None,
    }
}

fn parse_text_wrap(value: &str) -> Option<TextWrapMode> {
    match value.trim() {
        "wrap" => Some(TextWrapMode::Wrap),
        "nowrap" => Some(TextWrapMode::NoWrap),
        "balance" => Some(TextWrapMode::Balance),
        "pretty" => Some(TextWrapMode::Pretty),
        "stable" => Some(TextWrapMode::Stable),
        _ => None,
    }
}

fn parse_word_break(value: &str) -> Option<WordBreakMode> {
    match value.trim() {
        "normal" => Some(WordBreakMode::Normal),
        "break-all" => Some(WordBreakMode::BreakAll),
        "keep-all" => Some(WordBreakMode::KeepAll),
        "break-word" => Some(WordBreakMode::BreakWord),
        _ => None,
    }
}

fn parse_overflow_wrap(value: &str) -> Option<OverflowWrapMode> {
    match value.trim() {
        "normal" => Some(OverflowWrapMode::Normal),
        "break-word" => Some(OverflowWrapMode::BreakWord),
        "anywhere" => Some(OverflowWrapMode::Anywhere),
        _ => None,
    }
}

fn parse_hyphens(value: &str) -> Option<HyphensMode> {
    match value.trim() {
        "none" => Some(HyphensMode::None),
        "manual" => Some(HyphensMode::Manual),
        "auto" => Some(HyphensMode::Auto),
        _ => None,
    }
}

fn parse_overflow(value: &str) -> Option<OverflowMode> {
    match value.trim() {
        "visible" => Some(OverflowMode::Visible),
        "hidden" => Some(OverflowMode::Hidden),
        "scroll" => Some(OverflowMode::Scroll),
        "auto" => Some(OverflowMode::Auto),
        "clip" => Some(OverflowMode::Clip),
        _ => None,
    }
}

fn parse_visibility(value: &str) -> Option<VisibilityMode> {
    match value.trim() {
        "visible" => Some(VisibilityMode::Visible),
        "hidden" => Some(VisibilityMode::Hidden),
        "collapse" => Some(VisibilityMode::Collapse),
        _ => None,
    }
}

fn parse_isolation(value: &str) -> Option<IsolationMode> {
    match value.trim() {
        "auto" => Some(IsolationMode::Auto),
        "isolate" => Some(IsolationMode::Isolate),
        _ => None,
    }
}

fn parse_blend_mode(value: &str) -> Option<BlendMode> {
    match value.trim() {
        "normal" => Some(BlendMode::Normal),
        "multiply" => Some(BlendMode::Multiply),
        "screen" => Some(BlendMode::Screen),
        "overlay" => Some(BlendMode::Overlay),
        "darken" => Some(BlendMode::Darken),
        "lighten" => Some(BlendMode::Lighten),
        "color-dodge" => Some(BlendMode::ColorDodge),
        "color-burn" => Some(BlendMode::ColorBurn),
        "hard-light" => Some(BlendMode::HardLight),
        "soft-light" => Some(BlendMode::SoftLight),
        "difference" => Some(BlendMode::Difference),
        "exclusion" => Some(BlendMode::Exclusion),
        "hue" => Some(BlendMode::Hue),
        "saturation" => Some(BlendMode::Saturation),
        "color" => Some(BlendMode::Color),
        "luminosity" => Some(BlendMode::Luminosity),
        "plus-darker" => Some(BlendMode::PlusDarker),
        "plus-lighter" => Some(BlendMode::PlusLighter),
        _ => None,
    }
}

fn parse_float(value: &str) -> Option<FloatMode> {
    match value.trim() {
        "left" => Some(FloatMode::Left),
        "right" => Some(FloatMode::Right),
        "inline-start" => Some(FloatMode::InlineStart),
        "inline-end" => Some(FloatMode::InlineEnd),
        "footnote" => Some(FloatMode::Footnote),
        "none" => Some(FloatMode::None),
        _ => None,
    }
}

fn parse_clear(value: &str) -> Option<ClearMode> {
    match value.trim() {
        "left" => Some(ClearMode::Left),
        "right" => Some(ClearMode::Right),
        "both" => Some(ClearMode::Both),
        "inline-start" => Some(ClearMode::InlineStart),
        "inline-end" => Some(ClearMode::InlineEnd),
        "none" => Some(ClearMode::None),
        _ => None,
    }
}

fn parse_table_layout(value: &str) -> Option<TableLayout> {
    match value.trim() {
        "auto" => Some(TableLayout::Auto),
        "fixed" => Some(TableLayout::Fixed),
        _ => None,
    }
}

fn parse_border_collapse(value: &str) -> Option<BorderCollapse> {
    match value.trim() {
        "collapse" => Some(BorderCollapse::Collapse),
        "separate" => Some(BorderCollapse::Separate),
        _ => None,
    }
}

fn parse_caption_side(value: &str) -> Option<CaptionSide> {
    match value.trim() {
        "top" => Some(CaptionSide::Top),
        "bottom" => Some(CaptionSide::Bottom),
        _ => None,
    }
}

fn parse_z_index(value: &str) -> Option<i32> {
    value.trim().parse::<i32>().ok()
}

fn parse_opacity(value: &str) -> Option<f64> {
    let value = value.trim();
    if let Some(percent) = value.strip_suffix('%') {
        return Some((percent.trim().parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0));
    }
    value.parse::<f64>().ok().map(|value| value.clamp(0.0, 1.0))
}

fn parse_css_string_token(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn parse_non_empty_css_string(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn join_css_functions<const N: usize>(values: [Option<&str>; N]) -> Option<String> {
    let parts = values
        .into_iter()
        .flatten()
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn is_tailwind_translate_pipeline(value: &str) -> bool {
    value.contains("var(--tw-translate-x)") || value.contains("var(--tw-translate-y)")
}

fn is_tailwind_scale_pipeline(value: &str) -> bool {
    value.contains("var(--tw-scale-x)") || value.contains("var(--tw-scale-y)")
}

fn is_tailwind_border_spacing_pipeline(value: &str) -> bool {
    value.contains("var(--tw-border-spacing-x)") || value.contains("var(--tw-border-spacing-y)")
}

fn is_tailwind_transform_pipeline(value: &str) -> bool {
    value.contains("var(--tw-rotate-x)")
        || value.contains("var(--tw-rotate-y)")
        || value.contains("var(--tw-rotate-z)")
        || value.contains("var(--tw-skew-x)")
        || value.contains("var(--tw-skew-y)")
}

fn is_tailwind_filter_pipeline(value: &str) -> bool {
    value.contains("var(--tw-blur)")
        || value.contains("var(--tw-brightness)")
        || value.contains("var(--tw-contrast)")
        || value.contains("var(--tw-drop-shadow)")
        || value.contains("var(--tw-grayscale)")
        || value.contains("var(--tw-hue-rotate)")
        || value.contains("var(--tw-invert)")
        || value.contains("var(--tw-saturate)")
        || value.contains("var(--tw-sepia)")
}

fn is_tailwind_backdrop_filter_pipeline(value: &str) -> bool {
    value.contains("var(--tw-backdrop-blur)")
        || value.contains("var(--tw-backdrop-brightness)")
        || value.contains("var(--tw-backdrop-contrast)")
        || value.contains("var(--tw-backdrop-grayscale)")
        || value.contains("var(--tw-backdrop-hue-rotate)")
        || value.contains("var(--tw-backdrop-invert)")
        || value.contains("var(--tw-backdrop-opacity)")
        || value.contains("var(--tw-backdrop-saturate)")
        || value.contains("var(--tw-backdrop-sepia)")
}

fn is_tailwind_font_variant_numeric_pipeline(value: &str) -> bool {
    value.contains("var(--tw-ordinal)")
        || value.contains("var(--tw-slashed-zero)")
        || value.contains("var(--tw-numeric-figure)")
        || value.contains("var(--tw-numeric-spacing)")
        || value.contains("var(--tw-numeric-fraction)")
}

fn parse_pointer_events(value: &str) -> Option<PointerEvents> {
    match value.trim() {
        "auto" => Some(PointerEvents::Auto),
        "none" => Some(PointerEvents::None),
        "visiblePainted" | "visible-painted" => Some(PointerEvents::VisiblePainted),
        "visibleFill" | "visible-fill" => Some(PointerEvents::VisibleFill),
        "visibleStroke" | "visible-stroke" => Some(PointerEvents::VisibleStroke),
        "visible" => Some(PointerEvents::Visible),
        "painted" => Some(PointerEvents::Painted),
        "fill" => Some(PointerEvents::Fill),
        "stroke" => Some(PointerEvents::Stroke),
        "bounding-box" | "boundingBox" => Some(PointerEvents::BoundingBox),
        "all" => Some(PointerEvents::All),
        _ => None,
    }
}

fn parse_user_select(value: &str) -> Option<UserSelect> {
    match value.trim() {
        "auto" => Some(UserSelect::Auto),
        "text" => Some(UserSelect::Text),
        "none" => Some(UserSelect::None),
        "all" => Some(UserSelect::All),
        "contain" => Some(UserSelect::Contain),
        _ => None,
    }
}

fn parse_backface_visibility(value: &str) -> Option<BackfaceVisibility> {
    match value.trim() {
        "visible" => Some(BackfaceVisibility::Visible),
        "hidden" => Some(BackfaceVisibility::Hidden),
        _ => None,
    }
}

fn parse_resize(value: &str) -> Option<ResizeMode> {
    match value.trim() {
        "none" => Some(ResizeMode::None),
        "both" => Some(ResizeMode::Both),
        "horizontal" => Some(ResizeMode::Horizontal),
        "vertical" => Some(ResizeMode::Vertical),
        "block" => Some(ResizeMode::Block),
        "inline" => Some(ResizeMode::Inline),
        _ => None,
    }
}

fn parse_scroll_behavior(value: &str) -> Option<ScrollBehavior> {
    match value.trim() {
        "auto" => Some(ScrollBehavior::Auto),
        "smooth" => Some(ScrollBehavior::Smooth),
        _ => None,
    }
}

fn parse_overscroll_behavior(value: &str) -> Option<OverscrollBehavior> {
    match value.trim() {
        "auto" => Some(OverscrollBehavior::Auto),
        "contain" => Some(OverscrollBehavior::Contain),
        "none" => Some(OverscrollBehavior::None),
        _ => None,
    }
}

fn parse_edge_insets(value: &str) -> EdgeInsets {
    let values = value
        .split_whitespace()
        .filter_map(parse_length)
        .collect::<Vec<_>>();
    let mut edges = EdgeInsets::default();
    match values.as_slice() {
        [] => {}
        [all] => edges.set_all(Some(all.clone())),
        [vertical, horizontal] => {
            edges.top = Some(vertical.clone());
            edges.bottom = Some(vertical.clone());
            edges.left = Some(horizontal.clone());
            edges.right = Some(horizontal.clone());
        }
        [top, horizontal, bottom] => {
            edges.top = Some(top.clone());
            edges.left = Some(horizontal.clone());
            edges.right = Some(horizontal.clone());
            edges.bottom = Some(bottom.clone());
        }
        [top, right, bottom, left, ..] => {
            edges.top = Some(top.clone());
            edges.right = Some(right.clone());
            edges.bottom = Some(bottom.clone());
            edges.left = Some(left.clone());
        }
    }
    edges
}

fn parse_edge_colors(value: &str) -> EdgeColors {
    let values = parse_edge_values(value, parse_color);
    let mut edges = EdgeColors::default();
    match values.as_slice() {
        [] => {}
        [all] => edges.set_all(Some(all.clone())),
        [vertical, horizontal] => {
            edges.top = Some(vertical.clone());
            edges.bottom = Some(vertical.clone());
            edges.left = Some(horizontal.clone());
            edges.right = Some(horizontal.clone());
        }
        [top, horizontal, bottom] => {
            edges.top = Some(top.clone());
            edges.left = Some(horizontal.clone());
            edges.right = Some(horizontal.clone());
            edges.bottom = Some(bottom.clone());
        }
        [top, right, bottom, left, ..] => {
            edges.top = Some(top.clone());
            edges.right = Some(right.clone());
            edges.bottom = Some(bottom.clone());
            edges.left = Some(left.clone());
        }
    }
    edges
}

fn parse_edge_border_styles(value: &str) -> EdgeBorderStyles {
    let values = parse_edge_values(value, parse_border_style);
    let mut edges = EdgeBorderStyles::default();
    match values.as_slice() {
        [] => {}
        [all] => edges.set_all(Some(*all)),
        [vertical, horizontal] => {
            edges.top = Some(*vertical);
            edges.bottom = Some(*vertical);
            edges.left = Some(*horizontal);
            edges.right = Some(*horizontal);
        }
        [top, horizontal, bottom] => {
            edges.top = Some(*top);
            edges.left = Some(*horizontal);
            edges.right = Some(*horizontal);
            edges.bottom = Some(*bottom);
        }
        [top, right, bottom, left, ..] => {
            edges.top = Some(*top);
            edges.right = Some(*right);
            edges.bottom = Some(*bottom);
            edges.left = Some(*left);
        }
    }
    edges
}

fn parse_edge_values<T>(value: &str, parser: impl Fn(&str) -> Option<T>) -> Vec<T> {
    if let Some(value) = parser(value) {
        return vec![value];
    }
    value.split_whitespace().filter_map(parser).collect()
}

fn parse_corner_radius(value: &str) -> Option<CornerRadius> {
    if let Some(length) = parse_length(value) {
        return Some(CornerRadius::circular(length));
    }
    let values = value
        .split_whitespace()
        .filter_map(parse_length)
        .collect::<Vec<_>>();
    match values.as_slice() {
        [horizontal] => Some(CornerRadius::circular(horizontal.clone())),
        [horizontal, vertical, ..] => Some(CornerRadius::elliptical(
            horizontal.clone(),
            vertical.clone(),
        )),
        [] => None,
    }
}

fn parse_corner_radii(value: &str) -> CornerRadii {
    let (horizontal, vertical) = value
        .split_once('/')
        .map_or((value, None), |(horizontal, vertical)| {
            (horizontal.trim(), Some(vertical.trim()))
        });
    let horizontal = parse_corner_radius_values(horizontal);
    let vertical = vertical.map(parse_corner_radius_values);
    let mut radii = CornerRadii::default();
    let Some(horizontal) = expand_corner_values(&horizontal) else {
        return radii;
    };
    let vertical = vertical
        .as_ref()
        .and_then(|values| expand_corner_values(values));
    radii.top_left = Some(make_corner_radius(
        &horizontal[0],
        vertical.as_ref().map(|r| &r[0]),
    ));
    radii.top_right = Some(make_corner_radius(
        &horizontal[1],
        vertical.as_ref().map(|r| &r[1]),
    ));
    radii.bottom_right = Some(make_corner_radius(
        &horizontal[2],
        vertical.as_ref().map(|r| &r[2]),
    ));
    radii.bottom_left = Some(make_corner_radius(
        &horizontal[3],
        vertical.as_ref().map(|r| &r[3]),
    ));
    radii
}

fn parse_corner_radius_values(value: &str) -> Vec<StyleLength> {
    if let Some(length) = parse_length(value) {
        return vec![length];
    }
    value.split_whitespace().filter_map(parse_length).collect()
}

fn expand_corner_values(values: &[StyleLength]) -> Option<[StyleLength; 4]> {
    match values {
        [all] => Some([all.clone(), all.clone(), all.clone(), all.clone()]),
        [top_left_bottom_right, top_right_bottom_left] => Some([
            top_left_bottom_right.clone(),
            top_right_bottom_left.clone(),
            top_left_bottom_right.clone(),
            top_right_bottom_left.clone(),
        ]),
        [top_left, top_right_bottom_left, bottom_right] => Some([
            top_left.clone(),
            top_right_bottom_left.clone(),
            bottom_right.clone(),
            top_right_bottom_left.clone(),
        ]),
        [top_left, top_right, bottom_right, bottom_left, ..] => Some([
            top_left.clone(),
            top_right.clone(),
            bottom_right.clone(),
            bottom_left.clone(),
        ]),
        [] => None,
    }
}

fn make_corner_radius(horizontal: &StyleLength, vertical: Option<&StyleLength>) -> CornerRadius {
    CornerRadius {
        horizontal: horizontal.clone(),
        vertical: vertical.cloned(),
    }
}
