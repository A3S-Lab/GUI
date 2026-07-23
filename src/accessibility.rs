use crate::host::HostNodeId;
use crate::native::{effective_input_type, NativeElement, NativeRole, ValueSensitivity};
use serde::{Deserialize, Serialize};

mod conformance;
mod live_region;
#[cfg(any(
    test,
    all(target_os = "macos", feature = "appkit-native"),
    all(target_os = "linux", feature = "gtk4-native"),
    all(target_os = "windows", feature = "winui-native")
))]
pub(crate) mod relationship_registry;
pub(crate) mod structure;
#[cfg(any(
    test,
    all(target_os = "macos", feature = "appkit-native"),
    all(target_os = "linux", feature = "gtk4-native"),
    all(target_os = "windows", feature = "winui-native")
))]
pub(crate) mod structure_registry;
pub use conformance::{
    AccessibilityConformanceIssue, AccessibilityConformanceReport, AccessibilityIssueCode,
    AccessibilityIssueSeverity,
};
pub(crate) use live_region::{
    accessibility_live_setting, live_region_announces_on_initial_render,
    live_region_implicit_atomic, AccessibilityLiveSetting,
};
#[cfg(test)]
mod conformance_tests;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessibilityAnnouncementPriority {
    #[default]
    Polite,
    Assertive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityAnnouncement {
    pub node: HostNodeId,
    pub message: String,
    pub priority: AccessibilityAnnouncementPriority,
}

impl AccessibilityAnnouncement {
    pub fn new(
        node: HostNodeId,
        message: impl Into<String>,
        priority: AccessibilityAnnouncementPriority,
    ) -> Self {
        Self {
            node,
            message: message.into(),
            priority,
        }
    }

