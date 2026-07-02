use crate::error::GuiResult;
use crate::html::HTML_TAG_METADATA_KEY;
use crate::native::{NativeElement, NativeProps, NativeRole};

use super::{AriaComponent, AriaElement, AriaProps};

#[derive(Debug, Clone)]
pub struct ReactAriaMapper;

impl Default for ReactAriaMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl ReactAriaMapper {
    pub fn new() -> Self {
        Self
    }

    pub fn map(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        self.map_element(element)
    }

    fn map_element(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        match element.component {
            AriaComponent::Button => Ok(simple_leaf(
                element,
                NativeRole::Button,
                self.best_label(element)?,
            )),
            AriaComponent::Label | AriaComponent::SelectValue => Ok(simple_leaf(
                element,
                NativeRole::Text,
                self.best_label(element)?,
            )),
            AriaComponent::Text => self.map_text(element),
            AriaComponent::Abbreviation => Ok(simple_leaf(
                element,
                NativeRole::Abbreviation,
                self.best_label(element)?,
            )),
            AriaComponent::Citation => Ok(simple_leaf(
                element,
                NativeRole::Citation,
                self.best_label(element)?,
            )),
            AriaComponent::Definition => Ok(simple_leaf(
                element,
                NativeRole::Definition,
                self.best_label(element)?,
            )),
            AriaComponent::DataValue => Ok(simple_leaf(
                element,
                NativeRole::DataValue,
                self.best_label(element)?,
            )),
            AriaComponent::InsertedText => Ok(simple_leaf(
                element,
                NativeRole::InsertedText,
                self.best_label(element)?,
            )),
            AriaComponent::DeletedText => Ok(simple_leaf(
                element,
                NativeRole::DeletedText,
                self.best_label(element)?,
            )),
            AriaComponent::MarkedText => Ok(simple_leaf(
                element,
                NativeRole::MarkedText,
                self.best_label(element)?,
            )),
            AriaComponent::Time => Ok(simple_leaf(
                element,
                NativeRole::Time,
                self.best_label(element)?,
            )),
            AriaComponent::Emphasis => Ok(simple_leaf(
                element,
                NativeRole::Emphasis,
                self.best_label(element)?,
            )),
            AriaComponent::StrongText => Ok(simple_leaf(
                element,
                NativeRole::StrongText,
                self.best_label(element)?,
            )),
            AriaComponent::Code => Ok(simple_leaf(
                element,
                NativeRole::Code,
                self.best_label(element)?,
            )),
            AriaComponent::KeyboardInput => Ok(simple_leaf(
                element,
                NativeRole::KeyboardInput,
                self.best_label(element)?,
            )),
            AriaComponent::SampleOutput => Ok(simple_leaf(
                element,
                NativeRole::SampleOutput,
                self.best_label(element)?,
            )),
            AriaComponent::Variable => Ok(simple_leaf(
                element,
                NativeRole::Variable,
                self.best_label(element)?,
            )),
            AriaComponent::InlineQuote => Ok(simple_leaf(
                element,
                NativeRole::InlineQuote,
                self.best_label(element)?,
            )),
            AriaComponent::Subscript => Ok(simple_leaf(
                element,
                NativeRole::Subscript,
                self.best_label(element)?,
            )),
            AriaComponent::Superscript => Ok(simple_leaf(
                element,
                NativeRole::Superscript,
                self.best_label(element)?,
            )),
            AriaComponent::SmallText => Ok(simple_leaf(
                element,
                NativeRole::SmallText,
                self.best_label(element)?,
            )),
            AriaComponent::BoldText => Ok(simple_leaf(
                element,
                NativeRole::BoldText,
                self.best_label(element)?,
            )),
            AriaComponent::ItalicText => Ok(simple_leaf(
                element,
                NativeRole::ItalicText,
                self.best_label(element)?,
            )),
            AriaComponent::StruckText => Ok(simple_leaf(
                element,
                NativeRole::StruckText,
                self.best_label(element)?,
            )),
            AriaComponent::UnderlinedText => Ok(simple_leaf(
                element,
                NativeRole::UnderlinedText,
                self.best_label(element)?,
            )),
            AriaComponent::BidirectionalIsolate => Ok(simple_leaf(
                element,
                NativeRole::BidirectionalIsolate,
                self.best_label(element)?,
            )),
            AriaComponent::BidirectionalOverride => Ok(simple_leaf(
                element,
                NativeRole::BidirectionalOverride,
                self.best_label(element)?,
            )),
            AriaComponent::Paragraph => {
                self.map_container_with_label(element, NativeRole::Paragraph)
            }
            AriaComponent::PreformattedText => {
                self.map_container_with_label(element, NativeRole::PreformattedText)
            }
            AriaComponent::BlockQuote => {
                self.map_container_with_label(element, NativeRole::BlockQuote)
            }
            AriaComponent::ContactAddress => {
                self.map_container_with_label(element, NativeRole::ContactAddress)
            }
            AriaComponent::LineBreak => Ok(simple_leaf(
                element,
                NativeRole::LineBreak,
                self.best_label(element)?,
            )),
            AriaComponent::WordBreakOpportunity => Ok(simple_leaf(
                element,
                NativeRole::WordBreakOpportunity,
                self.best_label(element)?,
            )),
            AriaComponent::NoBreakText => {
                self.map_container_with_label(element, NativeRole::NoBreakText)
            }
            AriaComponent::CenteredText => {
                self.map_container_with_label(element, NativeRole::CenteredText)
            }
            AriaComponent::FontText => self.map_container_with_label(element, NativeRole::FontText),
            AriaComponent::BigText => self.map_container_with_label(element, NativeRole::BigText),
            AriaComponent::TeletypeText => {
                self.map_container_with_label(element, NativeRole::TeletypeText)
            }
            AriaComponent::Applet => self.map_container_with_label(element, NativeRole::Applet),
            AriaComponent::BackgroundSound => Ok(simple_leaf(
                element,
                NativeRole::BackgroundSound,
                self.best_label(element)?,
            )),
            AriaComponent::Frame => self.map_container_with_label(element, NativeRole::Frame),
            AriaComponent::FrameSet => self.map_container_with_label(element, NativeRole::FrameSet),
            AriaComponent::NoEmbedFallback => {
                self.map_container_with_label(element, NativeRole::NoEmbedFallback)
            }
            AriaComponent::NoFramesFallback => {
                self.map_container_with_label(element, NativeRole::NoFramesFallback)
            }
            AriaComponent::Marquee => self.map_container_with_label(element, NativeRole::Marquee),
            AriaComponent::Math => self.map_container_with_label(element, NativeRole::Math),
            AriaComponent::NextId => Ok(simple_leaf(
                element,
                NativeRole::NextId,
                self.best_label(element)?,
            )),
            AriaComponent::SelectedContent => {
                self.map_container_with_label(element, NativeRole::SelectedContent)
            }
            AriaComponent::Document => self.map_container_with_label(element, NativeRole::Document),
            AriaComponent::DocumentHead => self.map_container(element, NativeRole::DocumentHead),
            AriaComponent::DocumentBody => {
                self.map_container_with_label(element, NativeRole::DocumentBody)
            }
            AriaComponent::DocumentTitle => {
                self.map_container_with_label(element, NativeRole::DocumentTitle)
            }
            AriaComponent::Metadata => self.map_container(element, NativeRole::Metadata),
            AriaComponent::ResourceLink => self.map_container(element, NativeRole::ResourceLink),
            AriaComponent::StyleSheet => self.map_container(element, NativeRole::StyleSheet),
            AriaComponent::Script => self.map_container(element, NativeRole::Script),
            AriaComponent::Template => self.map_container(element, NativeRole::Template),
            AriaComponent::Slot => self.map_container_with_label(element, NativeRole::Slot),
            AriaComponent::Heading => Ok(simple_leaf(
                element,
                NativeRole::Heading,
                self.best_label(element)?,
            )),
            AriaComponent::HeadingGroup => {
                self.map_container_with_label(element, NativeRole::HeadingGroup)
            }
            AriaComponent::Ruby => self.map_container_with_label(element, NativeRole::Ruby),
            AriaComponent::RubyBase => Ok(simple_leaf(
                element,
                NativeRole::RubyBase,
                self.best_label(element)?,
            )),
            AriaComponent::RubyText => Ok(simple_leaf(
                element,
                NativeRole::RubyText,
                self.best_label(element)?,
            )),
            AriaComponent::RubyParenthesis => Ok(simple_leaf(
                element,
                NativeRole::RubyParenthesis,
                self.best_label(element)?,
            )),
            AriaComponent::RubyTextContainer => {
                self.map_container_with_label(element, NativeRole::RubyTextContainer)
            }
            AriaComponent::Main => self.map_container(element, NativeRole::Main),
            AriaComponent::Navigation => self.map_container(element, NativeRole::Navigation),
            AriaComponent::Header => self.map_container(element, NativeRole::Header),
            AriaComponent::Footer => self.map_container(element, NativeRole::Footer),
            AriaComponent::Article => self.map_container_with_label(element, NativeRole::Article),
            AriaComponent::Section => self.map_container_with_label(element, NativeRole::Section),
            AriaComponent::Aside => self.map_container_with_label(element, NativeRole::Aside),
            AriaComponent::Search => self.map_container_with_label(element, NativeRole::Search),
            AriaComponent::Disclosure => {
                self.map_container_with_label(element, NativeRole::Disclosure)
            }
            AriaComponent::DisclosureSummary => Ok(simple_leaf(
                element,
                NativeRole::DisclosureSummary,
                self.best_label(element)?,
            )),
            AriaComponent::Figure => self.map_container_with_label(element, NativeRole::Figure),
            AriaComponent::FigureCaption => {
                self.map_container_with_label(element, NativeRole::FigureCaption)
            }
            AriaComponent::DescriptionList => {
                self.map_container(element, NativeRole::DescriptionList)
            }
            AriaComponent::DescriptionTerm => {
                self.map_container_with_label(element, NativeRole::DescriptionTerm)
            }
            AriaComponent::DescriptionDetails => {
                self.map_container_with_label(element, NativeRole::DescriptionDetails)
            }
            AriaComponent::Image => self.map_container_with_label(element, NativeRole::Image),
            AriaComponent::Media => self.map_container_with_label(element, NativeRole::Media),
            AriaComponent::Canvas => self.map_container(element, NativeRole::Canvas),
            AriaComponent::EmbeddedContent => {
                self.map_container(element, NativeRole::EmbeddedContent)
            }
            AriaComponent::Link => Ok(simple_leaf(
                element,
                NativeRole::Link,
                self.best_label(element)?,
            )),
            AriaComponent::ImageMap => self.map_container_with_label(element, NativeRole::ImageMap),
            AriaComponent::ImageMapArea => Ok(simple_leaf(
                element,
                NativeRole::ImageMapArea,
                self.best_label(element)?,
            )),
            AriaComponent::TextField => self.map_text_field(element),
            AriaComponent::Input => Ok(simple_leaf(
                element,
                NativeRole::TextField,
                self.best_input_label(element)?,
            )),
            AriaComponent::Checkbox => Ok(simple_leaf(
                element,
                NativeRole::Checkbox,
                self.best_label(element)?,
            )),
            AriaComponent::Switch => Ok(simple_leaf(
                element,
                NativeRole::Switch,
                self.best_label(element)?,
            )),
            AriaComponent::RadioGroup => {
                self.map_container_with_label(element, NativeRole::RadioGroup)
            }
            AriaComponent::Radio => Ok(radio_leaf(element, self.best_label(element)?)),
            AriaComponent::Form => self.map_container_with_label(element, NativeRole::Form),
            AriaComponent::FieldSet => self.map_field_set(element),
            AriaComponent::Legend => Ok(simple_leaf(
                element,
                NativeRole::Legend,
                self.best_label(element)?,
            )),
            AriaComponent::OptionGroup => {
                self.map_container_with_label(element, NativeRole::OptionGroup)
            }
            AriaComponent::Output => Ok(simple_leaf(
                element,
                NativeRole::Output,
                self.best_label(element)?,
            )),
            AriaComponent::Meter => Ok(simple_leaf(
                element,
                NativeRole::Meter,
                self.best_label(element)?,
            )),
            AriaComponent::Select => self.map_select(element),
            AriaComponent::ListBox => self.map_container(element, NativeRole::ListBox),
            AriaComponent::ListBoxItem => Ok(simple_leaf(
                element,
                NativeRole::ListBoxItem,
                self.best_label(element)?,
            )),
            AriaComponent::Dialog => self.map_container_with_label(element, NativeRole::Dialog),
            AriaComponent::Popover => self.map_container(element, NativeRole::Popover),
            AriaComponent::Tabs => self.map_tabs(element),
            AriaComponent::TabList => self.map_container(element, NativeRole::TabList),
            AriaComponent::Tab => Ok(simple_leaf(
                element,
                NativeRole::Tab,
                self.best_label(element)?,
            )),
            AriaComponent::TabPanel => self.map_container(element, NativeRole::TabPanel),
            AriaComponent::Group => self.map_container(element, NativeRole::View),
            AriaComponent::Menu => self.map_container(element, NativeRole::Menu),
            AriaComponent::MenuItem => Ok(simple_leaf(
                element,
                NativeRole::MenuItem,
                self.best_label(element)?,
            )),
            AriaComponent::Separator => Ok(simple_leaf(element, NativeRole::Separator, None)),
            AriaComponent::Slider => Ok(simple_leaf(
                element,
                NativeRole::Slider,
                self.best_label(element)?,
            )),
            AriaComponent::ProgressBar => Ok(simple_leaf(
                element,
                NativeRole::ProgressBar,
                self.best_label(element)?,
            )),
            AriaComponent::Toolbar => self.map_container(element, NativeRole::Toolbar),
            AriaComponent::Table => self.map_container(element, NativeRole::Table),
            AriaComponent::TableSection => self.map_container(element, NativeRole::TableSection),
            AriaComponent::TableRow => self.map_container(element, NativeRole::TableRow),
            AriaComponent::TableCell => {
                self.map_container_with_label(element, NativeRole::TableCell)
            }
            AriaComponent::TableColumn => self.map_container(element, NativeRole::TableColumn),
            AriaComponent::TableCaption => {
                self.map_container_with_label(element, NativeRole::TableCaption)
            }
        }
    }

