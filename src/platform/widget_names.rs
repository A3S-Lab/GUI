use crate::accessibility::accessibility_role;
use crate::html::HTML_TAG_METADATA_KEY;
use crate::native::{
    normalize_props_for_native_role, NativeElement, NativeProps, NativeRole, ValueSensitivity,
};
use crate::style::{OverflowMode, PortableStyle};

use super::types::{
    NativeBackendKind, NativeContainerKind, NativeControlState, NativeTextInputKind,
    NativeWidgetBlueprint, NativeWidgetKind,
};

pub fn native_widget_name(_backend: NativeBackendKind, _role: NativeRole) -> &'static str {
    "a3s_gui::HeadlessNode"
}

pub fn widget_blueprint(
    backend: NativeBackendKind,
    element: &NativeElement,
) -> NativeWidgetBlueprint {
    let props = normalize_props_for_native_role(element.role, &element.props);
    let mut metadata = props.metadata.clone();
    if element.role == NativeRole::Popover {
        if let Some(open) = props.web.attributes.get("data-open") {
            metadata.insert("data-open".to_string(), open.clone());
        }
    }
    NativeWidgetBlueprint {
        backend,
        widget_kind: native_widget_kind(element.role, &props),
        widget_class: native_widget_name_for_props(backend, element.role, &props).to_string(),
        role: element.role,
        accessibility_role: accessibility_role(element.role),
        label: props.label.clone(),
        accessibility_label: props.effective_accessibility_label().map(ToOwned::to_owned),
        value: props.value.clone(),
        value_sensitivity: ValueSensitivity::from_input_type(props.input_type.as_deref()),
        action: props
            .action
            .clone()
            .or_else(|| props.web.primary_action().map(str::to_string)),
        class_name: props.web.class_name.clone(),
        control_state: NativeControlState::from_props(&props),
        style: props.web.style.clone(),
        portable_style: PortableStyle::from_web(&props.web),
        events: props.web.events.clone(),
        metadata,
    }
}

pub fn native_widget_kind(role: NativeRole, props: &NativeProps) -> NativeWidgetKind {
    if scrollable_container(role, props) {
        return NativeWidgetKind::ScrollContainer;
    }

    match role {
        NativeRole::Window => NativeWidgetKind::Window,
        NativeRole::DocumentTitle
        | NativeRole::Text
        | NativeRole::Abbreviation
        | NativeRole::Citation
        | NativeRole::Definition
        | NativeRole::DataValue
        | NativeRole::InsertedText
        | NativeRole::DeletedText
        | NativeRole::MarkedText
        | NativeRole::Time
        | NativeRole::Emphasis
        | NativeRole::StrongText
        | NativeRole::Code
        | NativeRole::KeyboardInput
        | NativeRole::SampleOutput
        | NativeRole::Variable
        | NativeRole::InlineQuote
        | NativeRole::Subscript
        | NativeRole::Superscript
        | NativeRole::SmallText
        | NativeRole::BoldText
        | NativeRole::ItalicText
        | NativeRole::StruckText
        | NativeRole::UnderlinedText
        | NativeRole::BidirectionalIsolate
        | NativeRole::BidirectionalOverride
        | NativeRole::LineBreak
        | NativeRole::WordBreakOpportunity
        | NativeRole::Heading
        | NativeRole::RubyBase
        | NativeRole::RubyText
        | NativeRole::RubyParenthesis
        | NativeRole::FigureCaption
        | NativeRole::DescriptionTerm
        | NativeRole::Legend
        | NativeRole::Output
        | NativeRole::TableCaption => NativeWidgetKind::Label,
        NativeRole::Button
        | NativeRole::Link
        | NativeRole::ImageMapArea
        | NativeRole::DisclosureSummary => NativeWidgetKind::Button,
        NativeRole::TextField => NativeWidgetKind::TextInput(text_input_kind(props)),
        NativeRole::Checkbox => NativeWidgetKind::Checkbox,
        NativeRole::Switch => NativeWidgetKind::Switch,
        NativeRole::RadioGroup => NativeWidgetKind::RadioGroup,
        NativeRole::Radio => NativeWidgetKind::Radio,
        NativeRole::Select | NativeRole::ComboBox => NativeWidgetKind::ComboBox,
        NativeRole::ListBox => NativeWidgetKind::List,
        NativeRole::ListBoxItem => NativeWidgetKind::ListItem,
        NativeRole::Tree => NativeWidgetKind::Tree,
        NativeRole::TreeItem => NativeWidgetKind::TreeItem,
        NativeRole::Table => NativeWidgetKind::Table,
        NativeRole::Dialog => NativeWidgetKind::Dialog,
        NativeRole::Popover => NativeWidgetKind::Popover,
        NativeRole::Tabs | NativeRole::TabList => NativeWidgetKind::Tabs,
        NativeRole::Tab => NativeWidgetKind::Tab,
        NativeRole::Menu => NativeWidgetKind::Menu,
        NativeRole::MenuItem => NativeWidgetKind::MenuItem,
        NativeRole::Separator => NativeWidgetKind::Separator,
        NativeRole::Slider => NativeWidgetKind::Slider,
        NativeRole::ProgressBar | NativeRole::Meter => NativeWidgetKind::Progress,
        NativeRole::Toolbar => NativeWidgetKind::Toolbar,
        NativeRole::Image => NativeWidgetKind::Image,
        NativeRole::Media => NativeWidgetKind::Media,
        NativeRole::Canvas | NativeRole::ImageMap => {
            NativeWidgetKind::Container(NativeContainerKind::Canvas)
        }
        NativeRole::EmbeddedContent => NativeWidgetKind::Container(NativeContainerKind::Embedded),
        NativeRole::TabPanel
        | NativeRole::TableSection
        | NativeRole::TableRow
        | NativeRole::TableCell
        | NativeRole::TableColumn => NativeWidgetKind::Container(NativeContainerKind::Grid),
        _ => NativeWidgetKind::Container(NativeContainerKind::Linear),
    }
}

