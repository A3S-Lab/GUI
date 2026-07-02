use crate::geometry::Orientation;
use crate::web::WebProps;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaComponent {
    Button,
    Label,
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
    Link,
    ImageMap,
    ImageMapArea,
    TextField,
    Input,
    Checkbox,
    Switch,
    RadioGroup,
    Radio,
    FieldSet,
    Legend,
    OptionGroup,
    Output,
    Meter,
    Select,
    SelectValue,
    ListBox,
    ListBoxItem,
    Dialog,
    Popover,
    Tabs,
    TabList,
    Tab,
    TabPanel,
    Group,
    Form,
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

impl AriaComponent {
    pub const fn as_str(self) -> &'static str {
        match self {
            AriaComponent::Button => "Button",
            AriaComponent::Label => "Label",
            AriaComponent::Document => "Document",
            AriaComponent::DocumentHead => "DocumentHead",
            AriaComponent::DocumentBody => "DocumentBody",
            AriaComponent::DocumentTitle => "DocumentTitle",
            AriaComponent::Metadata => "Metadata",
            AriaComponent::ResourceLink => "ResourceLink",
            AriaComponent::StyleSheet => "StyleSheet",
            AriaComponent::Script => "Script",
            AriaComponent::Template => "Template",
            AriaComponent::Slot => "Slot",
            AriaComponent::Text => "Text",
            AriaComponent::Abbreviation => "Abbreviation",
            AriaComponent::Citation => "Citation",
            AriaComponent::Definition => "Definition",
            AriaComponent::DataValue => "DataValue",
            AriaComponent::InsertedText => "InsertedText",
            AriaComponent::DeletedText => "DeletedText",
            AriaComponent::MarkedText => "MarkedText",
            AriaComponent::Time => "Time",
            AriaComponent::Emphasis => "Emphasis",
            AriaComponent::StrongText => "StrongText",
            AriaComponent::Code => "Code",
            AriaComponent::KeyboardInput => "KeyboardInput",
            AriaComponent::SampleOutput => "SampleOutput",
            AriaComponent::Variable => "Variable",
            AriaComponent::InlineQuote => "InlineQuote",
            AriaComponent::Subscript => "Subscript",
            AriaComponent::Superscript => "Superscript",
            AriaComponent::SmallText => "SmallText",
            AriaComponent::BoldText => "BoldText",
            AriaComponent::ItalicText => "ItalicText",
            AriaComponent::StruckText => "StruckText",
            AriaComponent::UnderlinedText => "UnderlinedText",
            AriaComponent::BidirectionalIsolate => "BidirectionalIsolate",
            AriaComponent::BidirectionalOverride => "BidirectionalOverride",
            AriaComponent::Paragraph => "Paragraph",
            AriaComponent::PreformattedText => "PreformattedText",
            AriaComponent::BlockQuote => "BlockQuote",
            AriaComponent::ContactAddress => "ContactAddress",
            AriaComponent::LineBreak => "LineBreak",
            AriaComponent::WordBreakOpportunity => "WordBreakOpportunity",
            AriaComponent::NoBreakText => "NoBreakText",
            AriaComponent::CenteredText => "CenteredText",
            AriaComponent::FontText => "FontText",
            AriaComponent::BigText => "BigText",
            AriaComponent::TeletypeText => "TeletypeText",
            AriaComponent::Applet => "Applet",
            AriaComponent::BackgroundSound => "BackgroundSound",
            AriaComponent::Frame => "Frame",
            AriaComponent::FrameSet => "FrameSet",
            AriaComponent::NoEmbedFallback => "NoEmbedFallback",
            AriaComponent::NoFramesFallback => "NoFramesFallback",
            AriaComponent::Marquee => "Marquee",
            AriaComponent::Math => "Math",
            AriaComponent::NextId => "NextId",
            AriaComponent::SelectedContent => "SelectedContent",
            AriaComponent::Heading => "Heading",
            AriaComponent::HeadingGroup => "HeadingGroup",
            AriaComponent::Ruby => "Ruby",
            AriaComponent::RubyBase => "RubyBase",
            AriaComponent::RubyText => "RubyText",
            AriaComponent::RubyParenthesis => "RubyParenthesis",
            AriaComponent::RubyTextContainer => "RubyTextContainer",
            AriaComponent::Main => "Main",
            AriaComponent::Navigation => "Navigation",
            AriaComponent::Header => "Header",
            AriaComponent::Footer => "Footer",
            AriaComponent::Article => "Article",
            AriaComponent::Section => "Section",
            AriaComponent::Aside => "Aside",
            AriaComponent::Search => "Search",
            AriaComponent::Disclosure => "Disclosure",
            AriaComponent::DisclosureSummary => "DisclosureSummary",
            AriaComponent::Figure => "Figure",
            AriaComponent::FigureCaption => "FigureCaption",
            AriaComponent::DescriptionList => "DescriptionList",
            AriaComponent::DescriptionTerm => "DescriptionTerm",
            AriaComponent::DescriptionDetails => "DescriptionDetails",
            AriaComponent::Image => "Image",
            AriaComponent::Media => "Media",
            AriaComponent::Canvas => "Canvas",
            AriaComponent::EmbeddedContent => "EmbeddedContent",
            AriaComponent::Link => "Link",
            AriaComponent::ImageMap => "ImageMap",
            AriaComponent::ImageMapArea => "ImageMapArea",
            AriaComponent::TextField => "TextField",
            AriaComponent::Input => "Input",
            AriaComponent::Checkbox => "Checkbox",
            AriaComponent::Switch => "Switch",
            AriaComponent::RadioGroup => "RadioGroup",
            AriaComponent::Radio => "Radio",
            AriaComponent::FieldSet => "FieldSet",
            AriaComponent::Legend => "Legend",
            AriaComponent::OptionGroup => "OptionGroup",
            AriaComponent::Output => "Output",
            AriaComponent::Meter => "Meter",
            AriaComponent::Select => "Select",
            AriaComponent::SelectValue => "SelectValue",
            AriaComponent::ListBox => "ListBox",
            AriaComponent::ListBoxItem => "ListBoxItem",
            AriaComponent::Dialog => "Dialog",
            AriaComponent::Popover => "Popover",
            AriaComponent::Tabs => "Tabs",
            AriaComponent::TabList => "TabList",
            AriaComponent::Tab => "Tab",
            AriaComponent::TabPanel => "TabPanel",
            AriaComponent::Group => "Group",
            AriaComponent::Form => "Form",
            AriaComponent::Menu => "Menu",
            AriaComponent::MenuItem => "MenuItem",
            AriaComponent::Separator => "Separator",
            AriaComponent::Slider => "Slider",
            AriaComponent::ProgressBar => "ProgressBar",
            AriaComponent::Toolbar => "Toolbar",
            AriaComponent::Table => "Table",
            AriaComponent::TableSection => "TableSection",
            AriaComponent::TableRow => "TableRow",
            AriaComponent::TableCell => "TableCell",
            AriaComponent::TableColumn => "TableColumn",
            AriaComponent::TableCaption => "TableCaption",
        }
    }
}

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
    pub is_selected: bool,
    pub is_checked: Option<bool>,
    pub is_expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub value_number: Option<f64>,
    pub step_value: Option<f64>,
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
            is_selected: false,
            is_checked: None,
            is_expanded: None,
            orientation: None,
            min_value: None,
            max_value: None,
            value_number: None,
            step_value: None,
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

#[derive(Debug, Clone, PartialEq)]
pub struct AriaElement {
    pub key: String,
    pub component: AriaComponent,
    pub props: AriaProps,
    pub children: Vec<AriaElement>,
}

impl AriaElement {
    pub fn new(key: impl Into<String>, component: AriaComponent) -> Self {
        Self {
            key: key.into(),
            component,
            props: AriaProps::default(),
            children: Vec::new(),
        }
    }

    pub fn text(key: impl Into<String>, text: impl Into<String>) -> Self {
        Self::new(key, AriaComponent::Text).with_props(AriaProps::new().text_value(text))
    }

    pub fn with_props(mut self, props: AriaProps) -> Self {
        self.props = props;
        self
    }

    pub fn child(mut self, child: AriaElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = AriaElement>) -> Self {
        self.children.extend(children);
        self
    }
}
