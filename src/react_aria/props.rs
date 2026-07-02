use crate::geometry::Orientation;
use crate::html::{HtmlCollectionProps, HtmlResourcePolicyProps};
use crate::web::WebProps;

mod html_collection;
mod html_resource_policy;

#[derive(Debug, Clone, PartialEq)]
pub struct AriaProps {
    pub label: Option<String>,
    pub text_value: Option<String>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub action: Option<String>,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub is_multiple: bool,
    pub auto_focus: bool,
    pub is_selected: bool,
    pub is_checked: Option<bool>,
    pub is_expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub value_number: Option<f64>,
    pub step_value: Option<f64>,
    pub autocomplete: Option<String>,
    pub input_mode: Option<String>,
    pub pattern: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub rows: Option<u32>,
    pub cols: Option<u32>,
    pub size: Option<u32>,
    pub title: Option<String>,
    pub hidden: bool,
    pub lang: Option<String>,
    pub dir: Option<String>,
    pub tab_index: Option<i32>,
    pub explicit_role: Option<String>,
    pub access_key: Option<String>,
    pub content_editable: Option<String>,
    pub draggable: Option<String>,
    pub spell_check: Option<bool>,
    pub translate: Option<bool>,
    pub inert: bool,
    pub popover: Option<String>,
    pub name: Option<String>,
    pub form: Option<String>,
    pub input_type: Option<String>,
    pub accept: Option<String>,
    pub capture: Option<String>,
    pub alt: Option<String>,
    pub href: Option<String>,
    pub src: Option<String>,
    pub srcset: Option<String>,
    pub sizes: Option<String>,
    pub media: Option<String>,
    pub resource_type: Option<String>,
    pub intrinsic_width: Option<u32>,
    pub intrinsic_height: Option<u32>,
    pub loading: Option<String>,
    pub decoding: Option<String>,
    pub fetch_priority: Option<String>,
    pub cross_origin: Option<String>,
    pub referrer_policy: Option<String>,
    pub poster: Option<String>,
    pub controls: bool,
    pub autoplay: bool,
    pub loop_playback: bool,
    pub muted: bool,
    pub plays_inline: bool,
    pub preload: Option<String>,
    pub track_kind: Option<String>,
    pub srclang: Option<String>,
    pub track_label: Option<String>,
    pub default_track: bool,
    pub list: Option<String>,
    pub dirname: Option<String>,
    pub form_action: Option<String>,
    pub form_enctype: Option<String>,
    pub form_method: Option<String>,
    pub form_target: Option<String>,
    pub form_no_validate: bool,
    pub html_resource_policy: HtmlResourcePolicyProps,
    pub html_collection: HtmlCollectionProps,
    pub web: WebProps,
}

impl Default for AriaProps {
    fn default() -> Self {
        Self {
            label: None,
            text_value: None,
            value: None,
            placeholder: None,
            action: None,
            is_disabled: false,
            is_required: false,
            is_invalid: false,
            is_read_only: false,
            is_multiple: false,
            auto_focus: false,
            is_selected: false,
            is_checked: None,
            is_expanded: None,
            orientation: None,
            min_value: None,
            max_value: None,
            value_number: None,
            step_value: None,
            autocomplete: None,
            input_mode: None,
            pattern: None,
            min_length: None,
            max_length: None,
            rows: None,
            cols: None,
            size: None,
            title: None,
            hidden: false,
            lang: None,
            dir: None,
            tab_index: None,
            explicit_role: None,
            access_key: None,
            content_editable: None,
            draggable: None,
            spell_check: None,
            translate: None,
            inert: false,
            popover: None,
            name: None,
            form: None,
            input_type: None,
            accept: None,
            capture: None,
            alt: None,
            href: None,
            src: None,
            srcset: None,
            sizes: None,
            media: None,
            resource_type: None,
            intrinsic_width: None,
            intrinsic_height: None,
            loading: None,
            decoding: None,
            fetch_priority: None,
            cross_origin: None,
            referrer_policy: None,
            poster: None,
            controls: false,
            autoplay: false,
            loop_playback: false,
            muted: false,
            plays_inline: false,
            preload: None,
            track_kind: None,
            srclang: None,
            track_label: None,
            default_track: false,
            list: None,
            dirname: None,
            form_action: None,
            form_enctype: None,
            form_method: None,
            form_target: None,
            form_no_validate: false,
            html_resource_policy: HtmlResourcePolicyProps::default(),
            html_collection: HtmlCollectionProps::default(),
            web: WebProps::default(),
        }
    }
}

