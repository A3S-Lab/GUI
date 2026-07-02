use crate::error::{GuiError, GuiResult};
use crate::html::component_for_intrinsic_tag;
use crate::react_aria::AriaComponent;
use crate::svg::component_for_svg_tag;

use super::CompiledProps;

pub(super) fn component_from_jsx_tag(tag: &str, props: &CompiledProps) -> GuiResult<AriaComponent> {
    match tag {
        "Button" | "button" => Ok(AriaComponent::Button),
        "Link" => Ok(AriaComponent::Link),
        "a" => Ok(component_for_anchor_element(props)),
        "Label" | "label" => Ok(AriaComponent::Label),
        "Document" | "html" => Ok(AriaComponent::Document),
        "DocumentHead" | "head" => Ok(AriaComponent::DocumentHead),
        "DocumentBody" | "body" => Ok(AriaComponent::DocumentBody),
        "DocumentTitle" | "title" => Ok(AriaComponent::DocumentTitle),
        "Metadata" | "base" | "meta" => Ok(AriaComponent::Metadata),
        "ResourceLink" | "link" => Ok(AriaComponent::ResourceLink),
        "StyleSheet" | "style" => Ok(AriaComponent::StyleSheet),
        "Script" | "script" | "noscript" => Ok(AriaComponent::Script),
        "Template" | "template" => Ok(AriaComponent::Template),
        "Slot" | "slot" => Ok(AriaComponent::Slot),
        "Abbreviation" | "abbr" | "acronym" => Ok(AriaComponent::Abbreviation),
        "Citation" | "cite" => Ok(AriaComponent::Citation),
        "Definition" | "dfn" => Ok(AriaComponent::Definition),
        "DataValue" | "data" => Ok(AriaComponent::DataValue),
        "InsertedText" | "ins" => Ok(AriaComponent::InsertedText),
        "DeletedText" | "del" => Ok(AriaComponent::DeletedText),
        "MarkedText" | "mark" => Ok(AriaComponent::MarkedText),
        "Time" | "time" => Ok(AriaComponent::Time),
        "Emphasis" | "em" => Ok(AriaComponent::Emphasis),
        "StrongText" | "strong" => Ok(AriaComponent::StrongText),
        "Code" | "code" => Ok(AriaComponent::Code),
        "KeyboardInput" | "kbd" => Ok(AriaComponent::KeyboardInput),
        "SampleOutput" | "samp" => Ok(AriaComponent::SampleOutput),
        "Variable" | "var" => Ok(AriaComponent::Variable),
        "InlineQuote" | "q" => Ok(AriaComponent::InlineQuote),
        "Subscript" | "sub" => Ok(AriaComponent::Subscript),
        "Superscript" | "sup" => Ok(AriaComponent::Superscript),
        "SmallText" | "small" => Ok(AriaComponent::SmallText),
        "BoldText" | "b" => Ok(AriaComponent::BoldText),
        "ItalicText" | "i" => Ok(AriaComponent::ItalicText),
        "StruckText" | "s" | "strike" => Ok(AriaComponent::StruckText),
        "UnderlinedText" | "u" => Ok(AriaComponent::UnderlinedText),
        "BidirectionalIsolate" | "bdi" => Ok(AriaComponent::BidirectionalIsolate),
        "BidirectionalOverride" | "bdo" => Ok(AriaComponent::BidirectionalOverride),
        "Paragraph" | "p" => Ok(AriaComponent::Paragraph),
        "PreformattedText" | "pre" | "listing" | "plaintext" | "xmp" => {
            Ok(AriaComponent::PreformattedText)
        }
        "BlockQuote" | "blockquote" => Ok(AriaComponent::BlockQuote),
        "ContactAddress" | "address" => Ok(AriaComponent::ContactAddress),
        "LineBreak" | "br" => Ok(AriaComponent::LineBreak),
        "WordBreakOpportunity" | "wbr" => Ok(AriaComponent::WordBreakOpportunity),
        "NoBreakText" | "nobr" => Ok(AriaComponent::NoBreakText),
        "CenteredText" | "center" => Ok(AriaComponent::CenteredText),
        "FontText" | "font" | "basefont" => Ok(AriaComponent::FontText),
        "BigText" | "big" => Ok(AriaComponent::BigText),
        "TeletypeText" | "tt" => Ok(AriaComponent::TeletypeText),
        "Applet" | "applet" => Ok(AriaComponent::Applet),
        "BackgroundSound" | "bgsound" => Ok(AriaComponent::BackgroundSound),
        "Frame" | "frame" => Ok(AriaComponent::Frame),
        "FrameSet" | "frameset" => Ok(AriaComponent::FrameSet),
        "NoEmbedFallback" | "noembed" => Ok(AriaComponent::NoEmbedFallback),
        "NoFramesFallback" | "noframes" => Ok(AriaComponent::NoFramesFallback),
        "Marquee" | "marquee" => Ok(AriaComponent::Marquee),
        "Math" | "math" => Ok(AriaComponent::Math),
        "NextId" | "nextid" => Ok(AriaComponent::NextId),
        "SelectedContent" | "selectedcontent" => Ok(AriaComponent::SelectedContent),
        "Text" | "span" => Ok(AriaComponent::Text),
        "Heading" => Ok(AriaComponent::Heading),
        "HeadingGroup" | "hgroup" => Ok(AriaComponent::HeadingGroup),
        "Ruby" | "ruby" => Ok(AriaComponent::Ruby),
        "RubyBase" | "rb" => Ok(AriaComponent::RubyBase),
        "RubyText" | "rt" => Ok(AriaComponent::RubyText),
        "RubyParenthesis" | "rp" => Ok(AriaComponent::RubyParenthesis),
        "RubyTextContainer" | "rtc" => Ok(AriaComponent::RubyTextContainer),
        "TextField" => Ok(AriaComponent::TextField),
        "Input" | "textarea" => Ok(AriaComponent::Input),
        "input" => component_for_intrinsic_tag(tag, &props.attributes).ok_or_else(|| {
            GuiError::UnsupportedAriaComponent {
                component: tag.to_string(),
            }
        }),
        "Checkbox" => Ok(AriaComponent::Checkbox),
        "Switch" => Ok(AriaComponent::Switch),
        "RadioGroup" => Ok(AriaComponent::RadioGroup),
        "Radio" => Ok(AriaComponent::Radio),
        "Form" | "form" => Ok(AriaComponent::Form),
        "FieldSet" | "fieldset" => Ok(AriaComponent::FieldSet),
        "Legend" | "legend" => Ok(AriaComponent::Legend),
        "OptionGroup" | "optgroup" => Ok(AriaComponent::OptionGroup),
        "Output" | "output" => Ok(AriaComponent::Output),
        "Meter" | "meter" => Ok(AriaComponent::Meter),
        "ImageMap" | "map" => Ok(AriaComponent::ImageMap),
        "ImageMapArea" | "area" => Ok(AriaComponent::ImageMapArea),
        "Select" | "select" => Ok(AriaComponent::Select),
        "SelectValue" => Ok(AriaComponent::SelectValue),
        "ListBox" | "ul" | "ol" | "datalist" | "dir" => Ok(AriaComponent::ListBox),
        "ListBoxItem" | "option" | "li" => Ok(AriaComponent::ListBoxItem),
        "Dialog" | "dialog" => Ok(AriaComponent::Dialog),
        "Popover" => Ok(AriaComponent::Popover),
        "Tabs" => Ok(AriaComponent::Tabs),
        "TabList" => Ok(AriaComponent::TabList),
        "Tab" => Ok(AriaComponent::Tab),
        "TabPanel" => Ok(AriaComponent::TabPanel),
        "Main" => Ok(AriaComponent::Main),
        "Navigation" => Ok(AriaComponent::Navigation),
        "Header" => Ok(AriaComponent::Header),
        "Footer" => Ok(AriaComponent::Footer),
        "Article" => Ok(AriaComponent::Article),
        "Section" => Ok(AriaComponent::Section),
        "Aside" => Ok(AriaComponent::Aside),
        "Search" => Ok(AriaComponent::Search),
        "Disclosure" | "details" => Ok(AriaComponent::Disclosure),
        "DisclosureSummary" | "summary" => Ok(AriaComponent::DisclosureSummary),
        "Figure" | "figure" => Ok(AriaComponent::Figure),
        "FigureCaption" | "figcaption" => Ok(AriaComponent::FigureCaption),
        "DescriptionList" | "dl" => Ok(AriaComponent::DescriptionList),
        "DescriptionTerm" | "dt" => Ok(AriaComponent::DescriptionTerm),
        "DescriptionDetails" | "dd" => Ok(AriaComponent::DescriptionDetails),
        "Group" | "div" => Ok(AriaComponent::Group),
        "Menu" => Ok(AriaComponent::Menu),
        "MenuItem" => Ok(AriaComponent::MenuItem),
        "Separator" | "hr" => Ok(AriaComponent::Separator),
        "Slider" => Ok(AriaComponent::Slider),
        "ProgressBar" | "progress" => Ok(AriaComponent::ProgressBar),
        "Toolbar" => Ok(AriaComponent::Toolbar),
        other => component_for_intrinsic_tag(other, &props.attributes)
            .or_else(|| component_for_svg_tag(other))
            .ok_or_else(|| GuiError::UnsupportedAriaComponent {
                component: other.to_string(),
            }),
    }
}

fn component_for_anchor_element(props: &CompiledProps) -> AriaComponent {
    if component_for_intrinsic_tag("a", &props.attributes) == Some(AriaComponent::Link) {
        AriaComponent::Link
    } else if props.events.contains_key("onClick") || props.events.contains_key("onPress") {
        AriaComponent::Button
    } else {
        AriaComponent::Group
    }
}
