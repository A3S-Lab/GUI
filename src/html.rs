use std::collections::BTreeMap;

use crate::react_aria::AriaComponent;

mod collections;
mod registry;
mod resource_policy;

pub use collections::HtmlCollectionProps;
pub use registry::{HTML_CONFORMING_ELEMENTS, HTML_ELEMENTS};
pub use resource_policy::HtmlResourcePolicyProps;

pub const HTML_TAG_METADATA_KEY: &str = "data-a3s-html-tag";

pub fn is_html_element(tag: &str) -> bool {
    canonical_html_tag(tag).is_some()
}

pub fn is_custom_element(tag: &str) -> bool {
    tag.contains('-')
        && tag
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
}

pub fn canonical_html_tag(tag: &str) -> Option<&'static str> {
    let tag = tag.trim().to_ascii_lowercase();
    HTML_ELEMENTS
        .iter()
        .copied()
        .find(|candidate| *candidate == tag)
}

pub fn component_for_html_tag(
    tag: &str,
    attributes: &BTreeMap<String, String>,
) -> Option<AriaComponent> {
    let tag = canonical_html_tag(tag)?;
    Some(match tag {
        "button" => AriaComponent::Button,
        "label" => AriaComponent::Label,
        "legend" => AriaComponent::Legend,
        "input" => component_for_input_type(attributes.get("type").map(String::as_str)),
        "textarea" => AriaComponent::Input,
        "select" => AriaComponent::Select,
        "optgroup" => AriaComponent::OptionGroup,
        "option" => AriaComponent::ListBoxItem,
        "ul" | "ol" | "datalist" | "dir" => AriaComponent::ListBox,
        "li" => AriaComponent::ListBoxItem,
        "html" => AriaComponent::Document,
        "head" => AriaComponent::DocumentHead,
        "body" => AriaComponent::DocumentBody,
        "title" => AriaComponent::DocumentTitle,
        "base" | "meta" => AriaComponent::Metadata,
        "link" => AriaComponent::ResourceLink,
        "style" => AriaComponent::StyleSheet,
        "script" | "noscript" => AriaComponent::Script,
        "template" => AriaComponent::Template,
        "slot" => AriaComponent::Slot,
        "abbr" | "acronym" => AriaComponent::Abbreviation,
        "cite" => AriaComponent::Citation,
        "dfn" => AriaComponent::Definition,
        "data" => AriaComponent::DataValue,
        "ins" => AriaComponent::InsertedText,
        "del" => AriaComponent::DeletedText,
        "mark" => AriaComponent::MarkedText,
        "time" => AriaComponent::Time,
        "em" => AriaComponent::Emphasis,
        "strong" => AriaComponent::StrongText,
        "code" => AriaComponent::Code,
        "kbd" => AriaComponent::KeyboardInput,
        "samp" => AriaComponent::SampleOutput,
        "var" => AriaComponent::Variable,
        "q" => AriaComponent::InlineQuote,
        "sub" => AriaComponent::Subscript,
        "sup" => AriaComponent::Superscript,
        "small" => AriaComponent::SmallText,
        "b" => AriaComponent::BoldText,
        "i" => AriaComponent::ItalicText,
        "s" | "strike" => AriaComponent::StruckText,
        "u" => AriaComponent::UnderlinedText,
        "bdi" => AriaComponent::BidirectionalIsolate,
        "bdo" => AriaComponent::BidirectionalOverride,
        "p" => AriaComponent::Paragraph,
        "pre" | "listing" | "plaintext" | "xmp" => AriaComponent::PreformattedText,
        "blockquote" => AriaComponent::BlockQuote,
        "address" => AriaComponent::ContactAddress,
        "br" => AriaComponent::LineBreak,
        "wbr" => AriaComponent::WordBreakOpportunity,
        "nobr" => AriaComponent::NoBreakText,
        "center" => AriaComponent::CenteredText,
        "font" | "basefont" => AriaComponent::FontText,
        "big" => AriaComponent::BigText,
        "tt" => AriaComponent::TeletypeText,
        "applet" => AriaComponent::Applet,
        "bgsound" => AriaComponent::BackgroundSound,
        "frame" => AriaComponent::Frame,
        "frameset" => AriaComponent::FrameSet,
        "noembed" => AriaComponent::NoEmbedFallback,
        "noframes" => AriaComponent::NoFramesFallback,
        "marquee" => AriaComponent::Marquee,
        "math" => AriaComponent::Math,
        "nextid" => AriaComponent::NextId,
        "selectedcontent" => AriaComponent::SelectedContent,
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => AriaComponent::Heading,
        "hgroup" => AriaComponent::HeadingGroup,
        "ruby" => AriaComponent::Ruby,
        "rb" => AriaComponent::RubyBase,
        "rt" => AriaComponent::RubyText,
        "rp" => AriaComponent::RubyParenthesis,
        "rtc" => AriaComponent::RubyTextContainer,
        "main" => AriaComponent::Main,
        "nav" => AriaComponent::Navigation,
        "header" => AriaComponent::Header,
        "footer" => AriaComponent::Footer,
        "article" => AriaComponent::Article,
        "section" => AriaComponent::Section,
        "aside" => AriaComponent::Aside,
        "search" => AriaComponent::Search,
        "details" => AriaComponent::Disclosure,
        "summary" => AriaComponent::DisclosureSummary,
        "figure" => AriaComponent::Figure,
        "figcaption" => AriaComponent::FigureCaption,
        "dl" => AriaComponent::DescriptionList,
        "dt" => AriaComponent::DescriptionTerm,
        "dd" => AriaComponent::DescriptionDetails,
        "img" | "picture" => AriaComponent::Image,
        "audio" | "video" => AriaComponent::Media,
        "canvas" => AriaComponent::Canvas,
        "embed" | "iframe" | "object" | "source" | "track" | "param" => {
            AriaComponent::EmbeddedContent
        }
        "table" => AriaComponent::Table,
        "thead" | "tbody" | "tfoot" | "colgroup" => AriaComponent::TableSection,
        "tr" => AriaComponent::TableRow,
        "td" | "th" => AriaComponent::TableCell,
        "col" => AriaComponent::TableColumn,
        "caption" => AriaComponent::TableCaption,
        "dialog" => AriaComponent::Dialog,
        "menu" => AriaComponent::Menu,
        "hr" => AriaComponent::Separator,
        "meter" => AriaComponent::Meter,
        "progress" => AriaComponent::ProgressBar,
        "fieldset" => AriaComponent::FieldSet,
        "output" => AriaComponent::Output,
        "form" => AriaComponent::Form,
        "a" => component_for_anchor(attributes),
        "map" => AriaComponent::ImageMap,
        "area" => AriaComponent::ImageMapArea,
        tag if is_text_html_tag(tag) => AriaComponent::Text,
        _ => AriaComponent::Group,
    })
}