impl AriaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn text_value(mut self, text_value: impl Into<String>) -> Self {
        self.text_value = Some(text_value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.is_required = required;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.is_invalid = invalid;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.is_multiple = multiple;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.is_checked = Some(checked);
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = Some(expanded);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn range(mut self, min: Option<f64>, max: Option<f64>, current: Option<f64>) -> Self {
        self.min_value = min;
        self.max_value = max;
        self.value_number = current;
        self
    }

    pub fn step(mut self, step: Option<f64>) -> Self {
        self.step_value = step;
        self
    }

    pub fn autocomplete(mut self, autocomplete: impl Into<String>) -> Self {
        self.autocomplete = Some(autocomplete.into());
        self
    }

    pub fn input_mode(mut self, input_mode: impl Into<String>) -> Self {
        self.input_mode = Some(input_mode.into());
        self
    }

    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    pub fn min_length(mut self, min_length: Option<u32>) -> Self {
        self.min_length = min_length;
        self
    }

    pub fn max_length(mut self, max_length: Option<u32>) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn rows(mut self, rows: Option<u32>) -> Self {
        self.rows = rows;
        self
    }

    pub fn cols(mut self, cols: Option<u32>) -> Self {
        self.cols = cols;
        self
    }

    pub fn size(mut self, size: Option<u32>) -> Self {
        self.size = size;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }

    pub fn dir(mut self, dir: impl Into<String>) -> Self {
        self.dir = Some(dir.into());
        self
    }

    pub fn tab_index(mut self, tab_index: Option<i32>) -> Self {
        self.tab_index = tab_index;
        self
    }

    pub fn explicit_role(mut self, explicit_role: impl Into<String>) -> Self {
        self.explicit_role = Some(explicit_role.into());
        self
    }

    pub fn access_key(mut self, access_key: impl Into<String>) -> Self {
        self.access_key = Some(access_key.into());
        self
    }

    pub fn content_editable(mut self, content_editable: impl Into<String>) -> Self {
        self.content_editable = Some(content_editable.into());
        self
    }

    pub fn draggable(mut self, draggable: impl Into<String>) -> Self {
        self.draggable = Some(draggable.into());
        self
    }

    pub fn spell_check(mut self, spell_check: Option<bool>) -> Self {
        self.spell_check = spell_check;
        self
    }

    pub fn translate(mut self, translate: Option<bool>) -> Self {
        self.translate = translate;
        self
    }

    pub fn inert(mut self, inert: bool) -> Self {
        self.inert = inert;
        self
    }

    pub fn popover(mut self, popover: impl Into<String>) -> Self {
        self.popover = Some(popover.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn form(mut self, form: impl Into<String>) -> Self {
        self.form = Some(form.into());
        self
    }

    pub fn input_type(mut self, input_type: impl Into<String>) -> Self {
        self.input_type = Some(input_type.into());
        self
    }

    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    pub fn capture(mut self, capture: impl Into<String>) -> Self {
        self.capture = Some(capture.into());
        self
    }

    pub fn alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn src(mut self, src: impl Into<String>) -> Self {
        self.src = Some(src.into());
        self
    }

    pub fn srcset(mut self, srcset: impl Into<String>) -> Self {
        self.srcset = Some(srcset.into());
        self
    }

    pub fn sizes(mut self, sizes: impl Into<String>) -> Self {
        self.sizes = Some(sizes.into());
        self
    }

    pub fn media(mut self, media: impl Into<String>) -> Self {
        self.media = Some(media.into());
        self
    }

    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.resource_type = Some(resource_type.into());
        self
    }

    pub fn intrinsic_width(mut self, intrinsic_width: Option<u32>) -> Self {
        self.intrinsic_width = intrinsic_width;
        self
    }

    pub fn intrinsic_height(mut self, intrinsic_height: Option<u32>) -> Self {
        self.intrinsic_height = intrinsic_height;
        self
    }

    pub fn loading(mut self, loading: impl Into<String>) -> Self {
        self.loading = Some(loading.into());
        self
    }

    pub fn decoding(mut self, decoding: impl Into<String>) -> Self {
        self.decoding = Some(decoding.into());
        self
    }

    pub fn fetch_priority(mut self, fetch_priority: impl Into<String>) -> Self {
        self.fetch_priority = Some(fetch_priority.into());
        self
    }

    pub fn cross_origin(mut self, cross_origin: impl Into<String>) -> Self {
        self.cross_origin = Some(cross_origin.into());
        self
    }

    pub fn referrer_policy(mut self, referrer_policy: impl Into<String>) -> Self {
        self.referrer_policy = Some(referrer_policy.into());
        self
    }

    pub fn poster(mut self, poster: impl Into<String>) -> Self {
        self.poster = Some(poster.into());
        self
    }

    pub fn controls(mut self, controls: bool) -> Self {
        self.controls = controls;
        self
    }

    pub fn autoplay(mut self, autoplay: bool) -> Self {
        self.autoplay = autoplay;
        self
    }

    pub fn loop_playback(mut self, loop_playback: bool) -> Self {
        self.loop_playback = loop_playback;
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn plays_inline(mut self, plays_inline: bool) -> Self {
        self.plays_inline = plays_inline;
        self
    }

    pub fn preload(mut self, preload: impl Into<String>) -> Self {
        self.preload = Some(preload.into());
        self
    }

    pub fn track_kind(mut self, track_kind: impl Into<String>) -> Self {
        self.track_kind = Some(track_kind.into());
        self
    }

    pub fn srclang(mut self, srclang: impl Into<String>) -> Self {
        self.srclang = Some(srclang.into());
        self
    }

    pub fn track_label(mut self, track_label: impl Into<String>) -> Self {
        self.track_label = Some(track_label.into());
        self
    }

    pub fn default_track(mut self, default_track: bool) -> Self {
        self.default_track = default_track;
        self
    }

    pub fn list(mut self, list: impl Into<String>) -> Self {
        self.list = Some(list.into());
        self
    }

    pub fn dirname(mut self, dirname: impl Into<String>) -> Self {
        self.dirname = Some(dirname.into());
        self
    }

    pub fn form_action(mut self, form_action: impl Into<String>) -> Self {
        self.form_action = Some(form_action.into());
        self
    }

    pub fn form_enctype(mut self, form_enctype: impl Into<String>) -> Self {
        self.form_enctype = Some(form_enctype.into());
        self
    }

    pub fn form_method(mut self, form_method: impl Into<String>) -> Self {
        self.form_method = Some(form_method.into());
        self
    }

    pub fn form_target(mut self, form_target: impl Into<String>) -> Self {
        self.form_target = Some(form_target.into());
        self
    }

    pub fn form_no_validate(mut self, form_no_validate: bool) -> Self {
        self.form_no_validate = form_no_validate;
        self
    }

    pub fn dom_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.web = self.web.attribute(name, value);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.web = self.web.id(id);
        self
    }

    pub fn class_name(mut self, class_name: impl Into<String>) -> Self {
        self.web = self.web.class_name(class_name);
        self
    }

    pub fn style(mut self, property: impl Into<String>, value: impl Into<String>) -> Self {
        self.web = self.web.style(property, value);
        self
    }

    pub fn event(mut self, name: impl Into<String>, action: impl Into<String>) -> Self {
        self.web = self.web.event(name, action);
        self
    }

    pub fn on_click(mut self, action: impl Into<String>) -> Self {
        self.web = self.web.on_click(action);
        self
    }

    pub fn on_press(mut self, action: impl Into<String>) -> Self {
        self.web = self.web.on_press(action);
        self
    }

    pub fn on_change(mut self, action: impl Into<String>) -> Self {
        self.web = self.web.on_change(action);
        self
    }

    pub fn on_selection_change(mut self, action: impl Into<String>) -> Self {
        self.web = self.web.on_selection_change(action);
        self
    }

    pub fn web(mut self, web: WebProps) -> Self {
        self.web = web;
        self
    }
}