    pub fn assertive(node: HostNodeId, message: impl Into<String>) -> Self {
        Self::new(node, message, AccessibilityAnnouncementPriority::Assertive)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessibilityRole {
    Window,
    Group,
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
    StaticText,
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
    RadioButton,
    Form,
    FieldSet,
    Legend,
    OptionGroup,
    Output,
    Meter,
    ComboBox,
    ListBox,
    ListBoxOption,
    Tree,
    TreeItem,
    Dialog,
    Popover,
    TabGroup,
    TabList,
    Tab,
    TabPanel,
    Menu,
    MenuItem,
    Separator,
    Slider,
    ProgressIndicator,
    Toolbar,
    Table,
    TableSection,
    TableRow,
    TableCell,
    TableColumn,
    TableCaption,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityRelationshipProps {
    pub labelled_by: Option<String>,
    pub described_by: Option<String>,
    pub details: Option<String>,
    pub controls: Option<String>,
    pub owns: Option<String>,
    pub flow_to: Option<String>,
    pub error_message: Option<String>,
    pub active_descendant: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityDescriptionProps {
    pub description: Option<String>,
    pub role_description: Option<String>,
    pub key_shortcuts: Option<String>,
    pub value_text: Option<String>,
}

impl AccessibilityDescriptionProps {
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn role_description(mut self, role_description: impl Into<String>) -> Self {
        self.role_description = Some(role_description.into());
        self
    }

    pub fn key_shortcuts(mut self, key_shortcuts: impl Into<String>) -> Self {
        self.key_shortcuts = Some(key_shortcuts.into());
        self
    }

    pub fn value_text(mut self, value_text: impl Into<String>) -> Self {
        self.value_text = Some(value_text.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityStructureProps {
    pub level: Option<u32>,
    pub position_in_set: Option<i32>,
    pub set_size: Option<i32>,
    pub row_count: Option<i32>,
    pub row_index: Option<i32>,
    pub row_span: Option<u32>,
    pub column_count: Option<i32>,
    pub column_index: Option<i32>,
    pub column_span: Option<u32>,
    pub row_index_text: Option<String>,
    pub column_index_text: Option<String>,
    pub sort: Option<String>,
}

impl AccessibilityStructureProps {
    pub fn level(mut self, level: Option<u32>) -> Self {
        self.level = level;
        self
    }

    pub fn position_in_set(mut self, position_in_set: Option<i32>) -> Self {
        self.position_in_set = position_in_set;
        self
    }

    pub fn set_size(mut self, set_size: Option<i32>) -> Self {
        self.set_size = set_size;
        self
    }

    pub fn row_count(mut self, row_count: Option<i32>) -> Self {
        self.row_count = row_count;
        self
    }

    pub fn row_index(mut self, row_index: Option<i32>) -> Self {
        self.row_index = row_index;
        self
    }

    pub fn row_span(mut self, row_span: Option<u32>) -> Self {
        self.row_span = row_span;
        self
    }

    pub fn column_count(mut self, column_count: Option<i32>) -> Self {
        self.column_count = column_count;
        self
    }

    pub fn column_index(mut self, column_index: Option<i32>) -> Self {
        self.column_index = column_index;
        self
    }

    pub fn column_span(mut self, column_span: Option<u32>) -> Self {
        self.column_span = column_span;
        self
    }

    pub fn row_index_text(mut self, row_index_text: impl Into<String>) -> Self {
        self.row_index_text = Some(row_index_text.into());
        self
    }

    pub fn column_index_text(mut self, column_index_text: impl Into<String>) -> Self {
        self.column_index_text = Some(column_index_text.into());
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityStateProps {
    pub hidden: Option<bool>,
    pub autocomplete: Option<String>,
    pub multiline: Option<bool>,
    pub current: Option<String>,
    pub has_popup: Option<String>,
    pub pressed: Option<String>,
    pub live: Option<String>,
    pub atomic: Option<bool>,
    pub busy: Option<bool>,
    pub relevant: Option<String>,
    pub modal: Option<bool>,
}

impl AccessibilityStateProps {
    pub fn hidden(mut self, hidden: Option<bool>) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn autocomplete(mut self, autocomplete: impl Into<String>) -> Self {
        self.autocomplete = Some(autocomplete.into());
        self
    }

    pub fn multiline(mut self, multiline: Option<bool>) -> Self {
        self.multiline = multiline;
        self
    }

    pub fn current(mut self, current: impl Into<String>) -> Self {
        self.current = Some(current.into());
        self
    }

    pub fn has_popup(mut self, has_popup: impl Into<String>) -> Self {
        self.has_popup = Some(has_popup.into());
        self
    }

    pub fn pressed(mut self, pressed: impl Into<String>) -> Self {
        self.pressed = Some(pressed.into());
        self
    }

    pub fn live(mut self, live: impl Into<String>) -> Self {
        self.live = Some(live.into());
        self
    }

    pub fn atomic(mut self, atomic: Option<bool>) -> Self {
        self.atomic = atomic;
        self
    }

    pub fn busy(mut self, busy: Option<bool>) -> Self {
        self.busy = busy;
        self
    }

    pub fn relevant(mut self, relevant: impl Into<String>) -> Self {
        self.relevant = Some(relevant.into());
        self
    }

    pub fn modal(mut self, modal: Option<bool>) -> Self {
        self.modal = modal;
        self
    }
}

impl AccessibilityRelationshipProps {
    pub fn labelled_by(mut self, labelled_by: impl Into<String>) -> Self {
        self.labelled_by = Some(labelled_by.into());
        self
    }

    pub fn described_by(mut self, described_by: impl Into<String>) -> Self {
        self.described_by = Some(described_by.into());
        self
    }

    pub fn details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn controls(mut self, controls: impl Into<String>) -> Self {
        self.controls = Some(controls.into());
        self
    }

    pub fn owns(mut self, owns: impl Into<String>) -> Self {
        self.owns = Some(owns.into());
        self
    }

    pub fn flow_to(mut self, flow_to: impl Into<String>) -> Self {
        self.flow_to = Some(flow_to.into());
        self
    }

    pub fn error_message(mut self, error_message: impl Into<String>) -> Self {
        self.error_message = Some(error_message.into());
        self
    }

    pub fn active_descendant(mut self, active_descendant: impl Into<String>) -> Self {
        self.active_descendant = Some(active_descendant.into());
        self
    }
}

impl Default for AccessibilityRelationshipProps {
    fn default() -> Self {
        Self {
            labelled_by: None,
            described_by: None,
            details: None,
            controls: None,
            owns: None,
            flow_to: None,
            error_message: None,
            active_descendant: None,
        }
    }
}

#[derive(Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<HostNodeId>,
    pub role: AccessibilityRole,
    pub label: Option<String>,
    pub value: Option<String>,
    #[serde(default)]
    pub value_sensitivity: ValueSensitivity,
    pub relationships: AccessibilityRelationshipProps,
    pub description: AccessibilityDescriptionProps,
    pub structure: AccessibilityStructureProps,
    pub state: AccessibilityStateProps,
    pub disabled: bool,
    pub required: bool,
    pub invalid: bool,
    #[serde(default)]
    pub read_only: bool,
    #[serde(default)]
    pub multiple: bool,
    pub focused: bool,
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    pub children: Vec<AccessibilityNode>,
}

impl std::fmt::Debug for AccessibilityNode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut description = self.description.clone();
        if self.value_sensitivity.is_sensitive() {
            description.value_text = None;
        }
        formatter
            .debug_struct("AccessibilityNode")
            .field("node", &self.node)
            .field("role", &self.role)
            .field("label", &self.label)
            .field(
                "value",
                &self.value_sensitivity.redact(self.value.as_deref()),
            )
            .field("value_sensitivity", &self.value_sensitivity)
            .field("relationships", &self.relationships)
            .field("description", &description)
            .field("structure", &self.structure)
            .field("state", &self.state)
            .field("disabled", &self.disabled)
            .field("required", &self.required)
            .field("invalid", &self.invalid)
            .field("read_only", &self.read_only)
            .field("multiple", &self.multiple)
            .field("focused", &self.focused)
            .field("selected", &self.selected)
            .field("checked", &self.checked)
            .field("expanded", &self.expanded)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AccessibilityNodeWire<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    node: Option<HostNodeId>,
    role: AccessibilityRole,
    label: Option<&'a str>,
    value: Option<&'a str>,
    relationships: &'a AccessibilityRelationshipProps,
    description: AccessibilityDescriptionProps,
    structure: &'a AccessibilityStructureProps,
    state: &'a AccessibilityStateProps,
    disabled: bool,
    required: bool,
    invalid: bool,
    read_only: bool,
    multiple: bool,
    focused: bool,
    selected: bool,
    checked: Option<bool>,
    expanded: Option<bool>,
    children: &'a [AccessibilityNode],
}

impl Serialize for AccessibilityNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut description = self.description.clone();
        if self.value_sensitivity.is_sensitive() {
            description.value_text = None;
        }
        AccessibilityNodeWire {
            node: self.node,
            role: self.role,
            label: self.label.as_deref(),
            value: self.value_sensitivity.redact(self.value.as_deref()),
            relationships: &self.relationships,
            description,
            structure: &self.structure,
            state: &self.state,
            disabled: self.disabled,
            required: self.required,
            invalid: self.invalid,
            read_only: self.read_only,
            multiple: self.multiple,
            focused: self.focused,
            selected: self.selected,
            checked: self.checked,
            expanded: self.expanded,
            children: &self.children,
        }
        .serialize(serializer)
    }
}

pub trait AccessibilityTreeHost {
    fn accessibility_tree(&self) -> Option<AccessibilityNode>;
}

impl AccessibilityNode {
    pub fn from_native(element: &NativeElement) -> Self {
        let value_sensitivity =
            ValueSensitivity::from_input_type(effective_input_type(&element.props));
        let mut description = element.props.accessibility_description.clone();
        if value_sensitivity.is_sensitive() {
            description.value_text = None;
        }
        Self {
            node: None,
            role: accessibility_role(element.role),
            label: element
                .props
                .effective_accessibility_label()
                .map(ToOwned::to_owned),
            value: value_sensitivity
                .redact(element.props.value.as_deref())
                .map(ToOwned::to_owned),
            value_sensitivity,
            relationships: element.props.accessibility_relationships.clone(),
            description,
            structure: element.props.accessibility_structure.clone(),
            state: element.props.accessibility_state.clone(),
            disabled: element.props.disabled,
            required: element.props.required,
            invalid: element.props.invalid,
            read_only: element.props.read_only,
            multiple: element.props.multiple,
            focused: false,
            selected: element.props.selected,
            checked: element.props.checked,
            expanded: element.props.expanded,
            children: element
                .children
                .iter()
                .map(AccessibilityNode::from_native)
                .collect(),
        }
    }
}

pub fn accessibility_role(role: NativeRole) -> AccessibilityRole {
    match role {
        NativeRole::Window => AccessibilityRole::Window,
        NativeRole::View => AccessibilityRole::Group,
        NativeRole::Document => AccessibilityRole::Document,
        NativeRole::DocumentHead => AccessibilityRole::DocumentHead,
        NativeRole::DocumentBody => AccessibilityRole::DocumentBody,
        NativeRole::DocumentTitle => AccessibilityRole::DocumentTitle,
        NativeRole::Metadata => AccessibilityRole::Metadata,
        NativeRole::ResourceLink => AccessibilityRole::ResourceLink,
        NativeRole::StyleSheet => AccessibilityRole::StyleSheet,
        NativeRole::Script => AccessibilityRole::Script,
        NativeRole::Template => AccessibilityRole::Template,
        NativeRole::Slot => AccessibilityRole::Slot,
        NativeRole::Text => AccessibilityRole::StaticText,
        NativeRole::Abbreviation => AccessibilityRole::Abbreviation,
        NativeRole::Citation => AccessibilityRole::Citation,
        NativeRole::Definition => AccessibilityRole::Definition,
        NativeRole::DataValue => AccessibilityRole::DataValue,
        NativeRole::InsertedText => AccessibilityRole::InsertedText,
        NativeRole::DeletedText => AccessibilityRole::DeletedText,
        NativeRole::MarkedText => AccessibilityRole::MarkedText,
        NativeRole::Time => AccessibilityRole::Time,
        NativeRole::Emphasis => AccessibilityRole::Emphasis,
        NativeRole::StrongText => AccessibilityRole::StrongText,
        NativeRole::Code => AccessibilityRole::Code,
        NativeRole::KeyboardInput => AccessibilityRole::KeyboardInput,
        NativeRole::SampleOutput => AccessibilityRole::SampleOutput,
        NativeRole::Variable => AccessibilityRole::Variable,
        NativeRole::InlineQuote => AccessibilityRole::InlineQuote,
        NativeRole::Subscript => AccessibilityRole::Subscript,
        NativeRole::Superscript => AccessibilityRole::Superscript,
        NativeRole::SmallText => AccessibilityRole::SmallText,
        NativeRole::BoldText => AccessibilityRole::BoldText,
        NativeRole::ItalicText => AccessibilityRole::ItalicText,
        NativeRole::StruckText => AccessibilityRole::StruckText,
        NativeRole::UnderlinedText => AccessibilityRole::UnderlinedText,
        NativeRole::BidirectionalIsolate => AccessibilityRole::BidirectionalIsolate,
        NativeRole::BidirectionalOverride => AccessibilityRole::BidirectionalOverride,
        NativeRole::Paragraph => AccessibilityRole::Paragraph,
        NativeRole::PreformattedText => AccessibilityRole::PreformattedText,
        NativeRole::BlockQuote => AccessibilityRole::BlockQuote,
        NativeRole::ContactAddress => AccessibilityRole::ContactAddress,
        NativeRole::LineBreak => AccessibilityRole::LineBreak,
        NativeRole::WordBreakOpportunity => AccessibilityRole::WordBreakOpportunity,
        NativeRole::NoBreakText => AccessibilityRole::NoBreakText,
        NativeRole::CenteredText => AccessibilityRole::CenteredText,
        NativeRole::FontText => AccessibilityRole::FontText,
        NativeRole::BigText => AccessibilityRole::BigText,
        NativeRole::TeletypeText => AccessibilityRole::TeletypeText,
        NativeRole::Applet => AccessibilityRole::Applet,
        NativeRole::BackgroundSound => AccessibilityRole::BackgroundSound,
        NativeRole::Frame => AccessibilityRole::Frame,
        NativeRole::FrameSet => AccessibilityRole::FrameSet,
        NativeRole::NoEmbedFallback => AccessibilityRole::NoEmbedFallback,
        NativeRole::NoFramesFallback => AccessibilityRole::NoFramesFallback,
        NativeRole::Marquee => AccessibilityRole::Marquee,
        NativeRole::Math => AccessibilityRole::Math,
        NativeRole::NextId => AccessibilityRole::NextId,
        NativeRole::SelectedContent => AccessibilityRole::SelectedContent,
        NativeRole::Heading => AccessibilityRole::Heading,
        NativeRole::HeadingGroup => AccessibilityRole::HeadingGroup,
        NativeRole::Ruby => AccessibilityRole::Ruby,
        NativeRole::RubyBase => AccessibilityRole::RubyBase,
        NativeRole::RubyText => AccessibilityRole::RubyText,
        NativeRole::RubyParenthesis => AccessibilityRole::RubyParenthesis,
        NativeRole::RubyTextContainer => AccessibilityRole::RubyTextContainer,
        NativeRole::Main => AccessibilityRole::Main,
        NativeRole::Navigation => AccessibilityRole::Navigation,
        NativeRole::Header => AccessibilityRole::Header,
        NativeRole::Footer => AccessibilityRole::Footer,
        NativeRole::Article => AccessibilityRole::Article,
        NativeRole::Section => AccessibilityRole::Section,
        NativeRole::Aside => AccessibilityRole::Aside,
        NativeRole::Search => AccessibilityRole::Search,
        NativeRole::Disclosure => AccessibilityRole::Disclosure,
        NativeRole::DisclosureSummary => AccessibilityRole::DisclosureSummary,
        NativeRole::Figure => AccessibilityRole::Figure,
        NativeRole::FigureCaption => AccessibilityRole::FigureCaption,
        NativeRole::DescriptionList => AccessibilityRole::DescriptionList,
        NativeRole::DescriptionTerm => AccessibilityRole::DescriptionTerm,
        NativeRole::DescriptionDetails => AccessibilityRole::DescriptionDetails,
        NativeRole::Image => AccessibilityRole::Image,
        NativeRole::Media => AccessibilityRole::Media,
        NativeRole::Canvas => AccessibilityRole::Canvas,
        NativeRole::EmbeddedContent => AccessibilityRole::EmbeddedContent,
        NativeRole::Button => AccessibilityRole::Button,
        NativeRole::Link => AccessibilityRole::Link,
        NativeRole::ImageMap => AccessibilityRole::ImageMap,
        NativeRole::ImageMapArea => AccessibilityRole::ImageMapArea,
        NativeRole::TextField => AccessibilityRole::TextField,
        NativeRole::Checkbox => AccessibilityRole::Checkbox,
        NativeRole::Switch => AccessibilityRole::Switch,
        NativeRole::RadioGroup => AccessibilityRole::RadioGroup,
        NativeRole::Radio => AccessibilityRole::RadioButton,
        NativeRole::Form => AccessibilityRole::Form,
        NativeRole::FieldSet => AccessibilityRole::FieldSet,
        NativeRole::Legend => AccessibilityRole::Legend,
        NativeRole::OptionGroup => AccessibilityRole::OptionGroup,
        NativeRole::Output => AccessibilityRole::Output,
        NativeRole::Meter => AccessibilityRole::Meter,
        NativeRole::Select | NativeRole::ComboBox => AccessibilityRole::ComboBox,
        NativeRole::ListBox => AccessibilityRole::ListBox,
        NativeRole::ListBoxItem => AccessibilityRole::ListBoxOption,
        NativeRole::Tree => AccessibilityRole::Tree,
        NativeRole::TreeItem => AccessibilityRole::TreeItem,
        NativeRole::Dialog => AccessibilityRole::Dialog,
        NativeRole::Popover => AccessibilityRole::Popover,
        NativeRole::Tabs => AccessibilityRole::TabGroup,
        NativeRole::TabList => AccessibilityRole::TabList,
        NativeRole::Tab => AccessibilityRole::Tab,
        NativeRole::TabPanel => AccessibilityRole::TabPanel,
        NativeRole::Menu => AccessibilityRole::Menu,
        NativeRole::MenuItem => AccessibilityRole::MenuItem,
        NativeRole::Separator => AccessibilityRole::Separator,
        NativeRole::Slider => AccessibilityRole::Slider,
        NativeRole::ProgressBar => AccessibilityRole::ProgressIndicator,
        NativeRole::Toolbar => AccessibilityRole::Toolbar,
        NativeRole::Table => AccessibilityRole::Table,
        NativeRole::TableSection => AccessibilityRole::TableSection,
        NativeRole::TableRow => AccessibilityRole::TableRow,
        NativeRole::TableCell => AccessibilityRole::TableCell,
        NativeRole::TableColumn => AccessibilityRole::TableColumn,
        NativeRole::TableCaption => AccessibilityRole::TableCaption,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::{NativeElement, NativeProps, NativeRole};

    #[test]
    fn accessibility_node_defaults_missing_state_flags_to_false() {
        let node: AccessibilityNode = serde_json::from_str(
            r#"
            {
              "role": "listBox",
              "label": "Projects",
              "value": null,
              "relationships": {},
              "description": {},
              "structure": {},
              "state": {},
              "disabled": false,
              "required": false,
              "invalid": false,
              "focused": false,
              "selected": false,
              "checked": null,
              "expanded": null,
              "children": []
            }
            "#,
        )
        .unwrap();

        assert!(!node.read_only);
        assert!(!node.multiple);
    }

    #[test]
    fn accessibility_node_projects_control_state_from_native_props() {
        let element = NativeElement::new("projects", NativeRole::ListBox)
            .with_props(NativeProps::new().read_only(true).multiple(true));

        let node = AccessibilityNode::from_native(&element);

        assert!(node.read_only);
        assert!(node.multiple);
    }

    #[test]
    fn sensitive_accessibility_nodes_are_defensively_redacted_on_serialize() {
        let node: AccessibilityNode = serde_json::from_value(serde_json::json!({
            "role": "textField",
            "label": "Password",
            "value": "accessibility-password-secret",
            "valueSensitivity": "sensitive",
            "relationships": {},
            "description": {"valueText": "described-password-secret"},
            "structure": {},
            "state": {},
            "disabled": false,
            "required": false,
            "invalid": false,
            "focused": true,
            "selected": false,
            "checked": null,
            "expanded": null,
            "children": []
        }))
        .unwrap();

        assert_eq!(node.value.as_deref(), Some("accessibility-password-secret"));
        let wire = serde_json::to_string(&node).unwrap();
        let debug = format!("{node:?}");

        assert!(!wire.contains("accessibility-password-secret"));
        assert!(!wire.contains("described-password-secret"));
        assert!(!wire.contains("valueSensitivity"));
        assert!(!debug.contains("accessibility-password-secret"));
        assert!(!debug.contains("described-password-secret"));
    }
}