pub fn component_for_intrinsic_tag(
    tag: &str,
    attributes: &BTreeMap<String, String>,
) -> Option<AriaComponent> {
    component_for_html_tag(tag, attributes).or_else(|| {
        if is_custom_element(tag) {
            Some(AriaComponent::Group)
        } else {
            None
        }
    })
}

fn component_for_input_type(input_type: Option<&str>) -> AriaComponent {
    match input_type
        .unwrap_or("text")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "button" | "submit" | "reset" | "image" => AriaComponent::Button,
        "checkbox" => AriaComponent::Checkbox,
        "radio" => AriaComponent::Radio,
        "range" => AriaComponent::Slider,
        _ => AriaComponent::Input,
    }
}

fn component_for_anchor(attributes: &BTreeMap<String, String>) -> AriaComponent {
    if attributes
        .get("href")
        .map(String::as_str)
        .is_some_and(|value| !value.trim().is_empty())
    {
        AriaComponent::Link
    } else {
        AriaComponent::Group
    }
}

fn is_text_html_tag(tag: &str) -> bool {
    matches!(tag, "span")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn assert_unique_sorted(elements: &[&str]) {
        let mut seen = BTreeSet::new();
        for element in elements {
            assert!(seen.insert(*element), "{element} should be listed once");
        }
        for pair in elements.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} should sort before {}",
                pair[0],
                pair[1]
            );
        }
    }

    #[test]
    fn conforming_html_elements_are_a_registry_baseline() {
        assert_unique_sorted(HTML_CONFORMING_ELEMENTS);
        assert_unique_sorted(HTML_ELEMENTS);

        for tag in HTML_CONFORMING_ELEMENTS {
            assert!(
                HTML_ELEMENTS.contains(tag),
                "{tag} should be included in the full HTML registry"
            );
            assert_eq!(canonical_html_tag(tag), Some(*tag));
        }
    }

    #[test]
    fn recognizes_html_elements_case_insensitively() {
        assert!(is_html_element("DIV"));
        assert!(is_html_element("selectedcontent"));
        assert!(is_html_element("marquee"));
        assert!(!is_html_element("not-html"));
    }

    #[test]
    fn maps_input_types_to_native_semantics() {
        assert_eq!(
            component_for_html_tag(
                "input",
                &BTreeMap::from([("type".into(), "checkbox".into())])
            ),
            Some(AriaComponent::Checkbox)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "range".into())])),
            Some(AriaComponent::Slider)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "submit".into())])),
            Some(AriaComponent::Button)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "reset".into())])),
            Some(AriaComponent::Button)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "button".into())])),
            Some(AriaComponent::Button)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "image".into())])),
            Some(AriaComponent::Button)
        );
    }

    #[test]
    fn maps_form_grouping_and_value_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("form", &attributes),
            Some(AriaComponent::Form)
        );
        assert_eq!(
            component_for_html_tag("fieldset", &attributes),
            Some(AriaComponent::FieldSet)
        );
        assert_eq!(
            component_for_html_tag("legend", &attributes),
            Some(AriaComponent::Legend)
        );
        assert_eq!(
            component_for_html_tag("optgroup", &attributes),
            Some(AriaComponent::OptionGroup)
        );
        assert_eq!(
            component_for_html_tag("output", &attributes),
            Some(AriaComponent::Output)
        );
        assert_eq!(
            component_for_html_tag("meter", &attributes),
            Some(AriaComponent::Meter)
        );
        assert_eq!(
            component_for_html_tag("progress", &attributes),
            Some(AriaComponent::ProgressBar)
        );
    }

    #[test]
    fn maps_link_and_image_map_tags_to_native_semantics() {
        let empty_attributes = BTreeMap::new();
        let href_attributes = BTreeMap::from([("href".to_string(), "/docs".to_string())]);

        assert_eq!(
            component_for_html_tag("a", &href_attributes),
            Some(AriaComponent::Link)
        );
        assert_eq!(
            component_for_html_tag("a", &empty_attributes),
            Some(AriaComponent::Group)
        );
        assert_eq!(
            component_for_html_tag("map", &empty_attributes),
            Some(AriaComponent::ImageMap)
        );
        assert_eq!(
            component_for_html_tag("area", &empty_attributes),
            Some(AriaComponent::ImageMapArea)
        );
    }

    #[test]
    fn maps_document_metadata_template_and_slot_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("html", &attributes),
            Some(AriaComponent::Document)
        );
        assert_eq!(
            component_for_html_tag("head", &attributes),
            Some(AriaComponent::DocumentHead)
        );
        assert_eq!(
            component_for_html_tag("body", &attributes),
            Some(AriaComponent::DocumentBody)
        );
        assert_eq!(
            component_for_html_tag("title", &attributes),
            Some(AriaComponent::DocumentTitle)
        );
        assert_eq!(
            component_for_html_tag("base", &attributes),
            Some(AriaComponent::Metadata)
        );
        assert_eq!(
            component_for_html_tag("meta", &attributes),
            Some(AriaComponent::Metadata)
        );
        assert_eq!(
            component_for_html_tag("link", &attributes),
            Some(AriaComponent::ResourceLink)
        );
        assert_eq!(
            component_for_html_tag("style", &attributes),
            Some(AriaComponent::StyleSheet)
        );
        assert_eq!(
            component_for_html_tag("script", &attributes),
            Some(AriaComponent::Script)
        );
        assert_eq!(
            component_for_html_tag("noscript", &attributes),
            Some(AriaComponent::Script)
        );
        assert_eq!(
            component_for_html_tag("template", &attributes),
            Some(AriaComponent::Template)
        );
        assert_eq!(
            component_for_html_tag("slot", &attributes),
            Some(AriaComponent::Slot)
        );
        assert_eq!(
            component_for_html_tag("hgroup", &attributes),
            Some(AriaComponent::HeadingGroup)
        );
    }

    #[test]
    fn maps_ruby_annotation_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("ruby", &attributes),
            Some(AriaComponent::Ruby)
        );
        assert_eq!(
            component_for_html_tag("rb", &attributes),
            Some(AriaComponent::RubyBase)
        );
        assert_eq!(
            component_for_html_tag("rt", &attributes),
            Some(AriaComponent::RubyText)
        );
        assert_eq!(
            component_for_html_tag("rp", &attributes),
            Some(AriaComponent::RubyParenthesis)
        );
        assert_eq!(
            component_for_html_tag("rtc", &attributes),
            Some(AriaComponent::RubyTextContainer)
        );
    }

    #[test]
    fn maps_text_annotation_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("abbr", &attributes),
            Some(AriaComponent::Abbreviation)
        );
        assert_eq!(
            component_for_html_tag("acronym", &attributes),
            Some(AriaComponent::Abbreviation)
        );
        assert_eq!(
            component_for_html_tag("cite", &attributes),
            Some(AriaComponent::Citation)
        );
        assert_eq!(
            component_for_html_tag("dfn", &attributes),
            Some(AriaComponent::Definition)
        );
        assert_eq!(
            component_for_html_tag("data", &attributes),
            Some(AriaComponent::DataValue)
        );
        assert_eq!(
            component_for_html_tag("ins", &attributes),
            Some(AriaComponent::InsertedText)
        );
        assert_eq!(
            component_for_html_tag("del", &attributes),
            Some(AriaComponent::DeletedText)
        );
        assert_eq!(
            component_for_html_tag("mark", &attributes),
            Some(AriaComponent::MarkedText)
        );
        assert_eq!(
            component_for_html_tag("time", &attributes),
            Some(AriaComponent::Time)
        );
    }

    #[test]
    fn maps_phrasing_text_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("em", &attributes),
            Some(AriaComponent::Emphasis)
        );
        assert_eq!(
            component_for_html_tag("strong", &attributes),
            Some(AriaComponent::StrongText)
        );
        assert_eq!(
            component_for_html_tag("code", &attributes),
            Some(AriaComponent::Code)
        );
        assert_eq!(
            component_for_html_tag("kbd", &attributes),
            Some(AriaComponent::KeyboardInput)
        );
        assert_eq!(
            component_for_html_tag("samp", &attributes),
            Some(AriaComponent::SampleOutput)
        );
        assert_eq!(
            component_for_html_tag("var", &attributes),
            Some(AriaComponent::Variable)
        );
        assert_eq!(
            component_for_html_tag("q", &attributes),
            Some(AriaComponent::InlineQuote)
        );
        assert_eq!(
            component_for_html_tag("sub", &attributes),
            Some(AriaComponent::Subscript)
        );
        assert_eq!(
            component_for_html_tag("sup", &attributes),
            Some(AriaComponent::Superscript)
        );
        assert_eq!(
            component_for_html_tag("small", &attributes),
            Some(AriaComponent::SmallText)
        );
        assert_eq!(
            component_for_html_tag("b", &attributes),
            Some(AriaComponent::BoldText)
        );
        assert_eq!(
            component_for_html_tag("i", &attributes),
            Some(AriaComponent::ItalicText)
        );
        assert_eq!(
            component_for_html_tag("s", &attributes),
            Some(AriaComponent::StruckText)
        );
        assert_eq!(
            component_for_html_tag("strike", &attributes),
            Some(AriaComponent::StruckText)
        );
        assert_eq!(
            component_for_html_tag("u", &attributes),
            Some(AriaComponent::UnderlinedText)
        );
        assert_eq!(
            component_for_html_tag("bdi", &attributes),
            Some(AriaComponent::BidirectionalIsolate)
        );
        assert_eq!(
            component_for_html_tag("bdo", &attributes),
            Some(AriaComponent::BidirectionalOverride)
        );
    }

    #[test]
    fn maps_flow_and_legacy_text_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("p", &attributes),
            Some(AriaComponent::Paragraph)
        );
        assert_eq!(
            component_for_html_tag("pre", &attributes),
            Some(AriaComponent::PreformattedText)
        );
        assert_eq!(
            component_for_html_tag("listing", &attributes),
            Some(AriaComponent::PreformattedText)
        );
        assert_eq!(
            component_for_html_tag("plaintext", &attributes),
            Some(AriaComponent::PreformattedText)
        );
        assert_eq!(
            component_for_html_tag("xmp", &attributes),
            Some(AriaComponent::PreformattedText)
        );
        assert_eq!(
            component_for_html_tag("blockquote", &attributes),
            Some(AriaComponent::BlockQuote)
        );
        assert_eq!(
            component_for_html_tag("address", &attributes),
            Some(AriaComponent::ContactAddress)
        );
        assert_eq!(
            component_for_html_tag("br", &attributes),
            Some(AriaComponent::LineBreak)
        );
        assert_eq!(
            component_for_html_tag("wbr", &attributes),
            Some(AriaComponent::WordBreakOpportunity)
        );
        assert_eq!(
            component_for_html_tag("nobr", &attributes),
            Some(AriaComponent::NoBreakText)
        );
        assert_eq!(
            component_for_html_tag("center", &attributes),
            Some(AriaComponent::CenteredText)
        );
        assert_eq!(
            component_for_html_tag("font", &attributes),
            Some(AriaComponent::FontText)
        );
        assert_eq!(
            component_for_html_tag("basefont", &attributes),
            Some(AriaComponent::FontText)
        );
        assert_eq!(
            component_for_html_tag("big", &attributes),
            Some(AriaComponent::BigText)
        );
        assert_eq!(
            component_for_html_tag("tt", &attributes),
            Some(AriaComponent::TeletypeText)
        );
        assert_eq!(
            component_for_html_tag("dir", &attributes),
            Some(AriaComponent::ListBox)
        );
    }

    #[test]
    fn maps_remaining_legacy_and_foreign_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("applet", &attributes),
            Some(AriaComponent::Applet)
        );
        assert_eq!(
            component_for_html_tag("bgsound", &attributes),
            Some(AriaComponent::BackgroundSound)
        );
        assert_eq!(
            component_for_html_tag("frame", &attributes),
            Some(AriaComponent::Frame)
        );
        assert_eq!(
            component_for_html_tag("frameset", &attributes),
            Some(AriaComponent::FrameSet)
        );
        assert_eq!(
            component_for_html_tag("noembed", &attributes),
            Some(AriaComponent::NoEmbedFallback)
        );
        assert_eq!(
            component_for_html_tag("noframes", &attributes),
            Some(AriaComponent::NoFramesFallback)
        );
        assert_eq!(
            component_for_html_tag("marquee", &attributes),
            Some(AriaComponent::Marquee)
        );
        assert_eq!(
            component_for_html_tag("math", &attributes),
            Some(AriaComponent::Math)
        );
        assert_eq!(
            component_for_html_tag("nextid", &attributes),
            Some(AriaComponent::NextId)
        );
        assert_eq!(
            component_for_html_tag("selectedcontent", &attributes),
            Some(AriaComponent::SelectedContent)
        );
    }

    #[test]
    fn maps_embedded_media_and_table_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("img", &attributes),
            Some(AriaComponent::Image)
        );
        assert_eq!(
            component_for_html_tag("picture", &attributes),
            Some(AriaComponent::Image)
        );
        assert_eq!(
            component_for_html_tag("video", &attributes),
            Some(AriaComponent::Media)
        );
        assert_eq!(
            component_for_html_tag("audio", &attributes),
            Some(AriaComponent::Media)
        );
        assert_eq!(
            component_for_html_tag("canvas", &attributes),
            Some(AriaComponent::Canvas)
        );
        assert_eq!(
            component_for_html_tag("iframe", &attributes),
            Some(AriaComponent::EmbeddedContent)
        );
        assert_eq!(
            component_for_html_tag("table", &attributes),
            Some(AriaComponent::Table)
        );
        assert_eq!(
            component_for_html_tag("tbody", &attributes),
            Some(AriaComponent::TableSection)
        );
        assert_eq!(
            component_for_html_tag("tr", &attributes),
            Some(AriaComponent::TableRow)
        );
        assert_eq!(
            component_for_html_tag("td", &attributes),
            Some(AriaComponent::TableCell)
        );
        assert_eq!(
            component_for_html_tag("col", &attributes),
            Some(AriaComponent::TableColumn)
        );
        assert_eq!(
            component_for_html_tag("caption", &attributes),
            Some(AriaComponent::TableCaption)
        );
    }

    #[test]
    fn maps_sectioning_landmark_and_heading_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("h1", &attributes),
            Some(AriaComponent::Heading)
        );
        assert_eq!(
            component_for_html_tag("main", &attributes),
            Some(AriaComponent::Main)
        );
        assert_eq!(
            component_for_html_tag("nav", &attributes),
            Some(AriaComponent::Navigation)
        );
        assert_eq!(
            component_for_html_tag("header", &attributes),
            Some(AriaComponent::Header)
        );
        assert_eq!(
            component_for_html_tag("footer", &attributes),
            Some(AriaComponent::Footer)
        );
        assert_eq!(
            component_for_html_tag("article", &attributes),
            Some(AriaComponent::Article)
        );
        assert_eq!(
            component_for_html_tag("section", &attributes),
            Some(AriaComponent::Section)
        );
        assert_eq!(
            component_for_html_tag("aside", &attributes),
            Some(AriaComponent::Aside)
        );
        assert_eq!(
            component_for_html_tag("search", &attributes),
            Some(AriaComponent::Search)
        );
    }

    #[test]
    fn maps_disclosure_figure_and_description_list_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("details", &attributes),
            Some(AriaComponent::Disclosure)
        );
        assert_eq!(
            component_for_html_tag("summary", &attributes),
            Some(AriaComponent::DisclosureSummary)
        );
        assert_eq!(
            component_for_html_tag("figure", &attributes),
            Some(AriaComponent::Figure)
        );
        assert_eq!(
            component_for_html_tag("figcaption", &attributes),
            Some(AriaComponent::FigureCaption)
        );
        assert_eq!(
            component_for_html_tag("dl", &attributes),
            Some(AriaComponent::DescriptionList)
        );
        assert_eq!(
            component_for_html_tag("dt", &attributes),
            Some(AriaComponent::DescriptionTerm)
        );
        assert_eq!(
            component_for_html_tag("dd", &attributes),
            Some(AriaComponent::DescriptionDetails)
        );
    }
}
