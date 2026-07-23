use crate::error::GuiResult;
use crate::html::HTML_TAG_METADATA_KEY;
use crate::native::{NativeElement, NativeProps, NativeRole};

use super::{SemanticComponent, SemanticElement, SemanticProps};

mod tree;

#[derive(Debug, Clone)]
pub struct SemanticMapper;

impl Default for SemanticMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticMapper {
    pub fn new() -> Self {
        Self
    }

    pub fn map(&self, element: &SemanticElement) -> GuiResult<NativeElement> {
        self.map_element(element)
    }

    fn map_element(&self, element: &SemanticElement) -> GuiResult<NativeElement> {
        match element.component {
            SemanticComponent::Button => Ok(simple_leaf(
                element,
                NativeRole::Button,
                self.best_label(element)?,
            )),
            SemanticComponent::Label | SemanticComponent::SelectValue => Ok(simple_leaf(
                element,
                NativeRole::Text,
                self.best_label(element)?,
            )),
            SemanticComponent::Text => self.map_text(element),
            SemanticComponent::Abbreviation => Ok(simple_leaf(
                element,
                NativeRole::Abbreviation,
                self.best_label(element)?,
            )),
            SemanticComponent::Citation => Ok(simple_leaf(
                element,
                NativeRole::Citation,
                self.best_label(element)?,
            )),
            SemanticComponent::Definition => Ok(simple_leaf(
                element,
                NativeRole::Definition,
                self.best_label(element)?,
            )),
            SemanticComponent::DataValue => Ok(simple_leaf(
                element,
                NativeRole::DataValue,
                self.best_label(element)?,
            )),
            SemanticComponent::InsertedText => Ok(simple_leaf(
                element,
                NativeRole::InsertedText,
                self.best_label(element)?,
            )),
            SemanticComponent::DeletedText => Ok(simple_leaf(
                element,
                NativeRole::DeletedText,
                self.best_label(element)?,
            )),
            SemanticComponent::MarkedText => Ok(simple_leaf(
                element,
                NativeRole::MarkedText,
                self.best_label(element)?,
            )),
            SemanticComponent::Time => Ok(simple_leaf(
                element,
                NativeRole::Time,
                self.best_label(element)?,
            )),
            SemanticComponent::Emphasis => Ok(simple_leaf(
                element,
                NativeRole::Emphasis,
                self.best_label(element)?,
            )),
            SemanticComponent::StrongText => Ok(simple_leaf(
                element,
                NativeRole::StrongText,
                self.best_label(element)?,
            )),
            SemanticComponent::Code => Ok(simple_leaf(
                element,
                NativeRole::Code,
                self.best_label(element)?,
            )),
            SemanticComponent::KeyboardInput => Ok(simple_leaf(
                element,
                NativeRole::KeyboardInput,
                self.best_label(element)?,
            )),
            SemanticComponent::SampleOutput => Ok(simple_leaf(
                element,
                NativeRole::SampleOutput,
                self.best_label(element)?,
            )),
            SemanticComponent::Variable => Ok(simple_leaf(
                element,
                NativeRole::Variable,
                self.best_label(element)?,
            )),
            SemanticComponent::InlineQuote => Ok(simple_leaf(
                element,
                NativeRole::InlineQuote,
                self.best_label(element)?,
            )),
            SemanticComponent::Subscript => Ok(simple_leaf(
                element,
                NativeRole::Subscript,
                self.best_label(element)?,
            )),
            SemanticComponent::Superscript => Ok(simple_leaf(
                element,
                NativeRole::Superscript,
                self.best_label(element)?,
            )),
            SemanticComponent::SmallText => Ok(simple_leaf(
                element,
                NativeRole::SmallText,
                self.best_label(element)?,
            )),
            SemanticComponent::BoldText => Ok(simple_leaf(
                element,
                NativeRole::BoldText,
                self.best_label(element)?,
            )),
            SemanticComponent::ItalicText => Ok(simple_leaf(
                element,
                NativeRole::ItalicText,
                self.best_label(element)?,
            )),
            SemanticComponent::StruckText => Ok(simple_leaf(
                element,
                NativeRole::StruckText,
                self.best_label(element)?,
            )),
            SemanticComponent::UnderlinedText => Ok(simple_leaf(
                element,
                NativeRole::UnderlinedText,
                self.best_label(element)?,
            )),
            SemanticComponent::BidirectionalIsolate => Ok(simple_leaf(
                element,
                NativeRole::BidirectionalIsolate,
                self.best_label(element)?,
            )),
            SemanticComponent::BidirectionalOverride => Ok(simple_leaf(
                element,
                NativeRole::BidirectionalOverride,
                self.best_label(element)?,
            )),
            SemanticComponent::Paragraph => {
                self.map_container_with_label(element, NativeRole::Paragraph)
            }
            SemanticComponent::PreformattedText => {
                self.map_container_with_label(element, NativeRole::PreformattedText)
            }
            SemanticComponent::BlockQuote => {
                self.map_container_with_label(element, NativeRole::BlockQuote)
            }
            SemanticComponent::ContactAddress => {
                self.map_container_with_label(element, NativeRole::ContactAddress)
            }
            SemanticComponent::LineBreak => Ok(simple_leaf(
                element,
                NativeRole::LineBreak,
                self.best_label(element)?,
            )),
            SemanticComponent::WordBreakOpportunity => Ok(simple_leaf(
                element,
                NativeRole::WordBreakOpportunity,
                self.best_label(element)?,
            )),
            SemanticComponent::NoBreakText => {
                self.map_container_with_label(element, NativeRole::NoBreakText)
            }
            SemanticComponent::CenteredText => {
                self.map_container_with_label(element, NativeRole::CenteredText)
            }
            SemanticComponent::FontText => {
                self.map_container_with_label(element, NativeRole::FontText)
            }
            SemanticComponent::BigText => {
                self.map_container_with_label(element, NativeRole::BigText)
            }
            SemanticComponent::TeletypeText => {
                self.map_container_with_label(element, NativeRole::TeletypeText)
            }
            SemanticComponent::Applet => self.map_container_with_label(element, NativeRole::Applet),
            SemanticComponent::BackgroundSound => Ok(simple_leaf(
                element,
                NativeRole::BackgroundSound,
                self.best_label(element)?,
            )),
            SemanticComponent::Frame => self.map_container_with_label(element, NativeRole::Frame),
            SemanticComponent::FrameSet => {
                self.map_container_with_label(element, NativeRole::FrameSet)
            }
            SemanticComponent::NoEmbedFallback => {
                self.map_container_with_label(element, NativeRole::NoEmbedFallback)
            }
            SemanticComponent::NoFramesFallback => {
                self.map_container_with_label(element, NativeRole::NoFramesFallback)
            }
            SemanticComponent::Marquee => {
                self.map_container_with_label(element, NativeRole::Marquee)
            }
            SemanticComponent::Math => self.map_container_with_label(element, NativeRole::Math),
            SemanticComponent::NextId => Ok(simple_leaf(
                element,
                NativeRole::NextId,
                self.best_label(element)?,
            )),
            SemanticComponent::SelectedContent => {
                self.map_container_with_label(element, NativeRole::SelectedContent)
            }
            SemanticComponent::Document => {
                self.map_container_with_label(element, NativeRole::Document)
            }
            SemanticComponent::DocumentHead => {
                self.map_container(element, NativeRole::DocumentHead)
            }
            SemanticComponent::DocumentBody => {
                self.map_container_with_label(element, NativeRole::DocumentBody)
            }
            SemanticComponent::DocumentTitle => {
                self.map_container_with_label(element, NativeRole::DocumentTitle)
            }
            SemanticComponent::Metadata => self.map_container(element, NativeRole::Metadata),
            SemanticComponent::ResourceLink => {
                self.map_container(element, NativeRole::ResourceLink)
            }
            SemanticComponent::StyleSheet => self.map_container(element, NativeRole::StyleSheet),
            SemanticComponent::Script => self.map_container(element, NativeRole::Script),
            SemanticComponent::Template => self.map_container(element, NativeRole::Template),
            SemanticComponent::Slot => self.map_container_with_label(element, NativeRole::Slot),
            SemanticComponent::Heading => Ok(simple_leaf(
                element,
                NativeRole::Heading,
                self.best_label(element)?,
            )),
            SemanticComponent::HeadingGroup => {
                self.map_container_with_label(element, NativeRole::HeadingGroup)
            }
            SemanticComponent::Ruby => self.map_container_with_label(element, NativeRole::Ruby),
            SemanticComponent::RubyBase => Ok(simple_leaf(
                element,
                NativeRole::RubyBase,
                self.best_label(element)?,
            )),
            SemanticComponent::RubyText => Ok(simple_leaf(
                element,
                NativeRole::RubyText,
                self.best_label(element)?,
            )),
            SemanticComponent::RubyParenthesis => Ok(simple_leaf(
                element,
                NativeRole::RubyParenthesis,
                self.best_label(element)?,
            )),
            SemanticComponent::RubyTextContainer => {
                self.map_container_with_label(element, NativeRole::RubyTextContainer)
            }
            SemanticComponent::Main => self.map_container(element, NativeRole::Main),
            SemanticComponent::Navigation => self.map_container(element, NativeRole::Navigation),
            SemanticComponent::Header => self.map_container(element, NativeRole::Header),
            SemanticComponent::Footer => self.map_container(element, NativeRole::Footer),
            SemanticComponent::Article => {
                self.map_container_with_label(element, NativeRole::Article)
            }
            SemanticComponent::Section => {
                self.map_container_with_label(element, NativeRole::Section)
            }
            SemanticComponent::Aside => self.map_container_with_label(element, NativeRole::Aside),
            SemanticComponent::Search => self.map_container_with_label(element, NativeRole::Search),
            SemanticComponent::Disclosure => {
                self.map_container_with_label(element, NativeRole::Disclosure)
            }
            SemanticComponent::DisclosureSummary => Ok(simple_leaf(
                element,
                NativeRole::DisclosureSummary,
                self.best_label(element)?,
            )),
            SemanticComponent::Figure => self.map_container_with_label(element, NativeRole::Figure),
            SemanticComponent::FigureCaption => {
                self.map_container_with_label(element, NativeRole::FigureCaption)
            }
            SemanticComponent::DescriptionList => {
                self.map_container(element, NativeRole::DescriptionList)
            }
            SemanticComponent::DescriptionTerm => {
                self.map_container_with_label(element, NativeRole::DescriptionTerm)
            }
            SemanticComponent::DescriptionDetails => {
                self.map_container_with_label(element, NativeRole::DescriptionDetails)
            }
            SemanticComponent::Image => self.map_container_with_label(element, NativeRole::Image),
            SemanticComponent::Media => self.map_container_with_label(element, NativeRole::Media),
            SemanticComponent::Canvas => self.map_container(element, NativeRole::Canvas),
            SemanticComponent::EmbeddedContent => {
                self.map_container(element, NativeRole::EmbeddedContent)
            }
            SemanticComponent::Link => Ok(simple_leaf(
                element,
                NativeRole::Link,
                self.best_label(element)?,
            )),
            SemanticComponent::ImageMap => {
                self.map_container_with_label(element, NativeRole::ImageMap)
            }
            SemanticComponent::ImageMapArea => Ok(simple_leaf(
                element,
                NativeRole::ImageMapArea,
                self.best_label(element)?,
            )),
            SemanticComponent::TextField => self.map_text_field(element),
            SemanticComponent::Input => Ok(simple_leaf(
                element,
                NativeRole::TextField,
                self.best_input_label(element)?,
            )),
            SemanticComponent::Checkbox => Ok(simple_leaf(
                element,
                NativeRole::Checkbox,
                self.best_label(element)?,
            )),
            SemanticComponent::Switch => Ok(simple_leaf(
                element,
                NativeRole::Switch,
                self.best_label(element)?,
            )),
            SemanticComponent::RadioGroup => {
                self.map_container_with_label(element, NativeRole::RadioGroup)
            }
            SemanticComponent::Radio => Ok(radio_leaf(element, self.best_label(element)?)),
            SemanticComponent::Form => self.map_container_with_label(element, NativeRole::Form),
            SemanticComponent::FieldSet => self.map_field_set(element),
            SemanticComponent::Legend => Ok(simple_leaf(
                element,
                NativeRole::Legend,
                self.best_label(element)?,
            )),
            SemanticComponent::OptionGroup => {
                self.map_container_with_label(element, NativeRole::OptionGroup)
            }
            SemanticComponent::Output => Ok(simple_leaf(
                element,
                NativeRole::Output,
                self.best_label(element)?,
            )),
            SemanticComponent::Meter => Ok(simple_leaf(
                element,
                NativeRole::Meter,
                self.best_label(element)?,
            )),
            SemanticComponent::ComboBox => self.map_combo_box(element),
            SemanticComponent::Select => self.map_select(element),
            SemanticComponent::ListBox => self.map_container(element, NativeRole::ListBox),
            SemanticComponent::ListBoxItem => Ok(simple_leaf(
                element,
                NativeRole::ListBoxItem,
                self.best_label(element)?,
            )),
            SemanticComponent::Tree => self.map_tree(element),
            SemanticComponent::TreeItem => self.map_tree_item_row(element, None, 1, 1, 1),
            SemanticComponent::Dialog => self.map_container_with_label(element, NativeRole::Dialog),
            SemanticComponent::Popover => self.map_container(element, NativeRole::Popover),
            SemanticComponent::Tabs => self.map_tabs(element),
            SemanticComponent::TabList => self.map_container(element, NativeRole::TabList),
            SemanticComponent::Tab => Ok(simple_leaf(
                element,
                NativeRole::Tab,
                self.best_label(element)?,
            )),
            SemanticComponent::TabPanel => self.map_container(element, NativeRole::TabPanel),
            SemanticComponent::Group => self.map_container(element, NativeRole::View),
            SemanticComponent::Menu => self.map_container(element, NativeRole::Menu),
            SemanticComponent::MenuItem => Ok(simple_leaf(
                element,
                NativeRole::MenuItem,
                self.best_label(element)?,
            )),
            SemanticComponent::Separator => Ok(simple_leaf(element, NativeRole::Separator, None)),
            SemanticComponent::Slider => Ok(simple_leaf(
                element,
                NativeRole::Slider,
                self.best_label(element)?,
            )),
            SemanticComponent::ProgressBar => Ok(simple_leaf(
                element,
                NativeRole::ProgressBar,
                self.best_label(element)?,
            )),
            SemanticComponent::Toolbar => self.map_container(element, NativeRole::Toolbar),
            SemanticComponent::Table => self.map_container(element, NativeRole::Table),
            SemanticComponent::TableSection => {
                self.map_container(element, NativeRole::TableSection)
            }
            SemanticComponent::TableRow => self.map_container(element, NativeRole::TableRow),
            SemanticComponent::TableCell => {
                self.map_container_with_label(element, NativeRole::TableCell)
            }
            SemanticComponent::TableColumn => self.map_container(element, NativeRole::TableColumn),
            SemanticComponent::TableCaption => {
                self.map_container_with_label(element, NativeRole::TableCaption)
            }
        }
    }