    fn map_container(&self, element: &AriaElement, role: NativeRole) -> GuiResult<NativeElement> {
        let mut native = NativeElement::new(element.key.clone(), role).with_props(
            native_props_from_aria(&element.props, self.best_label(element)?),
        );
        native.children = element
            .children
            .iter()
            .map(|child| self.map_element(child))
            .collect::<GuiResult<Vec<_>>>()?;
        Ok(native)
    }

    fn map_container_with_label(
        &self,
        element: &AriaElement,
        role: NativeRole,
    ) -> GuiResult<NativeElement> {
        self.map_container(element, role)
    }

    fn map_text(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        if element.children.is_empty() {
            Ok(simple_leaf(
                element,
                NativeRole::Text,
                self.best_label(element)?,
            ))
        } else {
            self.map_container_with_label(element, NativeRole::Text)
        }
    }

    fn map_text_field(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        let label = element
            .props
            .label
            .clone()
            .or_else(|| aria_label(&element.props))
            .or_else(|| first_child_label(element, AriaComponent::Label))
            .or_else(|| first_text_child(element));
        let input = element
            .children
            .iter()
            .find(|child| child.component == AriaComponent::Input);
        let mut props = native_props_from_aria(&element.props, label);
        if props.value.is_none() {
            props.value = input.and_then(|child| child.props.value.clone());
        }
        if props.placeholder.is_none() {
            props.placeholder = input.and_then(|child| child.props.placeholder.clone());
        }
        if let Some(input) = input {
            if props.action.is_none() {
                props.action = input
                    .props
                    .action
                    .clone()
                    .or_else(|| input.props.web.primary_action().map(str::to_string));
            }
            for (name, action) in &input.props.web.events {
                props
                    .web
                    .events
                    .entry(name.clone())
                    .or_insert_with(|| action.clone());
            }
            for (name, value) in &input.props.web.style {
                props
                    .web
                    .style
                    .entry(name.clone())
                    .or_insert_with(|| value.clone());
            }
            for (name, value) in &input.props.web.attributes {
                props
                    .web
                    .attributes
                    .entry(name.clone())
                    .or_insert_with(|| value.clone());
            }
            props.metadata = props.web.metadata();
        }
        Ok(NativeElement::new(element.key.clone(), NativeRole::TextField).with_props(props))
    }

