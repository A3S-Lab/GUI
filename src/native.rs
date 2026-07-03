use std::collections::BTreeMap;

use crate::accessibility::{
    AccessibilityDescriptionProps, AccessibilityRelationshipProps, AccessibilityStateProps,
    AccessibilityStructureProps,
};
use crate::geometry::Orientation;
use crate::html::{
    HtmlActivationProps, HtmlCollectionProps, HtmlDialogProps, HtmlFormAssociationProps,
    HtmlMicrodataProps, HtmlResourcePolicyProps, HtmlShadowProps, HtmlTextAnnotationProps,
};
use crate::web::WebProps;
use serde::{Deserialize, Serialize};

mod accessibility_description;
mod accessibility_relationships;
mod accessibility_state;
mod accessibility_structure;
mod html_activation;
mod html_collection;
mod html_dialog;
mod html_form_association;
mod html_microdata;
mod html_resource_policy;
mod html_shadow;
mod html_text_annotation;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementKey(String);

impl ElementKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ElementKey {
    fn from(value: &str) -> Self {
        ElementKey::new(value)
    }
}

impl From<String> for ElementKey {
    fn from(value: String) -> Self {
        ElementKey::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeRole {
    Window,
    View,
    Document,
    DocumentHead,
    DocumentBody,
    DocumentTitle,
    Metadata,
    ResourceLink,
    StyleSheet,
    Script,
    Template,
    Slot,
    Text,
    Abbreviation,
    Citation,
    Definition,
    DataValue,
    InsertedText,
    DeletedText,
    MarkedText,
    Time,
    Emphasis,
    StrongText,
    Code,
    KeyboardInput,
    SampleOutput,
    Variable,
    InlineQuote,
    Subscript,
    Superscript,
    SmallText,
    BoldText,
    ItalicText,
    StruckText,
    UnderlinedText,
    BidirectionalIsolate,
    BidirectionalOverride,
    Paragraph,
    PreformattedText,
    BlockQuote,
    ContactAddress,
    LineBreak,
    WordBreakOpportunity,
    NoBreakText,
    CenteredText,
    FontText,
    BigText,
    TeletypeText,
    Applet,
    BackgroundSound,
    Frame,
    FrameSet,
    NoEmbedFallback,
    NoFramesFallback,
    Marquee,
    Math,
    NextId,
    SelectedContent,
    Heading,
    HeadingGroup,
    Ruby,
    RubyBase,
    RubyText,
    RubyParenthesis,
    RubyTextContainer,
    Main,
    Navigation,
    Header,
    Footer,
    Article,
    Section,
    Aside,
    Search,
    Disclosure,
    DisclosureSummary,
    Figure,
    FigureCaption,
    DescriptionList,
    DescriptionTerm,
    DescriptionDetails,
    Image,
    Media,
    Canvas,
    EmbeddedContent,
    Button,
    Link,
    ImageMap,
    ImageMapArea,
    TextField,
    Checkbox,
    Switch,
    RadioGroup,
    Radio,
    Form,
    FieldSet,
    Legend,
    OptionGroup,
    Output,
    Meter,
    Select,
    ComboBox,
    ListBox,
    ListBoxItem,
    Dialog,
    Popover,
    Tabs,
    TabList,
    Tab,
    TabPanel,
    Menu,
    MenuItem,
    Separator,
    Slider,
    ProgressBar,
    Toolbar,
    Table,
    TableSection,
    TableRow,
    TableCell,
    TableColumn,
    TableCaption,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeProps {
    pub label: Option<String>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub action: Option<String>,
    pub disabled: bool,
    pub required: bool,
    pub invalid: bool,
    pub read_only: bool,
    pub multiple: bool,
    pub auto_focus: bool,
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub current: Option<f64>,
    pub step: Option<f64>,
    pub autocomplete: Option<String>,
    pub input_mode: Option<String>,
    pub enter_key_hint: Option<String>,
    pub auto_capitalize: Option<String>,
    pub auto_correct: Option<String>,
    pub virtual_keyboard_policy: Option<String>,
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
    pub anchor: Option<String>,
    pub custom_element_is: Option<String>,
    pub nonce: Option<String>,
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
    pub html_activation: HtmlActivationProps,
    pub html_text_annotation: HtmlTextAnnotationProps,
    pub html_dialog: HtmlDialogProps,
    pub html_shadow: HtmlShadowProps,
    pub html_microdata: HtmlMicrodataProps,
    pub html_form_association: HtmlFormAssociationProps,
    pub html_collection: HtmlCollectionProps,
    pub accessibility_relationships: AccessibilityRelationshipProps,
    pub accessibility_description: AccessibilityDescriptionProps,
    pub accessibility_structure: AccessibilityStructureProps,
    pub accessibility_state: AccessibilityStateProps,
    pub web: WebProps,
    pub metadata: BTreeMap<String, String>,
}

impl Default for NativeProps {
    fn default() -> Self {
        Self {
            label: None,
            value: None,
            placeholder: None,
            action: None,
            disabled: false,
            required: false,
            invalid: false,
            read_only: false,
            multiple: false,
            auto_focus: false,
            selected: false,
            checked: None,
            expanded: None,
            orientation: None,
            min: None,
            max: None,
            current: None,
            step: None,
            autocomplete: None,
            input_mode: None,
            enter_key_hint: None,
            auto_capitalize: None,
            auto_correct: None,
            virtual_keyboard_policy: None,
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
            anchor: None,
            custom_element_is: None,
            nonce: None,
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
            html_activation: HtmlActivationProps::default(),
            html_text_annotation: HtmlTextAnnotationProps::default(),
            html_dialog: HtmlDialogProps::default(),
            html_shadow: HtmlShadowProps::default(),
            html_microdata: HtmlMicrodataProps::default(),
            html_form_association: HtmlFormAssociationProps::default(),
            html_collection: HtmlCollectionProps::default(),
            accessibility_relationships: AccessibilityRelationshipProps::default(),
            accessibility_description: AccessibilityDescriptionProps::default(),
            accessibility_structure: AccessibilityStructureProps::default(),
            accessibility_state: AccessibilityStateProps::default(),
            web: WebProps::default(),
            metadata: BTreeMap::new(),
        }
    }
}

impl NativeProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
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
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn range(mut self, min: Option<f64>, max: Option<f64>, current: Option<f64>) -> Self {
        self.min = min;
        self.max = max;
        self.current = current;
        self
    }

    pub fn step(mut self, step: Option<f64>) -> Self {
        self.step = step;
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

    pub fn enter_key_hint(mut self, enter_key_hint: impl Into<String>) -> Self {
        self.enter_key_hint = Some(enter_key_hint.into());
        self
    }

    pub fn auto_capitalize(mut self, auto_capitalize: impl Into<String>) -> Self {
        self.auto_capitalize = Some(auto_capitalize.into());
        self
    }

    pub fn auto_correct(mut self, auto_correct: impl Into<String>) -> Self {
        self.auto_correct = Some(auto_correct.into());
        self
    }

    pub fn virtual_keyboard_policy(mut self, virtual_keyboard_policy: impl Into<String>) -> Self {
        self.virtual_keyboard_policy = Some(virtual_keyboard_policy.into());
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

    pub fn anchor(mut self, anchor: impl Into<String>) -> Self {
        self.anchor = Some(anchor.into());
        self
    }

    pub fn custom_element_is(mut self, custom_element_is: impl Into<String>) -> Self {
        self.custom_element_is = Some(custom_element_is.into());
        self
    }

    pub fn nonce(mut self, nonce: impl Into<String>) -> Self {
        self.nonce = Some(nonce.into());
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

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn web(mut self, web: WebProps) -> Self {
        self.web = web;
        self
    }
}

pub(crate) fn normalize_props_for_native_role(
    role: NativeRole,
    props: &NativeProps,
) -> NativeProps {
    if !role_has_ranged_value(role, props) {
        return props.clone();
    }

    let mut normalized = props.clone();
    let (min, max) = normalize_range_bounds(props.min, props.max);
    normalized.min = min;
    normalized.max = max;
    normalized.step = normalize_range_step(props.step);
    normalized.current = None;

    let current = props
        .current
        .filter(|value| value.is_finite())
        .or_else(|| props.value.as_deref().and_then(parse_finite_number));
    if let Some(current) =
        current.and_then(|value| normalize_range_value(value, min, max, normalized.step))
    {
        normalized.current = Some(current);
        normalized.value = Some(format_normalized_number(current));
    }

    normalized
}

pub(crate) fn role_has_ranged_value(role: NativeRole, props: &NativeProps) -> bool {
    matches!(
        role,
        NativeRole::Meter | NativeRole::ProgressBar | NativeRole::Slider
    ) || (role == NativeRole::TextField && is_number_input_type(props.input_type.as_deref()))
}

pub(crate) fn is_number_input_type(input_type: Option<&str>) -> bool {
    input_type.is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("number"))
}

pub(crate) fn normalize_range_value(
    value: f64,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
) -> Option<f64> {
    if !value.is_finite() {
        return None;
    }
    let (min, max) = normalize_range_bounds(min, max);
    let value = clamp_range_value(value, min, max);
    let value = snap_range_step_value(value, min, normalize_range_step(step));
    Some(clamp_range_value(value, min, max))
}

pub(crate) fn format_normalized_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn normalize_range_bounds(min: Option<f64>, max: Option<f64>) -> (Option<f64>, Option<f64>) {
    let min = min.filter(|value| value.is_finite());
    let max = max.filter(|value| value.is_finite());
    match (min, max) {
        (Some(min), Some(max)) if max < min => (Some(min), Some(min)),
        bounds => bounds,
    }
}

fn normalize_range_step(step: Option<f64>) -> Option<f64> {
    step.filter(|value| value.is_finite() && *value > 0.0)
}

fn snap_range_step_value(value: f64, min: Option<f64>, step: Option<f64>) -> f64 {
    let Some(step) = step else {
        return value;
    };
    let base = min.unwrap_or(0.0);
    let step_count = ((value - base) / step).round();
    let snapped = base + step_count * step;
    if snapped.is_finite() {
        snapped
    } else {
        value
    }
}

fn clamp_range_value(value: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut value = value;
    if let Some(min) = min {
        value = value.max(min);
    }
    if let Some(max) = max {
        value = value.min(max);
    }
    value
}

fn parse_finite_number(value: &str) -> Option<f64> {
    value.parse::<f64>().ok().filter(|value| value.is_finite())
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeElement {
    pub key: ElementKey,
    pub role: NativeRole,
    pub props: NativeProps,
    pub children: Vec<NativeElement>,
}

impl NativeElement {
    pub fn new(key: impl Into<ElementKey>, role: NativeRole) -> Self {
        Self {
            key: key.into(),
            role,
            props: NativeProps::default(),
            children: Vec::new(),
        }
    }

    pub fn text(key: impl Into<ElementKey>, label: impl Into<String>) -> Self {
        Self::new(key, NativeRole::Text).with_props(NativeProps::new().label(label))
    }

    pub fn with_props(mut self, props: NativeProps) -> Self {
        self.props = props;
        self
    }

    pub fn child(mut self, child: NativeElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = NativeElement>) -> Self {
        self.children.extend(children);
        self
    }
}
