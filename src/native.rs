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
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub current: Option<f64>,
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
            selected: false,
            checked: None,
            expanded: None,
            orientation: None,
            min: None,
            max: None,
            current: None,
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
