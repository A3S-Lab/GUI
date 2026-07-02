use std::collections::BTreeMap;

use crate::geometry::Orientation;
use crate::web::WebProps;
use serde::{Deserialize, Serialize};

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
    pub pattern: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub rows: Option<u32>,
    pub cols: Option<u32>,
    pub size: Option<u32>,
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
            pattern: None,
            min_length: None,
            max_length: None,
            rows: None,
            cols: None,
            size: None,
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
