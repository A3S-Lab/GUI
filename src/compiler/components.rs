use std::collections::BTreeMap;

use crate::error::{GuiError, GuiResult};
use crate::html::component_for_intrinsic_tag;
use crate::semantic_ui::SemanticComponent;
use crate::svg::component_for_svg_tag;

use super::CompiledProps;

pub(super) fn component_from_rsx_tag(
    tag: &str,
    props: &CompiledProps,
) -> GuiResult<SemanticComponent> {
    if let Some(component) = SemanticComponent::from_name(tag) {
        return Ok(component);
    }

    match tag {
        "Fragment" => Ok(SemanticComponent::Group),
        "Button" | "button" => Ok(SemanticComponent::Button),
        "Link" => Ok(SemanticComponent::Link),
        "a" => Ok(component_for_anchor_element(props)),
        "Label" | "label" => Ok(SemanticComponent::Label),
        "Document" | "html" => Ok(SemanticComponent::Document),
        "DocumentHead" | "head" => Ok(SemanticComponent::DocumentHead),
        "DocumentBody" | "body" => Ok(SemanticComponent::DocumentBody),
        "DocumentTitle" | "title" => Ok(SemanticComponent::DocumentTitle),
        "Metadata" | "base" | "meta" => Ok(SemanticComponent::Metadata),
        "ResourceLink" | "link" => Ok(SemanticComponent::ResourceLink),
        "StyleSheet" | "style" => Ok(SemanticComponent::StyleSheet),
        "Script" | "script" | "noscript" => Ok(SemanticComponent::Script),
        "Template" | "template" => Ok(SemanticComponent::Template),
        "Slot" | "slot" => Ok(SemanticComponent::Slot),
        "Abbreviation" | "abbr" | "acronym" => Ok(SemanticComponent::Abbreviation),
        "Citation" | "cite" => Ok(SemanticComponent::Citation),
        "Definition" | "dfn" => Ok(SemanticComponent::Definition),
        "DataValue" | "data" => Ok(SemanticComponent::DataValue),
        "InsertedText" | "ins" => Ok(SemanticComponent::InsertedText),
        "DeletedText" | "del" => Ok(SemanticComponent::DeletedText),
        "MarkedText" | "mark" => Ok(SemanticComponent::MarkedText),
        "Time" | "time" => Ok(SemanticComponent::Time),
        "Emphasis" | "em" => Ok(SemanticComponent::Emphasis),
        "StrongText" | "strong" => Ok(SemanticComponent::StrongText),
        "Code" | "code" => Ok(SemanticComponent::Code),
        "KeyboardInput" | "kbd" => Ok(SemanticComponent::KeyboardInput),
        "SampleOutput" | "samp" => Ok(SemanticComponent::SampleOutput),
        "Variable" | "var" => Ok(SemanticComponent::Variable),
        "InlineQuote" | "q" => Ok(SemanticComponent::InlineQuote),
        "Subscript" | "sub" => Ok(SemanticComponent::Subscript),
        "Superscript" | "sup" => Ok(SemanticComponent::Superscript),
        "SmallText" | "small" => Ok(SemanticComponent::SmallText),
        "BoldText" | "b" => Ok(SemanticComponent::BoldText),
        "ItalicText" | "i" => Ok(SemanticComponent::ItalicText),
        "StruckText" | "s" | "strike" => Ok(SemanticComponent::StruckText),
        "UnderlinedText" | "u" => Ok(SemanticComponent::UnderlinedText),
        "BidirectionalIsolate" | "bdi" => Ok(SemanticComponent::BidirectionalIsolate),
        "BidirectionalOverride" | "bdo" => Ok(SemanticComponent::BidirectionalOverride),
        "Paragraph" | "p" => Ok(SemanticComponent::Paragraph),
        "PreformattedText" | "pre" | "listing" | "plaintext" | "xmp" => {
            Ok(SemanticComponent::PreformattedText)
        }
        "BlockQuote" | "blockquote" => Ok(SemanticComponent::BlockQuote),
        "ContactAddress" | "address" => Ok(SemanticComponent::ContactAddress),
        "LineBreak" | "br" => Ok(SemanticComponent::LineBreak),
        "WordBreakOpportunity" | "wbr" => Ok(SemanticComponent::WordBreakOpportunity),
        "NoBreakText" | "nobr" => Ok(SemanticComponent::NoBreakText),
        "CenteredText" | "center" => Ok(SemanticComponent::CenteredText),
        "FontText" | "font" | "basefont" => Ok(SemanticComponent::FontText),
        "BigText" | "big" => Ok(SemanticComponent::BigText),
        "TeletypeText" | "tt" => Ok(SemanticComponent::TeletypeText),
        "Applet" | "applet" => Ok(SemanticComponent::Applet),
        "BackgroundSound" | "bgsound" => Ok(SemanticComponent::BackgroundSound),
        "Frame" | "frame" => Ok(SemanticComponent::Frame),
        "FrameSet" | "frameset" => Ok(SemanticComponent::FrameSet),
        "NoEmbedFallback" | "noembed" => Ok(SemanticComponent::NoEmbedFallback),
        "NoFramesFallback" | "noframes" => Ok(SemanticComponent::NoFramesFallback),
        "Marquee" | "marquee" => Ok(SemanticComponent::Marquee),
        "Math" | "math" => Ok(SemanticComponent::Math),
        "NextId" | "nextid" => Ok(SemanticComponent::NextId),
        "SelectedContent" | "selectedcontent" => Ok(SemanticComponent::SelectedContent),
        "Text" | "span" => Ok(SemanticComponent::Text),
        "Heading" => Ok(SemanticComponent::Heading),
        "HeadingGroup" | "hgroup" => Ok(SemanticComponent::HeadingGroup),
        "Ruby" | "ruby" => Ok(SemanticComponent::Ruby),
        "RubyBase" | "rb" => Ok(SemanticComponent::RubyBase),
        "RubyText" | "rt" => Ok(SemanticComponent::RubyText),
        "RubyParenthesis" | "rp" => Ok(SemanticComponent::RubyParenthesis),
        "RubyTextContainer" | "rtc" => Ok(SemanticComponent::RubyTextContainer),
        "TextField" => Ok(SemanticComponent::TextField),
        "Input" | "textarea" => Ok(SemanticComponent::Input),
        "input" => component_for_intrinsic_tag(tag, &component_attributes_for_tag(tag, props))
            .ok_or_else(|| GuiError::UnsupportedSemanticComponent {
                component: tag.to_string(),
            }),
        "Checkbox" => Ok(SemanticComponent::Checkbox),
        "Switch" => Ok(SemanticComponent::Switch),
        "RadioGroup" => Ok(SemanticComponent::RadioGroup),
        "Radio" => Ok(SemanticComponent::Radio),
        "Form" | "form" => Ok(SemanticComponent::Form),
        "FieldSet" | "fieldset" => Ok(SemanticComponent::FieldSet),
        "Legend" | "legend" => Ok(SemanticComponent::Legend),
        "OptionGroup" | "optgroup" => Ok(SemanticComponent::OptionGroup),
        "Output" | "output" => Ok(SemanticComponent::Output),
        "Meter" | "meter" => Ok(SemanticComponent::Meter),
        "ImageMap" | "map" => Ok(SemanticComponent::ImageMap),
        "ImageMapArea" | "area" => Ok(SemanticComponent::ImageMapArea),
        "ComboBox" => Ok(SemanticComponent::ComboBox),
        "Select" | "select" => Ok(SemanticComponent::Select),
        "SelectValue" => Ok(SemanticComponent::SelectValue),
        "ListBox" | "ul" | "ol" | "datalist" | "dir" => Ok(SemanticComponent::ListBox),
        "ListBoxItem" | "option" | "li" => Ok(SemanticComponent::ListBoxItem),
        "Dialog" | "dialog" => Ok(SemanticComponent::Dialog),
        "Popover" => Ok(SemanticComponent::Popover),
        "Tabs" => Ok(SemanticComponent::Tabs),
        "TabList" => Ok(SemanticComponent::TabList),
        "Tab" => Ok(SemanticComponent::Tab),
        "TabPanel" => Ok(SemanticComponent::TabPanel),
        "Main" => Ok(SemanticComponent::Main),
        "Navigation" => Ok(SemanticComponent::Navigation),
        "Header" => Ok(SemanticComponent::Header),
        "Footer" => Ok(SemanticComponent::Footer),
        "Article" => Ok(SemanticComponent::Article),
        "Section" => Ok(SemanticComponent::Section),
        "Aside" => Ok(SemanticComponent::Aside),
        "Search" => Ok(SemanticComponent::Search),
        "Disclosure" | "details" => Ok(SemanticComponent::Disclosure),
        "DisclosureSummary" | "summary" => Ok(SemanticComponent::DisclosureSummary),
        "Figure" | "figure" => Ok(SemanticComponent::Figure),
        "FigureCaption" | "figcaption" => Ok(SemanticComponent::FigureCaption),
        "DescriptionList" | "dl" => Ok(SemanticComponent::DescriptionList),
        "DescriptionTerm" | "dt" => Ok(SemanticComponent::DescriptionTerm),
        "DescriptionDetails" | "dd" => Ok(SemanticComponent::DescriptionDetails),
        "Group" | "div" => Ok(SemanticComponent::Group),
        "Menu" => Ok(SemanticComponent::Menu),
        "MenuItem" => Ok(SemanticComponent::MenuItem),
        "Separator" | "hr" => Ok(SemanticComponent::Separator),
        "Slider" => Ok(SemanticComponent::Slider),
        "ProgressBar" | "progress" => Ok(SemanticComponent::ProgressBar),
        "Toolbar" => Ok(SemanticComponent::Toolbar),
        other => component_for_intrinsic_tag(other, &component_attributes_for_tag(other, props))
            .or_else(|| component_for_svg_tag(other))
            .ok_or_else(|| GuiError::UnsupportedSemanticComponent {
                component: other.to_string(),
            }),
    }
}

fn component_for_anchor_element(props: &CompiledProps) -> SemanticComponent {
    if component_for_intrinsic_tag("a", &component_attributes_for_tag("a", props))
        == Some(SemanticComponent::Link)
    {
        SemanticComponent::Link
    } else if props.events.contains_key("onClick") || props.events.contains_key("onPress") {
        SemanticComponent::Button
    } else {
        SemanticComponent::Group
    }
}

fn component_attributes_for_tag(tag: &str, props: &CompiledProps) -> BTreeMap<String, String> {
    let mut attributes = props.attributes.clone();
    match tag {
        "a" => {
            if !attributes.contains_key("href") {
                if let Some(href) = &props.href {
                    attributes.insert("href".to_string(), href.clone());
                }
            }
        }
        "input" => {
            if !attributes.contains_key("type") {
                if let Some(input_type) = &props.input_type {
                    attributes.insert("type".to_string(), input_type.clone());
                }
            }
        }
        _ => {}
    }
    attributes
}