    fn map_select(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        let label = element
            .props
            .label
            .clone()
            .or_else(|| aria_label(&element.props))
            .or_else(|| first_child_label(element, AriaComponent::Label));
        let mut native = NativeElement::new(element.key.clone(), NativeRole::Select)
            .with_props(native_props_from_aria(&element.props, label));

        if let Some(list_box) = element
            .children
            .iter()
            .find(|child| child.component == AriaComponent::ListBox)
        {
            native.children = list_box
                .children
                .iter()
                .map(|child| self.map_element(child))
                .collect::<GuiResult<Vec<_>>>()?;
        } else {
            native.children = element
                .children
                .iter()
                .filter(|child| {
                    !matches!(
                        child.component,
                        AriaComponent::Label | AriaComponent::SelectValue
                    )
                })
                .map(|child| self.map_element(child))
                .collect::<GuiResult<Vec<_>>>()?;
        }

        Ok(native)
    }

    fn map_field_set(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        let label = element
            .props
            .label
            .clone()
            .or_else(|| aria_label(&element.props))
            .or_else(|| first_child_label(element, AriaComponent::Legend))
            .or_else(|| first_text_child(element));
        let mut native = NativeElement::new(element.key.clone(), NativeRole::FieldSet)
            .with_props(native_props_from_aria(&element.props, label));
        native.children = element
            .children
            .iter()
            .map(|child| self.map_element(child))
            .collect::<GuiResult<Vec<_>>>()?;
        Ok(native)
    }

