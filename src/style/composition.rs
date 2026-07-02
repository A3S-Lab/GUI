use super::*;

impl PortableStyle {
    pub(super) fn apply_tailwind_custom_property(&mut self, property: &str, value: &str) {
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

    pub(super) fn apply_font_variant_numeric(&mut self, value: &str) {
        if is_tailwind_font_variant_numeric_pipeline(value) {
            self.font_variant_numeric = self
                .compose_font_variant_numeric()
                .or_else(|| parse_css_string_token(value));
            return;
        }
        self.font_variant_numeric = parse_css_string_token(value);
    }

    pub(super) fn compose_font_variant_numeric(&self) -> Option<String> {
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

    pub(super) fn apply_transform_property(&mut self, value: &str) {
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

    pub(super) fn resolve_tailwind_translate(&self, value: &str) -> Option<String> {
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

    pub(super) fn resolve_tailwind_rotate(&self, value: &str) -> Option<String> {
        if value == "var(--tw-rotate)" {
            return self.custom_properties.get("--tw-rotate").cloned();
        }
        parse_css_string_token(value)
    }

    pub(super) fn resolve_tailwind_scale(&self, value: &str) -> Option<String> {
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

    pub(super) fn resolve_tailwind_border_spacing(&self, value: &str) -> Option<String> {
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

    pub(super) fn compose_tailwind_transform(&self, gpu: bool) -> Option<String> {
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

    pub(super) fn apply_filter_property(&mut self, value: &str) {
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

    pub(super) fn apply_backdrop_filter_property(&mut self, value: &str) {
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

    pub(super) fn clear_filter_components(&mut self) {
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

    pub(super) fn clear_backdrop_filter_components(&mut self) {
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

    pub(super) fn compose_filter(&self) -> Option<String> {
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

    pub(super) fn compose_backdrop_filter(&self) -> Option<String> {
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

    pub(super) fn compose_tailwind_box_shadow(&self) -> Option<String> {
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

    pub(super) fn compose_tailwind_scrollbar_color(&self) -> Option<String> {
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
}