    fn map_container(
        &self,
        element: &SemanticElement,
        role: NativeRole,
    ) -> GuiResult<NativeElement> {
        let mut native = NativeElement::new(element.key.clone(), role).with_props(
            native_props_from_aria(&element.props, self.best_label(element)?),
        );
        native.children = element
            .children
            .iter()
            .map(|child| self.map_element(child))
            .collect::<GuiResult<Vec<_>>>()?
            .into_iter()
            .filter(|child| !empty_native_text(child))
            .collect();
        Ok(native)
    }

    fn map_container_with_label(
        &self,
        element: &SemanticElement,
        role: NativeRole,
    ) -> GuiResult<NativeElement> {
        self.map_container(element, role)
    }

    fn map_text(&self, element: &SemanticElement) -> GuiResult<NativeElement> {
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

    fn map_text_field(&self, element: &SemanticElement) -> GuiResult<NativeElement> {
        let label = element
            .props
            .label
            .clone()
            .or_else(|| first_child_label(element, SemanticComponent::Label))
            .or_else(|| first_text_child(element));
        let input = element
            .children
            .iter()
            .find(|child| child.component == SemanticComponent::Input);
        let mut props = native_props_from_aria(&element.props, label);
        if props.value.is_none() {
            props.value = input.and_then(|child| child.props.value.clone());
        }
        if props.placeholder.is_none() {
            props.placeholder = input.and_then(|child| child.props.placeholder.clone());
        }
        if props.input_type.is_none() {
            props.input_type = input.and_then(|child| child.props.input_type.clone());
        }
        if let Some(input_type) = props.input_type.clone() {
            props
                .web
                .attributes
                .entry("type".to_string())
                .or_insert(input_type);
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

    fn map_select(&self, element: &SemanticElement) -> GuiResult<NativeElement> {
        let label = non_empty_clone(element.props.label.as_ref())
            .or_else(|| first_child_label(element, SemanticComponent::Label));
        let mut native = NativeElement::new(element.key.clone(), NativeRole::Select)
            .with_props(native_props_from_aria(&element.props, label));

        if let Some(options) = self.map_list_box_options(element)? {
            native.children = options;
        } else {
            native.children = element
                .children
                .iter()
                .filter(|child| {
                    !matches!(
                        child.component,
                        SemanticComponent::Label | SemanticComponent::SelectValue
                    )
                })
                .map(|child| self.map_element(child))
                .collect::<GuiResult<Vec<_>>>()?;
        }

        Ok(native)
    }

    fn map_combo_box(&self, element: &SemanticElement) -> GuiResult<NativeElement> {
        let label = non_empty_clone(element.props.label.as_ref())
            .or_else(|| first_child_label(element, SemanticComponent::Label));
        let input = find_descendant_component(element, SemanticComponent::Input);
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
            for (name, value) in &input.props.web.attributes {
                props
                    .web
                    .attributes
                    .entry(name.clone())
                    .or_insert_with(|| value.clone());
            }
            props.metadata = props.web.metadata();
        }

        let mut native =
            NativeElement::new(element.key.clone(), NativeRole::ComboBox).with_props(props);
        if let Some(options) = self.map_list_box_options(element)? {
            native.children = options;
        }
        Ok(native)
    }

    fn map_list_box_options(
        &self,
        element: &SemanticElement,
    ) -> GuiResult<Option<Vec<NativeElement>>> {
        let Some(list_box) = find_descendant_component(element, SemanticComponent::ListBox) else {
            return Ok(None);
        };
        list_box
            .children
            .iter()
            .filter(|child| child.component == SemanticComponent::ListBoxItem)
            .map(|child| self.map_element(child))
            .collect::<GuiResult<Vec<_>>>()
            .map(Some)
    }

    fn map_field_set(&self, element: &SemanticElement) -> GuiResult<NativeElement> {
        let label = element
            .props
            .label
            .clone()
            .or_else(|| first_child_label(element, SemanticComponent::Legend))
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

    fn map_tabs(&self, element: &SemanticElement) -> GuiResult<NativeElement> {
        let mut native = NativeElement::new(element.key.clone(), NativeRole::Tabs).with_props(
            native_props_from_aria(&element.props, self.best_label(element)?),
        );

        let tabs = element
            .children
            .iter()
            .find(|child| child.component == SemanticComponent::TabList)
            .map(|tab_list| tab_list.children.as_slice())
            .unwrap_or(element.children.as_slice());
        let panels = element
            .children
            .iter()
            .filter(|child| child.component == SemanticComponent::TabPanel)
            .collect::<Vec<_>>();

        for (index, tab) in tabs
            .iter()
            .filter(|child| child.component == SemanticComponent::Tab)
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

    fn best_label(&self, element: &SemanticElement) -> GuiResult<Option<String>> {
        Ok(non_empty_clone(element.props.label.as_ref())
            .or_else(|| non_empty_clone(element.props.text_value.as_ref()))
            .or_else(|| first_text_child(element)))
    }

    fn best_input_label(&self, element: &SemanticElement) -> GuiResult<Option<String>> {
        if is_html_textarea_value(element) {
            return Ok(element
                .props
                .label
                .clone()
                .or_else(|| element.props.text_value.clone()));
        }

        self.best_label(element)
    }
}

fn simple_leaf(
    element: &SemanticElement,
    role: NativeRole,
    label: Option<String>,
) -> NativeElement {
    NativeElement::new(element.key.clone(), role)
        .with_props(native_props_from_aria(&element.props, label))
}

fn radio_leaf(element: &SemanticElement, label: Option<String>) -> NativeElement {
    let mut props = native_props_from_aria(&element.props, label);
    if props.checked.is_none() && props.selected {
        props.checked = Some(true);
    }
    NativeElement::new(element.key.clone(), NativeRole::Radio).with_props(props)
}

fn empty_native_text(element: &NativeElement) -> bool {
    element.role == NativeRole::Text
        && element.props.label.as_deref().is_none_or(str::is_empty)
        && element.props.value.as_deref().is_none_or(str::is_empty)
        && element.props.html_form_association == Default::default()
        && element.children.is_empty()
}

fn non_empty_clone(value: Option<&String>) -> Option<String> {
    value
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
}

fn find_descendant_component(
    element: &SemanticElement,
    component: SemanticComponent,
) -> Option<&SemanticElement> {
    element.children.iter().find_map(|child| {
        if child.component == component {
            Some(child)
        } else {
            find_descendant_component(child, component)
        }
    })
}

fn native_props_from_aria(props: &SemanticProps, label: Option<String>) -> NativeProps {
    let mut native = NativeProps::new()
        .disabled(props.is_disabled)
        .required(props.is_required)
        .invalid(props.is_invalid)
        .read_only(props.is_read_only)
        .multiple(props.is_multiple)
        .auto_focus(props.auto_focus)
        .selected(props.is_selected);
    native.accessibility_label = aria_label(props).or_else(|| label.clone());
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
    native.enter_key_hint = props.enter_key_hint.clone();
    native.auto_capitalize = props.auto_capitalize.clone();
    native.auto_correct = props.auto_correct.clone();
    native.virtual_keyboard_policy = props.virtual_keyboard_policy.clone();
    native.pattern = props.pattern.clone();
    native.min_length = props.min_length;
    native.max_length = props.max_length;
    native.rows = props.rows;
    native.cols = props.cols;
    native.size = props.size;
    native.title = props.title.clone();
    native.hidden = props.hidden;
    native.lang = props.lang.clone();
    native.dir = props.dir.clone();
    native.tab_index = props.tab_index;
    native.explicit_role = props.explicit_role.clone();
    native.access_key = props.access_key.clone();
    native.content_editable = props.content_editable.clone();
    native.draggable = props.draggable.clone();
    native.spell_check = props.spell_check;
    native.translate = props.translate;
    native.inert = props.inert;
    native.popover = props.popover.clone();
    native.anchor = props.anchor.clone();
    native.custom_element_is = props.custom_element_is.clone();
    native.nonce = props.nonce.clone();
    native.name = props.name.clone();
    native.form = props.form.clone();
    native.input_type = props.input_type.clone();
    native.accept = props.accept.clone();
    native.capture = props.capture.clone();
    native.alt = props.alt.clone();
    native.href = props.href.clone();
    native.src = props.src.clone();
    native.srcset = props.srcset.clone();
    native.sizes = props.sizes.clone();
    native.media = props.media.clone();
    native.resource_type = props.resource_type.clone();
    native.intrinsic_width = props.intrinsic_width;
    native.intrinsic_height = props.intrinsic_height;
    native.loading = props.loading.clone();
    native.decoding = props.decoding.clone();
    native.fetch_priority = props.fetch_priority.clone();
    native.cross_origin = props.cross_origin.clone();
    native.referrer_policy = props.referrer_policy.clone();
    native.poster = props.poster.clone();
    native.controls = props.controls;
    native.autoplay = props.autoplay;
    native.loop_playback = props.loop_playback;
    native.muted = props.muted;
    native.plays_inline = props.plays_inline;
    native.preload = props.preload.clone();
    native.track_kind = props.track_kind.clone();
    native.srclang = props.srclang.clone();
    native.track_label = props.track_label.clone();
    native.default_track = props.default_track;
    native.list = props.list.clone();
    native.dirname = props.dirname.clone();
    native.form_action = props.form_action.clone();
    native.form_enctype = props.form_enctype.clone();
    native.form_method = props.form_method.clone();
    native.form_target = props.form_target.clone();
    native.form_no_validate = props.form_no_validate;
    native.html_resource_policy = props.html_resource_policy.clone();
    native.html_activation = props.html_activation.clone();
    native.html_text_annotation = props.html_text_annotation.clone();
    native.html_dialog = props.html_dialog.clone();
    native.html_shadow = props.html_shadow.clone();
    native.html_microdata = props.html_microdata.clone();
    native.html_form_association = props.html_form_association.clone();
    native.html_collection = props.html_collection.clone();
    native.accessibility_relationships = props.accessibility_relationships.clone();
    native.accessibility_description = props.accessibility_description.clone();
    native.accessibility_structure = props.accessibility_structure.clone();
    native.accessibility_state = props.accessibility_state.clone();
    native.metadata = props.web.metadata();
    native.web = props.web.clone();
    native
}

fn first_child_label(element: &SemanticElement, component: SemanticComponent) -> Option<String> {
    element.children.iter().find_map(|child| {
        if child.component == component {
            non_empty_clone(child.props.label.as_ref())
                .or_else(|| non_empty_clone(child.props.text_value.as_ref()))
                .or_else(|| first_text_child(child))
        } else {
            None
        }
    })
}

fn first_text_child(element: &SemanticElement) -> Option<String> {
    if let Some(text) = direct_text_children(element) {
        return Some(text);
    }
    element.children.iter().find_map(|child| {
        (!starts_independent_label_scope(child.component))
            .then(|| first_text_child(child))
            .flatten()
    })
}

fn starts_independent_label_scope(component: SemanticComponent) -> bool {
    matches!(
        component,
        SemanticComponent::Button
            | SemanticComponent::Link
            | SemanticComponent::ImageMap
            | SemanticComponent::ImageMapArea
            | SemanticComponent::Disclosure
            | SemanticComponent::Figure
            | SemanticComponent::TextField
            | SemanticComponent::Input
            | SemanticComponent::Checkbox
            | SemanticComponent::Switch
            | SemanticComponent::RadioGroup
            | SemanticComponent::Radio
            | SemanticComponent::FieldSet
            | SemanticComponent::OptionGroup
            | SemanticComponent::Output
            | SemanticComponent::Meter
            | SemanticComponent::ComboBox
            | SemanticComponent::Select
            | SemanticComponent::ListBox
            | SemanticComponent::ListBoxItem
            | SemanticComponent::Tree
            | SemanticComponent::TreeItem
            | SemanticComponent::Dialog
            | SemanticComponent::Popover
            | SemanticComponent::Tabs
            | SemanticComponent::TabList
            | SemanticComponent::Tab
            | SemanticComponent::TabPanel
            | SemanticComponent::Form
            | SemanticComponent::Menu
            | SemanticComponent::MenuItem
            | SemanticComponent::Slider
            | SemanticComponent::ProgressBar
            | SemanticComponent::Toolbar
            | SemanticComponent::Table
            | SemanticComponent::TableSection
            | SemanticComponent::TableRow
            | SemanticComponent::TableCell
            | SemanticComponent::TableColumn
    )
}

fn direct_text_children(element: &SemanticElement) -> Option<String> {
    let mut text = String::new();
    for child in &element.children {
        if matches!(
            child.component,
            SemanticComponent::Text | SemanticComponent::Label
        ) {
            if let Some(value) = child
                .props
                .text_value
                .as_ref()
                .or(child.props.label.as_ref())
            {
                text.push_str(value);
            }
        }
    }
    let text = text.trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn is_html_textarea_value(element: &SemanticElement) -> bool {
    element.props.value.is_some()
        && element
            .props
            .web
            .attributes
            .get(HTML_TAG_METADATA_KEY)
            .is_some_and(|tag| tag == "textarea")
}

fn aria_label(props: &SemanticProps) -> Option<String> {
    props.web.attributes.get("aria-label").cloned()
}
