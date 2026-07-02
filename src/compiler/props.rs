use crate::geometry::Orientation;
use crate::html::{canonical_html_tag, HTML_TAG_METADATA_KEY};
use crate::react_aria::AriaProps;
use crate::svg::{canonical_svg_tag, SVG_TAG_METADATA_KEY};
use crate::web::WebProps;

use super::{CompiledJsxNode, CompiledOrientation, CompiledProps};

mod attributes;
mod controls;
mod global;
mod resources;
mod semantic;
mod states;

use controls::HtmlControlAliases;
use global::HtmlGlobalAliases;
use resources::HtmlResourceAliases;
use semantic::WebSemanticAliases;
use states::{
    has_explicit_textarea_value, html_details_open_state, html_fallback_label,
    html_numeric_value_state, html_placeholder_state, html_range_step_state,
    html_string_value_state, html_textarea_child_value,
};

impl CompiledProps {
    pub(super) fn into_aria_props_for_tag(
        mut self,
        tag: &str,
        children: &[CompiledJsxNode],
    ) -> AriaProps {
        if self.value.is_none() && !has_explicit_textarea_value(&self.attributes) {
            self.value = html_textarea_child_value(tag, children);
        }

        let mut web = WebProps::new();
        if let Some(html_tag) = canonical_html_tag(tag) {
            web = web.attribute(HTML_TAG_METADATA_KEY, html_tag);
        }
        if let Some(svg_tag) = canonical_svg_tag(tag) {
            web = web.attribute(SVG_TAG_METADATA_KEY, svg_tag);
        }
        if let Some(id) = self.id {
            web = web.id(id);
        }
        if let Some(class_name) = self.class_name {
            web = web.class_name(class_name);
        }
        if let Some(label) = self.aria_label {
            web = web.attribute("aria-label", label);
        }
        for (property, value) in self.style {
            web = web.style(property, value.to_portable_value());
        }
        for (name, value) in self.attributes {
            web = web.attribute(name, value);
        }
        for (name, action) in self.events {
            web = web.event(name, action);
        }
        let html_fallback_label = html_fallback_label(tag, &web, self.value.as_deref());
        let html_details_open = html_details_open_state(tag, &web);
        let html_placeholder = html_placeholder_state(tag, &web);
        let html_string_value = html_string_value_state(tag, &web);
        let html_numeric_value = html_numeric_value_state(tag, &web, self.value.as_deref());
        let html_range_step = html_range_step_state(tag, &web);
        let html_control = HtmlControlAliases::from_tag(tag, &web);
        let html_global = HtmlGlobalAliases::from_web(&web);
        let html_resource = HtmlResourceAliases::from_tag(tag, &web);
        let semantic = WebSemanticAliases::from_web(&web);

        let orientation = self.orientation.map(|orientation| match orientation {
            CompiledOrientation::Horizontal => Orientation::Horizontal,
            CompiledOrientation::Vertical => Orientation::Vertical,
        });

        let mut props = AriaProps::new().web(web);
        props.label = self.label.or(html_fallback_label);
        props.text_value = self.text_value;
        props.value = self.value.or(html_string_value);
        props.placeholder = self
            .placeholder
            .or(semantic.placeholder)
            .or(html_placeholder);
        props.action = self.action;
        props.is_disabled = self.is_disabled || semantic.disabled.unwrap_or(false);
        props.is_required = self.is_required || semantic.required.unwrap_or(false);
        props.is_invalid = self.is_invalid || semantic.invalid.unwrap_or(false);
        props.is_read_only = semantic.read_only.unwrap_or(false);
        props.is_multiple = semantic.multiple.unwrap_or(false);
        props.auto_focus = semantic.auto_focus.unwrap_or(false);
        props.is_selected = self.is_selected || semantic.selected.unwrap_or(false);
        props.is_checked = self.is_checked.or(semantic.checked);
        props.is_expanded = self.is_expanded.or(semantic.expanded).or(html_details_open);
        props.orientation = orientation.or(semantic.orientation);
        props.min_value = self.min_value.or(semantic.min_value);
        props.max_value = self.max_value.or(semantic.max_value);
        props.value_number = self
            .value_number
            .or(semantic.value_number)
            .or(html_numeric_value);
        props.step_value = self.step_value.or(semantic.step_value).or(html_range_step);
        props.autocomplete = semantic.autocomplete;
        props.input_mode = semantic.input_mode;
        props.pattern = semantic.pattern;
        props.min_length = semantic.min_length;
        props.max_length = semantic.max_length;
        props.rows = semantic.rows;
        props.cols = semantic.cols;
        props.size = semantic.size;
        props.title = html_global.title;
        props.hidden = html_global.hidden;
        props.lang = html_global.lang;
        props.dir = html_global.dir;
        props.tab_index = html_global.tab_index;
        props.explicit_role = html_global.explicit_role;
        props.access_key = html_global.access_key;
        props.content_editable = html_global.content_editable;
        props.draggable = html_global.draggable;
        props.spell_check = html_global.spell_check;
        props.translate = html_global.translate;
        props.inert = html_global.inert;
        props.popover = html_global.popover;
        props.name = html_control.name;
        props.form = html_control.form;
        props.input_type = html_control.input_type;
        props.accept = html_control.accept;
        props.capture = html_control.capture;
        props.alt = html_control.alt.or(html_resource.alt);
        props.href = html_resource.href;
        props.src = html_control.src.or(html_resource.src);
        props.srcset = html_resource.srcset;
        props.sizes = html_resource.sizes;
        props.media = html_resource.media;
        props.resource_type = html_resource.resource_type;
        props.intrinsic_width = html_resource.intrinsic_width;
        props.intrinsic_height = html_resource.intrinsic_height;
        props.loading = html_resource.loading;
        props.decoding = html_resource.decoding;
        props.fetch_priority = html_resource.fetch_priority;
        props.cross_origin = html_resource.cross_origin;
        props.referrer_policy = html_resource.referrer_policy;
        props.poster = html_resource.poster;
        props.controls = html_resource.controls;
        props.autoplay = html_resource.autoplay;
        props.loop_playback = html_resource.loop_playback;
        props.muted = html_resource.muted;
        props.plays_inline = html_resource.plays_inline;
        props.preload = html_resource.preload;
        props.track_kind = html_resource.track_kind;
        props.srclang = html_resource.srclang;
        props.track_label = html_resource.track_label;
        props.default_track = html_resource.default_track;
        props.list = html_control.list;
        props.dirname = html_control.dirname;
        props.form_action = html_control.form_action;
        props.form_enctype = html_control.form_enctype;
        props.form_method = html_control.form_method;
        props.form_target = html_control.form_target;
        props.form_no_validate = html_control.form_no_validate;
        props
    }
}
