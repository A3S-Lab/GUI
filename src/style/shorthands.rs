use super::*;

impl PortableStyle {
    pub(super) fn apply_container_shorthand(&mut self, value: &str) {
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

    pub(super) fn apply_border_shorthand(&mut self, value: &str) {
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

    pub(super) fn apply_border_side_shorthand(&mut self, edges: EdgeSelection, value: &str) {
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

    pub(super) fn apply_logical_border_side_shorthand(
        &mut self,
        edges: LogicalEdgeSelection,
        value: &str,
    ) {
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

    pub(super) fn apply_outline_shorthand(&mut self, value: &str) {
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

    pub(super) fn apply_background_shorthand(&mut self, value: &str) {
        self.background = parse_css_string_token(value);
        if let Some(color) = parse_background_shorthand_color(value) {
            self.background_color = Some(color);
        }
    }

    pub(super) fn apply_box_shadow_property(&mut self, value: &str) {
        if value.trim() == tailwind_box_shadow_pipeline() {
            self.box_shadow = self
                .compose_tailwind_box_shadow()
                .or_else(|| parse_css_string_token(value));
            return;
        }
        self.box_shadow = parse_css_string_token(value);
    }

    pub(super) fn apply_scrollbar_color_property(&mut self, value: &str) {
        if value.trim() == tailwind_scrollbar_color_pipeline() {
            self.scrollbar_color = self
                .compose_tailwind_scrollbar_color()
                .or_else(|| parse_css_string_token(value));
            return;
        }
        self.scrollbar_color = parse_css_string_token(value);
    }

    pub(super) fn apply_column_rule_shorthand(&mut self, value: &str) {
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

    pub(super) fn apply_text_decoration_shorthand(&mut self, value: &str) {
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

    pub(super) fn apply_text_emphasis_shorthand(&mut self, value: &str) {
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
}
