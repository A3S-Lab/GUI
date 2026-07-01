use std::collections::BTreeMap;

use crate::geometry::Orientation;
use crate::web::WebProps;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableStyle {
    pub declarations: BTreeMap<String, String>,
    pub custom_properties: BTreeMap<String, String>,
    pub variant_declarations: BTreeMap<String, BTreeMap<String, String>>,
    pub display: Option<DisplayMode>,
    pub box_sizing: Option<BoxSizing>,
    pub box_decoration_break: Option<BoxDecorationBreak>,
    pub position: Option<PositionMode>,
    pub flex_direction: Option<Orientation>,
    pub flex_wrap: Option<FlexWrap>,
    pub flex: Option<String>,
    pub flex_basis: Option<StyleLength>,
    pub flex_grow: Option<String>,
    pub flex_shrink: Option<String>,
    pub order: Option<String>,
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
    pub content_visibility: Option<ContentVisibility>,
    pub contain_intrinsic_size: Option<String>,
    pub contain_intrinsic_width: Option<String>,
    pub contain_intrinsic_height: Option<String>,
    pub contain_intrinsic_inline_size: Option<String>,
    pub contain_intrinsic_block_size: Option<String>,
    pub inset: EdgeInsets,
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub scroll_margin: EdgeInsets,
    pub scroll_padding: EdgeInsets,
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
    pub box_shadow: Option<String>,
    pub outline_width: Option<StyleLength>,
    pub outline_color: Option<StyleColor>,
    pub outline_style: Option<BorderStyle>,
    pub outline_offset: Option<StyleLength>,
    pub color: Option<StyleColor>,
    pub accent_color: Option<StyleColor>,
    pub caret_color: Option<StyleColor>,
    pub background_color: Option<StyleColor>,
    pub background_image: Option<String>,
    pub background_position: Option<String>,
    pub background_size: Option<String>,
    pub background_repeat: Option<String>,
    pub background_attachment: Option<BackgroundAttachment>,
    pub background_origin: Option<BackgroundBox>,
    pub background_clip: Option<BackgroundBox>,
    pub background_blend_mode: Option<String>,
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
    pub object_fit: Option<ObjectFit>,
    pub object_position: Option<String>,
    pub list_style_type: Option<String>,
    pub list_style_position: Option<ListStylePosition>,
    pub columns: Option<String>,
    pub column_count: Option<String>,
    pub column_width: Option<StyleLength>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<StyleLength>,
    pub font_weight: Option<FontWeight>,
    pub font_stretch: Option<String>,
    pub font_kerning: Option<String>,
    pub font_optical_sizing: Option<String>,
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
    pub letter_spacing: Option<StyleLength>,
    pub word_spacing: Option<StyleLength>,
    pub tab_size: Option<String>,
    pub text_align: Option<TextAlign>,
    pub direction: Option<TextDirection>,
    pub unicode_bidi: Option<UnicodeBidi>,
    pub writing_mode: Option<WritingMode>,
    pub text_orientation: Option<TextOrientation>,
    pub text_transform: Option<TextTransform>,
    pub text_indent: Option<StyleLength>,
    pub text_wrap: Option<TextWrapMode>,
    pub line_clamp: Option<String>,
    pub box_orient: Option<String>,
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
    pub color_interpolation: Option<String>,
    pub color_interpolation_filters: Option<String>,
    pub text_decoration_line: Option<String>,
    pub text_decoration_color: Option<StyleColor>,
    pub text_decoration_style: Option<TextDecorationStyle>,
    pub text_decoration_thickness: Option<StyleLength>,
    pub text_underline_offset: Option<StyleLength>,
    pub text_shadow: Option<String>,
    pub text_overflow: Option<TextOverflow>,
    pub line_break: Option<String>,
    pub white_space: Option<WhiteSpaceMode>,
    pub word_break: Option<WordBreakMode>,
    pub overflow_wrap: Option<OverflowWrapMode>,
    pub hyphens: Option<HyphensMode>,
    pub overflow_x: Option<OverflowMode>,
    pub overflow_y: Option<OverflowMode>,
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
    pub aspect_ratio: Option<String>,
    pub transform: Option<String>,
    pub translate: Option<String>,
    pub rotate: Option<String>,
    pub scale: Option<String>,
    pub transform_origin: Option<String>,
    pub transform_style: Option<String>,
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
    pub animation: Option<String>,
    pub animation_name: Option<String>,
    pub animation_duration: Option<StyleTime>,
    pub animation_timing_function: Option<String>,
    pub animation_delay: Option<StyleTime>,
    pub animation_iteration_count: Option<String>,
    pub animation_direction: Option<String>,
    pub animation_fill_mode: Option<String>,
    pub animation_play_state: Option<String>,
    pub will_change: Option<String>,
    pub appearance: Option<String>,
    pub resize: Option<ResizeMode>,
    pub scroll_behavior: Option<ScrollBehavior>,
    pub scroll_snap_type: Option<String>,
    pub scroll_snap_align: Option<String>,
    pub scroll_snap_stop: Option<String>,
    pub overscroll_behavior_x: Option<OverscrollBehavior>,
    pub overscroll_behavior_y: Option<OverscrollBehavior>,
    pub touch_action: Option<String>,
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
            for class in class_name.split_whitespace() {
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
            "display" => self.display = parse_display(value_ref),
            "box-sizing" => self.box_sizing = parse_box_sizing(value_ref),
            "box-decoration-break" => {
                self.box_decoration_break = parse_box_decoration_break(value_ref);
            }
            "position" => self.position = parse_position(value_ref),
            "flex-direction" => self.flex_direction = parse_flex_direction(value_ref),
            "flex-wrap" => self.flex_wrap = parse_flex_wrap(value_ref),
            "flex" => self.flex = parse_css_string_token(value_ref),
            "flex-basis" => self.flex_basis = parse_length(value_ref),
            "flex-grow" => self.flex_grow = parse_css_string_token(value_ref),
            "flex-shrink" => self.flex_shrink = parse_css_string_token(value_ref),
            "order" => self.order = parse_css_string_token(value_ref),
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
            "box-shadow" => self.box_shadow = parse_css_string_token(value_ref),
            "outline" => self.apply_outline_shorthand(value_ref),
            "outline-width" => self.outline_width = parse_length(value_ref),
            "outline-color" => self.outline_color = parse_color(value_ref),
            "outline-style" => self.outline_style = parse_border_style(value_ref),
            "outline-offset" => self.outline_offset = parse_length(value_ref),
            "color" => self.color = parse_color(value_ref),
            "accent-color" => self.accent_color = parse_color(value_ref),
            "caret-color" => self.caret_color = parse_color(value_ref),
            "background" | "background-color" => self.background_color = parse_color(value_ref),
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
            "object-fit" => self.object_fit = parse_object_fit(value_ref),
            "object-position" => self.object_position = parse_css_string_token(value_ref),
            "list-style" => self.list_style_type = parse_css_string_token(value_ref),
            "list-style-type" => self.list_style_type = parse_css_string_token(value_ref),
            "list-style-position" => {
                self.list_style_position = parse_list_style_position(value_ref);
            }
            "columns" => self.columns = parse_css_string_token(value_ref),
            "column-count" => self.column_count = parse_css_string_token(value_ref),
            "column-width" => self.column_width = parse_length(value_ref),
            "font-family" => self.font_family = parse_css_string_token(value_ref),
            "font-style" => self.font_style = parse_font_style(value_ref),
            "font-size" => self.font_size = parse_length(value_ref),
            "font-weight" => self.font_weight = parse_font_weight(value_ref),
            "font-stretch" => self.font_stretch = parse_css_string_token(value_ref),
            "font-kerning" => self.font_kerning = parse_css_string_token(value_ref),
            "font-optical-sizing" => {
                self.font_optical_sizing = parse_css_string_token(value_ref);
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
            "letter-spacing" => self.letter_spacing = parse_length(value_ref),
            "word-spacing" => self.word_spacing = parse_length(value_ref),
            "tab-size" | "-moz-tab-size" => self.tab_size = parse_css_string_token(value_ref),
            "text-align" => self.text_align = parse_text_align(value_ref),
            "direction" => self.direction = parse_text_direction(value_ref),
            "unicode-bidi" => self.unicode_bidi = parse_unicode_bidi(value_ref),
            "writing-mode" | "-webkit-writing-mode" => {
                self.writing_mode = parse_writing_mode(value_ref);
            }
            "text-orientation" => self.text_orientation = parse_text_orientation(value_ref),
            "text-transform" => self.text_transform = parse_text_transform(value_ref),
            "text-indent" => self.text_indent = parse_length(value_ref),
            "text-wrap" | "text-wrap-mode" => self.text_wrap = parse_text_wrap(value_ref),
            "line-clamp" | "-webkit-line-clamp" => {
                self.line_clamp = parse_css_string_token(value_ref);
            }
            "box-orient" | "-webkit-box-orient" => {
                self.box_orient = parse_css_string_token(value_ref);
            }
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
            "color-interpolation" => self.color_interpolation = parse_css_string_token(value_ref),
            "color-interpolation-filters" => {
                self.color_interpolation_filters = parse_css_string_token(value_ref);
            }
            "text-decoration" => self.apply_text_decoration_shorthand(value_ref),
            "text-decoration-line" => self.text_decoration_line = parse_css_string_token(value_ref),
            "text-decoration-color" => self.text_decoration_color = parse_color(value_ref),
            "text-decoration-style" => {
                self.text_decoration_style = parse_text_decoration_style(value_ref);
            }
            "text-decoration-thickness" => self.text_decoration_thickness = parse_length(value_ref),
            "text-underline-offset" => self.text_underline_offset = parse_length(value_ref),
            "text-shadow" => self.text_shadow = parse_css_string_token(value_ref),
            "text-overflow" => self.text_overflow = parse_text_overflow(value_ref),
            "line-break" => self.line_break = parse_css_string_token(value_ref),
            "white-space" => self.white_space = parse_white_space(value_ref),
            "word-break" => self.word_break = parse_word_break(value_ref),
            "overflow-wrap" => self.overflow_wrap = parse_overflow_wrap(value_ref),
            "hyphens" => self.hyphens = parse_hyphens(value_ref),
            "overflow" => {
                let overflow = parse_overflow(value_ref);
                self.overflow_x = overflow;
                self.overflow_y = overflow;
            }
            "overflow-x" => self.overflow_x = parse_overflow(value_ref),
            "overflow-y" => self.overflow_y = parse_overflow(value_ref),
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
            "aspect-ratio" => self.aspect_ratio = parse_css_string_token(value_ref),
            "transform" => self.apply_transform_property(value_ref),
            "translate" => self.translate = self.resolve_tailwind_translate(value_ref),
            "rotate" => self.rotate = self.resolve_tailwind_rotate(value_ref),
            "scale" => self.scale = self.resolve_tailwind_scale(value_ref),
            "transform-origin" => self.transform_origin = parse_css_string_token(value_ref),
            "transform-style" => self.transform_style = parse_css_string_token(value_ref),
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
            "will-change" => self.will_change = parse_css_string_token(value_ref),
            "appearance" => self.appearance = parse_css_string_token(value_ref),
            "resize" => self.resize = parse_resize(value_ref),
            "scroll-behavior" => self.scroll_behavior = parse_scroll_behavior(value_ref),
            "scroll-snap-type" => self.scroll_snap_type = parse_css_string_token(value_ref),
            "scroll-snap-align" => self.scroll_snap_align = parse_css_string_token(value_ref),
            "scroll-snap-stop" => self.scroll_snap_stop = parse_css_string_token(value_ref),
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
            "touch-action" => self.touch_action = parse_css_string_token(value_ref),
            "cursor" => self.cursor = parse_css_string_token(value_ref),
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

    fn apply_tailwind_utility(&mut self, class: &str) {
        let Some((variants, class)) = split_tailwind_class(class.trim()) else {
            return;
        };
        let class = class.strip_prefix('!').unwrap_or(class);
        if class.is_empty() {
            return;
        }
        let declarations = tailwind_utility_declarations(class);
        if !variants.is_empty() {
            let variant_key = variants.join(":");
            for (property, value) in declarations {
                self.record_variant_declaration(&variant_key, property, value);
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
            "flex" | "inline-flex" => self.display = Some(DisplayMode::Flex),
            "block" | "inline-block" => self.display = Some(DisplayMode::Block),
            "inline" => self.display = Some(DisplayMode::Inline),
            "grid" => self.display = Some(DisplayMode::Grid),
            "inline-grid" => self.display = Some(DisplayMode::InlineGrid),
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
    Flex,
    Block,
    Grid,
    InlineGrid,
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
    match value.trim() {
        "inline" => Some(DisplayMode::Inline),
        "flex" | "inline-flex" => Some(DisplayMode::Flex),
        "block" | "inline-block" => Some(DisplayMode::Block),
        "grid" => Some(DisplayMode::Grid),
        "inline-grid" => Some(DisplayMode::InlineGrid),
        "-webkit-box" => Some(DisplayMode::WebkitBox),
        "none" => Some(DisplayMode::None),
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

fn parse_time(value: &str) -> Option<StyleTime> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if value == "0" {
        return Some(StyleTime::Milliseconds(0.0));
    }
    if let Some(milliseconds) = value.strip_suffix("ms") {
        return milliseconds
            .trim()
            .parse::<f64>()
            .ok()
            .map(StyleTime::Milliseconds);
    }
    if let Some(seconds) = value.strip_suffix('s') {
        return seconds
            .trim()
            .parse::<f64>()
            .ok()
            .map(|value| StyleTime::Milliseconds(value * 1000.0));
    }
    if is_css_time_expression(value) {
        return Some(StyleTime::Css(value.to_string()));
    }
    None
}

fn is_css_time_expression(value: &str) -> bool {
    if matches!(
        value,
        "inherit" | "initial" | "unset" | "revert" | "revert-layer"
    ) {
        return true;
    }
    matches!(
        value.split_once('(').map(|(name, _)| name.trim()),
        Some("calc" | "min" | "max" | "clamp" | "var")
    ) && value.ends_with(')')
}

fn parse_length(value: &str) -> Option<StyleLength> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if value == "auto" {
        return Some(StyleLength::Auto);
    }
    if let Some(percent) = value.strip_suffix('%') {
        return percent.trim().parse::<f64>().ok().map(StyleLength::Percent);
    }
    if let Some(points) = value.strip_suffix("px") {
        return points.trim().parse::<f64>().ok().map(StyleLength::Points);
    }
    if let Some(rem) = value.strip_suffix("rem") {
        return rem
            .trim()
            .parse::<f64>()
            .ok()
            .map(|value| StyleLength::Points(value * 16.0));
    }
    if let Some(em) = value.strip_suffix("em") {
        return em
            .trim()
            .parse::<f64>()
            .ok()
            .map(|value| StyleLength::Points(value * 16.0));
    }
    if let Some(points) = value.strip_suffix("pt") {
        return points.trim().parse::<f64>().ok().map(StyleLength::Points);
    }
    if let Ok(points) = value.parse::<f64>() {
        return Some(StyleLength::Points(points));
    }
    if is_css_length_expression(value) {
        return Some(StyleLength::Css(value.to_string()));
    }
    None
}

fn is_css_length_expression(value: &str) -> bool {
    if matches!(
        value,
        "inherit"
            | "initial"
            | "unset"
            | "revert"
            | "revert-layer"
            | "normal"
            | "none"
            | "from-font"
            | "min-content"
            | "max-content"
            | "fit-content"
            | "stretch"
            | "contain"
    ) {
        return true;
    }
    if matches!(
        value.split_once('(').map(|(name, _)| name.trim()),
        Some("calc" | "min" | "max" | "clamp" | "var" | "env" | "fit-content")
    ) && value.ends_with(')')
    {
        return true;
    }
    let Some((number, unit)) = split_number_and_unit(value) else {
        return false;
    };
    number.parse::<f64>().is_ok() && is_css_length_unit(unit)
}

fn split_number_and_unit(value: &str) -> Option<(&str, &str)> {
    let mut split = value.len();
    for (index, ch) in value.char_indices().rev() {
        if ch.is_ascii_alphabetic() || ch == '%' {
            split = index;
        } else {
            break;
        }
    }
    if split == value.len() || split == 0 {
        return None;
    }
    Some((&value[..split], &value[split..]))
}

fn is_css_length_unit(unit: &str) -> bool {
    matches!(
        unit,
        "cap"
            | "ch"
            | "em"
            | "ex"
            | "ic"
            | "lh"
            | "rlh"
            | "rem"
            | "vw"
            | "svw"
            | "lvw"
            | "dvw"
            | "vh"
            | "svh"
            | "lvh"
            | "dvh"
            | "vi"
            | "svi"
            | "lvi"
            | "dvi"
            | "vb"
            | "svb"
            | "lvb"
            | "dvb"
            | "vmin"
            | "svmin"
            | "lvmin"
            | "dvmin"
            | "vmax"
            | "svmax"
            | "lvmax"
            | "dvmax"
            | "cm"
            | "mm"
            | "q"
            | "Q"
            | "in"
            | "pc"
            | "pt"
            | "px"
    )
}

fn parse_color(value: &str) -> Option<StyleColor> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex_color(hex);
    }
    if let Some(color) = parse_rgb_function(value) {
        return Some(color);
    }
    if let Some(color) = parse_hsl_function(value) {
        return Some(color);
    }
    if value.is_empty() {
        None
    } else {
        Some(StyleColor::Keyword(value.to_string()))
    }
}

fn parse_hex_color(hex: &str) -> Option<StyleColor> {
    match hex.len() {
        3 => Some(StyleColor::Rgba {
            red: expand_hex_digit(&hex[0..1])?,
            green: expand_hex_digit(&hex[1..2])?,
            blue: expand_hex_digit(&hex[2..3])?,
            alpha: 255,
        }),
        4 => Some(StyleColor::Rgba {
            red: expand_hex_digit(&hex[0..1])?,
            green: expand_hex_digit(&hex[1..2])?,
            blue: expand_hex_digit(&hex[2..3])?,
            alpha: expand_hex_digit(&hex[3..4])?,
        }),
        6 => Some(StyleColor::Rgba {
            red: u8::from_str_radix(&hex[0..2], 16).ok()?,
            green: u8::from_str_radix(&hex[2..4], 16).ok()?,
            blue: u8::from_str_radix(&hex[4..6], 16).ok()?,
            alpha: 255,
        }),
        8 => Some(StyleColor::Rgba {
            red: u8::from_str_radix(&hex[0..2], 16).ok()?,
            green: u8::from_str_radix(&hex[2..4], 16).ok()?,
            blue: u8::from_str_radix(&hex[4..6], 16).ok()?,
            alpha: u8::from_str_radix(&hex[6..8], 16).ok()?,
        }),
        _ => None,
    }
}

fn expand_hex_digit(hex: &str) -> Option<u8> {
    let value = u8::from_str_radix(hex, 16).ok()?;
    Some((value << 4) | value)
}

fn parse_rgb_function(value: &str) -> Option<StyleColor> {
    let content = value
        .strip_prefix("rgb(")
        .or_else(|| value.strip_prefix("rgba("))?
        .strip_suffix(')')?;
    let (channels, alpha) = parse_color_function_parts(content);
    if channels.len() < 3 {
        return None;
    }
    let red = parse_rgb_channel(&channels[0])?;
    let green = parse_rgb_channel(&channels[1])?;
    let blue = parse_rgb_channel(&channels[2])?;
    let alpha = alpha
        .as_deref()
        .and_then(parse_alpha_channel)
        .unwrap_or(255);
    Some(StyleColor::Rgba {
        red,
        green,
        blue,
        alpha,
    })
}

fn parse_rgb_channel(value: &str) -> Option<u8> {
    let value = value.trim();
    if let Some(percent) = value.strip_suffix('%') {
        let percent = percent.trim().parse::<f64>().ok()?;
        return Some(((percent.clamp(0.0, 100.0) / 100.0) * 255.0).round() as u8);
    }
    value.trim().parse::<u8>().ok()
}

fn parse_hsl_function(value: &str) -> Option<StyleColor> {
    let content = value
        .strip_prefix("hsl(")
        .or_else(|| value.strip_prefix("hsla("))?
        .strip_suffix(')')?;
    let (channels, alpha) = parse_color_function_parts(content);
    if channels.len() < 3 {
        return None;
    }
    let hue = parse_hue_degrees(&channels[0])?;
    let saturation = parse_percent_fraction(&channels[1])?;
    let lightness = parse_percent_fraction(&channels[2])?;
    let alpha = alpha
        .as_deref()
        .and_then(parse_alpha_channel)
        .unwrap_or(255);
    let (red, green, blue) = hsl_to_rgb(hue, saturation, lightness);
    Some(StyleColor::Rgba {
        red,
        green,
        blue,
        alpha,
    })
}

fn parse_color_function_parts(content: &str) -> (Vec<String>, Option<String>) {
    let content = content.replace(',', " ");
    let mut channels = Vec::new();
    let mut alpha = None;
    let mut alpha_next = false;
    for part in content.split_whitespace() {
        if part == "/" {
            alpha_next = true;
        } else if let Some((before, after)) = part.split_once('/') {
            if !before.is_empty() {
                channels.push(before.to_string());
            }
            if !after.is_empty() {
                alpha = Some(after.to_string());
            }
            alpha_next = false;
        } else if alpha_next {
            alpha = Some(part.to_string());
            alpha_next = false;
        } else {
            channels.push(part.to_string());
        }
    }
    if alpha.is_none() && channels.len() > 3 {
        alpha = channels.pop();
    }
    (channels, alpha)
}

fn parse_hue_degrees(value: &str) -> Option<f64> {
    let value = value.trim();
    let degrees = if let Some(degrees) = value.strip_suffix("deg") {
        degrees.trim().parse::<f64>().ok()?
    } else if let Some(turns) = value.strip_suffix("turn") {
        turns.trim().parse::<f64>().ok()? * 360.0
    } else if let Some(radians) = value.strip_suffix("rad") {
        radians.trim().parse::<f64>().ok()?.to_degrees()
    } else if let Some(gradians) = value.strip_suffix("grad") {
        gradians.trim().parse::<f64>().ok()? * 0.9
    } else {
        value.parse::<f64>().ok()?
    };
    Some(degrees.rem_euclid(360.0))
}

fn parse_percent_fraction(value: &str) -> Option<f64> {
    let value = value.trim().strip_suffix('%')?.trim();
    Some((value.parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
}

fn hsl_to_rgb(hue: f64, saturation: f64, lightness: f64) -> (u8, u8, u8) {
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let hue_prime = hue / 60.0;
    let x = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());
    let (red1, green1, blue1) = if (0.0..1.0).contains(&hue_prime) {
        (chroma, x, 0.0)
    } else if (1.0..2.0).contains(&hue_prime) {
        (x, chroma, 0.0)
    } else if (2.0..3.0).contains(&hue_prime) {
        (0.0, chroma, x)
    } else if (3.0..4.0).contains(&hue_prime) {
        (0.0, x, chroma)
    } else if (4.0..5.0).contains(&hue_prime) {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };
    let m = lightness - chroma / 2.0;
    (
        ((red1 + m) * 255.0).round() as u8,
        ((green1 + m) * 255.0).round() as u8,
        ((blue1 + m) * 255.0).round() as u8,
    )
}

fn parse_alpha_channel(value: &str) -> Option<u8> {
    let value = value.trim().trim_start_matches('/');
    if let Some(percent) = value.strip_suffix('%') {
        let percent = percent.trim().parse::<f64>().ok()?;
        return Some(((percent.clamp(0.0, 100.0) / 100.0) * 255.0).round() as u8);
    }
    let alpha = value.parse::<f64>().ok()?;
    Some((alpha.clamp(0.0, 1.0) * 255.0).round() as u8)
}

fn split_tailwind_class(class: &str) -> Option<(Vec<String>, &str)> {
    if class.is_empty() {
        return None;
    }
    let mut bracket_depth = 0usize;
    let mut start = 0usize;
    let mut variants = Vec::new();
    for (index, ch) in class.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ':' if bracket_depth == 0 => {
                variants.push(class[start..index].to_string());
                start = index + 1;
            }
            _ => {}
        }
    }
    Some((variants, &class[start..]))
}

fn tailwind_utility_declarations(class: &str) -> BTreeMap<String, String> {
    let mut declarations = BTreeMap::new();
    if let Some(arbitrary) = class
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        if let Some((property, value)) = arbitrary.split_once(':') {
            declarations.insert(
                normalize_css_property_name(property),
                tailwind_arbitrary_value(value.trim()),
            );
        }
        return declarations;
    }
    if let Some(radius) = tailwind_radius_declarations(class) {
        declarations.extend(radius);
        return declarations;
    }
    if class == "truncate" {
        declarations.insert("overflow".to_string(), "hidden".to_string());
        declarations.insert("text-overflow".to_string(), "ellipsis".to_string());
        declarations.insert("white-space".to_string(), "nowrap".to_string());
        return declarations;
    }
    if class == "break-normal" {
        declarations.insert("overflow-wrap".to_string(), "normal".to_string());
        declarations.insert("word-break".to_string(), "normal".to_string());
        return declarations;
    }
    if let Some(line_clamp) = tailwind_line_clamp_declarations(class) {
        declarations.extend(line_clamp);
        return declarations;
    }
    if let Some(font_features) = tailwind_font_feature_declarations(class) {
        declarations.extend(font_features);
        return declarations;
    }
    if let Some(container) = tailwind_container_declarations(class) {
        declarations.extend(container);
        return declarations;
    }
    if let Some(motion) = tailwind_motion_declarations(class) {
        declarations.extend(motion);
        return declarations;
    }
    if let Some(interaction) = tailwind_interaction_declarations(class) {
        declarations.extend(interaction);
        return declarations;
    }
    if let Some(svg) = tailwind_svg_presentation_declarations(class) {
        declarations.extend(svg);
        return declarations;
    }
    if let Some(formatting) = tailwind_formatting_declarations(class) {
        declarations.extend(formatting);
        return declarations;
    }
    if let Some(transform) = tailwind_transform_declarations(class) {
        declarations.extend(transform);
        return declarations;
    }
    if let Some(filter) = tailwind_filter_declarations(class) {
        declarations.extend(filter);
        return declarations;
    }
    if let Some(backdrop_filter) = tailwind_backdrop_filter_declarations(class) {
        declarations.extend(backdrop_filter);
        return declarations;
    }
    if let Some((property, value)) = tailwind_blend_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some((property, value)) = tailwind_mask_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    let declaration = match class {
        "inline" => Some(("display", "inline".to_string())),
        "flex" | "inline-flex" => Some(("display", "flex".to_string())),
        "block" | "inline-block" => Some(("display", "block".to_string())),
        "grid" => Some(("display", "grid".to_string())),
        "inline-grid" => Some(("display", "inline-grid".to_string())),
        "hidden" => Some(("display", "none".to_string())),
        "static" => Some(("position", "static".to_string())),
        "fixed" => Some(("position", "fixed".to_string())),
        "absolute" => Some(("position", "absolute".to_string())),
        "relative" => Some(("position", "relative".to_string())),
        "sticky" => Some(("position", "sticky".to_string())),
        "flex-row" => Some(("flex-direction", "row".to_string())),
        "flex-row-reverse" => Some(("flex-direction", "row-reverse".to_string())),
        "flex-col" => Some(("flex-direction", "column".to_string())),
        "flex-col-reverse" => Some(("flex-direction", "column-reverse".to_string())),
        "flex-wrap" => Some(("flex-wrap", "wrap".to_string())),
        "flex-nowrap" => Some(("flex-wrap", "nowrap".to_string())),
        "flex-wrap-reverse" => Some(("flex-wrap", "wrap-reverse".to_string())),
        "items-start" => Some(("align-items", "flex-start".to_string())),
        "items-center" => Some(("align-items", "center".to_string())),
        "items-end" => Some(("align-items", "flex-end".to_string())),
        "items-stretch" => Some(("align-items", "stretch".to_string())),
        "items-baseline" => Some(("align-items", "baseline".to_string())),
        "items-normal" => Some(("align-items", "normal".to_string())),
        "content-normal" => Some(("align-content", "normal".to_string())),
        "content-center" => Some(("align-content", "center".to_string())),
        "content-start" => Some(("align-content", "flex-start".to_string())),
        "content-end" => Some(("align-content", "flex-end".to_string())),
        "content-between" => Some(("align-content", "space-between".to_string())),
        "content-around" => Some(("align-content", "space-around".to_string())),
        "content-evenly" => Some(("align-content", "space-evenly".to_string())),
        "content-baseline" => Some(("align-content", "baseline".to_string())),
        "content-stretch" => Some(("align-content", "stretch".to_string())),
        "self-auto" => Some(("align-self", "auto".to_string())),
        "self-start" => Some(("align-self", "flex-start".to_string())),
        "self-center" => Some(("align-self", "center".to_string())),
        "self-end" => Some(("align-self", "flex-end".to_string())),
        "self-stretch" => Some(("align-self", "stretch".to_string())),
        "self-baseline" => Some(("align-self", "baseline".to_string())),
        "justify-normal" => Some(("justify-content", "normal".to_string())),
        "justify-start" => Some(("justify-content", "flex-start".to_string())),
        "justify-center" => Some(("justify-content", "center".to_string())),
        "justify-end" => Some(("justify-content", "flex-end".to_string())),
        "justify-between" => Some(("justify-content", "space-between".to_string())),
        "justify-around" => Some(("justify-content", "space-around".to_string())),
        "justify-evenly" => Some(("justify-content", "space-evenly".to_string())),
        "justify-stretch" => Some(("justify-content", "stretch".to_string())),
        "justify-items-normal" => Some(("justify-items", "normal".to_string())),
        "justify-items-start" => Some(("justify-items", "flex-start".to_string())),
        "justify-items-center" => Some(("justify-items", "center".to_string())),
        "justify-items-end" => Some(("justify-items", "flex-end".to_string())),
        "justify-items-stretch" => Some(("justify-items", "stretch".to_string())),
        "justify-self-auto" => Some(("justify-self", "auto".to_string())),
        "justify-self-start" => Some(("justify-self", "flex-start".to_string())),
        "justify-self-center" => Some(("justify-self", "center".to_string())),
        "justify-self-end" => Some(("justify-self", "flex-end".to_string())),
        "justify-self-stretch" => Some(("justify-self", "stretch".to_string())),
        "place-content-center" => Some(("place-content", "center".to_string())),
        "place-content-start" => Some(("place-content", "start".to_string())),
        "place-content-end" => Some(("place-content", "end".to_string())),
        "place-content-between" => Some(("place-content", "space-between".to_string())),
        "place-content-around" => Some(("place-content", "space-around".to_string())),
        "place-content-evenly" => Some(("place-content", "space-evenly".to_string())),
        "place-content-baseline" => Some(("place-content", "baseline".to_string())),
        "place-content-stretch" => Some(("place-content", "stretch".to_string())),
        "place-items-start" => Some(("place-items", "start".to_string())),
        "place-items-center" => Some(("place-items", "center".to_string())),
        "place-items-end" => Some(("place-items", "end".to_string())),
        "place-items-baseline" => Some(("place-items", "baseline".to_string())),
        "place-items-stretch" => Some(("place-items", "stretch".to_string())),
        "place-self-auto" => Some(("place-self", "auto".to_string())),
        "place-self-start" => Some(("place-self", "start".to_string())),
        "place-self-center" => Some(("place-self", "center".to_string())),
        "place-self-end" => Some(("place-self", "end".to_string())),
        "place-self-stretch" => Some(("place-self", "stretch".to_string())),
        "flex-1" => Some(("flex", "1".to_string())),
        "flex-auto" => Some(("flex", "auto".to_string())),
        "flex-initial" => Some(("flex", "0 auto".to_string())),
        "flex-none" => Some(("flex", "none".to_string())),
        "basis-auto" => Some(("flex-basis", "auto".to_string())),
        "basis-full" => Some(("flex-basis", "100%".to_string())),
        "grow" => Some(("flex-grow", "1".to_string())),
        "grow-0" => Some(("flex-grow", "0".to_string())),
        "shrink" => Some(("flex-shrink", "1".to_string())),
        "shrink-0" => Some(("flex-shrink", "0".to_string())),
        "order-first" => Some(("order", "-9999".to_string())),
        "order-last" => Some(("order", "9999".to_string())),
        "order-none" => Some(("order", "0".to_string())),
        "overflow-visible" => Some(("overflow", "visible".to_string())),
        "overflow-hidden" => Some(("overflow", "hidden".to_string())),
        "overflow-scroll" => Some(("overflow", "scroll".to_string())),
        "overflow-auto" => Some(("overflow", "auto".to_string())),
        "overflow-clip" => Some(("overflow", "clip".to_string())),
        "overflow-x-visible" => Some(("overflow-x", "visible".to_string())),
        "overflow-x-hidden" => Some(("overflow-x", "hidden".to_string())),
        "overflow-x-scroll" => Some(("overflow-x", "scroll".to_string())),
        "overflow-x-auto" => Some(("overflow-x", "auto".to_string())),
        "overflow-x-clip" => Some(("overflow-x", "clip".to_string())),
        "overflow-y-visible" => Some(("overflow-y", "visible".to_string())),
        "overflow-y-hidden" => Some(("overflow-y", "hidden".to_string())),
        "overflow-y-scroll" => Some(("overflow-y", "scroll".to_string())),
        "overflow-y-auto" => Some(("overflow-y", "auto".to_string())),
        "overflow-y-clip" => Some(("overflow-y", "clip".to_string())),
        "visible" => Some(("visibility", "visible".to_string())),
        "invisible" => Some(("visibility", "hidden".to_string())),
        "collapse" => Some(("visibility", "collapse".to_string())),
        "font-thin" => Some(("font-weight", "100".to_string())),
        "font-extralight" => Some(("font-weight", "200".to_string())),
        "font-light" => Some(("font-weight", "300".to_string())),
        "font-normal" => Some(("font-weight", "400".to_string())),
        "font-medium" => Some(("font-weight", "500".to_string())),
        "font-semibold" => Some(("font-weight", "600".to_string())),
        "font-bold" => Some(("font-weight", "700".to_string())),
        "font-extrabold" => Some(("font-weight", "800".to_string())),
        "font-black" => Some(("font-weight", "900".to_string())),
        "font-sans" => Some((
            "font-family",
            "ui-sans-serif, system-ui, sans-serif".to_string(),
        )),
        "font-serif" => Some(("font-family", "ui-serif, Georgia, serif".to_string())),
        "font-mono" => Some(("font-family", "ui-monospace, monospace".to_string())),
        "italic" => Some(("font-style", "italic".to_string())),
        "not-italic" => Some(("font-style", "normal".to_string())),
        "text-left" => Some(("text-align", "left".to_string())),
        "text-center" => Some(("text-align", "center".to_string())),
        "text-right" => Some(("text-align", "right".to_string())),
        "text-justify" => Some(("text-align", "justify".to_string())),
        "text-start" => Some(("text-align", "start".to_string())),
        "text-end" => Some(("text-align", "end".to_string())),
        "text-wrap" => Some(("text-wrap", "wrap".to_string())),
        "text-nowrap" => Some(("text-wrap", "nowrap".to_string())),
        "text-balance" => Some(("text-wrap", "balance".to_string())),
        "text-pretty" => Some(("text-wrap", "pretty".to_string())),
        "uppercase" => Some(("text-transform", "uppercase".to_string())),
        "lowercase" => Some(("text-transform", "lowercase".to_string())),
        "capitalize" => Some(("text-transform", "capitalize".to_string())),
        "normal-case" => Some(("text-transform", "none".to_string())),
        "underline" => Some(("text-decoration-line", "underline".to_string())),
        "overline" => Some(("text-decoration-line", "overline".to_string())),
        "line-through" => Some(("text-decoration-line", "line-through".to_string())),
        "no-underline" => Some(("text-decoration-line", "none".to_string())),
        "decoration-solid" => Some(("text-decoration-style", "solid".to_string())),
        "decoration-double" => Some(("text-decoration-style", "double".to_string())),
        "decoration-dotted" => Some(("text-decoration-style", "dotted".to_string())),
        "decoration-dashed" => Some(("text-decoration-style", "dashed".to_string())),
        "decoration-wavy" => Some(("text-decoration-style", "wavy".to_string())),
        "decoration-auto" => Some(("text-decoration-thickness", "auto".to_string())),
        "decoration-from-font" => Some(("text-decoration-thickness", "from-font".to_string())),
        "underline-offset-auto" => Some(("text-underline-offset", "auto".to_string())),
        "text-ellipsis" => Some(("text-overflow", "ellipsis".to_string())),
        "text-clip" => Some(("text-overflow", "clip".to_string())),
        "bg-fixed" => Some(("background-attachment", "fixed".to_string())),
        "bg-local" => Some(("background-attachment", "local".to_string())),
        "bg-scroll" => Some(("background-attachment", "scroll".to_string())),
        "bg-clip-border" => Some(("background-clip", "border-box".to_string())),
        "bg-clip-padding" => Some(("background-clip", "padding-box".to_string())),
        "bg-clip-content" => Some(("background-clip", "content-box".to_string())),
        "bg-clip-text" => Some(("background-clip", "text".to_string())),
        "bg-origin-border" => Some(("background-origin", "border-box".to_string())),
        "bg-origin-padding" => Some(("background-origin", "padding-box".to_string())),
        "bg-origin-content" => Some(("background-origin", "content-box".to_string())),
        "bg-cover" => Some(("background-size", "cover".to_string())),
        "bg-contain" => Some(("background-size", "contain".to_string())),
        "bg-auto" => Some(("background-size", "auto".to_string())),
        "bg-center" => Some(("background-position", "center".to_string())),
        "bg-top" => Some(("background-position", "top".to_string())),
        "bg-bottom" => Some(("background-position", "bottom".to_string())),
        "bg-left" => Some(("background-position", "left".to_string())),
        "bg-left-top" => Some(("background-position", "left top".to_string())),
        "bg-left-bottom" => Some(("background-position", "left bottom".to_string())),
        "bg-right" => Some(("background-position", "right".to_string())),
        "bg-right-top" => Some(("background-position", "right top".to_string())),
        "bg-right-bottom" => Some(("background-position", "right bottom".to_string())),
        "bg-no-repeat" => Some(("background-repeat", "no-repeat".to_string())),
        "bg-repeat" => Some(("background-repeat", "repeat".to_string())),
        "bg-repeat-x" => Some(("background-repeat", "repeat-x".to_string())),
        "bg-repeat-y" => Some(("background-repeat", "repeat-y".to_string())),
        "bg-repeat-round" => Some(("background-repeat", "round".to_string())),
        "bg-repeat-space" => Some(("background-repeat", "space".to_string())),
        "bg-none" => Some(("background-image", "none".to_string())),
        "object-contain" => Some(("object-fit", "contain".to_string())),
        "object-cover" => Some(("object-fit", "cover".to_string())),
        "object-fill" => Some(("object-fit", "fill".to_string())),
        "object-none" => Some(("object-fit", "none".to_string())),
        "object-scale-down" => Some(("object-fit", "scale-down".to_string())),
        "object-bottom" => Some(("object-position", "bottom".to_string())),
        "object-center" => Some(("object-position", "center".to_string())),
        "object-left" => Some(("object-position", "left".to_string())),
        "object-left-bottom" => Some(("object-position", "left bottom".to_string())),
        "object-left-top" => Some(("object-position", "left top".to_string())),
        "object-right" => Some(("object-position", "right".to_string())),
        "object-right-bottom" => Some(("object-position", "right bottom".to_string())),
        "object-right-top" => Some(("object-position", "right top".to_string())),
        "object-top" => Some(("object-position", "top".to_string())),
        "list-inside" => Some(("list-style-position", "inside".to_string())),
        "list-outside" => Some(("list-style-position", "outside".to_string())),
        "list-none" => Some(("list-style-type", "none".to_string())),
        "list-disc" => Some(("list-style-type", "disc".to_string())),
        "list-decimal" => Some(("list-style-type", "decimal".to_string())),
        "columns-auto" => Some(("columns", "auto".to_string())),
        "whitespace-normal" => Some(("white-space", "normal".to_string())),
        "whitespace-nowrap" => Some(("white-space", "nowrap".to_string())),
        "whitespace-pre" => Some(("white-space", "pre".to_string())),
        "whitespace-pre-line" => Some(("white-space", "pre-line".to_string())),
        "whitespace-pre-wrap" => Some(("white-space", "pre-wrap".to_string())),
        "whitespace-break-spaces" => Some(("white-space", "break-spaces".to_string())),
        "break-words" => Some(("overflow-wrap", "break-word".to_string())),
        "break-all" => Some(("word-break", "break-all".to_string())),
        "break-keep" => Some(("word-break", "keep-all".to_string())),
        "hyphens-none" => Some(("hyphens", "none".to_string())),
        "hyphens-manual" => Some(("hyphens", "manual".to_string())),
        "hyphens-auto" => Some(("hyphens", "auto".to_string())),
        "border" => Some(("border-width", "1px".to_string())),
        "border-solid" => Some(("border-style", "solid".to_string())),
        "border-dashed" => Some(("border-style", "dashed".to_string())),
        "border-dotted" => Some(("border-style", "dotted".to_string())),
        "border-double" => Some(("border-style", "double".to_string())),
        "border-hidden" => Some(("border-style", "hidden".to_string())),
        "border-none" => Some(("border-style", "none".to_string())),
        "outline" => Some(("outline-width", "1px".to_string())),
        "outline-none" => Some(("outline", "2px solid transparent".to_string())),
        "outline-hidden" => Some(("outline-style", "none".to_string())),
        "outline-solid" => Some(("outline-style", "solid".to_string())),
        "outline-dashed" => Some(("outline-style", "dashed".to_string())),
        "outline-dotted" => Some(("outline-style", "dotted".to_string())),
        "outline-double" => Some(("outline-style", "double".to_string())),
        "shadow" => Some((
            "box-shadow",
            "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-xs" => Some(("box-shadow", "0 1px rgb(0 0 0 / 0.05)".to_string())),
        "shadow-sm" => Some(("box-shadow", "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string())),
        "shadow-md" => Some((
            "box-shadow",
            "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-lg" => Some((
            "box-shadow",
            "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-xl" => Some((
            "box-shadow",
            "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-2xl" => Some((
            "box-shadow",
            "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
        )),
        "shadow-inner" => Some((
            "box-shadow",
            "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string(),
        )),
        "shadow-none" => Some(("box-shadow", "none".to_string())),
        "transform" => Some(("transform", "translateZ(0)".to_string())),
        "transform-none" => Some(("transform", "none".to_string())),
        "filter" => Some(("filter", "var(--tw-filter)".to_string())),
        "filter-none" => Some(("filter", "none".to_string())),
        "backdrop-filter" => Some(("backdrop-filter", "var(--tw-backdrop-filter)".to_string())),
        "backdrop-filter-none" => Some(("backdrop-filter", "none".to_string())),
        "pointer-events-auto" => Some(("pointer-events", "auto".to_string())),
        "pointer-events-none" => Some(("pointer-events", "none".to_string())),
        "select-auto" => Some(("user-select", "auto".to_string())),
        "select-text" => Some(("user-select", "text".to_string())),
        "select-none" => Some(("user-select", "none".to_string())),
        "select-all" => Some(("user-select", "all".to_string())),
        "resize-none" => Some(("resize", "none".to_string())),
        "resize" => Some(("resize", "both".to_string())),
        "resize-x" => Some(("resize", "horizontal".to_string())),
        "resize-y" => Some(("resize", "vertical".to_string())),
        "aspect-auto" => Some(("aspect-ratio", "auto".to_string())),
        "aspect-square" => Some(("aspect-ratio", "1 / 1".to_string())),
        "aspect-video" => Some(("aspect-ratio", "16 / 9".to_string())),
        "rounded" => Some(("border-radius", "4px".to_string())),
        "rounded-none" => Some(("border-radius", "0px".to_string())),
        "rounded-xs" => Some(("border-radius", "2px".to_string())),
        "rounded-sm" => Some(("border-radius", "4px".to_string())),
        "rounded-md" => Some(("border-radius", "6px".to_string())),
        "rounded-lg" => Some(("border-radius", "8px".to_string())),
        "rounded-xl" => Some(("border-radius", "12px".to_string())),
        "rounded-2xl" => Some(("border-radius", "16px".to_string())),
        "rounded-3xl" => Some(("border-radius", "24px".to_string())),
        "rounded-4xl" => Some(("border-radius", "32px".to_string())),
        "rounded-full" => Some(("border-radius", "calc(infinity * 1px)".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return declarations;
    }
    if let Some((property, value)) = tailwind_media_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_edge_utility(class, "scroll-m") {
        insert_edge_declarations(&mut declarations, "scroll-margin", edges, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_logical_edge_utility(class, "scroll-m", true) {
        insert_logical_edge_declaration(&mut declarations, "scroll-margin", edges, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_edge_utility(class, "scroll-p") {
        insert_edge_declarations(&mut declarations, "scroll-padding", edges, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_logical_edge_utility(class, "scroll-p", false) {
        insert_logical_edge_declaration(&mut declarations, "scroll-padding", edges, value);
        return declarations;
    }
    if let Some(text_size) = tailwind_text_size_declarations(class) {
        declarations.extend(text_size);
        return declarations;
    }
    if let Some((property, value)) = tailwind_visual_effect_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some((property, value)) = tailwind_grid_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some((properties, value)) = tailwind_inset_utility(class) {
        insert_position_declarations(&mut declarations, properties, value);
        return declarations;
    }
    if let Some((properties, value)) = tailwind_logical_inset_utility(class) {
        insert_logical_position_declaration(&mut declarations, properties, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_border_width_utility(class) {
        insert_border_width_declarations(&mut declarations, edges, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_logical_border_width_utility(class) {
        insert_logical_border_width_declaration(&mut declarations, edges, value);
        return declarations;
    }
    if let Some(border_color) = tailwind_border_color_declarations(class) {
        declarations.extend(border_color);
        return declarations;
    }
    if let Some((property, value)) = tailwind_prefixed_declaration(class) {
        declarations.insert(property, value);
    } else if let Some((edges, value)) = tailwind_edge_utility(class, "p") {
        insert_edge_declarations(&mut declarations, "padding", edges, value);
    } else if let Some((edges, value)) = tailwind_logical_edge_utility(class, "p", false) {
        insert_logical_edge_declaration(&mut declarations, "padding", edges, value);
    } else if let Some((edges, value)) = tailwind_edge_utility(class, "m") {
        insert_edge_declarations(&mut declarations, "margin", edges, value);
    } else if let Some((edges, value)) = tailwind_logical_edge_utility(class, "m", true) {
        insert_logical_edge_declaration(&mut declarations, "margin", edges, value);
    }
    declarations
}

fn tailwind_prefixed_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class.strip_prefix("w-").and_then(tailwind_length) {
        Some(("width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("h-").and_then(tailwind_length) {
        Some(("height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("min-w-").and_then(tailwind_length) {
        Some(("min-width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("min-h-").and_then(tailwind_length) {
        Some(("min-height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("max-w-").and_then(tailwind_length) {
        Some(("max-width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("max-h-").and_then(tailwind_length) {
        Some(("max-height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-").and_then(tailwind_length) {
        Some(("gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-x-").and_then(tailwind_length) {
        Some(("column-gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-y-").and_then(tailwind_length) {
        Some(("row-gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("opacity-").and_then(tailwind_opacity) {
        Some(("opacity".to_string(), trim_float(value)))
    } else if let Some(value) = tailwind_z_index(class) {
        Some(("z-index".to_string(), value))
    } else if let Some(value) = class.strip_prefix("flex-").and_then(tailwind_flex_value) {
        Some(("flex".to_string(), value))
    } else if let Some(value) = class.strip_prefix("basis-").and_then(tailwind_basis_value) {
        Some(("flex-basis".to_string(), value))
    } else if let Some(value) = class.strip_prefix("grow-").and_then(tailwind_number_token) {
        Some(("flex-grow".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("shrink-")
        .and_then(tailwind_number_token)
    {
        Some(("flex-shrink".to_string(), value))
    } else if let Some(value) = tailwind_order_value(class) {
        Some(("order".to_string(), value))
    } else if let Some(value) = class.strip_prefix("bg-").and_then(tailwind_color_css) {
        Some(("background-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("border-").and_then(tailwind_color_css) {
        Some(("border-color".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("accent-")
        .and_then(tailwind_accent_color_css)
    {
        Some(("accent-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("caret-").and_then(tailwind_color_css) {
        Some(("caret-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("font-").and_then(tailwind_font_family) {
        Some(("font-family".to_string(), value))
    } else if let Some(value) = tailwind_letter_spacing(class) {
        Some(("letter-spacing".to_string(), value))
    } else if let Some((property, value)) = tailwind_decoration_declaration(class) {
        Some((property, value))
    } else if let Some(value) = class
        .strip_prefix("underline-offset-")
        .and_then(tailwind_underline_offset)
    {
        Some(("text-underline-offset".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("leading-")
        .and_then(tailwind_line_height)
    {
        Some(("line-height".to_string(), value))
    } else if let Some(value) = tailwind_text_indent(class) {
        Some(("text-indent".to_string(), value))
    } else if let Some(value) = class.strip_prefix("text-").and_then(tailwind_color_css) {
        Some(("color".to_string(), value))
    } else {
        None
    }
}

fn tailwind_svg_presentation_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if let Some(value) = class
        .strip_prefix("fill-")
        .and_then(tailwind_svg_paint_value)
    {
        declarations.insert("fill".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("stroke-") {
        let (property, value) = tailwind_stroke_declaration(value)?;
        declarations.insert(property, value);
        return Some(declarations);
    }
    None
}

fn tailwind_svg_paint_value(value: &str) -> Option<String> {
    if value == "none" {
        return Some("none".to_string());
    }
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_color_css(value).or_else(|| tailwind_custom_var(value))
}

fn tailwind_stroke_declaration(value: &str) -> Option<(String, String)> {
    if let Some(value) = tailwind_typed_custom_var(value, "length") {
        return Some(("stroke-width".to_string(), value));
    }
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(("stroke".to_string(), value));
    }
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if is_likely_stroke_width_value(&value) {
            return Some(("stroke-width".to_string(), value));
        }
        return Some(("stroke".to_string(), value));
    }
    if let Ok(width) = value.parse::<f64>() {
        return Some(("stroke-width".to_string(), trim_float(width)));
    }
    tailwind_svg_paint_value(value).map(|value| ("stroke".to_string(), value))
}

fn is_likely_stroke_width_value(value: &str) -> bool {
    !value.trim().starts_with("var(") && parse_length(value).is_some()
}

fn tailwind_radius_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let suffix = class.strip_prefix("rounded")?;
    let (physical, logical, value) = if suffix.is_empty() {
        (Some(CornerSelection::All), None, "sm")
    } else {
        let suffix = suffix.strip_prefix('-')?;
        if let Some(value) = suffix.strip_prefix("ss-") {
            (None, Some(LogicalCornerSelection::StartStart), value)
        } else if let Some(value) = suffix.strip_prefix("se-") {
            (None, Some(LogicalCornerSelection::StartEnd), value)
        } else if let Some(value) = suffix.strip_prefix("ee-") {
            (None, Some(LogicalCornerSelection::EndEnd), value)
        } else if let Some(value) = suffix.strip_prefix("es-") {
            (None, Some(LogicalCornerSelection::EndStart), value)
        } else if let Some(value) = suffix.strip_prefix("s-") {
            (None, Some(LogicalCornerSelection::Start), value)
        } else if let Some(value) = suffix.strip_prefix("e-") {
            (None, Some(LogicalCornerSelection::End), value)
        } else if let Some(value) = suffix.strip_prefix("tl-") {
            (Some(CornerSelection::TopLeft), None, value)
        } else if let Some(value) = suffix.strip_prefix("tr-") {
            (Some(CornerSelection::TopRight), None, value)
        } else if let Some(value) = suffix.strip_prefix("br-") {
            (Some(CornerSelection::BottomRight), None, value)
        } else if let Some(value) = suffix.strip_prefix("bl-") {
            (Some(CornerSelection::BottomLeft), None, value)
        } else if let Some(value) = suffix.strip_prefix("t-") {
            (Some(CornerSelection::Top), None, value)
        } else if let Some(value) = suffix.strip_prefix("r-") {
            (Some(CornerSelection::Right), None, value)
        } else if let Some(value) = suffix.strip_prefix("b-") {
            (Some(CornerSelection::Bottom), None, value)
        } else if let Some(value) = suffix.strip_prefix("l-") {
            (Some(CornerSelection::Left), None, value)
        } else {
            (Some(CornerSelection::All), None, suffix)
        }
    };
    let radius = CornerRadius::circular(tailwind_radius_value(value)?);
    if let Some(selection) = physical {
        insert_corner_radius_declarations(&mut declarations, selection, radius);
    } else if let Some(selection) = logical {
        insert_logical_corner_radius_declarations(&mut declarations, selection, radius);
    }
    Some(declarations)
}

fn tailwind_radius_value(value: &str) -> Option<StyleLength> {
    if let Some(value) = tailwind_typed_custom_var(value, "length") {
        return Some(StyleLength::Css(value));
    }
    if let Some(variable) = tailwind_custom_var(value) {
        return Some(StyleLength::Css(variable));
    }
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
    }
    match value {
        "none" => Some(StyleLength::Points(0.0)),
        "xs" => Some(StyleLength::Points(2.0)),
        "sm" => Some(StyleLength::Points(4.0)),
        "md" => Some(StyleLength::Points(6.0)),
        "lg" => Some(StyleLength::Points(8.0)),
        "xl" => Some(StyleLength::Points(12.0)),
        "2xl" => Some(StyleLength::Points(16.0)),
        "3xl" => Some(StyleLength::Points(24.0)),
        "4xl" => Some(StyleLength::Points(32.0)),
        "full" => Some(StyleLength::Css("calc(infinity * 1px)".to_string())),
        _ if is_tailwind_identifier(value) => {
            Some(StyleLength::Css(format!("var(--radius-{value})")))
        }
        _ => None,
    }
}

fn tailwind_typed_custom_var(value: &str, expected_type: &str) -> Option<String> {
    let value = value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))?;
    let (value_type, variable) = value.split_once(':')?;
    if value_type == expected_type && !variable.trim().is_empty() {
        Some(format!("var({})", variable.trim()))
    } else {
        None
    }
}

fn tailwind_formatting_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let declaration = match class {
        "box-border" => Some(("box-sizing", "border-box".to_string())),
        "box-content" => Some(("box-sizing", "content-box".to_string())),
        "box-decoration-slice" => Some(("box-decoration-break", "slice".to_string())),
        "box-decoration-clone" => Some(("box-decoration-break", "clone".to_string())),
        "isolate" => Some(("isolation", "isolate".to_string())),
        "isolation-auto" => Some(("isolation", "auto".to_string())),
        "float-right" => Some(("float", "right".to_string())),
        "float-left" => Some(("float", "left".to_string())),
        "float-start" => Some(("float", "inline-start".to_string())),
        "float-end" => Some(("float", "inline-end".to_string())),
        "float-none" => Some(("float", "none".to_string())),
        "clear-right" => Some(("clear", "right".to_string())),
        "clear-left" => Some(("clear", "left".to_string())),
        "clear-both" => Some(("clear", "both".to_string())),
        "clear-start" => Some(("clear", "inline-start".to_string())),
        "clear-end" => Some(("clear", "inline-end".to_string())),
        "clear-none" => Some(("clear", "none".to_string())),
        "align-baseline" => Some(("vertical-align", "baseline".to_string())),
        "align-top" => Some(("vertical-align", "top".to_string())),
        "align-middle" => Some(("vertical-align", "middle".to_string())),
        "align-bottom" => Some(("vertical-align", "bottom".to_string())),
        "align-text-top" => Some(("vertical-align", "text-top".to_string())),
        "align-text-bottom" => Some(("vertical-align", "text-bottom".to_string())),
        "align-sub" => Some(("vertical-align", "sub".to_string())),
        "align-super" => Some(("vertical-align", "super".to_string())),
        "table-auto" => Some(("table-layout", "auto".to_string())),
        "table-fixed" => Some(("table-layout", "fixed".to_string())),
        "border-collapse" => Some(("border-collapse", "collapse".to_string())),
        "border-separate" => Some(("border-collapse", "separate".to_string())),
        "caption-top" => Some(("caption-side", "top".to_string())),
        "caption-bottom" => Some(("caption-side", "bottom".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("align-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "vertical-align".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("align-").and_then(tailwind_custom_var) {
        declarations.insert("vertical-align".to_string(), value);
        return Some(declarations);
    }
    if let Some((axis, value)) = tailwind_border_spacing_declaration(class) {
        insert_tailwind_border_spacing_declarations(&mut declarations, axis, value);
        return Some(declarations);
    }
    None
}

fn tailwind_container_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let (base, name) = class.split_once('/').unwrap_or((class, ""));
    let container_type = match base {
        "@container" => "inline-size",
        "@container-size" => "size",
        "@container-normal" => "normal",
        _ => return None,
    };
    let mut declarations = BTreeMap::new();
    declarations.insert("container-type".to_string(), container_type.to_string());
    if !name.is_empty() {
        declarations.insert("container-name".to_string(), tailwind_container_name(name));
    }
    Some(declarations)
}

fn tailwind_container_name(value: &str) -> String {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        tailwind_arbitrary_value(arbitrary)
    } else {
        value.to_string()
    }
}

fn tailwind_border_spacing_declaration(class: &str) -> Option<(SpacingAxis, String)> {
    let (axis, value) = if let Some(value) = class.strip_prefix("border-spacing-x-") {
        (SpacingAxis::X, value)
    } else if let Some(value) = class.strip_prefix("border-spacing-y-") {
        (SpacingAxis::Y, value)
    } else if let Some(value) = class.strip_prefix("border-spacing-") {
        (SpacingAxis::Both, value)
    } else {
        return None;
    };
    Some((axis, tailwind_spacing_value(value)?))
}

#[derive(Debug, Clone, Copy)]
enum SpacingAxis {
    Both,
    X,
    Y,
}

fn insert_tailwind_border_spacing_declarations(
    declarations: &mut BTreeMap<String, String>,
    axis: SpacingAxis,
    value: String,
) {
    match axis {
        SpacingAxis::Both => {
            declarations.insert("--tw-border-spacing-x".to_string(), value.clone());
            declarations.insert("--tw-border-spacing-y".to_string(), value);
        }
        SpacingAxis::X => {
            declarations.insert("--tw-border-spacing-x".to_string(), value);
        }
        SpacingAxis::Y => {
            declarations.insert("--tw-border-spacing-y".to_string(), value);
        }
    }
    declarations.insert(
        "border-spacing".to_string(),
        "var(--tw-border-spacing-x) var(--tw-border-spacing-y)".to_string(),
    );
}

fn tailwind_spacing_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value).or_else(|| tailwind_length(value).map(style_length_css))
}

fn tailwind_transform_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let declaration = match class {
        "transform-none" => Some(("transform", "none".to_string())),
        "transform-gpu" => Some(("transform", tailwind_transform_pipeline(true))),
        "transform-cpu" | "transform" => Some(("transform", tailwind_transform_pipeline(false))),
        "transform-flat" => Some(("transform-style", "flat".to_string())),
        "transform-3d" => Some(("transform-style", "preserve-3d".to_string())),
        "backface-visible" => Some(("backface-visibility", "visible".to_string())),
        "backface-hidden" => Some(("backface-visibility", "hidden".to_string())),
        "perspective-none" => Some(("perspective", "none".to_string())),
        "perspective-dramatic" => Some(("perspective", "100px".to_string())),
        "perspective-near" => Some(("perspective", "300px".to_string())),
        "perspective-normal" => Some(("perspective", "500px".to_string())),
        "perspective-midrange" => Some(("perspective", "800px".to_string())),
        "perspective-distant" => Some(("perspective", "1200px".to_string())),
        "origin-center" => Some(("transform-origin", "center".to_string())),
        "origin-top" => Some(("transform-origin", "top".to_string())),
        "origin-top-right" => Some(("transform-origin", "top right".to_string())),
        "origin-right" => Some(("transform-origin", "right".to_string())),
        "origin-bottom-right" => Some(("transform-origin", "bottom right".to_string())),
        "origin-bottom" => Some(("transform-origin", "bottom".to_string())),
        "origin-bottom-left" => Some(("transform-origin", "bottom left".to_string())),
        "origin-left" => Some(("transform-origin", "left".to_string())),
        "origin-top-left" => Some(("transform-origin", "top left".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return Some(declarations);
    }

    if let Some(value) = class
        .strip_prefix("transform-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("transform".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("transform-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("transform".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("origin-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "transform-origin".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("origin-").and_then(tailwind_custom_var) {
        declarations.insert("transform-origin".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("perspective-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("perspective".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("perspective-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("perspective".to_string(), value);
        return Some(declarations);
    }
    if let Some((axis, value)) = tailwind_translate_declaration(class) {
        insert_tailwind_axis_declarations(
            &mut declarations,
            "translate",
            "--tw-translate",
            axis,
            value,
            "0",
        );
        return Some(declarations);
    }
    if let Some((axis, value)) = tailwind_scale_declaration(class) {
        insert_tailwind_axis_declarations(
            &mut declarations,
            "scale",
            "--tw-scale",
            axis,
            value,
            "100%",
        );
        return Some(declarations);
    }
    if let Some(value) = tailwind_rotate_declaration(class) {
        declarations.insert("--tw-rotate".to_string(), value);
        declarations.insert("rotate".to_string(), "var(--tw-rotate)".to_string());
        return Some(declarations);
    }
    if let Some((property, value)) = tailwind_transform_function_declaration(class) {
        declarations.insert(property, value);
        declarations.insert("transform".to_string(), tailwind_transform_pipeline(false));
        return Some(declarations);
    }
    None
}

fn insert_tailwind_axis_declarations(
    declarations: &mut BTreeMap<String, String>,
    property: &str,
    variable_prefix: &str,
    axis: TransformAxis,
    value: String,
    default_value: &str,
) {
    match axis {
        TransformAxis::All => {
            declarations.insert(format!("{variable_prefix}-x"), value.clone());
            declarations.insert(format!("{variable_prefix}-y"), value);
            declarations.insert(
                property.to_string(),
                format!("var({variable_prefix}-x) var({variable_prefix}-y)"),
            );
        }
        TransformAxis::X => {
            declarations.insert(format!("{variable_prefix}-x"), value);
            declarations.insert(
                property.to_string(),
                format!("var({variable_prefix}-x) var({variable_prefix}-y, {default_value})"),
            );
        }
        TransformAxis::Y => {
            declarations.insert(format!("{variable_prefix}-y"), value);
            declarations.insert(
                property.to_string(),
                format!("var({variable_prefix}-x, {default_value}) var({variable_prefix}-y)"),
            );
        }
        TransformAxis::Z => {
            declarations.insert(format!("{variable_prefix}-z"), value);
            declarations.insert(
                property.to_string(),
                format!(
                    "var({variable_prefix}-x, {default_value}) var({variable_prefix}-y, {default_value}) var({variable_prefix}-z)"
                ),
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TransformAxis {
    All,
    X,
    Y,
    Z,
}

fn tailwind_translate_declaration(class: &str) -> Option<(TransformAxis, String)> {
    let (negative, class) = strip_negative_prefix(class);
    let (axis, value) = if let Some(value) = class.strip_prefix("translate-x-") {
        (TransformAxis::X, value)
    } else if let Some(value) = class.strip_prefix("translate-y-") {
        (TransformAxis::Y, value)
    } else if let Some(value) = class.strip_prefix("translate-z-") {
        (TransformAxis::Z, value)
    } else if let Some(value) = class.strip_prefix("translate-") {
        (TransformAxis::All, value)
    } else {
        return None;
    };
    Some((axis, tailwind_signed_length_value(value, negative)?))
}

fn tailwind_scale_declaration(class: &str) -> Option<(TransformAxis, String)> {
    let (negative, class) = strip_negative_prefix(class);
    let (axis, value) = if let Some(value) = class.strip_prefix("scale-x-") {
        (TransformAxis::X, value)
    } else if let Some(value) = class.strip_prefix("scale-y-") {
        (TransformAxis::Y, value)
    } else if let Some(value) = class.strip_prefix("scale-z-") {
        (TransformAxis::Z, value)
    } else if let Some(value) = class.strip_prefix("scale-") {
        (TransformAxis::All, value)
    } else {
        return None;
    };
    Some((axis, tailwind_signed_scale_value(value, negative)?))
}

fn tailwind_rotate_declaration(class: &str) -> Option<String> {
    let (negative, class) = strip_negative_prefix(class);
    let value = class.strip_prefix("rotate-")?;
    tailwind_signed_angle_value(value, negative)
}

fn tailwind_transform_function_declaration(class: &str) -> Option<(String, String)> {
    let (negative, class) = strip_negative_prefix(class);
    if let Some(value) = class.strip_prefix("rotate-x-") {
        return Some((
            "--tw-rotate-x".to_string(),
            format!("rotateX({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("rotate-y-") {
        return Some((
            "--tw-rotate-y".to_string(),
            format!("rotateY({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("rotate-z-") {
        return Some((
            "--tw-rotate-z".to_string(),
            format!("rotateZ({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("skew-x-") {
        return Some((
            "--tw-skew-x".to_string(),
            format!("skewX({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("skew-y-") {
        return Some((
            "--tw-skew-y".to_string(),
            format!("skewY({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    None
}

fn strip_negative_prefix(value: &str) -> (bool, &str) {
    if let Some(value) = value.strip_prefix('-') {
        (true, value)
    } else {
        (false, value)
    }
}

fn tailwind_signed_length_value(value: &str, negative: bool) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        if value == "full" {
            Some("100%".to_string())
        } else {
            tailwind_length(value).map(style_length_css)
        }
    })?;
    Some(if negative {
        negate_css_value(&value)
    } else {
        value
    })
}

fn tailwind_signed_scale_value(value: &str, negative: bool) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}%", trim_float(value)))
    })?;
    Some(if negative {
        negate_css_value(&value)
    } else {
        value
    })
}

fn tailwind_signed_angle_value(value: &str, negative: bool) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}deg", trim_float(value)))
    })?;
    Some(if negative {
        negate_css_value(&value)
    } else {
        value
    })
}

fn negate_css_value(value: &str) -> String {
    if value.starts_with("var(") || value.starts_with("calc(") {
        format!("calc({value} * -1)")
    } else if let Some(number) = value.strip_prefix('-') {
        number.to_string()
    } else {
        format!("-{value}")
    }
}

fn tailwind_transform_pipeline(gpu: bool) -> String {
    let pipeline = "var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)";
    if gpu {
        format!("translateZ(0) {pipeline}")
    } else {
        pipeline.to_string()
    }
}

fn tailwind_filter_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if class == "filter-none" {
        declarations.insert("filter".to_string(), "none".to_string());
        return Some(declarations);
    }
    if class == "filter" {
        declarations.insert("filter".to_string(), tailwind_filter_pipeline());
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("filter".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("filter-").and_then(tailwind_custom_var) {
        declarations.insert("filter".to_string(), value);
        return Some(declarations);
    }
    if let Some((property, value)) = tailwind_filter_component_declaration(class, "") {
        declarations.insert(property, value);
        declarations.insert("filter".to_string(), tailwind_filter_pipeline());
        return Some(declarations);
    }
    None
}

fn tailwind_backdrop_filter_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if matches!(class, "backdrop-filter-none" | "backdrop-none") {
        declarations.insert("backdrop-filter".to_string(), "none".to_string());
        return Some(declarations);
    }
    if matches!(class, "backdrop-filter" | "backdrop") {
        declarations.insert(
            "backdrop-filter".to_string(),
            tailwind_backdrop_filter_pipeline(),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-[")
        .or_else(|| class.strip_prefix("backdrop-["))
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "backdrop-filter".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-")
        .or_else(|| class.strip_prefix("backdrop-"))
        .and_then(tailwind_custom_var)
    {
        declarations.insert("backdrop-filter".to_string(), value);
        return Some(declarations);
    }
    if let Some((property, value)) = tailwind_filter_component_declaration(class, "backdrop-") {
        declarations.insert(property, value);
        declarations.insert(
            "backdrop-filter".to_string(),
            tailwind_backdrop_filter_pipeline(),
        );
        return Some(declarations);
    }
    None
}

fn tailwind_blend_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class.strip_prefix("mix-blend-") {
        return tailwind_blend_mode_value(value, true)
            .map(|value| ("mix-blend-mode".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("bg-blend-") {
        return tailwind_blend_mode_value(value, false)
            .map(|value| ("background-blend-mode".to_string(), value));
    }
    None
}

fn tailwind_blend_mode_value(value: &str, include_plus_modes: bool) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    let known = matches!(
        value,
        "normal"
            | "multiply"
            | "screen"
            | "overlay"
            | "darken"
            | "lighten"
            | "color-dodge"
            | "color-burn"
            | "hard-light"
            | "soft-light"
            | "difference"
            | "exclusion"
            | "hue"
            | "saturation"
            | "color"
            | "luminosity"
    ) || (include_plus_modes && matches!(value, "plus-darker" | "plus-lighter"));
    known.then(|| value.to_string())
}

fn tailwind_mask_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("mask-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        if let Some((property, value)) = tailwind_mask_arbitrary_property(value) {
            return Some((property, value));
        }
        return Some(("mask-image".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class.strip_prefix("mask-").and_then(tailwind_custom_var) {
        return Some(("mask-image".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-image-") {
        return Some(("mask-image".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-size-") {
        return Some(("mask-size".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-position-") {
        return Some(("mask-position".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-repeat-") {
        return Some(("mask-repeat".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-origin-") {
        return Some(("mask-origin".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-clip-") {
        return Some(("mask-clip".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-composite-") {
        return Some(("mask-composite".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-mode-") {
        return Some(("mask-mode".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-type-") {
        return Some(("mask-type".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-border-") {
        return Some(("mask-border".to_string(), value));
    }

    let declaration = match class {
        "mask-none" => Some(("mask-image", "none")),
        "mask-alpha" => Some(("mask-mode", "alpha")),
        "mask-luminance" => Some(("mask-mode", "luminance")),
        "mask-match" => Some(("mask-mode", "match-source")),
        "mask-type-alpha" => Some(("mask-type", "alpha")),
        "mask-type-luminance" => Some(("mask-type", "luminance")),
        "mask-auto" => Some(("mask-size", "auto")),
        "mask-cover" => Some(("mask-size", "cover")),
        "mask-contain" => Some(("mask-size", "contain")),
        "mask-repeat" => Some(("mask-repeat", "repeat")),
        "mask-no-repeat" => Some(("mask-repeat", "no-repeat")),
        "mask-repeat-x" => Some(("mask-repeat", "repeat-x")),
        "mask-repeat-y" => Some(("mask-repeat", "repeat-y")),
        "mask-repeat-space" => Some(("mask-repeat", "space")),
        "mask-repeat-round" => Some(("mask-repeat", "round")),
        "mask-center" => Some(("mask-position", "center")),
        "mask-top" => Some(("mask-position", "top")),
        "mask-right" => Some(("mask-position", "right")),
        "mask-bottom" => Some(("mask-position", "bottom")),
        "mask-left" => Some(("mask-position", "left")),
        "mask-top-left" | "mask-left-top" => Some(("mask-position", "top left")),
        "mask-top-right" | "mask-right-top" => Some(("mask-position", "top right")),
        "mask-bottom-right" | "mask-right-bottom" => Some(("mask-position", "bottom right")),
        "mask-bottom-left" | "mask-left-bottom" => Some(("mask-position", "bottom left")),
        "mask-origin-border" => Some(("mask-origin", "border-box")),
        "mask-origin-padding" => Some(("mask-origin", "padding-box")),
        "mask-origin-content" => Some(("mask-origin", "content-box")),
        "mask-origin-fill" => Some(("mask-origin", "fill-box")),
        "mask-origin-stroke" => Some(("mask-origin", "stroke-box")),
        "mask-origin-view" => Some(("mask-origin", "view-box")),
        "mask-clip-border" => Some(("mask-clip", "border-box")),
        "mask-clip-padding" => Some(("mask-clip", "padding-box")),
        "mask-clip-content" => Some(("mask-clip", "content-box")),
        "mask-clip-fill" => Some(("mask-clip", "fill-box")),
        "mask-clip-stroke" => Some(("mask-clip", "stroke-box")),
        "mask-clip-view" => Some(("mask-clip", "view-box")),
        "mask-no-clip" => Some(("mask-clip", "no-clip")),
        "mask-add" => Some(("mask-composite", "add")),
        "mask-subtract" => Some(("mask-composite", "subtract")),
        "mask-intersect" => Some(("mask-composite", "intersect")),
        "mask-exclude" => Some(("mask-composite", "exclude")),
        _ => None,
    }?;
    Some((declaration.0.to_string(), declaration.1.to_string()))
}

fn tailwind_mask_arbitrary_property(value: &str) -> Option<(String, String)> {
    let (name, value) = value.split_once(':')?;
    let property = match name {
        "image" => "mask-image",
        "mode" => "mask-mode",
        "repeat" => "mask-repeat",
        "position" => "mask-position",
        "size" => "mask-size",
        "origin" => "mask-origin",
        "clip" => "mask-clip",
        "composite" => "mask-composite",
        "type" => "mask-type",
        "border" => "mask-border",
        "border-source" => "mask-border-source",
        "border-mode" => "mask-border-mode",
        "border-slice" => "mask-border-slice",
        "border-width" => "mask-border-width",
        "border-outset" => "mask-border-outset",
        "border-repeat" => "mask-border-repeat",
        _ => return None,
    };
    Some((property.to_string(), tailwind_arbitrary_value(value)))
}

fn tailwind_mask_prefixed_value(class: &str, prefix: &str) -> Option<String> {
    class
        .strip_prefix(prefix)
        .and_then(tailwind_arbitrary_or_custom_var)
}

fn tailwind_filter_component_declaration(class: &str, prefix: &str) -> Option<(String, String)> {
    let class = class.strip_prefix(prefix)?;
    let variable_prefix = if prefix.is_empty() {
        "--tw"
    } else {
        "--tw-backdrop"
    };
    if let Some(value) = class.strip_prefix("blur") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_blur_value(value)?;
        return Some((format!("{variable_prefix}-blur"), value));
    }
    if let Some(value) = class.strip_prefix("brightness-") {
        let value = tailwind_percent_filter_value(value, "brightness")?;
        return Some((format!("{variable_prefix}-brightness"), value));
    }
    if let Some(value) = class.strip_prefix("contrast-") {
        let value = tailwind_percent_filter_value(value, "contrast")?;
        return Some((format!("{variable_prefix}-contrast"), value));
    }
    if prefix.is_empty() {
        if let Some(value) = class.strip_prefix("drop-shadow") {
            let value = tailwind_optional_suffix(value)?;
            let value = tailwind_drop_shadow_value(value)?;
            return Some(("--tw-drop-shadow".to_string(), value));
        }
    }
    if let Some(value) = class.strip_prefix("grayscale") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_binary_filter_value(value, "grayscale")?;
        return Some((format!("{variable_prefix}-grayscale"), value));
    }
    if let Some(value) = class.strip_prefix("hue-rotate-") {
        let (negative, value) = strip_negative_prefix(value);
        let value = tailwind_signed_angle_value(value, negative)?;
        return Some((
            format!("{variable_prefix}-hue-rotate"),
            format!("hue-rotate({value})"),
        ));
    }
    if let Some(value) = class.strip_prefix("-hue-rotate-") {
        let value = tailwind_signed_angle_value(value, true)?;
        return Some((
            format!("{variable_prefix}-hue-rotate"),
            format!("hue-rotate({value})"),
        ));
    }
    if let Some(value) = class.strip_prefix("invert") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_binary_filter_value(value, "invert")?;
        return Some((format!("{variable_prefix}-invert"), value));
    }
    if prefix == "backdrop-" {
        if let Some(value) = class.strip_prefix("opacity-") {
            let value = tailwind_percent_filter_value(value, "opacity")?;
            return Some(("--tw-backdrop-opacity".to_string(), value));
        }
    }
    if let Some(value) = class.strip_prefix("saturate-") {
        let value = tailwind_percent_filter_value(value, "saturate")?;
        return Some((format!("{variable_prefix}-saturate"), value));
    }
    if let Some(value) = class.strip_prefix("sepia") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_binary_filter_value(value, "sepia")?;
        return Some((format!("{variable_prefix}-sepia"), value));
    }
    None
}

fn tailwind_optional_suffix(value: &str) -> Option<&str> {
    if value.is_empty() {
        Some("DEFAULT")
    } else {
        value.strip_prefix('-')
    }
}

fn tailwind_blur_value(value: &str) -> Option<String> {
    match value {
        "DEFAULT" => Some("blur(8px)".to_string()),
        "none" => Some(String::new()),
        "xs" => Some("blur(4px)".to_string()),
        "sm" => Some("blur(8px)".to_string()),
        "md" => Some("blur(12px)".to_string()),
        "lg" => Some("blur(16px)".to_string()),
        "xl" => Some("blur(24px)".to_string()),
        "2xl" => Some("blur(40px)".to_string()),
        "3xl" => Some("blur(64px)".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value).map(|value| format!("blur({value})")),
    }
}

fn tailwind_percent_filter_value(value: &str, function: &str) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}%", trim_float(value)))
    })?;
    Some(format!("{function}({value})"))
}

fn tailwind_binary_filter_value(value: &str, function: &str) -> Option<String> {
    let value = match value {
        "DEFAULT" => "100%".to_string(),
        "0" => "0%".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)?,
    };
    Some(format!("{function}({value})"))
}

fn tailwind_drop_shadow_value(value: &str) -> Option<String> {
    let shadow = match value {
        "DEFAULT" => "0 1px 2px rgb(0 0 0 / 0.1), 0 1px 1px rgb(0 0 0 / 0.06)".to_string(),
        "xs" => "0 1px 1px rgb(0 0 0 / 0.05)".to_string(),
        "sm" => "0 1px 2px rgb(0 0 0 / 0.15)".to_string(),
        "md" => "0 3px 3px rgb(0 0 0 / 0.12)".to_string(),
        "lg" => "0 4px 4px rgb(0 0 0 / 0.15)".to_string(),
        "xl" => "0 9px 7px rgb(0 0 0 / 0.1)".to_string(),
        "2xl" => "0 25px 25px rgb(0 0 0 / 0.15)".to_string(),
        "none" => "0 0 #0000".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)?,
    };
    Some(format!("drop-shadow({shadow})"))
}

fn tailwind_filter_pipeline() -> String {
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-drop-shadow) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia)"
        .to_string()
}

fn tailwind_backdrop_filter_pipeline() -> String {
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)"
        .to_string()
}

fn tailwind_motion_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "transition" => {
            declarations.insert(
                "transition-property".to_string(),
                "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter".to_string(),
            );
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-all" => {
            declarations.insert("transition-property".to_string(), "all".to_string());
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-colors" => {
            declarations.insert(
                "transition-property".to_string(),
                "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke".to_string(),
            );
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-opacity" => {
            declarations.insert("transition-property".to_string(), "opacity".to_string());
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-shadow" => {
            declarations.insert("transition-property".to_string(), "box-shadow".to_string());
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-transform" => {
            declarations.insert(
                "transition-property".to_string(),
                "transform, translate, scale, rotate".to_string(),
            );
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-none" => {
            declarations.insert("transition-property".to_string(), "none".to_string());
            return Some(declarations);
        }
        "transition-discrete" => {
            declarations.insert(
                "transition-behavior".to_string(),
                "allow-discrete".to_string(),
            );
            return Some(declarations);
        }
        "transition-normal" => {
            declarations.insert("transition-behavior".to_string(), "normal".to_string());
            return Some(declarations);
        }
        "animate-spin" => {
            declarations.insert(
                "animation".to_string(),
                "spin 1s linear infinite".to_string(),
            );
            return Some(declarations);
        }
        "animate-ping" => {
            declarations.insert(
                "animation".to_string(),
                "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            );
            return Some(declarations);
        }
        "animate-pulse" => {
            declarations.insert(
                "animation".to_string(),
                "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            );
            return Some(declarations);
        }
        "animate-bounce" => {
            declarations.insert("animation".to_string(), "bounce 1s infinite".to_string());
            return Some(declarations);
        }
        "animate-none" => {
            declarations.insert("animation".to_string(), "none".to_string());
            return Some(declarations);
        }
        _ => {}
    }

    if let Some(value) = class
        .strip_prefix("transition-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "transition-property".to_string(),
            tailwind_arbitrary_value(value),
        );
        insert_tailwind_default_transition(&mut declarations);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("transition-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("transition-property".to_string(), value);
        insert_tailwind_default_transition(&mut declarations);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("duration-")
        .and_then(tailwind_time_value)
    {
        declarations.insert("transition-duration".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("delay-").and_then(tailwind_time_value) {
        declarations.insert("transition-delay".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("ease-").and_then(tailwind_easing_value) {
        declarations.insert("transition-timing-function".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("animate-")
        .and_then(tailwind_animation_value)
    {
        declarations.insert("animation".to_string(), value);
        return Some(declarations);
    }

    None
}

fn insert_tailwind_default_transition(declarations: &mut BTreeMap<String, String>) {
    declarations.insert(
        "transition-timing-function".to_string(),
        "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
    );
    declarations.insert("transition-duration".to_string(), "150ms".to_string());
}

fn tailwind_time_value(value: &str) -> Option<String> {
    if value == "initial" {
        return Some("initial".to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    value.parse::<f64>().ok().map(|value| {
        if value == 0.0 {
            "0ms".to_string()
        } else {
            format!("{}ms", trim_float(value))
        }
    })
}

fn tailwind_easing_value(value: &str) -> Option<String> {
    match value {
        "linear" => Some("linear".to_string()),
        "in" => Some("cubic-bezier(0.4, 0, 1, 1)".to_string()),
        "out" => Some("cubic-bezier(0, 0, 0.2, 1)".to_string()),
        "in-out" => Some("cubic-bezier(0.4, 0, 0.2, 1)".to_string()),
        "initial" => Some("initial".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value),
    }
}

fn tailwind_animation_value(value: &str) -> Option<String> {
    match value {
        "none" => Some("none".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value),
    }
}

fn tailwind_interaction_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let declaration = match class {
        "appearance-none" => Some(("appearance", "none".to_string())),
        "appearance-auto" => Some(("appearance", "auto".to_string())),
        "will-change-auto" => Some(("will-change", "auto".to_string())),
        "will-change-scroll" => Some(("will-change", "scroll-position".to_string())),
        "will-change-contents" => Some(("will-change", "contents".to_string())),
        "will-change-transform" => Some(("will-change", "transform".to_string())),
        "scroll-auto" => Some(("scroll-behavior", "auto".to_string())),
        "scroll-smooth" => Some(("scroll-behavior", "smooth".to_string())),
        "snap-none" => Some(("scroll-snap-type", "none".to_string())),
        "snap-x" => Some((
            "scroll-snap-type",
            "x var(--tw-scroll-snap-strictness)".to_string(),
        )),
        "snap-y" => Some((
            "scroll-snap-type",
            "y var(--tw-scroll-snap-strictness)".to_string(),
        )),
        "snap-both" => Some((
            "scroll-snap-type",
            "both var(--tw-scroll-snap-strictness)".to_string(),
        )),
        "snap-mandatory" => Some(("--tw-scroll-snap-strictness", "mandatory".to_string())),
        "snap-proximity" => Some(("--tw-scroll-snap-strictness", "proximity".to_string())),
        "snap-start" => Some(("scroll-snap-align", "start".to_string())),
        "snap-end" => Some(("scroll-snap-align", "end".to_string())),
        "snap-center" => Some(("scroll-snap-align", "center".to_string())),
        "snap-align-none" => Some(("scroll-snap-align", "none".to_string())),
        "snap-normal" => Some(("scroll-snap-stop", "normal".to_string())),
        "snap-always" => Some(("scroll-snap-stop", "always".to_string())),
        "overscroll-auto" => Some(("overscroll-behavior", "auto".to_string())),
        "overscroll-contain" => Some(("overscroll-behavior", "contain".to_string())),
        "overscroll-none" => Some(("overscroll-behavior", "none".to_string())),
        "overscroll-x-auto" => Some(("overscroll-behavior-x", "auto".to_string())),
        "overscroll-x-contain" => Some(("overscroll-behavior-x", "contain".to_string())),
        "overscroll-x-none" => Some(("overscroll-behavior-x", "none".to_string())),
        "overscroll-y-auto" => Some(("overscroll-behavior-y", "auto".to_string())),
        "overscroll-y-contain" => Some(("overscroll-behavior-y", "contain".to_string())),
        "overscroll-y-none" => Some(("overscroll-behavior-y", "none".to_string())),
        "touch-auto" => Some(("touch-action", "auto".to_string())),
        "touch-none" => Some(("touch-action", "none".to_string())),
        "touch-pan-x" => Some(("touch-action", "pan-x".to_string())),
        "touch-pan-left" => Some(("touch-action", "pan-left".to_string())),
        "touch-pan-right" => Some(("touch-action", "pan-right".to_string())),
        "touch-pan-y" => Some(("touch-action", "pan-y".to_string())),
        "touch-pan-up" => Some(("touch-action", "pan-up".to_string())),
        "touch-pan-down" => Some(("touch-action", "pan-down".to_string())),
        "touch-pinch-zoom" => Some(("touch-action", "pinch-zoom".to_string())),
        "touch-manipulation" => Some(("touch-action", "manipulation".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("appearance-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("appearance".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("will-change-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("will-change".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("will-change-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("will-change".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("touch-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("touch-action".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    None
}

fn tailwind_media_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("bg-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        if let Some((hint, hinted_value)) = value.split_once(':') {
            if let Some(property) = match hint {
                "color" => Some("background-color"),
                "image" | "url" => Some("background-image"),
                "length" | "size" => Some("background-size"),
                "position" => Some("background-position"),
                _ => None,
            } {
                return Some((property.to_string(), tailwind_arbitrary_value(hinted_value)));
            }
        }
        let value = tailwind_arbitrary_value(value);
        let property = if is_css_background_image_value(&value) {
            "background-image"
        } else if parse_color(&value).is_some() {
            "background-color"
        } else if is_background_position_value(&value) {
            "background-position"
        } else {
            "background-size"
        };
        return Some((property.to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("object-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "object-position".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("list-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "list-style-type".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("columns-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("columns".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class.strip_prefix("columns-") {
        if let Some(value) = tailwind_columns_value(value) {
            return Some(("columns".to_string(), value));
        }
    }
    None
}

fn is_css_background_image_value(value: &str) -> bool {
    matches!(
        value.split_once('(').map(|(name, _)| name.trim()),
        Some(
            "url"
                | "image"
                | "image-set"
                | "linear-gradient"
                | "radial-gradient"
                | "conic-gradient"
                | "repeating-linear-gradient"
                | "repeating-radial-gradient"
                | "repeating-conic-gradient"
        )
    ) && value.ends_with(')')
}

fn is_background_position_value(value: &str) -> bool {
    let parts = value.split_whitespace().collect::<Vec<_>>();
    !parts.is_empty()
        && parts.iter().all(|part| {
            matches!(*part, "top" | "right" | "bottom" | "left" | "center")
                || parse_length(part).is_some()
        })
}

fn tailwind_columns_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_container_width(value).map(ToString::to_string))
        .or_else(|| value.parse::<u16>().ok().map(|value| value.to_string()))
}

fn tailwind_flex_value(value: &str) -> Option<String> {
    match value {
        "auto" => Some("auto".to_string()),
        "initial" => Some("0 auto".to_string()),
        "none" => Some("none".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| tailwind_fraction(value).map(|value| format!("calc({value} * 100%)")))
            .or_else(|| value.parse::<f64>().ok().map(trim_float)),
    }
}

fn tailwind_basis_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_container_width(value).map(ToString::to_string))
        .or_else(|| tailwind_length(value).map(style_length_css))
}

fn tailwind_number_token(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value).or_else(|| value.parse::<f64>().ok().map(trim_float))
}

fn tailwind_order_value(class: &str) -> Option<String> {
    let negative = class.starts_with("-order-");
    let value = if negative {
        class.strip_prefix("-order-")?
    } else {
        class.strip_prefix("order-")?
    };
    let value = match value {
        "first" if !negative => "-9999".to_string(),
        "last" if !negative => "9999".to_string(),
        "none" if !negative => "0".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| value.parse::<i32>().ok().map(|value| value.to_string()))?,
    };
    Some(if negative {
        format!("calc({value} * -1)")
    } else {
        value
    })
}

fn tailwind_fraction(value: &str) -> Option<String> {
    let (numerator, denominator) = value.split_once('/')?;
    let numerator = numerator.parse::<f64>().ok()?;
    let denominator = denominator.parse::<f64>().ok()?;
    if denominator == 0.0 {
        None
    } else {
        Some(trim_float(numerator / denominator))
    }
}

fn tailwind_container_width(value: &str) -> Option<&'static str> {
    match value {
        "3xs" => Some("16rem"),
        "2xs" => Some("18rem"),
        "xs" => Some("20rem"),
        "sm" => Some("24rem"),
        "md" => Some("28rem"),
        "lg" => Some("32rem"),
        "xl" => Some("36rem"),
        "2xl" => Some("42rem"),
        "3xl" => Some("48rem"),
        "4xl" => Some("56rem"),
        "5xl" => Some("64rem"),
        "6xl" => Some("72rem"),
        "7xl" => Some("80rem"),
        _ => None,
    }
}

fn tailwind_font_family(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
}

fn tailwind_letter_spacing(class: &str) -> Option<String> {
    let negative = class.starts_with("-tracking-");
    let value = if negative {
        class.strip_prefix("-tracking-")?
    } else {
        class.strip_prefix("tracking-")?
    };
    let value = match value {
        "tighter" => "-0.05em".to_string(),
        "tight" => "-0.025em".to_string(),
        "normal" => "0em".to_string(),
        "wide" => "0.025em".to_string(),
        "wider" => "0.05em".to_string(),
        "widest" => "0.1em".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| parse_length(value).map(style_length_css))?,
    };
    Some(if negative {
        format!("calc({value} * -1)")
    } else {
        value
    })
}

fn tailwind_decoration_declaration(class: &str) -> Option<(String, String)> {
    let value = class.strip_prefix("decoration-")?;
    if let Some(value) = tailwind_decoration_thickness(value) {
        return Some(("text-decoration-thickness".to_string(), value));
    }
    tailwind_color_css(value).map(|value| ("text-decoration-color".to_string(), value))
}

fn tailwind_decoration_thickness(value: &str) -> Option<String> {
    match value {
        "auto" => Some("auto".to_string()),
        "from-font" => Some("from-font".to_string()),
        _ => tailwind_border_width(value).map(style_length_css),
    }
}

fn tailwind_underline_offset(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_border_width(value).map(style_length_css))
}

fn tailwind_visual_effect_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("shadow-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("box-shadow".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("outline".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-offset-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "outline-offset".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("cursor-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("cursor".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("aspect-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("aspect-ratio".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("transform-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("transform".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("filter".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "backdrop-filter".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("outline-offset-")
        .and_then(tailwind_length)
    {
        return Some(("outline-offset".to_string(), style_length_css(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-")
        .and_then(tailwind_border_width)
    {
        return Some(("outline-width".to_string(), style_length_css(value)));
    }
    if let Some(value) = class.strip_prefix("outline-").and_then(tailwind_color_css) {
        return Some(("outline-color".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("cursor-") {
        if is_tailwind_cursor(value) {
            return Some(("cursor".to_string(), value.to_string()));
        }
    }
    if let Some(value) = class.strip_prefix("aspect-") {
        if let Some((width, height)) = value.split_once('/') {
            if width.parse::<f64>().is_ok() && height.parse::<f64>().is_ok() {
                return Some(("aspect-ratio".to_string(), format!("{width} / {height}")));
            }
        }
    }
    if let Some(value) = tailwind_transform_declaration(class) {
        return Some(("transform".to_string(), value));
    }
    None
}

fn tailwind_grid_declaration(class: &str) -> Option<(String, String)> {
    let declaration = match class {
        "grid-flow-row" => Some(("grid-auto-flow", "row".to_string())),
        "grid-flow-col" => Some(("grid-auto-flow", "column".to_string())),
        "grid-flow-dense" => Some(("grid-auto-flow", "dense".to_string())),
        "grid-flow-row-dense" => Some(("grid-auto-flow", "row dense".to_string())),
        "grid-flow-col-dense" => Some(("grid-auto-flow", "column dense".to_string())),
        "auto-cols-auto" => Some(("grid-auto-columns", "auto".to_string())),
        "auto-cols-min" => Some(("grid-auto-columns", "min-content".to_string())),
        "auto-cols-max" => Some(("grid-auto-columns", "max-content".to_string())),
        "auto-cols-fr" => Some(("grid-auto-columns", "minmax(0, 1fr)".to_string())),
        "auto-rows-auto" => Some(("grid-auto-rows", "auto".to_string())),
        "auto-rows-min" => Some(("grid-auto-rows", "min-content".to_string())),
        "auto-rows-max" => Some(("grid-auto-rows", "max-content".to_string())),
        "auto-rows-fr" => Some(("grid-auto-rows", "minmax(0, 1fr)".to_string())),
        "col-auto" => Some(("grid-column", "auto".to_string())),
        "col-span-full" => Some(("grid-column", "1 / -1".to_string())),
        "row-auto" => Some(("grid-row", "auto".to_string())),
        "row-span-full" => Some(("grid-row", "1 / -1".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        return Some((property.to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("grid-cols-")
        .and_then(tailwind_grid_track_list)
    {
        return Some(("grid-template-columns".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("grid-rows-")
        .and_then(tailwind_grid_track_list)
    {
        return Some(("grid-template-rows".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("auto-cols-")
        .and_then(tailwind_grid_auto_track)
    {
        return Some(("grid-auto-columns".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("auto-rows-")
        .and_then(tailwind_grid_auto_track)
    {
        return Some(("grid-auto-rows".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("col-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("grid-column".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("row-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("grid-row".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class.strip_prefix("col-").and_then(tailwind_custom_var) {
        return Some(("grid-column".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("row-").and_then(tailwind_custom_var) {
        return Some(("grid-row".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("col-span-").and_then(tailwind_grid_line) {
        return Some((
            "grid-column".to_string(),
            format!("span {value} / span {value}"),
        ));
    }
    if let Some(value) = class.strip_prefix("row-span-").and_then(tailwind_grid_line) {
        return Some((
            "grid-row".to_string(),
            format!("span {value} / span {value}"),
        ));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "col-start-") {
        return Some(("grid-column-start".to_string(), value));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "col-end-") {
        return Some(("grid-column-end".to_string(), value));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "row-start-") {
        return Some(("grid-row-start".to_string(), value));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "row-end-") {
        return Some(("grid-row-end".to_string(), value));
    }
    None
}

fn tailwind_grid_track_list(value: &str) -> Option<String> {
    if matches!(value, "none" | "subgrid") {
        return Some(value.to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    let count = value.parse::<u16>().ok()?;
    if count == 0 {
        return None;
    }
    Some(format!("repeat({count}, minmax(0, 1fr))"))
}

fn tailwind_grid_auto_track(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    match value {
        "auto" => Some("auto".to_string()),
        "min" => Some("min-content".to_string()),
        "max" => Some("max-content".to_string()),
        "fr" => Some("minmax(0, 1fr)".to_string()),
        _ => None,
    }
}

fn tailwind_grid_line_utility(class: &str, prefix: &str) -> Option<String> {
    if let Some(value) = class.strip_prefix(prefix).and_then(tailwind_grid_line) {
        return Some(value);
    }
    let negative_prefix = format!("-{prefix}");
    let value = class
        .strip_prefix(&negative_prefix)
        .and_then(tailwind_grid_line)?;
    Some(format!("calc({value} * -1)"))
}

fn tailwind_grid_line(value: &str) -> Option<String> {
    if value == "auto" {
        return Some("auto".to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    value.parse::<u16>().ok().map(|value| value.to_string())
}

fn tailwind_arbitrary_or_custom_var(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    tailwind_custom_var(value)
}

fn tailwind_custom_var(value: &str) -> Option<String> {
    let variable = value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))?
        .trim();
    if variable.is_empty() {
        None
    } else {
        Some(format!("var({variable})"))
    }
}

fn insert_edge_declarations(
    declarations: &mut BTreeMap<String, String>,
    prefix: &str,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert(prefix.to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert(format!("{prefix}-inline"), value);
        }
        EdgeSelection::Y => {
            declarations.insert(format!("{prefix}-block"), value);
        }
        EdgeSelection::Top => {
            declarations.insert(format!("{prefix}-top"), value);
        }
        EdgeSelection::Right => {
            declarations.insert(format!("{prefix}-right"), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert(format!("{prefix}-bottom"), value);
        }
        EdgeSelection::Left => {
            declarations.insert(format!("{prefix}-left"), value);
        }
    }
}

fn insert_position_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert("inset".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("inset-inline".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("inset-block".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("top".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("right".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("bottom".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("left".to_string(), value);
        }
    }
}

fn insert_logical_edge_declaration(
    declarations: &mut BTreeMap<String, String>,
    prefix: &str,
    edges: LogicalEdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    let property = match edges {
        LogicalEdgeSelection::Block => format!("{prefix}-block"),
        LogicalEdgeSelection::Inline => format!("{prefix}-inline"),
        LogicalEdgeSelection::BlockStart => format!("{prefix}-block-start"),
        LogicalEdgeSelection::BlockEnd => format!("{prefix}-block-end"),
        LogicalEdgeSelection::InlineStart => format!("{prefix}-inline-start"),
        LogicalEdgeSelection::InlineEnd => format!("{prefix}-inline-end"),
    };
    declarations.insert(property, value);
}

fn insert_logical_position_declaration(
    declarations: &mut BTreeMap<String, String>,
    edges: LogicalEdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    let property = match edges {
        LogicalEdgeSelection::Block => "inset-block",
        LogicalEdgeSelection::Inline => "inset-inline",
        LogicalEdgeSelection::BlockStart => "inset-block-start",
        LogicalEdgeSelection::BlockEnd => "inset-block-end",
        LogicalEdgeSelection::InlineStart => "inset-inline-start",
        LogicalEdgeSelection::InlineEnd => "inset-inline-end",
    };
    declarations.insert(property.to_string(), value);
}

fn insert_corner_radius_declarations(
    declarations: &mut BTreeMap<String, String>,
    corners: CornerSelection,
    radius: CornerRadius,
) {
    let value = corner_radius_css(radius);
    match corners {
        CornerSelection::All => {
            declarations.insert("border-radius".to_string(), value);
        }
        CornerSelection::Top => {
            declarations.insert("border-top-left-radius".to_string(), value.clone());
            declarations.insert("border-top-right-radius".to_string(), value);
        }
        CornerSelection::Right => {
            declarations.insert("border-top-right-radius".to_string(), value.clone());
            declarations.insert("border-bottom-right-radius".to_string(), value);
        }
        CornerSelection::Bottom => {
            declarations.insert("border-bottom-right-radius".to_string(), value.clone());
            declarations.insert("border-bottom-left-radius".to_string(), value);
        }
        CornerSelection::Left => {
            declarations.insert("border-top-left-radius".to_string(), value.clone());
            declarations.insert("border-bottom-left-radius".to_string(), value);
        }
        CornerSelection::TopLeft => {
            declarations.insert("border-top-left-radius".to_string(), value);
        }
        CornerSelection::TopRight => {
            declarations.insert("border-top-right-radius".to_string(), value);
        }
        CornerSelection::BottomRight => {
            declarations.insert("border-bottom-right-radius".to_string(), value);
        }
        CornerSelection::BottomLeft => {
            declarations.insert("border-bottom-left-radius".to_string(), value);
        }
    }
}

fn insert_logical_corner_radius_declarations(
    declarations: &mut BTreeMap<String, String>,
    corners: LogicalCornerSelection,
    radius: CornerRadius,
) {
    let value = corner_radius_css(radius);
    match corners {
        LogicalCornerSelection::Start => {
            declarations.insert("border-start-start-radius".to_string(), value.clone());
            declarations.insert("border-end-start-radius".to_string(), value);
        }
        LogicalCornerSelection::End => {
            declarations.insert("border-start-end-radius".to_string(), value.clone());
            declarations.insert("border-end-end-radius".to_string(), value);
        }
        LogicalCornerSelection::StartStart => {
            declarations.insert("border-start-start-radius".to_string(), value);
        }
        LogicalCornerSelection::StartEnd => {
            declarations.insert("border-start-end-radius".to_string(), value);
        }
        LogicalCornerSelection::EndEnd => {
            declarations.insert("border-end-end-radius".to_string(), value);
        }
        LogicalCornerSelection::EndStart => {
            declarations.insert("border-end-start-radius".to_string(), value);
        }
    }
}

fn insert_border_color_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: String,
) {
    match edges {
        EdgeSelection::All => {
            declarations.insert("border-color".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("border-inline-color".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("border-block-color".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("border-top-color".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("border-right-color".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("border-bottom-color".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("border-left-color".to_string(), value);
        }
    }
}

fn insert_logical_border_color_declaration(
    declarations: &mut BTreeMap<String, String>,
    edges: LogicalEdgeSelection,
    value: String,
) {
    let property = match edges {
        LogicalEdgeSelection::Block => "border-block-color",
        LogicalEdgeSelection::Inline => "border-inline-color",
        LogicalEdgeSelection::BlockStart => "border-block-start-color",
        LogicalEdgeSelection::BlockEnd => "border-block-end-color",
        LogicalEdgeSelection::InlineStart => "border-inline-start-color",
        LogicalEdgeSelection::InlineEnd => "border-inline-end-color",
    };
    declarations.insert(property.to_string(), value);
}

fn insert_logical_border_width_declaration(
    declarations: &mut BTreeMap<String, String>,
    edges: LogicalEdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    let property = match edges {
        LogicalEdgeSelection::Block => "border-block-width",
        LogicalEdgeSelection::Inline => "border-inline-width",
        LogicalEdgeSelection::BlockStart => "border-block-start-width",
        LogicalEdgeSelection::BlockEnd => "border-block-end-width",
        LogicalEdgeSelection::InlineStart => "border-inline-start-width",
        LogicalEdgeSelection::InlineEnd => "border-inline-end-width",
    };
    declarations.insert(property.to_string(), value);
}

fn insert_border_width_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert("border-width".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("border-inline-width".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("border-block-width".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("border-top-width".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("border-right-width".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("border-bottom-width".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("border-left-width".to_string(), value);
        }
    }
}

fn style_length_css(value: StyleLength) -> String {
    match value {
        StyleLength::Points(value) => format!("{}px", trim_float(value)),
        StyleLength::Percent(value) => format!("{}%", trim_float(value)),
        StyleLength::Auto => "auto".to_string(),
        StyleLength::Css(value) => value,
    }
}

fn corner_radius_css(radius: CornerRadius) -> String {
    let horizontal = style_length_css(radius.horizontal);
    if let Some(vertical) = radius.vertical {
        format!("{horizontal} {}", style_length_css(vertical))
    } else {
        horizontal
    }
}

fn trim_float(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn tailwind_arbitrary_value(value: &str) -> String {
    value.replace('_', " ")
}

fn is_tailwind_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
}

fn is_tailwind_cursor(value: &str) -> bool {
    matches!(
        value,
        "auto"
            | "default"
            | "pointer"
            | "wait"
            | "text"
            | "move"
            | "help"
            | "not-allowed"
            | "none"
            | "context-menu"
            | "progress"
            | "cell"
            | "crosshair"
            | "vertical-text"
            | "alias"
            | "copy"
            | "no-drop"
            | "grab"
            | "grabbing"
            | "all-scroll"
            | "col-resize"
            | "row-resize"
            | "n-resize"
            | "e-resize"
            | "s-resize"
            | "w-resize"
            | "ne-resize"
            | "nw-resize"
            | "se-resize"
            | "sw-resize"
            | "ew-resize"
            | "ns-resize"
            | "nesw-resize"
            | "nwse-resize"
            | "zoom-in"
            | "zoom-out"
    )
}

fn tailwind_transform_declaration(class: &str) -> Option<String> {
    if let Some(suffix) = class.strip_prefix("rotate-") {
        if let Some(value) = tailwind_rotate_value(suffix) {
            return Some(format!("rotate({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-rotate-") {
        if let Some(value) = tailwind_rotate_value(suffix) {
            return Some(format!("rotate(-{value})"));
        }
    }
    if let Some(value) = class.strip_prefix("scale-").and_then(tailwind_scale_value) {
        return Some(format!("scale({value})"));
    }
    if let Some(value) = class
        .strip_prefix("scale-x-")
        .and_then(tailwind_scale_value)
    {
        return Some(format!("scaleX({value})"));
    }
    if let Some(value) = class
        .strip_prefix("scale-y-")
        .and_then(tailwind_scale_value)
    {
        return Some(format!("scaleY({value})"));
    }
    if let Some(suffix) = class.strip_prefix("translate-x-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateX({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-translate-x-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateX(-{value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("translate-y-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateY({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-translate-y-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateY(-{value})"));
        }
    }
    None
}

fn tailwind_rotate_value(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    Some(format!("{}deg", trim_float(value.parse::<f64>().ok()?)))
}

fn tailwind_scale_value(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    Some(trim_float(value.parse::<f64>().ok()? / 100.0))
}

fn tailwind_translate_value(value: &str) -> Option<String> {
    tailwind_length(value).map(style_length_css)
}

fn tailwind_text_size_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if let Some(arbitrary) = class
        .strip_prefix("text-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if parse_length(&value).is_some() {
            declarations.insert("font-size".to_string(), value);
            return Some(declarations);
        }
        return None;
    }
    let (font_size, line_height) = match class {
        "text-xs" => ("0.75rem", "1rem"),
        "text-sm" => ("0.875rem", "1.25rem"),
        "text-base" => ("1rem", "1.5rem"),
        "text-lg" => ("1.125rem", "1.75rem"),
        "text-xl" => ("1.25rem", "1.75rem"),
        "text-2xl" => ("1.5rem", "2rem"),
        "text-3xl" => ("1.875rem", "2.25rem"),
        "text-4xl" => ("2.25rem", "2.5rem"),
        "text-5xl" => ("3rem", "1"),
        "text-6xl" => ("3.75rem", "1"),
        "text-7xl" => ("4.5rem", "1"),
        "text-8xl" => ("6rem", "1"),
        "text-9xl" => ("8rem", "1"),
        _ => return None,
    };
    declarations.insert("font-size".to_string(), font_size.to_string());
    declarations.insert("line-height".to_string(), line_height.to_string());
    Some(declarations)
}

fn tailwind_line_clamp_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let value = class.strip_prefix("line-clamp-")?;
    let mut declarations = BTreeMap::new();
    if value == "none" {
        declarations.insert("overflow".to_string(), "visible".to_string());
        declarations.insert("display".to_string(), "block".to_string());
        declarations.insert("-webkit-box-orient".to_string(), "horizontal".to_string());
        declarations.insert("-webkit-line-clamp".to_string(), "unset".to_string());
        return Some(declarations);
    }
    let value = tailwind_line_clamp_value(value)?;
    declarations.insert("overflow".to_string(), "hidden".to_string());
    declarations.insert("display".to_string(), "-webkit-box".to_string());
    declarations.insert("-webkit-box-orient".to_string(), "vertical".to_string());
    declarations.insert("-webkit-line-clamp".to_string(), value);
    Some(declarations)
}

fn tailwind_line_clamp_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "number") {
        return Some(value);
    }
    if let Some(value) = tailwind_custom_var(value) {
        return Some(value);
    }
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    value.parse::<u32>().ok().map(|value| value.to_string())
}

fn tailwind_length(value: &str) -> Option<StyleLength> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
    }
    if let Some(variable) = tailwind_custom_var(value) {
        return Some(StyleLength::Css(variable));
    }
    if value == "full" {
        return Some(StyleLength::Percent(100.0));
    }
    if value == "screen" {
        return Some(StyleLength::Percent(100.0));
    }
    if value == "auto" {
        return Some(StyleLength::Auto);
    }
    if value == "px" {
        return Some(StyleLength::Points(1.0));
    }
    if let Some((numerator, denominator)) = value.split_once('/') {
        let numerator = numerator.parse::<f64>().ok()?;
        let denominator = denominator.parse::<f64>().ok()?;
        if denominator != 0.0 {
            return Some(StyleLength::Percent((numerator / denominator) * 100.0));
        }
    }
    let value = value.parse::<f64>().ok()?;
    Some(StyleLength::Points(value * 4.0))
}

fn tailwind_line_height(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    match value {
        "none" => Some("1".to_string()),
        "tight" => Some("1.25".to_string()),
        "snug" => Some("1.375".to_string()),
        "normal" => Some("1.5".to_string()),
        "relaxed" => Some("1.625".to_string()),
        "loose" => Some("2".to_string()),
        _ => tailwind_length(value).map(style_length_css),
    }
}

fn tailwind_text_indent(class: &str) -> Option<String> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let value = class.strip_prefix("indent-")?;
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some(style_length_css(length))
}

fn tailwind_opacity(value: &str) -> Option<f64> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return arbitrary.parse::<f64>().ok();
    }
    value.parse::<f64>().ok().map(|value| value / 100.0)
}

fn tailwind_color(value: &str) -> Option<StyleColor> {
    let (value, opacity) = split_tailwind_color_opacity(value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let color = parse_color(&tailwind_arbitrary_value(arbitrary))?;
        return Some(apply_tailwind_color_opacity(color, opacity));
    }
    let color = match value {
        "black" => parse_color("#000"),
        "white" => parse_color("#fff"),
        "transparent" => Some(StyleColor::Keyword("transparent".to_string())),
        "current" => Some(StyleColor::Keyword("currentColor".to_string())),
        "inherit" => Some(StyleColor::Keyword("inherit".to_string())),
        other if is_tailwind_palette_color(other) => Some(StyleColor::Keyword(other.to_string())),
        _ => None,
    }?;
    Some(apply_tailwind_color_opacity(color, opacity))
}

fn tailwind_accent_color_css(value: &str) -> Option<String> {
    if value == "auto" {
        Some("auto".to_string())
    } else {
        tailwind_color_css(value)
    }
}

fn tailwind_color_css(value: &str) -> Option<String> {
    let (value, opacity) = split_tailwind_color_opacity(value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if let Some(color) = parse_color(&value) {
            return Some(style_color_css(&apply_tailwind_color_opacity(
                color, opacity,
            )));
        }
        return Some(apply_tailwind_keyword_opacity(value, opacity));
    }
    let color = match value {
        "black" => parse_color("#000")
            .map(|color| style_color_css(&apply_tailwind_color_opacity(color, opacity))),
        "white" => parse_color("#fff")
            .map(|color| style_color_css(&apply_tailwind_color_opacity(color, opacity))),
        "transparent" => Some("transparent".to_string()),
        "current" => Some("currentColor".to_string()),
        "inherit" => Some("inherit".to_string()),
        other if is_tailwind_palette_color(other) => Some(other.to_string()),
        _ => None,
    }?;
    Some(match value {
        "black" | "white" => color,
        _ => apply_tailwind_keyword_opacity(color, opacity),
    })
}

fn split_tailwind_color_opacity(value: &str) -> (&str, Option<&str>) {
    let mut bracket_depth = 0usize;
    for (index, ch) in value.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '/' if bracket_depth == 0 => return (&value[..index], Some(&value[index + 1..])),
            _ => {}
        }
    }
    (value, None)
}

fn apply_tailwind_color_opacity(color: StyleColor, opacity: Option<&str>) -> StyleColor {
    let Some(alpha) = opacity.and_then(tailwind_opacity_alpha) else {
        return color;
    };
    match color {
        StyleColor::Rgba {
            red, green, blue, ..
        } => StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        },
        StyleColor::Keyword(value) => {
            StyleColor::Keyword(apply_tailwind_keyword_opacity(value, opacity))
        }
    }
}

fn apply_tailwind_keyword_opacity(value: String, opacity: Option<&str>) -> String {
    let Some(opacity) = opacity else {
        return value;
    };
    let Some(percent) = tailwind_opacity_percent(opacity) else {
        return value;
    };
    if value == "transparent" {
        value
    } else {
        format!("{value} / {percent}")
    }
}

fn tailwind_opacity_alpha(value: &str) -> Option<u8> {
    let opacity = tailwind_opacity(value)?;
    Some((opacity.clamp(0.0, 1.0) * 255.0).round() as u8)
}

fn tailwind_opacity_percent(value: &str) -> Option<String> {
    let opacity = tailwind_opacity(value)?;
    Some(format!("{}%", trim_float(opacity.clamp(0.0, 1.0) * 100.0)))
}

fn style_color_css(color: &StyleColor) -> String {
    match color {
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        } if *alpha < 255 => {
            let alpha = trim_float((*alpha as f64 / 255.0 * 100.0).round() / 100.0);
            format!("rgba({red}, {green}, {blue}, {alpha})")
        }
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha: _,
        } => format!("rgb({red}, {green}, {blue})"),
        StyleColor::Keyword(value) => value.clone(),
    }
}

fn tailwind_z_index(class: &str) -> Option<String> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let value = class.strip_prefix("z-")?;
    if value == "auto" {
        return Some("auto".to_string());
    }
    let value = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .map(tailwind_arbitrary_value)
        .unwrap_or_else(|| value.to_string());
    if negative {
        Some(format!("-{value}"))
    } else {
        Some(value)
    }
}

fn is_tailwind_palette_color(value: &str) -> bool {
    let Some((name, shade)) = value.rsplit_once('-') else {
        return false;
    };
    matches!(
        name,
        "slate"
            | "gray"
            | "zinc"
            | "neutral"
            | "stone"
            | "red"
            | "orange"
            | "amber"
            | "yellow"
            | "lime"
            | "green"
            | "emerald"
            | "teal"
            | "cyan"
            | "sky"
            | "blue"
            | "indigo"
            | "violet"
            | "purple"
            | "fuchsia"
            | "pink"
            | "rose"
    ) && matches!(
        shade,
        "50" | "100" | "200" | "300" | "400" | "500" | "600" | "700" | "800" | "900" | "950"
    )
}

fn tailwind_inset_utility(class: &str) -> Option<(EdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let (edges, value) = if let Some(value) = class.strip_prefix("inset-x-") {
        (EdgeSelection::X, value)
    } else if let Some(value) = class.strip_prefix("inset-y-") {
        (EdgeSelection::Y, value)
    } else if let Some(value) = class.strip_prefix("inset-") {
        (EdgeSelection::All, value)
    } else if let Some(value) = class.strip_prefix("top-") {
        (EdgeSelection::Top, value)
    } else if let Some(value) = class.strip_prefix("right-") {
        (EdgeSelection::Right, value)
    } else if let Some(value) = class.strip_prefix("bottom-") {
        (EdgeSelection::Bottom, value)
    } else if let Some(value) = class.strip_prefix("left-") {
        (EdgeSelection::Left, value)
    } else {
        return None;
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

fn tailwind_logical_inset_utility(class: &str) -> Option<(LogicalEdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let (edges, value) = if let Some(value) = class.strip_prefix("start-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if let Some(value) = class.strip_prefix("end-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else if let Some(value) = class.strip_prefix("inset-s-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if let Some(value) = class.strip_prefix("inset-e-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else if let Some(value) = class.strip_prefix("inset-bs-") {
        (LogicalEdgeSelection::BlockStart, value)
    } else if let Some(value) = class.strip_prefix("inset-be-") {
        (LogicalEdgeSelection::BlockEnd, value)
    } else if let Some(value) = class.strip_prefix("inset-is-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if let Some(value) = class.strip_prefix("inset-ie-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else {
        return None;
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

fn tailwind_border_width_utility(class: &str) -> Option<(EdgeSelection, StyleLength)> {
    let suffix = class.strip_prefix("border")?;
    if suffix.is_empty() {
        return Some((EdgeSelection::All, StyleLength::Points(1.0)));
    }
    let suffix = suffix.strip_prefix('-')?;
    let (edges, value) = if suffix == "x" {
        (EdgeSelection::X, "1")
    } else if let Some(value) = suffix.strip_prefix("x-") {
        (EdgeSelection::X, value)
    } else if suffix == "y" {
        (EdgeSelection::Y, "1")
    } else if let Some(value) = suffix.strip_prefix("y-") {
        (EdgeSelection::Y, value)
    } else if suffix == "t" {
        (EdgeSelection::Top, "1")
    } else if let Some(value) = suffix.strip_prefix("t-") {
        (EdgeSelection::Top, value)
    } else if suffix == "r" {
        (EdgeSelection::Right, "1")
    } else if let Some(value) = suffix.strip_prefix("r-") {
        (EdgeSelection::Right, value)
    } else if suffix == "b" {
        (EdgeSelection::Bottom, "1")
    } else if let Some(value) = suffix.strip_prefix("b-") {
        (EdgeSelection::Bottom, value)
    } else if suffix == "l" {
        (EdgeSelection::Left, "1")
    } else if let Some(value) = suffix.strip_prefix("l-") {
        (EdgeSelection::Left, value)
    } else {
        (EdgeSelection::All, suffix)
    };
    Some((edges, tailwind_border_width(value)?))
}

fn tailwind_logical_border_width_utility(
    class: &str,
) -> Option<(LogicalEdgeSelection, StyleLength)> {
    let suffix = class.strip_prefix("border-")?;
    let (edges, value) = if suffix == "s" {
        (LogicalEdgeSelection::InlineStart, "1")
    } else if let Some(value) = suffix.strip_prefix("s-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if suffix == "e" {
        (LogicalEdgeSelection::InlineEnd, "1")
    } else if let Some(value) = suffix.strip_prefix("e-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else if suffix == "bs" {
        (LogicalEdgeSelection::BlockStart, "1")
    } else if let Some(value) = suffix.strip_prefix("bs-") {
        (LogicalEdgeSelection::BlockStart, value)
    } else if suffix == "be" {
        (LogicalEdgeSelection::BlockEnd, "1")
    } else if let Some(value) = suffix.strip_prefix("be-") {
        (LogicalEdgeSelection::BlockEnd, value)
    } else if suffix == "is" {
        (LogicalEdgeSelection::InlineStart, "1")
    } else if let Some(value) = suffix.strip_prefix("is-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if suffix == "ie" {
        (LogicalEdgeSelection::InlineEnd, "1")
    } else if let Some(value) = suffix.strip_prefix("ie-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else {
        return None;
    };
    Some((edges, tailwind_border_width(value)?))
}

fn tailwind_border_color_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let suffix = class.strip_prefix("border-")?;
    let mut declarations = BTreeMap::new();
    if let Some((edges, value)) = tailwind_border_color_edge_value(suffix) {
        let color = tailwind_border_color_value(value)?;
        insert_border_color_declarations(&mut declarations, edges, color);
        return Some(declarations);
    }
    if let Some((edges, value)) = tailwind_logical_border_color_edge_value(suffix) {
        let color = tailwind_border_color_value(value)?;
        insert_logical_border_color_declaration(&mut declarations, edges, color);
        return Some(declarations);
    }
    let color = tailwind_border_color_value(suffix)?;
    declarations.insert("border-color".to_string(), color);
    Some(declarations)
}

fn tailwind_border_color_edge_value(value: &str) -> Option<(EdgeSelection, &str)> {
    if let Some(value) = value.strip_prefix("x-") {
        Some((EdgeSelection::X, value))
    } else if let Some(value) = value.strip_prefix("y-") {
        Some((EdgeSelection::Y, value))
    } else if let Some(value) = value.strip_prefix("t-") {
        Some((EdgeSelection::Top, value))
    } else if let Some(value) = value.strip_prefix("r-") {
        Some((EdgeSelection::Right, value))
    } else if let Some(value) = value.strip_prefix("b-") {
        Some((EdgeSelection::Bottom, value))
    } else if let Some(value) = value.strip_prefix("l-") {
        Some((EdgeSelection::Left, value))
    } else {
        None
    }
}

fn tailwind_logical_border_color_edge_value(value: &str) -> Option<(LogicalEdgeSelection, &str)> {
    if let Some(value) = value.strip_prefix("s-") {
        Some((LogicalEdgeSelection::InlineStart, value))
    } else if let Some(value) = value.strip_prefix("e-") {
        Some((LogicalEdgeSelection::InlineEnd, value))
    } else if let Some(value) = value.strip_prefix("bs-") {
        Some((LogicalEdgeSelection::BlockStart, value))
    } else if let Some(value) = value.strip_prefix("be-") {
        Some((LogicalEdgeSelection::BlockEnd, value))
    } else if let Some(value) = value.strip_prefix("is-") {
        Some((LogicalEdgeSelection::InlineStart, value))
    } else if let Some(value) = value.strip_prefix("ie-") {
        Some((LogicalEdgeSelection::InlineEnd, value))
    } else {
        None
    }
}

fn tailwind_border_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    if let Some(value) = tailwind_custom_var(value) {
        return Some(value);
    }
    tailwind_color_css(value)
}

fn tailwind_border_width(value: &str) -> Option<StyleLength> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
    }
    if value == "px" {
        return Some(StyleLength::Points(1.0));
    }
    value.parse::<f64>().ok().map(StyleLength::Points)
}

fn negate_style_length(value: StyleLength) -> Option<StyleLength> {
    match value {
        StyleLength::Points(value) => Some(StyleLength::Points(-value)),
        StyleLength::Percent(value) => Some(StyleLength::Percent(-value)),
        StyleLength::Css(value) => Some(StyleLength::Css(format!("calc({value} * -1)"))),
        StyleLength::Auto => None,
    }
}

fn tailwind_edge_utility(class: &str, prefix: &str) -> Option<(EdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let suffix = class.strip_prefix(prefix)?;
    let (edges, value) = match suffix.as_bytes() {
        [b'-', ..] => (EdgeSelection::All, &suffix[1..]),
        [b'x', b'-', ..] => (EdgeSelection::X, &suffix[2..]),
        [b'y', b'-', ..] => (EdgeSelection::Y, &suffix[2..]),
        [b't', b'-', ..] => (EdgeSelection::Top, &suffix[2..]),
        [b'r', b'-', ..] => (EdgeSelection::Right, &suffix[2..]),
        [b'b', b'-', ..] => (EdgeSelection::Bottom, &suffix[2..]),
        [b'l', b'-', ..] => (EdgeSelection::Left, &suffix[2..]),
        _ => return None,
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

fn tailwind_logical_edge_utility(
    class: &str,
    prefix: &str,
    allow_negative: bool,
) -> Option<(LogicalEdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    if negative && !allow_negative {
        return None;
    }
    let class = class.strip_prefix('-').unwrap_or(class);
    let suffix = class.strip_prefix(prefix)?;
    let (edges, value) = match suffix.as_bytes() {
        [b's', b'-', ..] => (LogicalEdgeSelection::InlineStart, &suffix[2..]),
        [b'e', b'-', ..] => (LogicalEdgeSelection::InlineEnd, &suffix[2..]),
        [b'b', b's', b'-', ..] => (LogicalEdgeSelection::BlockStart, &suffix[3..]),
        [b'b', b'e', b'-', ..] => (LogicalEdgeSelection::BlockEnd, &suffix[3..]),
        [b'i', b's', b'-', ..] => (LogicalEdgeSelection::InlineStart, &suffix[3..]),
        [b'i', b'e', b'-', ..] => (LogicalEdgeSelection::InlineEnd, &suffix[3..]),
        _ => return None,
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_web_style_into_portable_tokens() {
        let web = WebProps::new()
            .style("display", "flex")
            .style("flexDirection", "row")
            .style("minWidth", "280")
            .style("gap", "8px")
            .style("position", "absolute")
            .style("inset", "1px 2px 3px 4px")
            .style("paddingTop", "12")
            .style("margin", "1px 2px 3px 4px")
            .style("border", "2px solid #000")
            .style("fontWeight", "700")
            .style("lineHeight", "1.5rem")
            .style("textAlign", "center")
            .style("overflow", "hidden")
            .style("--brand-accent", "#663399")
            .style("backgroundColor", "#663399")
            .style("boxShadow", "0 1px 3px black");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Flex));
        assert_eq!(style.flex_direction, Some(Orientation::Horizontal));
        assert_eq!(style.min_width, Some(StyleLength::Points(280.0)));
        assert_eq!(style.gap, Some(StyleLength::Points(8.0)));
        assert_eq!(style.position, Some(PositionMode::Absolute));
        assert_eq!(style.inset.top, Some(StyleLength::Points(1.0)));
        assert_eq!(style.inset.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.inset.bottom, Some(StyleLength::Points(3.0)));
        assert_eq!(style.inset.left, Some(StyleLength::Points(4.0)));
        assert_eq!(style.padding.top, Some(StyleLength::Points(12.0)));
        assert_eq!(style.margin.top, Some(StyleLength::Points(1.0)));
        assert_eq!(style.margin.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.margin.bottom, Some(StyleLength::Points(3.0)));
        assert_eq!(style.margin.left, Some(StyleLength::Points(4.0)));
        assert_eq!(style.border_width.top, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_width.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_style, Some(BorderStyle::Solid));
        assert_eq!(
            style.border_color,
            Some(StyleColor::Rgba {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 255,
            })
        );
        assert_eq!(style.font_weight, Some(FontWeight::Number(700)));
        assert_eq!(style.line_height, Some(StyleLength::Points(24.0)));
        assert_eq!(style.text_align, Some(TextAlign::Center));
        assert_eq!(style.overflow_x, Some(OverflowMode::Hidden));
        assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
        assert_eq!(
            style.background_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 255,
            })
        );
        assert_eq!(
            style
                .declarations
                .get("background-color")
                .map(String::as_str),
            Some("#663399")
        );
        assert_eq!(
            style
                .custom_properties
                .get("--brand-accent")
                .map(String::as_str),
            Some("#663399")
        );
        assert_eq!(style.box_shadow.as_deref(), Some("0 1px 3px black"));
        assert!(!style.unsupported.contains_key("box-shadow"));
    }

    #[test]
    fn parses_tailwind_utilities_before_inline_style_overrides() {
        let web = WebProps::new()
            .class_name(
                "flex flex-col items-center justify-between min-w-[280px] gap-4 p-2 \
                 mx-auto bg-[#663399] text-white rounded-lg opacity-50 \
                 hover:bg-blue-600 md:flex-row focus:[outline:2px_solid_blue]",
            )
            .style("gap", "10px");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Flex));
        assert_eq!(style.flex_direction, Some(Orientation::Vertical));
        assert_eq!(style.align_items, Some(AlignItems::Center));
        assert_eq!(style.justify_content, Some(JustifyContent::SpaceBetween));
        assert_eq!(style.min_width, Some(StyleLength::Points(280.0)));
        assert_eq!(style.gap, Some(StyleLength::Points(10.0)));
        assert_eq!(style.padding.top, Some(StyleLength::Points(8.0)));
        assert_eq!(style.padding.right, Some(StyleLength::Points(8.0)));
        assert_eq!(style.margin.left, Some(StyleLength::Auto));
        assert_eq!(style.margin.right, Some(StyleLength::Auto));
        assert_eq!(style.border_radius, Some(StyleLength::Points(8.0)));
        assert_eq!(style.opacity, Some(0.5));
        assert_eq!(
            style.declarations.get("min-width").map(String::as_str),
            Some("280px")
        );
        assert_eq!(
            style.declarations.get("gap").map(String::as_str),
            Some("10px")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("background-color"))
                .map(String::as_str),
            Some("blue-600")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("flex-direction"))
                .map(String::as_str),
            Some("row")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("outline"))
                .map(String::as_str),
            Some("2px solid blue")
        );
        assert_eq!(
            style.background_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 255,
            })
        );
        assert_eq!(
            style.color,
            Some(StyleColor::Rgba {
                red: 255,
                green: 255,
                blue: 255,
                alpha: 255,
            })
        );
    }

    #[test]
    fn parses_common_tailwind_layout_text_and_border_utilities() {
        let web = WebProps::new().class_name(
            "grid relative inset-x-4 -top-2 z-10 visible flex-wrap gap-x-3 gap-y-5 \
             overflow-x-auto overflow-y-hidden border border-x-2 border-b-[3px] \
             border-dashed border-red-500 text-sm text-center font-semibold leading-tight",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Grid));
        assert_eq!(style.position, Some(PositionMode::Relative));
        assert_eq!(style.inset.left, Some(StyleLength::Points(16.0)));
        assert_eq!(style.inset.right, Some(StyleLength::Points(16.0)));
        assert_eq!(style.inset.top, Some(StyleLength::Points(-8.0)));
        assert_eq!(style.z_index, Some(10));
        assert_eq!(style.visibility, Some(VisibilityMode::Visible));
        assert_eq!(style.flex_wrap, Some(FlexWrap::Wrap));
        assert_eq!(style.column_gap, Some(StyleLength::Points(12.0)));
        assert_eq!(style.row_gap, Some(StyleLength::Points(20.0)));
        assert_eq!(style.overflow_x, Some(OverflowMode::Auto));
        assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
        assert_eq!(style.border_width.top, Some(StyleLength::Points(1.0)));
        assert_eq!(style.border_width.left, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_width.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_width.bottom, Some(StyleLength::Points(3.0)));
        assert_eq!(style.border_style, Some(BorderStyle::Dashed));
        assert_eq!(
            style.border_color,
            Some(StyleColor::Keyword("red-500".to_string()))
        );
        assert_eq!(style.font_size, Some(StyleLength::Points(14.0)));
        assert_eq!(style.line_height, Some(StyleLength::Points(1.25)));
        assert_eq!(style.text_align, Some(TextAlign::Center));
        assert_eq!(style.font_weight, Some(FontWeight::Number(600)));
        assert_eq!(
            style
                .declarations
                .get("border-inline-width")
                .map(String::as_str),
            Some("2px")
        );
        assert_eq!(
            style
                .declarations
                .get("border-bottom-width")
                .map(String::as_str),
            Some("3px")
        );
        assert_eq!(
            style.declarations.get("top").map(String::as_str),
            Some("-8px")
        );
        assert_eq!(
            style.declarations.get("font-size").map(String::as_str),
            Some("0.875rem")
        );
        assert_eq!(
            style.declarations.get("line-height").map(String::as_str),
            Some("1.25")
        );
    }

    #[test]
    fn parses_css_border_side_color_style_and_logical_width_tokens() {
        let web = WebProps::new()
            .style("borderColor", "#111 #222 #333 #444")
            .style("borderStyle", "solid dashed dotted double")
            .style("borderTop", "2px groove #ff0000")
            .style("borderInlineWidth", "3px")
            .style("borderBlockWidth", "4px 5px")
            .style("borderInlineStartColor", "currentColor")
            .style("borderBlockEndStyle", "hidden");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.border_width.top, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_width.left, Some(StyleLength::Points(3.0)));
        assert_eq!(style.border_width.right, Some(StyleLength::Points(3.0)));
        assert_eq!(
            style.logical_border_width.inline_start,
            Some(StyleLength::Points(3.0))
        );
        assert_eq!(
            style.logical_border_width.inline_end,
            Some(StyleLength::Points(3.0))
        );
        assert_eq!(
            style.logical_border_width.block_start,
            Some(StyleLength::Points(4.0))
        );
        assert_eq!(
            style.logical_border_width.block_end,
            Some(StyleLength::Points(5.0))
        );
        assert_eq!(
            style.border_colors.top,
            Some(StyleColor::Rgba {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 255,
            })
        );
        assert_eq!(
            style.border_colors.right,
            Some(StyleColor::Rgba {
                red: 0x22,
                green: 0x22,
                blue: 0x22,
                alpha: 255,
            })
        );
        assert_eq!(
            style.logical_border_colors.inline_start,
            Some(StyleColor::Keyword("currentColor".to_string()))
        );
        assert_eq!(style.border_styles.top, Some(BorderStyle::Groove));
        assert_eq!(style.border_styles.right, Some(BorderStyle::Dashed));
        assert_eq!(style.border_styles.bottom, Some(BorderStyle::Dotted));
        assert_eq!(style.border_styles.left, Some(BorderStyle::Double));
        assert_eq!(
            style.logical_border_styles.block_end,
            Some(BorderStyle::Hidden)
        );
        assert_eq!(
            style
                .declarations
                .get("border-inline-start-color")
                .map(String::as_str),
            Some("currentColor")
        );
        assert!(!style.unsupported.contains_key("border-inline-start-color"));
        assert!(!style.unsupported.contains_key("border-block-end-style"));
    }

    #[test]
    fn parses_tailwind_border_side_color_and_logical_width_utilities() {
        let web = WebProps::new().class_name(
            "border-x-2 border-s-4 border-x-blue-600 border-t-red-500 \
             border-e-[#663399]/50 border-bs-current border-dashed \
             md:border-s-green-500 hover:border-b-(--accent-border)",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.border_width.left, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_width.right, Some(StyleLength::Points(2.0)));
        assert_eq!(
            style.logical_border_width.inline_start,
            Some(StyleLength::Points(4.0))
        );
        assert_eq!(
            style.logical_border_width.inline_end,
            Some(StyleLength::Points(2.0))
        );
        assert_eq!(
            style.border_colors.top,
            Some(StyleColor::Keyword("red-500".to_string()))
        );
        assert_eq!(
            style.border_colors.left,
            Some(StyleColor::Keyword("blue-600".to_string()))
        );
        assert_eq!(
            style.border_colors.right,
            Some(StyleColor::Keyword("blue-600".to_string()))
        );
        assert_eq!(
            style.logical_border_colors.inline_end,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 128,
            })
        );
        assert_eq!(
            style.logical_border_colors.block_start,
            Some(StyleColor::Keyword("currentColor".to_string()))
        );
        assert_eq!(style.border_style, Some(BorderStyle::Dashed));
        assert_eq!(style.border_styles.top, Some(BorderStyle::Dashed));
        assert_eq!(
            style
                .declarations
                .get("border-inline-width")
                .map(String::as_str),
            Some("2px")
        );
        assert_eq!(
            style
                .declarations
                .get("border-inline-start-width")
                .map(String::as_str),
            Some("4px")
        );
        assert_eq!(
            style
                .declarations
                .get("border-inline-end-color")
                .map(String::as_str),
            Some("rgba(102, 51, 153, 0.5)")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("border-inline-start-color"))
                .map(String::as_str),
            Some("green-500")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("border-bottom-color"))
                .map(String::as_str),
            Some("var(--accent-border)")
        );
    }

    #[test]
    fn parses_css_logical_edge_properties_into_portable_tokens() {
        let web = WebProps::new()
            .style("insetBlock", "3px 4px")
            .style("insetInlineStart", "1rem")
            .style("insetInlineEnd", "2rem")
            .style("paddingInline", "10px")
            .style("paddingBlockEnd", "4px")
            .style("marginBlock", "1px 2px")
            .style("marginInlineStart", "auto")
            .style("scrollMarginBlockStart", "5px")
            .style("scrollMarginInline", "6px")
            .style("scrollPaddingBlock", "8px 9px")
            .style("scrollPaddingInlineEnd", "7px");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.logical_inset.block_start,
            Some(StyleLength::Points(3.0))
        );
        assert_eq!(
            style.logical_inset.block_end,
            Some(StyleLength::Points(4.0))
        );
        assert_eq!(
            style.logical_inset.inline_start,
            Some(StyleLength::Points(16.0))
        );
        assert_eq!(
            style.logical_inset.inline_end,
            Some(StyleLength::Points(32.0))
        );
        assert_eq!(
            style.logical_padding.inline_start,
            Some(StyleLength::Points(10.0))
        );
        assert_eq!(
            style.logical_padding.inline_end,
            Some(StyleLength::Points(10.0))
        );
        assert_eq!(
            style.logical_padding.block_end,
            Some(StyleLength::Points(4.0))
        );
        assert_eq!(
            style.logical_margin.block_start,
            Some(StyleLength::Points(1.0))
        );
        assert_eq!(
            style.logical_margin.block_end,
            Some(StyleLength::Points(2.0))
        );
        assert_eq!(style.logical_margin.inline_start, Some(StyleLength::Auto));
        assert_eq!(
            style.logical_scroll_margin.block_start,
            Some(StyleLength::Points(5.0))
        );
        assert_eq!(
            style.logical_scroll_margin.inline_start,
            Some(StyleLength::Points(6.0))
        );
        assert_eq!(
            style.logical_scroll_margin.inline_end,
            Some(StyleLength::Points(6.0))
        );
        assert_eq!(
            style.logical_scroll_padding.block_start,
            Some(StyleLength::Points(8.0))
        );
        assert_eq!(
            style.logical_scroll_padding.block_end,
            Some(StyleLength::Points(9.0))
        );
        assert_eq!(
            style.logical_scroll_padding.inline_end,
            Some(StyleLength::Points(7.0))
        );
        assert_eq!(style.padding.left, Some(StyleLength::Points(10.0)));
        assert_eq!(style.padding.right, Some(StyleLength::Points(10.0)));
        assert_eq!(
            style
                .declarations
                .get("margin-inline-start")
                .map(String::as_str),
            Some("auto")
        );
        assert!(!style.unsupported.contains_key("inset-inline-start"));
        assert!(!style.unsupported.contains_key("padding-block-end"));
    }

    #[test]
    fn parses_tailwind_logical_spacing_and_inset_utilities() {
        let web = WebProps::new().class_name(
            "start-4 end-[2rem] inset-bs-1 inset-be-(--footer) \
             ms-auto me-2 -mbs-1 pbs-3 pie-4 \
             scroll-ms-2 scroll-me-[10px] scroll-pbs-1 scroll-pe-(--snap) \
             md:start-8 hover:ms-[calc(1rem_+_2px)]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.logical_inset.inline_start,
            Some(StyleLength::Points(16.0))
        );
        assert_eq!(
            style.logical_inset.inline_end,
            Some(StyleLength::Points(32.0))
        );
        assert_eq!(
            style.logical_inset.block_start,
            Some(StyleLength::Points(4.0))
        );
        assert_eq!(
            style.logical_inset.block_end,
            Some(StyleLength::Css("var(--footer)".to_string()))
        );
        assert_eq!(style.logical_margin.inline_start, Some(StyleLength::Auto));
        assert_eq!(
            style.logical_margin.inline_end,
            Some(StyleLength::Points(8.0))
        );
        assert_eq!(
            style.logical_margin.block_start,
            Some(StyleLength::Points(-4.0))
        );
        assert_eq!(
            style.logical_padding.block_start,
            Some(StyleLength::Points(12.0))
        );
        assert_eq!(
            style.logical_padding.inline_end,
            Some(StyleLength::Points(16.0))
        );
        assert_eq!(
            style.logical_scroll_margin.inline_start,
            Some(StyleLength::Points(8.0))
        );
        assert_eq!(
            style.logical_scroll_margin.inline_end,
            Some(StyleLength::Points(10.0))
        );
        assert_eq!(
            style.logical_scroll_padding.block_start,
            Some(StyleLength::Points(4.0))
        );
        assert_eq!(
            style.logical_scroll_padding.inline_end,
            Some(StyleLength::Css("var(--snap)".to_string()))
        );
        assert_eq!(
            style
                .declarations
                .get("inset-inline-start")
                .map(String::as_str),
            Some("16px")
        );
        assert_eq!(
            style
                .declarations
                .get("margin-block-start")
                .map(String::as_str),
            Some("-4px")
        );
        assert_eq!(
            style
                .declarations
                .get("scroll-padding-inline-end")
                .map(String::as_str),
            Some("var(--snap)")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("inset-inline-start"))
                .map(String::as_str),
            Some("32px")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("margin-inline-start"))
                .map(String::as_str),
            Some("calc(1rem + 2px)")
        );
    }

    #[test]
    fn parses_css_border_radius_corners_into_portable_tokens() {
        let web = WebProps::new()
            .style("borderRadius", "1px 2px 3px 4px / 5px 6px 7px 8px")
            .style("borderTopLeftRadius", "10px 20px")
            .style("borderStartStartRadius", "12px")
            .style("borderStartEndRadius", "13px")
            .style("borderEndEndRadius", "14px")
            .style("borderEndStartRadius", "15px");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.border_radii.top_left,
            Some(CornerRadius::elliptical(
                StyleLength::Points(10.0),
                StyleLength::Points(20.0)
            ))
        );
        assert_eq!(
            style.border_radii.top_right,
            Some(CornerRadius::elliptical(
                StyleLength::Points(2.0),
                StyleLength::Points(6.0)
            ))
        );
        assert_eq!(
            style.border_radii.bottom_right,
            Some(CornerRadius::elliptical(
                StyleLength::Points(3.0),
                StyleLength::Points(7.0)
            ))
        );
        assert_eq!(
            style.border_radii.bottom_left,
            Some(CornerRadius::elliptical(
                StyleLength::Points(4.0),
                StyleLength::Points(8.0)
            ))
        );
        assert_eq!(
            style.logical_border_radii.start_start,
            Some(CornerRadius::circular(StyleLength::Points(12.0)))
        );
        assert_eq!(
            style.logical_border_radii.start_end,
            Some(CornerRadius::circular(StyleLength::Points(13.0)))
        );
        assert_eq!(
            style.logical_border_radii.end_end,
            Some(CornerRadius::circular(StyleLength::Points(14.0)))
        );
        assert_eq!(
            style.logical_border_radii.end_start,
            Some(CornerRadius::circular(StyleLength::Points(15.0)))
        );
        assert_eq!(
            style
                .declarations
                .get("border-top-left-radius")
                .map(String::as_str),
            Some("10px 20px")
        );
        assert!(!style.unsupported.contains_key("border-top-left-radius"));
        assert!(!style.unsupported.contains_key("border-start-start-radius"));
    }

    #[test]
    fn parses_tailwind_border_radius_corner_utilities() {
        let web = WebProps::new().class_name(
            "rounded-full rounded-t-lg rounded-br-[2rem] rounded-s-sm \
             rounded-ee-(--radius-end) md:rounded-ss-xl \
             hover:rounded-bl-[calc(1rem_+_2px)]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.border_radius,
            Some(StyleLength::Css("calc(infinity * 1px)".to_string()))
        );
        assert_eq!(
            style.border_radii.top_left,
            Some(CornerRadius::circular(StyleLength::Points(8.0)))
        );
        assert_eq!(
            style.border_radii.top_right,
            Some(CornerRadius::circular(StyleLength::Points(8.0)))
        );
        assert_eq!(
            style.border_radii.bottom_right,
            Some(CornerRadius::circular(StyleLength::Points(32.0)))
        );
        assert_eq!(
            style.logical_border_radii.start_start,
            Some(CornerRadius::circular(StyleLength::Points(4.0)))
        );
        assert_eq!(
            style.logical_border_radii.end_start,
            Some(CornerRadius::circular(StyleLength::Points(4.0)))
        );
        assert_eq!(
            style.logical_border_radii.end_end,
            Some(CornerRadius::circular(StyleLength::Css(
                "var(--radius-end)".to_string()
            )))
        );
        assert_eq!(
            style
                .declarations
                .get("border-bottom-right-radius")
                .map(String::as_str),
            Some("32px")
        );
        assert_eq!(
            style
                .declarations
                .get("border-end-end-radius")
                .map(String::as_str),
            Some("var(--radius-end)")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("border-start-start-radius"))
                .map(String::as_str),
            Some("12px")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("border-bottom-left-radius"))
                .map(String::as_str),
            Some("calc(1rem + 2px)")
        );
    }

    #[test]
    fn parses_css_formatting_and_table_layout_properties() {
        let web = WebProps::new()
            .style("boxSizing", "border-box")
            .style("boxDecorationBreak", "clone")
            .style("isolation", "isolate")
            .style("float", "inline-start")
            .style("clear", "both")
            .style("verticalAlign", "text-top")
            .style("tableLayout", "fixed")
            .style("borderCollapse", "separate")
            .style("borderSpacing", "4px 8px")
            .style("captionSide", "bottom");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.box_sizing, Some(BoxSizing::BorderBox));
        assert_eq!(style.box_decoration_break, Some(BoxDecorationBreak::Clone));
        assert_eq!(style.isolation, Some(IsolationMode::Isolate));
        assert_eq!(style.float, Some(FloatMode::InlineStart));
        assert_eq!(style.clear, Some(ClearMode::Both));
        assert_eq!(style.vertical_align.as_deref(), Some("text-top"));
        assert_eq!(style.table_layout, Some(TableLayout::Fixed));
        assert_eq!(style.border_collapse, Some(BorderCollapse::Separate));
        assert_eq!(style.border_spacing.as_deref(), Some("4px 8px"));
        assert_eq!(style.caption_side, Some(CaptionSide::Bottom));
        assert!(!style.unsupported.contains_key("box-sizing"));
        assert!(!style.unsupported.contains_key("table-layout"));
    }

    #[test]
    fn parses_tailwind_formatting_and_table_layout_utilities() {
        let web = WebProps::new().class_name(
            "box-border box-decoration-clone isolate float-start clear-both \
             align-text-bottom table-fixed border-separate border-spacing-x-2 \
             border-spacing-y-4 caption-bottom hover:align-[4px] md:table-auto",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.box_sizing, Some(BoxSizing::BorderBox));
        assert_eq!(style.box_decoration_break, Some(BoxDecorationBreak::Clone));
        assert_eq!(style.isolation, Some(IsolationMode::Isolate));
        assert_eq!(style.float, Some(FloatMode::InlineStart));
        assert_eq!(style.clear, Some(ClearMode::Both));
        assert_eq!(style.vertical_align.as_deref(), Some("text-bottom"));
        assert_eq!(style.table_layout, Some(TableLayout::Fixed));
        assert_eq!(style.border_collapse, Some(BorderCollapse::Separate));
        assert_eq!(style.border_spacing.as_deref(), Some("8px 16px"));
        assert_eq!(style.caption_side, Some(CaptionSide::Bottom));
        assert_eq!(
            style
                .custom_properties
                .get("--tw-border-spacing-x")
                .map(String::as_str),
            Some("8px")
        );
        assert_eq!(
            style
                .custom_properties
                .get("--tw-border-spacing-y")
                .map(String::as_str),
            Some("16px")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("vertical-align"))
                .map(String::as_str),
            Some("4px")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("table-layout"))
                .map(String::as_str),
            Some("auto")
        );
    }

    #[test]
    fn parses_css_blend_mode_properties() {
        let web = WebProps::new()
            .style("mixBlendMode", "plus-lighter")
            .style("backgroundBlendMode", "multiply, screen");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.mix_blend_mode, Some(BlendMode::PlusLighter));
        assert_eq!(
            style.background_blend_mode.as_deref(),
            Some("multiply, screen")
        );
        assert_eq!(
            style.declarations.get("mix-blend-mode").map(String::as_str),
            Some("plus-lighter")
        );
        assert_eq!(
            style
                .declarations
                .get("background-blend-mode")
                .map(String::as_str),
            Some("multiply, screen")
        );
        assert!(!style.unsupported.contains_key("mix-blend-mode"));
        assert!(!style.unsupported.contains_key("background-blend-mode"));
    }

    #[test]
    fn parses_tailwind_blend_mode_utilities() {
        let web = WebProps::new().class_name(
            "mix-blend-color-dodge bg-blend-overlay hover:mix-blend-plus-lighter \
             md:bg-blend-[multiply,_screen] focus:mix-blend-(--blend-mode)",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.mix_blend_mode, Some(BlendMode::ColorDodge));
        assert_eq!(style.background_blend_mode.as_deref(), Some("overlay"));
        assert_eq!(
            style.declarations.get("mix-blend-mode").map(String::as_str),
            Some("color-dodge")
        );
        assert_eq!(
            style
                .declarations
                .get("background-blend-mode")
                .map(String::as_str),
            Some("overlay")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("mix-blend-mode"))
                .map(String::as_str),
            Some("plus-lighter")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("background-blend-mode"))
                .map(String::as_str),
            Some("multiply, screen")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("mix-blend-mode"))
                .map(String::as_str),
            Some("var(--blend-mode)")
        );
    }

    #[test]
    fn parses_css_masking_properties() {
        let web = WebProps::new()
            .style("clipPath", "circle(50% at center)")
            .style("mask", "url(mask.svg) center / contain no-repeat")
            .style("-webkitMaskImage", "linear-gradient(black, transparent)")
            .style("maskMode", "luminance")
            .style("maskRepeat", "no-repeat")
            .style("maskPosition", "center")
            .style("maskSize", "cover")
            .style("maskOrigin", "border-box")
            .style("maskClip", "content-box")
            .style("maskComposite", "exclude")
            .style("maskType", "alpha")
            .style("maskBorder", "url(border.svg) 30 fill / 10px")
            .style("maskBorderSource", "url(border.svg)")
            .style("maskBorderMode", "luminance")
            .style("maskBorderSlice", "30 fill")
            .style("maskBorderWidth", "10px")
            .style("maskBorderOutset", "2px")
            .style("maskBorderRepeat", "round");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.clip_path.as_deref(), Some("circle(50% at center)"));
        assert_eq!(
            style.mask.as_deref(),
            Some("url(mask.svg) center / contain no-repeat")
        );
        assert_eq!(
            style.mask_image.as_deref(),
            Some("linear-gradient(black, transparent)")
        );
        assert_eq!(style.mask_mode.as_deref(), Some("luminance"));
        assert_eq!(style.mask_repeat.as_deref(), Some("no-repeat"));
        assert_eq!(style.mask_position.as_deref(), Some("center"));
        assert_eq!(style.mask_size.as_deref(), Some("cover"));
        assert_eq!(style.mask_origin.as_deref(), Some("border-box"));
        assert_eq!(style.mask_clip.as_deref(), Some("content-box"));
        assert_eq!(style.mask_composite.as_deref(), Some("exclude"));
        assert_eq!(style.mask_type.as_deref(), Some("alpha"));
        assert_eq!(
            style.mask_border.as_deref(),
            Some("url(border.svg) 30 fill / 10px")
        );
        assert_eq!(style.mask_border_source.as_deref(), Some("url(border.svg)"));
        assert_eq!(style.mask_border_mode.as_deref(), Some("luminance"));
        assert_eq!(style.mask_border_slice.as_deref(), Some("30 fill"));
        assert_eq!(style.mask_border_width.as_deref(), Some("10px"));
        assert_eq!(style.mask_border_outset.as_deref(), Some("2px"));
        assert_eq!(style.mask_border_repeat.as_deref(), Some("round"));
        assert!(!style.unsupported.contains_key("clip-path"));
        assert!(!style.unsupported.contains_key("-webkit-mask-image"));
        assert!(!style.unsupported.contains_key("mask-border"));
    }

    #[test]
    fn parses_tailwind_mask_utilities() {
        let web = WebProps::new().class_name(
            "mask-[url(/mask.svg)] mask-cover mask-no-repeat mask-center \
             mask-origin-content mask-clip-padding mask-add mask-alpha \
             mask-type-luminance hover:mask-size-[50%_50%] \
             md:mask-[position:30%_50%,70%_50%] focus:mask-(--mask-image)",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.mask_image.as_deref(), Some("url(/mask.svg)"));
        assert_eq!(style.mask_size.as_deref(), Some("cover"));
        assert_eq!(style.mask_repeat.as_deref(), Some("no-repeat"));
        assert_eq!(style.mask_position.as_deref(), Some("center"));
        assert_eq!(style.mask_origin.as_deref(), Some("content-box"));
        assert_eq!(style.mask_clip.as_deref(), Some("padding-box"));
        assert_eq!(style.mask_composite.as_deref(), Some("add"));
        assert_eq!(style.mask_mode.as_deref(), Some("alpha"));
        assert_eq!(style.mask_type.as_deref(), Some("luminance"));
        assert_eq!(
            style.declarations.get("mask-image").map(String::as_str),
            Some("url(/mask.svg)")
        );
        assert_eq!(
            style.declarations.get("mask-composite").map(String::as_str),
            Some("add")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("mask-size"))
                .map(String::as_str),
            Some("50% 50%")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("mask-position"))
                .map(String::as_str),
            Some("30% 50%,70% 50%")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("mask-image"))
                .map(String::as_str),
            Some("var(--mask-image)")
        );
    }

    #[test]
    fn parses_css_containment_and_container_properties() {
        let web = WebProps::new()
            .style("contain", "layout paint")
            .style("container", "sidebar / inline-size")
            .style("containerType", "size")
            .style("containerName", "main")
            .style("contentVisibility", "auto")
            .style("containIntrinsicSize", "auto 320px")
            .style("containIntrinsicWidth", "240px")
            .style("containIntrinsicHeight", "120px")
            .style("containIntrinsicInlineSize", "50vw")
            .style("containIntrinsicBlockSize", "25vh");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.contain.as_deref(), Some("layout paint"));
        assert_eq!(style.container.as_deref(), Some("sidebar / inline-size"));
        assert_eq!(style.container_type, Some(ContainerType::Size));
        assert_eq!(style.container_name.as_deref(), Some("main"));
        assert_eq!(style.content_visibility, Some(ContentVisibility::Auto));
        assert_eq!(style.contain_intrinsic_size.as_deref(), Some("auto 320px"));
        assert_eq!(style.contain_intrinsic_width.as_deref(), Some("240px"));
        assert_eq!(style.contain_intrinsic_height.as_deref(), Some("120px"));
        assert_eq!(style.contain_intrinsic_inline_size.as_deref(), Some("50vw"));
        assert_eq!(style.contain_intrinsic_block_size.as_deref(), Some("25vh"));
        assert!(!style.unsupported.contains_key("contain"));
        assert!(!style.unsupported.contains_key("container-type"));
        assert!(!style.unsupported.contains_key("content-visibility"));
    }

    #[test]
    fn parses_tailwind_container_query_markers_and_variants() {
        let web = WebProps::new().class_name(
            "@container/sidebar @md:flex @container-size/[detail_panel] \
             hover:@container-normal lg:@container/[wide_panel]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.container_type, Some(ContainerType::Size));
        assert_eq!(style.container_name.as_deref(), Some("detail panel"));
        assert_eq!(
            style.declarations.get("container-type").map(String::as_str),
            Some("size")
        );
        assert_eq!(
            style.declarations.get("container-name").map(String::as_str),
            Some("detail panel")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("@md")
                .and_then(|styles| styles.get("display"))
                .map(String::as_str),
            Some("flex")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("container-type"))
                .map(String::as_str),
            Some("normal")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("lg")
                .and_then(|styles| styles.get("container-type"))
                .map(String::as_str),
            Some("inline-size")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("lg")
                .and_then(|styles| styles.get("container-name"))
                .map(String::as_str),
            Some("wide panel")
        );
    }

    #[test]
    fn parses_css_grid_properties_into_portable_tokens() {
        let web = WebProps::new()
            .style("display", "grid")
            .style("grid", "auto-flow 1fr / 100px")
            .style("gridTemplateColumns", "repeat(3, minmax(0, 1fr))")
            .style("gridTemplateRows", "auto 1fr")
            .style("gridTemplateAreas", "\"header header\" \"nav main\"")
            .style("gridAutoColumns", "minmax(0, 1fr)")
            .style("gridAutoRows", "min-content")
            .style("gridAutoFlow", "column dense")
            .style("gridColumn", "span 2 / span 2")
            .style("gridColumnStart", "1")
            .style("gridColumnEnd", "-1")
            .style("gridRow", "1 / -1")
            .style("gridRowStart", "2")
            .style("gridRowEnd", "4")
            .style("gridArea", "main");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Grid));
        assert_eq!(style.grid.as_deref(), Some("auto-flow 1fr / 100px"));
        assert_eq!(
            style.grid_template_columns.as_deref(),
            Some("repeat(3, minmax(0, 1fr))")
        );
        assert_eq!(style.grid_template_rows.as_deref(), Some("auto 1fr"));
        assert_eq!(
            style.grid_template_areas.as_deref(),
            Some("\"header header\" \"nav main\"")
        );
        assert_eq!(style.grid_auto_columns.as_deref(), Some("minmax(0, 1fr)"));
        assert_eq!(style.grid_auto_rows.as_deref(), Some("min-content"));
        assert_eq!(style.grid_auto_flow, Some(GridAutoFlow::ColumnDense));
        assert_eq!(style.grid_column.as_deref(), Some("span 2 / span 2"));
        assert_eq!(style.grid_column_start.as_deref(), Some("1"));
        assert_eq!(style.grid_column_end.as_deref(), Some("-1"));
        assert_eq!(style.grid_row.as_deref(), Some("1 / -1"));
        assert_eq!(style.grid_row_start.as_deref(), Some("2"));
        assert_eq!(style.grid_row_end.as_deref(), Some("4"));
        assert_eq!(style.grid_area.as_deref(), Some("main"));
        assert!(!style.unsupported.contains_key("grid-template-columns"));
        assert!(!style.unsupported.contains_key("grid-auto-flow"));
    }

    #[test]
    fn parses_tailwind_grid_utilities_into_portable_tokens() {
        let web = WebProps::new().class_name(
            "grid grid-cols-3 grid-rows-[auto_1fr] auto-cols-fr auto-rows-min \
             grid-flow-col-dense col-span-2 -col-start-2 col-end-[-1] \
             row-span-full row-start-2 row-end-4 \
             md:grid-cols-6 hover:col-span-[3]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Grid));
        assert_eq!(
            style.grid_template_columns.as_deref(),
            Some("repeat(3, minmax(0, 1fr))")
        );
        assert_eq!(style.grid_template_rows.as_deref(), Some("auto 1fr"));
        assert_eq!(style.grid_auto_columns.as_deref(), Some("minmax(0, 1fr)"));
        assert_eq!(style.grid_auto_rows.as_deref(), Some("min-content"));
        assert_eq!(style.grid_auto_flow, Some(GridAutoFlow::ColumnDense));
        assert_eq!(style.grid_column.as_deref(), Some("span 2 / span 2"));
        assert_eq!(style.grid_column_start.as_deref(), Some("calc(2 * -1)"));
        assert_eq!(style.grid_column_end.as_deref(), Some("-1"));
        assert_eq!(style.grid_row.as_deref(), Some("1 / -1"));
        assert_eq!(style.grid_row_start.as_deref(), Some("2"));
        assert_eq!(style.grid_row_end.as_deref(), Some("4"));
        assert_eq!(
            style
                .declarations
                .get("grid-template-columns")
                .map(String::as_str),
            Some("repeat(3, minmax(0, 1fr))")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("grid-template-columns"))
                .map(String::as_str),
            Some("repeat(6, minmax(0, 1fr))")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("grid-column"))
                .map(String::as_str),
            Some("span 3 / span 3")
        );
    }

    #[test]
    fn parses_css_flex_item_and_box_alignment_properties() {
        let web = WebProps::new()
            .style("flex", "1")
            .style("flexBasis", "25%")
            .style("flexGrow", "2")
            .style("flexShrink", "0")
            .style("order", "3")
            .style("alignContent", "space-between")
            .style("alignSelf", "stretch")
            .style("justifyItems", "center")
            .style("justifySelf", "end")
            .style("placeContent", "center stretch")
            .style("placeItems", "start")
            .style("placeSelf", "end");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.flex.as_deref(), Some("1"));
        assert_eq!(style.flex_basis, Some(StyleLength::Percent(25.0)));
        assert_eq!(style.flex_grow.as_deref(), Some("2"));
        assert_eq!(style.flex_shrink.as_deref(), Some("0"));
        assert_eq!(style.order.as_deref(), Some("3"));
        assert_eq!(style.align_content, Some(JustifyContent::SpaceBetween));
        assert_eq!(style.align_self, Some(SelfAlignment::Stretch));
        assert_eq!(style.justify_items, Some(AlignItems::Center));
        assert_eq!(style.justify_self, Some(SelfAlignment::End));
        assert_eq!(style.place_content.as_deref(), Some("center stretch"));
        assert_eq!(style.place_items.as_deref(), Some("start"));
        assert_eq!(style.place_self.as_deref(), Some("end"));
        assert!(!style.unsupported.contains_key("flex-basis"));
        assert!(!style.unsupported.contains_key("align-self"));
    }

    #[test]
    fn parses_tailwind_flex_item_and_box_alignment_utilities() {
        let web = WebProps::new().class_name(
            "flex-1 basis-1/2 grow-2 shrink-0 order-first -order-2 \
             content-between self-end justify-items-center justify-self-stretch \
             place-content-evenly place-items-baseline place-self-start \
             md:basis-[calc(50%_-_1rem)] hover:order-[7]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.flex.as_deref(), Some("1"));
        assert_eq!(style.flex_basis, Some(StyleLength::Percent(50.0)));
        assert_eq!(style.flex_grow.as_deref(), Some("2"));
        assert_eq!(style.flex_shrink.as_deref(), Some("0"));
        assert_eq!(style.order.as_deref(), Some("calc(2 * -1)"));
        assert_eq!(style.align_content, Some(JustifyContent::SpaceBetween));
        assert_eq!(style.align_self, Some(SelfAlignment::End));
        assert_eq!(style.justify_items, Some(AlignItems::Center));
        assert_eq!(style.justify_self, Some(SelfAlignment::Stretch));
        assert_eq!(style.place_content.as_deref(), Some("space-evenly"));
        assert_eq!(style.place_items.as_deref(), Some("baseline"));
        assert_eq!(style.place_self.as_deref(), Some("start"));
        assert_eq!(
            style.declarations.get("flex-basis").map(String::as_str),
            Some("50%")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("flex-basis"))
                .map(String::as_str),
            Some("calc(50% - 1rem)")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("order"))
                .map(String::as_str),
            Some("7")
        );
    }

    #[test]
    fn parses_css_typography_text_properties_into_portable_tokens() {
        let web = WebProps::new()
            .style("fontFamily", "ui-monospace, monospace")
            .style("fontStyle", "italic")
            .style("letterSpacing", "0.025em")
            .style("direction", "rtl")
            .style("unicodeBidi", "isolate-override")
            .style("-webkitWritingMode", "vertical-lr")
            .style("textOrientation", "upright")
            .style("textTransform", "uppercase")
            .style("textIndent", "2rem")
            .style("textWrap", "balance")
            .style("lineClamp", "3")
            .style("display", "-webkit-box")
            .style("-webkitBoxOrient", "vertical")
            .style("textDecorationLine", "underline")
            .style("textDecorationColor", "#663399")
            .style("textDecorationStyle", "wavy")
            .style("textDecorationThickness", "from-font")
            .style("textUnderlineOffset", "4px")
            .style("textOverflow", "ellipsis")
            .style("whiteSpace", "nowrap")
            .style("wordBreak", "keep-all")
            .style("overflowWrap", "anywhere")
            .style("hyphens", "auto");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.font_family.as_deref(),
            Some("ui-monospace, monospace")
        );
        assert_eq!(style.font_style, Some(FontStyle::Italic));
        assert_eq!(style.letter_spacing, Some(StyleLength::Points(0.4)));
        assert_eq!(style.direction, Some(TextDirection::Rtl));
        assert_eq!(style.unicode_bidi, Some(UnicodeBidi::IsolateOverride));
        assert_eq!(style.writing_mode, Some(WritingMode::VerticalLr));
        assert_eq!(style.text_orientation, Some(TextOrientation::Upright));
        assert_eq!(style.text_transform, Some(TextTransform::Uppercase));
        assert_eq!(style.text_indent, Some(StyleLength::Points(32.0)));
        assert_eq!(style.text_wrap, Some(TextWrapMode::Balance));
        assert_eq!(style.line_clamp.as_deref(), Some("3"));
        assert_eq!(style.display, Some(DisplayMode::WebkitBox));
        assert_eq!(style.box_orient.as_deref(), Some("vertical"));
        assert_eq!(style.text_decoration_line.as_deref(), Some("underline"));
        assert_eq!(
            style.text_decoration_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 255,
            })
        );
        assert_eq!(style.text_decoration_style, Some(TextDecorationStyle::Wavy));
        assert_eq!(
            style.text_decoration_thickness,
            Some(StyleLength::Css("from-font".to_string()))
        );
        assert_eq!(style.text_underline_offset, Some(StyleLength::Points(4.0)));
        assert_eq!(style.text_overflow, Some(TextOverflow::Ellipsis));
        assert_eq!(style.white_space, Some(WhiteSpaceMode::NoWrap));
        assert_eq!(style.word_break, Some(WordBreakMode::KeepAll));
        assert_eq!(style.overflow_wrap, Some(OverflowWrapMode::Anywhere));
        assert_eq!(style.hyphens, Some(HyphensMode::Auto));
        assert!(!style.unsupported.contains_key("text-decoration-line"));
        assert!(!style.unsupported.contains_key("white-space"));
        assert!(!style.unsupported.contains_key("-webkit-writing-mode"));
        assert!(!style.unsupported.contains_key("text-wrap"));
        assert!(!style.unsupported.contains_key("-webkit-line-clamp"));
    }

    #[test]
    fn parses_tailwind_typography_text_utilities() {
        let web = WebProps::new().class_name(
            "font-mono italic tracking-wide uppercase underline decoration-wavy \
             decoration-[#663399]/50 decoration-2 underline-offset-4 truncate \
             whitespace-pre-wrap break-all hyphens-auto -indent-[2px] text-balance \
             [direction:rtl] [unicode-bidi:isolate] [writing-mode:vertical-rl] \
             [text-orientation:upright] \
             line-clamp-3 md:tracking-[0.2em] hover:decoration-[3px] \
             focus:text-pretty lg:line-clamp-none ltr:[direction:ltr] \
             rtl:[unicode-bidi:plaintext] md:[writing-mode:horizontal-tb] \
             hover:[text-orientation:sideways]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.font_family.as_deref(),
            Some("ui-monospace, monospace")
        );
        assert_eq!(style.font_style, Some(FontStyle::Italic));
        assert_eq!(style.letter_spacing, Some(StyleLength::Points(0.4)));
        assert_eq!(
            style.declarations.get("letter-spacing").map(String::as_str),
            Some("0.025em")
        );
        assert_eq!(style.text_transform, Some(TextTransform::Uppercase));
        assert_eq!(style.direction, Some(TextDirection::Rtl));
        assert_eq!(style.unicode_bidi, Some(UnicodeBidi::Isolate));
        assert_eq!(style.writing_mode, Some(WritingMode::VerticalRl));
        assert_eq!(style.text_orientation, Some(TextOrientation::Upright));
        assert_eq!(style.text_indent, Some(StyleLength::Points(-2.0)));
        assert_eq!(style.text_wrap, Some(TextWrapMode::Balance));
        assert_eq!(style.line_clamp.as_deref(), Some("3"));
        assert_eq!(style.box_orient.as_deref(), Some("vertical"));
        assert_eq!(style.display, Some(DisplayMode::WebkitBox));
        assert_eq!(style.overflow_x, Some(OverflowMode::Hidden));
        assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
        assert_eq!(style.text_decoration_line.as_deref(), Some("underline"));
        assert_eq!(style.text_decoration_style, Some(TextDecorationStyle::Wavy));
        assert_eq!(
            style.text_decoration_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 128,
            })
        );
        assert_eq!(
            style.text_decoration_thickness,
            Some(StyleLength::Points(2.0))
        );
        assert_eq!(style.text_underline_offset, Some(StyleLength::Points(4.0)));
        assert_eq!(style.text_overflow, Some(TextOverflow::Ellipsis));
        assert_eq!(style.overflow_x, Some(OverflowMode::Hidden));
        assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
        assert_eq!(style.white_space, Some(WhiteSpaceMode::PreWrap));
        assert_eq!(style.word_break, Some(WordBreakMode::BreakAll));
        assert_eq!(style.hyphens, Some(HyphensMode::Auto));
        assert_eq!(
            style
                .variant_declarations
                .get("ltr")
                .and_then(|styles| styles.get("direction"))
                .map(String::as_str),
            Some("ltr")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("rtl")
                .and_then(|styles| styles.get("unicode-bidi"))
                .map(String::as_str),
            Some("plaintext")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("letter-spacing"))
                .map(String::as_str),
            Some("0.2em")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("writing-mode"))
                .map(String::as_str),
            Some("horizontal-tb")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("text-decoration-thickness"))
                .map(String::as_str),
            Some("3px")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("text-orientation"))
                .map(String::as_str),
            Some("sideways")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("text-wrap"))
                .map(String::as_str),
            Some("pretty")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("lg")
                .and_then(|styles| styles.get("-webkit-line-clamp"))
                .map(String::as_str),
            Some("unset")
        );
    }

    #[test]
    fn parses_css_background_object_list_and_columns_properties() {
        let web = WebProps::new()
            .style("backgroundImage", "url('/hero.png')")
            .style("backgroundPosition", "center top")
            .style("backgroundSize", "cover")
            .style("backgroundRepeat", "no-repeat")
            .style("backgroundAttachment", "fixed")
            .style("backgroundOrigin", "content-box")
            .style("backgroundClip", "padding-box")
            .style("objectFit", "cover")
            .style("objectPosition", "left bottom")
            .style("listStyleType", "disc")
            .style("listStylePosition", "inside")
            .style("columns", "3")
            .style("columnCount", "2")
            .style("columnWidth", "12rem");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.background_image.as_deref(), Some("url('/hero.png')"));
        assert_eq!(style.background_position.as_deref(), Some("center top"));
        assert_eq!(style.background_size.as_deref(), Some("cover"));
        assert_eq!(style.background_repeat.as_deref(), Some("no-repeat"));
        assert_eq!(
            style.background_attachment,
            Some(BackgroundAttachment::Fixed)
        );
        assert_eq!(style.background_origin, Some(BackgroundBox::ContentBox));
        assert_eq!(style.background_clip, Some(BackgroundBox::PaddingBox));
        assert_eq!(style.object_fit, Some(ObjectFit::Cover));
        assert_eq!(style.object_position.as_deref(), Some("left bottom"));
        assert_eq!(style.list_style_type.as_deref(), Some("disc"));
        assert_eq!(style.list_style_position, Some(ListStylePosition::Inside));
        assert_eq!(style.columns.as_deref(), Some("3"));
        assert_eq!(style.column_count.as_deref(), Some("2"));
        assert_eq!(style.column_width, Some(StyleLength::Points(192.0)));
        assert!(!style.unsupported.contains_key("background-image"));
        assert!(!style.unsupported.contains_key("object-fit"));
    }

    #[test]
    fn parses_tailwind_background_object_list_and_columns_utilities() {
        let web = WebProps::new().class_name(
            "bg-[url('/hero.png')] bg-cover bg-center bg-no-repeat bg-fixed \
             bg-origin-content bg-clip-padding object-cover object-left-bottom \
             list-inside list-disc columns-3 \
             md:bg-[length:50%_auto] hover:object-[25%_75%]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.background_image.as_deref(), Some("url('/hero.png')"));
        assert_eq!(style.background_size.as_deref(), Some("cover"));
        assert_eq!(style.background_position.as_deref(), Some("center"));
        assert_eq!(style.background_repeat.as_deref(), Some("no-repeat"));
        assert_eq!(
            style.background_attachment,
            Some(BackgroundAttachment::Fixed)
        );
        assert_eq!(style.background_origin, Some(BackgroundBox::ContentBox));
        assert_eq!(style.background_clip, Some(BackgroundBox::PaddingBox));
        assert_eq!(style.object_fit, Some(ObjectFit::Cover));
        assert_eq!(style.object_position.as_deref(), Some("left bottom"));
        assert_eq!(style.list_style_position, Some(ListStylePosition::Inside));
        assert_eq!(style.list_style_type.as_deref(), Some("disc"));
        assert_eq!(style.columns.as_deref(), Some("3"));
        assert_eq!(
            style
                .declarations
                .get("background-image")
                .map(String::as_str),
            Some("url('/hero.png')")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("background-size"))
                .map(String::as_str),
            Some("50% auto")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("object-position"))
                .map(String::as_str),
            Some("25% 75%")
        );
    }

    #[test]
    fn preserves_css_length_expressions_as_portable_tokens() {
        let web = WebProps::new()
            .style("width", "calc(100% - 2rem)")
            .style("height", "50dvh")
            .style("minWidth", "min-content")
            .style("maxHeight", "clamp(240px, 50vh, 640px)")
            .style("gap", "var(--space)")
            .style("borderWidth", "fit-content");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.width,
            Some(StyleLength::Css("calc(100% - 2rem)".to_string()))
        );
        assert_eq!(style.height, Some(StyleLength::Css("50dvh".to_string())));
        assert_eq!(
            style.min_width,
            Some(StyleLength::Css("min-content".to_string()))
        );
        assert_eq!(
            style.max_height,
            Some(StyleLength::Css("clamp(240px, 50vh, 640px)".to_string()))
        );
        assert_eq!(
            style.gap,
            Some(StyleLength::Css("var(--space)".to_string()))
        );
        assert_eq!(
            style.border_width.top,
            Some(StyleLength::Css("fit-content".to_string()))
        );
    }

    #[test]
    fn preserves_tailwind_arbitrary_css_length_expressions() {
        let web = WebProps::new().class_name(
            "w-[calc(100%_-_2rem)] h-[50dvh] min-w-[min-content] \
             max-h-[clamp(240px,_50vh,_640px)] gap-[var(--space)]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.width,
            Some(StyleLength::Css("calc(100% - 2rem)".to_string()))
        );
        assert_eq!(style.height, Some(StyleLength::Css("50dvh".to_string())));
        assert_eq!(
            style.min_width,
            Some(StyleLength::Css("min-content".to_string()))
        );
        assert_eq!(
            style.max_height,
            Some(StyleLength::Css("clamp(240px, 50vh, 640px)".to_string()))
        );
        assert_eq!(
            style.gap,
            Some(StyleLength::Css("var(--space)".to_string()))
        );
        assert_eq!(
            style.declarations.get("width").map(String::as_str),
            Some("calc(100% - 2rem)")
        );
    }

    #[test]
    fn parses_css_color_functions_and_alpha_syntax() {
        let web = WebProps::new()
            .style("color", "hsl(210 50% 40% / 50%)")
            .style("backgroundColor", "rgb(10 20 30 / 25%)")
            .style("borderColor", "hsla(120, 100%, 25%, 0.75)");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.color,
            Some(StyleColor::Rgba {
                red: 51,
                green: 102,
                blue: 153,
                alpha: 128,
            })
        );
        assert_eq!(
            style.background_color,
            Some(StyleColor::Rgba {
                red: 10,
                green: 20,
                blue: 30,
                alpha: 64,
            })
        );
        assert_eq!(
            style.border_color,
            Some(StyleColor::Rgba {
                red: 0,
                green: 128,
                blue: 0,
                alpha: 191,
            })
        );
    }

    #[test]
    fn preserves_tailwind_color_opacity_modifiers() {
        let web = WebProps::new()
            .class_name("bg-[#663399]/50 text-white/75 border-blue-600/25 hover:bg-black/40");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.background_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 128,
            })
        );
        assert_eq!(
            style.color,
            Some(StyleColor::Rgba {
                red: 255,
                green: 255,
                blue: 255,
                alpha: 191,
            })
        );
        assert_eq!(
            style.border_color,
            Some(StyleColor::Keyword("blue-600 / 25%".to_string()))
        );
        assert_eq!(
            style
                .declarations
                .get("background-color")
                .map(String::as_str),
            Some("rgba(102, 51, 153, 0.5)")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("background-color"))
                .map(String::as_str),
            Some("rgba(0, 0, 0, 0.4)")
        );
    }

    #[test]
    fn parses_svg_presentation_properties_into_portable_tokens() {
        let web = WebProps::new()
            .style("fill", "#663399")
            .style("fillOpacity", "50%")
            .style("fillRule", "evenodd")
            .style("clipRule", "nonzero")
            .style("stroke", "currentColor")
            .style("strokeWidth", "2")
            .style("strokeLinecap", "round")
            .style("strokeLinejoin", "bevel")
            .style("strokeMiterlimit", "4")
            .style("strokeDasharray", "2 4")
            .style("strokeDashoffset", "1px")
            .style("strokeOpacity", "0.25")
            .style("vectorEffect", "non-scaling-stroke")
            .style("paintOrder", "stroke fill markers")
            .style("shapeRendering", "geometricPrecision")
            .style("textRendering", "optimizeLegibility")
            .style("colorInterpolation", "sRGB")
            .style("colorInterpolationFilters", "linearRGB");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.fill,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 255,
            })
        );
        assert_eq!(style.fill_opacity, Some(0.5));
        assert_eq!(style.fill_rule, Some(FillRule::Evenodd));
        assert_eq!(style.clip_rule, Some(FillRule::Nonzero));
        assert_eq!(
            style.stroke,
            Some(StyleColor::Keyword("currentColor".to_string()))
        );
        assert_eq!(style.stroke_width, Some(StyleLength::Points(2.0)));
        assert_eq!(style.stroke_linecap, Some(StrokeLineCap::Round));
        assert_eq!(style.stroke_linejoin, Some(StrokeLineJoin::Bevel));
        assert_eq!(style.stroke_miterlimit.as_deref(), Some("4"));
        assert_eq!(style.stroke_dasharray.as_deref(), Some("2 4"));
        assert_eq!(style.stroke_dashoffset, Some(StyleLength::Points(1.0)));
        assert_eq!(style.stroke_opacity, Some(0.25));
        assert_eq!(style.vector_effect.as_deref(), Some("non-scaling-stroke"));
        assert_eq!(style.paint_order.as_deref(), Some("stroke fill markers"));
        assert_eq!(style.shape_rendering.as_deref(), Some("geometricPrecision"));
        assert_eq!(style.text_rendering.as_deref(), Some("optimizeLegibility"));
        assert_eq!(style.color_interpolation.as_deref(), Some("sRGB"));
        assert_eq!(
            style.color_interpolation_filters.as_deref(),
            Some("linearRGB")
        );
        assert!(!style.unsupported.contains_key("fill-rule"));
        assert!(!style.unsupported.contains_key("stroke-width"));
    }

    #[test]
    fn parses_tailwind_svg_presentation_utilities() {
        let web = WebProps::new().class_name(
            "fill-[#663399]/50 stroke-current stroke-2 hover:fill-none \
             md:stroke-[3px] focus:stroke-[#ff0000] active:fill-(--icon-fill)",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.fill,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 128,
            })
        );
        assert_eq!(
            style.stroke,
            Some(StyleColor::Keyword("currentColor".to_string()))
        );
        assert_eq!(style.stroke_width, Some(StyleLength::Points(2.0)));
        assert_eq!(
            style.declarations.get("fill").map(String::as_str),
            Some("rgba(102, 51, 153, 0.5)")
        );
        assert_eq!(
            style.declarations.get("stroke-width").map(String::as_str),
            Some("2")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("fill"))
                .map(String::as_str),
            Some("none")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("stroke-width"))
                .map(String::as_str),
            Some("3px")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("stroke"))
                .map(String::as_str),
            Some("#ff0000")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("active")
                .and_then(|styles| styles.get("fill"))
                .map(String::as_str),
            Some("var(--icon-fill)")
        );
    }

    #[test]
    fn parses_css_visual_effect_and_interaction_properties() {
        let web = WebProps::new()
            .style("boxShadow", "0 2px 8px rgb(0 0 0 / 25%)")
            .style("outline", "2px dashed #ff0000")
            .style("outlineOffset", "4px")
            .style("transform", "translateX(4px) rotate(15deg)")
            .style("filter", "blur(4px)")
            .style("backdropFilter", "saturate(150%)")
            .style("aspectRatio", "4 / 3")
            .style("cursor", "pointer")
            .style("pointerEvents", "none")
            .style("userSelect", "text");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.box_shadow.as_deref(),
            Some("0 2px 8px rgb(0 0 0 / 25%)")
        );
        assert_eq!(style.outline_width, Some(StyleLength::Points(2.0)));
        assert_eq!(style.outline_style, Some(BorderStyle::Dashed));
        assert_eq!(
            style.outline_color,
            Some(StyleColor::Rgba {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 255,
            })
        );
        assert_eq!(style.outline_offset, Some(StyleLength::Points(4.0)));
        assert_eq!(
            style.transform.as_deref(),
            Some("translateX(4px) rotate(15deg)")
        );
        assert_eq!(style.filter.as_deref(), Some("blur(4px)"));
        assert_eq!(style.backdrop_filter.as_deref(), Some("saturate(150%)"));
        assert_eq!(style.aspect_ratio.as_deref(), Some("4 / 3"));
        assert_eq!(style.cursor.as_deref(), Some("pointer"));
        assert_eq!(style.pointer_events, Some(PointerEvents::None));
        assert_eq!(style.user_select, Some(UserSelect::Text));
        assert!(!style.unsupported.contains_key("box-shadow"));
    }

    #[test]
    fn parses_tailwind_visual_effect_and_interaction_utilities() {
        let web = WebProps::new().class_name(
            "shadow-lg outline-2 outline-offset-4 outline-blue-600 cursor-pointer \
             pointer-events-none select-none aspect-video filter-none backdrop-filter-none \
             rotate-45 hover:shadow-[0_0_4px_black] focus:outline-[3px_solid_red]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.box_shadow.as_deref(),
            Some("0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)")
        );
        assert_eq!(style.outline_width, Some(StyleLength::Points(2.0)));
        assert_eq!(style.outline_offset, Some(StyleLength::Points(16.0)));
        assert_eq!(
            style.outline_color,
            Some(StyleColor::Keyword("blue-600".to_string()))
        );
        assert_eq!(style.cursor.as_deref(), Some("pointer"));
        assert_eq!(style.pointer_events, Some(PointerEvents::None));
        assert_eq!(style.user_select, Some(UserSelect::None));
        assert_eq!(style.aspect_ratio.as_deref(), Some("16 / 9"));
        assert_eq!(style.filter.as_deref(), Some("none"));
        assert_eq!(style.backdrop_filter.as_deref(), Some("none"));
        assert_eq!(style.rotate.as_deref(), Some("45deg"));
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("box-shadow"))
                .map(String::as_str),
            Some("0 0 4px black")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("outline"))
                .map(String::as_str),
            Some("3px solid red")
        );
    }

    #[test]
    fn parses_composable_tailwind_transform_and_filter_utilities() {
        let web = WebProps::new().class_name(
            "translate-x-4 translate-y-2 scale-x-125 scale-y-75 -rotate-45 \
             rotate-x-12 rotate-y-[35deg] skew-x-6 transform-gpu origin-top-right \
             perspective-near backface-hidden blur-sm brightness-125 contrast-150 \
             grayscale hue-rotate-15 invert-0 saturate-200 sepia drop-shadow-md \
             backdrop-blur-md backdrop-brightness-75 backdrop-contrast-125 \
             backdrop-grayscale backdrop-hue-rotate-30 backdrop-invert \
             backdrop-opacity-50 backdrop-saturate-150 backdrop-sepia \
             hover:blur-[2px] focus:translate-x-[calc(100%_-_1rem)]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.translate.as_deref(), Some("16px 8px"));
        assert_eq!(style.scale.as_deref(), Some("125% 75%"));
        assert_eq!(style.rotate.as_deref(), Some("-45deg"));
        assert_eq!(
            style.transform.as_deref(),
            Some("translateZ(0) rotateX(12deg) rotateY(35deg) skewX(6deg)")
        );
        assert_eq!(style.transform_origin.as_deref(), Some("top right"));
        assert_eq!(style.perspective, Some(StyleLength::Points(300.0)));
        assert_eq!(style.backface_visibility, Some(BackfaceVisibility::Hidden));
        assert_eq!(style.filter_blur.as_deref(), Some("blur(8px)"));
        assert_eq!(style.filter_brightness.as_deref(), Some("brightness(125%)"));
        assert_eq!(style.filter_contrast.as_deref(), Some("contrast(150%)"));
        assert_eq!(style.filter_grayscale.as_deref(), Some("grayscale(100%)"));
        assert_eq!(
            style.filter_hue_rotate.as_deref(),
            Some("hue-rotate(15deg)")
        );
        assert_eq!(style.filter_invert.as_deref(), Some("invert(0%)"));
        assert_eq!(style.filter_saturate.as_deref(), Some("saturate(200%)"));
        assert_eq!(style.filter_sepia.as_deref(), Some("sepia(100%)"));
        assert_eq!(
            style.filter.as_deref(),
            Some("blur(8px) brightness(125%) contrast(150%) drop-shadow(0 3px 3px rgb(0 0 0 / 0.12)) grayscale(100%) hue-rotate(15deg) invert(0%) saturate(200%) sepia(100%)")
        );
        assert_eq!(
            style.backdrop_filter.as_deref(),
            Some("blur(12px) brightness(75%) contrast(125%) grayscale(100%) hue-rotate(30deg) invert(100%) opacity(50%) saturate(150%) sepia(100%)")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("--tw-blur"))
                .map(String::as_str),
            Some("blur(2px)")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("filter"))
                .map(String::as_str),
            Some(tailwind_filter_pipeline().as_str())
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("translate"))
                .map(String::as_str),
            Some("var(--tw-translate-x) var(--tw-translate-y, 0)")
        );
    }

    #[test]
    fn parses_css_motion_scroll_and_interaction_properties() {
        let web = WebProps::new()
            .style("transition", "opacity 200ms ease-in")
            .style("transitionProperty", "opacity")
            .style("transitionDuration", "200ms")
            .style("transitionTimingFunction", "ease-in")
            .style("transitionDelay", "0.25s")
            .style("transitionBehavior", "allow-discrete")
            .style("animation", "fade 1.5s ease-out both")
            .style("animationName", "fade")
            .style("animationDuration", "1.5s")
            .style("animationTimingFunction", "ease-out")
            .style("animationDelay", "var(--delay)")
            .style("animationIterationCount", "infinite")
            .style("animationDirection", "alternate")
            .style("animationFillMode", "both")
            .style("animationPlayState", "running")
            .style("willChange", "transform")
            .style("appearance", "none")
            .style("accentColor", "#663399")
            .style("caretColor", "currentColor")
            .style("resize", "horizontal")
            .style("scrollBehavior", "smooth")
            .style("scrollMargin", "1px 2px")
            .style("scrollMarginTop", "12px")
            .style("scrollPadding", "4px 8px")
            .style("scrollPaddingInline", "10px")
            .style("scrollSnapType", "x mandatory")
            .style("scrollSnapAlign", "center")
            .style("scrollSnapStop", "always")
            .style("overscrollBehavior", "contain")
            .style("overscrollBehaviorX", "none")
            .style("touchAction", "pan-x pinch-zoom");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.transition.as_deref(), Some("opacity 200ms ease-in"));
        assert_eq!(style.transition_property.as_deref(), Some("opacity"));
        assert_eq!(
            style.transition_duration,
            Some(StyleTime::Milliseconds(200.0))
        );
        assert_eq!(style.transition_timing_function.as_deref(), Some("ease-in"));
        assert_eq!(style.transition_delay, Some(StyleTime::Milliseconds(250.0)));
        assert_eq!(style.transition_behavior.as_deref(), Some("allow-discrete"));
        assert_eq!(style.animation.as_deref(), Some("fade 1.5s ease-out both"));
        assert_eq!(style.animation_name.as_deref(), Some("fade"));
        assert_eq!(
            style.animation_duration,
            Some(StyleTime::Milliseconds(1500.0))
        );
        assert_eq!(style.animation_timing_function.as_deref(), Some("ease-out"));
        assert_eq!(
            style.animation_delay,
            Some(StyleTime::Css("var(--delay)".to_string()))
        );
        assert_eq!(style.animation_iteration_count.as_deref(), Some("infinite"));
        assert_eq!(style.animation_direction.as_deref(), Some("alternate"));
        assert_eq!(style.animation_fill_mode.as_deref(), Some("both"));
        assert_eq!(style.animation_play_state.as_deref(), Some("running"));
        assert_eq!(style.will_change.as_deref(), Some("transform"));
        assert_eq!(style.appearance.as_deref(), Some("none"));
        assert_eq!(
            style.accent_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 255,
            })
        );
        assert_eq!(
            style.caret_color,
            Some(StyleColor::Keyword("currentColor".to_string()))
        );
        assert_eq!(style.resize, Some(ResizeMode::Horizontal));
        assert_eq!(style.scroll_behavior, Some(ScrollBehavior::Smooth));
        assert_eq!(style.scroll_margin.top, Some(StyleLength::Points(12.0)));
        assert_eq!(style.scroll_margin.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.scroll_margin.bottom, Some(StyleLength::Points(1.0)));
        assert_eq!(style.scroll_margin.left, Some(StyleLength::Points(2.0)));
        assert_eq!(style.scroll_padding.top, Some(StyleLength::Points(4.0)));
        assert_eq!(style.scroll_padding.right, Some(StyleLength::Points(10.0)));
        assert_eq!(style.scroll_padding.bottom, Some(StyleLength::Points(4.0)));
        assert_eq!(style.scroll_padding.left, Some(StyleLength::Points(10.0)));
        assert_eq!(style.scroll_snap_type.as_deref(), Some("x mandatory"));
        assert_eq!(style.scroll_snap_align.as_deref(), Some("center"));
        assert_eq!(style.scroll_snap_stop.as_deref(), Some("always"));
        assert_eq!(style.overscroll_behavior_x, Some(OverscrollBehavior::None));
        assert_eq!(
            style.overscroll_behavior_y,
            Some(OverscrollBehavior::Contain)
        );
        assert_eq!(style.touch_action.as_deref(), Some("pan-x pinch-zoom"));
        assert!(!style.unsupported.contains_key("transition-duration"));
        assert!(!style.unsupported.contains_key("scroll-snap-type"));
    }

    #[test]
    fn parses_tailwind_motion_scroll_and_interaction_utilities() {
        let web = WebProps::new().class_name(
            "transition-opacity duration-300 delay-75 ease-in-out transition-discrete \
             animate-spin will-change-transform appearance-none accent-[#663399]/50 \
             caret-white resize-y scroll-smooth scroll-mt-4 scroll-px-2 \
             snap-x snap-mandatory snap-center snap-always overscroll-x-contain \
             overscroll-y-none touch-pan-x md:duration-[1s] \
             hover:animate-[wiggle_1s_ease-in-out_infinite] focus:will-change-[opacity]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.transition_property.as_deref(), Some("opacity"));
        assert_eq!(
            style.transition_duration,
            Some(StyleTime::Milliseconds(300.0))
        );
        assert_eq!(style.transition_delay, Some(StyleTime::Milliseconds(75.0)));
        assert_eq!(
            style.transition_timing_function.as_deref(),
            Some("cubic-bezier(0.4, 0, 0.2, 1)")
        );
        assert_eq!(style.transition_behavior.as_deref(), Some("allow-discrete"));
        assert_eq!(style.animation.as_deref(), Some("spin 1s linear infinite"));
        assert_eq!(style.will_change.as_deref(), Some("transform"));
        assert_eq!(style.appearance.as_deref(), Some("none"));
        assert_eq!(
            style.accent_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 128,
            })
        );
        assert_eq!(
            style.caret_color,
            Some(StyleColor::Rgba {
                red: 255,
                green: 255,
                blue: 255,
                alpha: 255,
            })
        );
        assert_eq!(style.resize, Some(ResizeMode::Vertical));
        assert_eq!(style.scroll_behavior, Some(ScrollBehavior::Smooth));
        assert_eq!(style.scroll_margin.top, Some(StyleLength::Points(16.0)));
        assert_eq!(style.scroll_padding.left, Some(StyleLength::Points(8.0)));
        assert_eq!(style.scroll_padding.right, Some(StyleLength::Points(8.0)));
        assert_eq!(
            style.scroll_snap_type.as_deref(),
            Some("x var(--tw-scroll-snap-strictness)")
        );
        assert_eq!(
            style
                .custom_properties
                .get("--tw-scroll-snap-strictness")
                .map(String::as_str),
            Some("mandatory")
        );
        assert_eq!(style.scroll_snap_align.as_deref(), Some("center"));
        assert_eq!(style.scroll_snap_stop.as_deref(), Some("always"));
        assert_eq!(
            style.overscroll_behavior_x,
            Some(OverscrollBehavior::Contain)
        );
        assert_eq!(style.overscroll_behavior_y, Some(OverscrollBehavior::None));
        assert_eq!(style.touch_action.as_deref(), Some("pan-x"));
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("transition-duration"))
                .map(String::as_str),
            Some("1s")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("animation"))
                .map(String::as_str),
            Some("wiggle 1s ease-in-out infinite")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("will-change"))
                .map(String::as_str),
            Some("opacity")
        );
    }
}