    fn map_tabs(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        let mut native = NativeElement::new(element.key.clone(), NativeRole::Tabs).with_props(
            native_props_from_aria(&element.props, self.best_label(element)?),
        );

        let tabs = element
            .children
            .iter()
            .find(|child| child.component == AriaComponent::TabList)
            .map(|tab_list| tab_list.children.as_slice())
            .unwrap_or(element.children.as_slice());
        let panels = element
            .children
            .iter()
            .filter(|child| child.component == AriaComponent::TabPanel)
            .collect::<Vec<_>>();

        for (index, tab) in tabs
            .iter()
            .filter(|child| child.component == AriaComponent::Tab)
            .enumerate()
        {
            let mut native_tab = simple_leaf(tab, NativeRole::Tab, self.best_label(tab)?);
            if let Some(panel) = panels.get(index) {
                native_tab.children.push(self.map_element(panel)?);
            }
            native.children.push(native_tab);
        }

        if native.children.is_empty() {
            native.children = element
                .children
                .iter()
                .map(|child| self.map_element(child))
                .collect::<GuiResult<Vec<_>>>()?;
        }

        Ok(native)
    }

    fn best_label(&self, element: &AriaElement) -> GuiResult<Option<String>> {
        Ok(element
            .props
            .label
            .clone()
            .or_else(|| aria_label(&element.props))
            .or_else(|| element.props.text_value.clone())
            .or_else(|| first_text_child(element)))
    }