fn text_input_kind(props: &NativeProps) -> NativeTextInputKind {
    if props
        .metadata
        .get(HTML_TAG_METADATA_KEY)
        .is_some_and(|tag| tag == "textarea")
    {
        return NativeTextInputKind::Multiline;
    }
    match props.input_type.as_deref().map(str::trim) {
        Some(value) if value.eq_ignore_ascii_case("search") => NativeTextInputKind::Search,
        Some(value) if value.eq_ignore_ascii_case("number") => NativeTextInputKind::Number,
        Some(value) if value.eq_ignore_ascii_case("password") => NativeTextInputKind::Password,
        _ => NativeTextInputKind::SingleLine,
    }
}

fn native_widget_name_for_props(
    backend: NativeBackendKind,
    role: NativeRole,
    _props: &NativeProps,
) -> &'static str {
    native_widget_name(backend, role)
}

fn scrollable_container(role: NativeRole, props: &NativeProps) -> bool {
    if !scrollable_container_role(role) {
        return false;
    }
    let style = PortableStyle::from_web(&props.web);
    scrollable_overflow(style.overflow_y)
        || scrollable_overflow(style.overflow_x)
        || scrollable_overflow(style.overflow_block)
        || scrollable_overflow(style.overflow_inline)
}

fn scrollable_container_role(role: NativeRole) -> bool {
    matches!(
        role,
        NativeRole::View
            | NativeRole::Document
            | NativeRole::DocumentBody
            | NativeRole::Paragraph
            | NativeRole::PreformattedText
            | NativeRole::BlockQuote
            | NativeRole::Main
            | NativeRole::Navigation
            | NativeRole::Header
            | NativeRole::Footer
            | NativeRole::Article
            | NativeRole::Section
            | NativeRole::Aside
            | NativeRole::Search
            | NativeRole::Disclosure
            | NativeRole::Figure
            | NativeRole::DescriptionList
            | NativeRole::Form
            | NativeRole::FieldSet
            | NativeRole::OptionGroup
            | NativeRole::TabPanel
            | NativeRole::Toolbar
            | NativeRole::TableSection
    )
}

fn scrollable_overflow(value: Option<OverflowMode>) -> bool {
    matches!(value, Some(OverflowMode::Auto | OverflowMode::Scroll))
}
