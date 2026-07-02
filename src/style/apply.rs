use super::*;

impl PortableStyle {
    pub(super) fn apply(&mut self, property: &str, value: &str) {
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
}