    fn best_input_label(&self, element: &AriaElement) -> GuiResult<Option<String>> {
        if is_html_textarea_value(element) {
            return Ok(element
                .props
                .label
                .clone()
                .or_else(|| aria_label(&element.props))
                .or_else(|| element.props.text_value.clone()));
        }

        self.best_label(element)
    }
}

fn simple_leaf(element: &AriaElement, role: NativeRole, label: Option<String>) -> NativeElement {
    NativeElement::new(element.key.clone(), role)
        .with_props(native_props_from_aria(&element.props, label))
}

fn radio_leaf(element: &AriaElement, label: Option<String>) -> NativeElement {
    let mut props = native_props_from_aria(&element.props, label);
    if props.checked.is_none() && props.selected {
        props.checked = Some(true);
    }
    NativeElement::new(element.key.clone(), NativeRole::Radio).with_props(props)
}

fn native_props_from_aria(props: &AriaProps, label: Option<String>) -> NativeProps {
    let mut native = NativeProps::new()
        .disabled(props.is_disabled)
        .required(props.is_required)
        .invalid(props.is_invalid)
        .read_only(props.is_read_only)
        .multiple(props.is_multiple)
        .auto_focus(props.auto_focus)
        .selected(props.is_selected);
    native.label = label;
    native.value = props.value.clone();
    native.placeholder = props.placeholder.clone();
    native.action = props
        .action
        .clone()
        .or_else(|| props.web.primary_action().map(str::to_string));
    native.checked = props.is_checked;
    native.expanded = props.is_expanded;
    native.orientation = props.orientation;
    native.min = props.min_value;
    native.max = props.max_value;
    native.current = props.value_number;
    native.step = props.step_value;
    native.autocomplete = props.autocomplete.clone();
    native.input_mode = props.input_mode.clone();
    native.pattern = props.pattern.clone();
    native.min_length = props.min_length;
    native.max_length = props.max_length;
    native.rows = props.rows;
    native.cols = props.cols;
    native.size = props.size;
    native.metadata = props.web.metadata();
    native.web = props.web.clone();
    native
}

fn first_child_label(element: &AriaElement, component: AriaComponent) -> Option<String> {
    element.children.iter().find_map(|child| {
        if child.component == component {
            child
                .props
                .label
                .clone()
                .or_else(|| aria_label(&child.props))
                .or_else(|| child.props.text_value.clone())
                .or_else(|| first_text_child(child))
        } else {
            None
        }
    })
}

fn first_text_child(element: &AriaElement) -> Option<String> {
    element.children.iter().find_map(|child| {
        if child.component == AriaComponent::Text || child.component == AriaComponent::Label {
            child
                .props
                .text_value
                .clone()
                .or_else(|| child.props.label.clone())
        } else {
            first_text_child(child)
        }
    })
}

fn is_html_textarea_value(element: &AriaElement) -> bool {
    element.props.value.is_some()
        && element
            .props
            .web
            .attributes
            .get(HTML_TAG_METADATA_KEY)
            .is_some_and(|tag| tag == "textarea")
}

fn aria_label(props: &AriaProps) -> Option<String> {
    props.web.attributes.get("aria-label").cloned()
}
