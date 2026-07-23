use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::geometry::Orientation;
use crate::web::WebProps;
use serde::{Deserialize, Serialize};

use super::tailwind;
use super::types::*;

const PORTABLE_STYLE_CACHE_LIMIT: usize = 1024;

thread_local! {
    static PORTABLE_STYLE_CACHE: RefCell<BTreeMap<PortableStyleCacheKey, PortableStyle>> =
        RefCell::new(BTreeMap::new());
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PortableStyleCacheKey {
    class_name: Option<String>,
    style: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableStyle {
    pub declarations: BTreeMap<String, String>,
    pub custom_properties: BTreeMap<String, String>,
    pub variant_declarations: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variant_declaration_order: Vec<(String, String)>,
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
    pub tailwind_shadow: Option<String>,
    pub tailwind_shadow_color: Option<String>,
    pub tailwind_inset_shadow: Option<String>,
    pub tailwind_inset_shadow_color: Option<String>,
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeSizeConstraints {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
}

impl PortableStyle {
    pub fn from_web(web: &WebProps) -> Self {
        let key = PortableStyleCacheKey {
            class_name: web.class_name.clone(),
            style: web.style.clone(),
        };
        if let Some(style) = PORTABLE_STYLE_CACHE.with(|cache| cache.borrow().get(&key).cloned()) {
            return style;
        }

        let mut style = PortableStyle::default();
        if let Some(class_name) = &web.class_name {
            for class in tailwind::ordered_class_tokens(class_name) {
                style.apply_tailwind_utility(class);
            }
        }
        for (property, value) in &web.style {
            style.apply(property, value);
        }
        PORTABLE_STYLE_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() >= PORTABLE_STYLE_CACHE_LIMIT {
                if let Some(first) = cache.keys().next().cloned() {
                    cache.remove(&first);
                }
            }
            cache.insert(key, style.clone());
        });
        style
    }

    pub fn renders_native_widget(&self) -> bool {
        self.display != Some(DisplayMode::None)
            && self.content_visibility != Some(ContentVisibility::Hidden)
            && !matches!(
                self.visibility,
                Some(VisibilityMode::Hidden | VisibilityMode::Collapse)
            )
    }

    pub fn makes_native_widget_inert(&self) -> bool {
        self.interactivity
            .as_deref()
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("inert"))
    }

    pub fn native_size_constraints(&self) -> NativeSizeConstraints {
        let mut constraints = NativeSizeConstraints {
            width: native_size_points(self.width.as_ref()),
            height: native_size_points(self.height.as_ref()),
            min_width: native_size_points(self.min_width.as_ref()),
            min_height: native_size_points(self.min_height.as_ref()),
            max_width: native_size_points(self.max_width.as_ref()),
            max_height: native_size_points(self.max_height.as_ref()),
        };

        if self.uses_vertical_inline_axis() {
            constraints.width = constraints
                .width
                .or_else(|| native_size_points(self.block_size.as_ref()));
            constraints.height = constraints
                .height
                .or_else(|| native_size_points(self.inline_size.as_ref()));
            constraints.min_width = constraints
                .min_width
                .or_else(|| native_size_points(self.min_block_size.as_ref()));
            constraints.min_height = constraints
                .min_height
                .or_else(|| native_size_points(self.min_inline_size.as_ref()));
            constraints.max_width = constraints
                .max_width
                .or_else(|| native_size_points(self.max_block_size.as_ref()));
            constraints.max_height = constraints
                .max_height
                .or_else(|| native_size_points(self.max_inline_size.as_ref()));
        } else {
            constraints.width = constraints
                .width
                .or_else(|| native_size_points(self.inline_size.as_ref()));
            constraints.height = constraints
                .height
                .or_else(|| native_size_points(self.block_size.as_ref()));
            constraints.min_width = constraints
                .min_width
                .or_else(|| native_size_points(self.min_inline_size.as_ref()));
            constraints.min_height = constraints
                .min_height
                .or_else(|| native_size_points(self.min_block_size.as_ref()));
            constraints.max_width = constraints
                .max_width
                .or_else(|| native_size_points(self.max_inline_size.as_ref()));
            constraints.max_height = constraints
                .max_height
                .or_else(|| native_size_points(self.max_block_size.as_ref()));
        }

        constraints
    }

    fn uses_vertical_inline_axis(&self) -> bool {
        matches!(
            self.writing_mode,
            Some(
                WritingMode::VerticalRl
                    | WritingMode::VerticalLr
                    | WritingMode::SidewaysRl
                    | WritingMode::SidewaysLr
            )
        )
    }
}

fn native_size_points(value: Option<&StyleLength>) -> Option<f64> {
    value
        .and_then(StyleLength::points)
        .filter(|value| value.is_finite() && *value >= 0.0)
}
