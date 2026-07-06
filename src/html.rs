use std::collections::BTreeMap;

use crate::semantic_ui::SemanticComponent;

mod activation;
mod collections;
mod dialog;
mod form_association;
mod microdata;
mod registry;
mod resource_policy;
mod shadow;
mod text_annotation;

pub use activation::HtmlActivationProps;
pub use collections::HtmlCollectionProps;
pub use dialog::HtmlDialogProps;
pub use form_association::HtmlFormAssociationProps;
pub use microdata::HtmlMicrodataProps;
pub use registry::{HTML_CONFORMING_ELEMENTS, HTML_ELEMENTS};
pub use resource_policy::HtmlResourcePolicyProps;
pub use shadow::HtmlShadowProps;
pub use text_annotation::HtmlTextAnnotationProps;

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
) -> Option<SemanticComponent> {
    let tag = canonical_html_tag(tag)?;
    Some(match tag {
        "button" => SemanticComponent::Button,
        "label" => SemanticComponent::Label,
        "legend" => SemanticComponent::Legend,
        "input" => component_for_input_type(attributes.get("type").map(String::as_str)),
        "textarea" => SemanticComponent::Input,
        "select" => SemanticComponent::Select,
        "optgroup" => SemanticComponent::OptionGroup,
        "option" => SemanticComponent::ListBoxItem,
        "ul" | "ol" | "datalist" | "dir" => SemanticComponent::ListBox,
        "li" => SemanticComponent::ListBoxItem,
        "html" => SemanticComponent::Document,
        "head" => SemanticComponent::DocumentHead,
        "body" => SemanticComponent::DocumentBody,
        "title" => SemanticComponent::DocumentTitle,
        "base" | "meta" => SemanticComponent::Metadata,
        "link" => SemanticComponent::ResourceLink,
        "style" => SemanticComponent::StyleSheet,
        "script" | "noscript" => SemanticComponent::Script,
        "template" => SemanticComponent::Template,
        "slot" => SemanticComponent::Slot,
        "abbr" | "acronym" => SemanticComponent::Abbreviation,
        "cite" => SemanticComponent::Citation,
        "dfn" => SemanticComponent::Definition,
        "data" => SemanticComponent::DataValue,
        "ins" => SemanticComponent::InsertedText,
        "del" => SemanticComponent::DeletedText,
        "mark" => SemanticComponent::MarkedText,
        "time" => SemanticComponent::Time,
        "em" => SemanticComponent::Emphasis,
        "strong" => SemanticComponent::StrongText,
        "code" => SemanticComponent::Code,
        "kbd" => SemanticComponent::KeyboardInput,
        "samp" => SemanticComponent::SampleOutput,
        "var" => SemanticComponent::Variable,
        "q" => SemanticComponent::InlineQuote,
        "sub" => SemanticComponent::Subscript,
        "sup" => SemanticComponent::Superscript,
        "small" => SemanticComponent::SmallText,
        "b" => SemanticComponent::BoldText,
        "i" => SemanticComponent::ItalicText,
        "s" | "strike" => SemanticComponent::StruckText,
        "u" => SemanticComponent::UnderlinedText,
        "bdi" => SemanticComponent::BidirectionalIsolate,
        "bdo" => SemanticComponent::BidirectionalOverride,
        "p" => SemanticComponent::Paragraph,
        "pre" | "listing" | "plaintext" | "xmp" => SemanticComponent::PreformattedText,
        "blockquote" => SemanticComponent::BlockQuote,
        "address" => SemanticComponent::ContactAddress,
        "br" => SemanticComponent::LineBreak,
        "wbr" => SemanticComponent::WordBreakOpportunity,
        "nobr" => SemanticComponent::NoBreakText,
        "center" => SemanticComponent::CenteredText,
        "font" | "basefont" => SemanticComponent::FontText,
        "big" => SemanticComponent::BigText,
        "tt" => SemanticComponent::TeletypeText,
        "applet" => SemanticComponent::Applet,
        "bgsound" => SemanticComponent::BackgroundSound,
        "frame" => SemanticComponent::Frame,
        "frameset" => SemanticComponent::FrameSet,
        "noembed" => SemanticComponent::NoEmbedFallback,
        "noframes" => SemanticComponent::NoFramesFallback,
        "marquee" => SemanticComponent::Marquee,
        "math" => SemanticComponent::Math,
        "nextid" => SemanticComponent::NextId,
        "selectedcontent" => SemanticComponent::SelectedContent,
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => SemanticComponent::Heading,
        "hgroup" => SemanticComponent::HeadingGroup,
        "ruby" => SemanticComponent::Ruby,
        "rb" => SemanticComponent::RubyBase,
        "rt" => SemanticComponent::RubyText,
        "rp" => SemanticComponent::RubyParenthesis,
        "rtc" => SemanticComponent::RubyTextContainer,
        "main" => SemanticComponent::Main,
        "nav" => SemanticComponent::Navigation,
        "header" => SemanticComponent::Header,
        "footer" => SemanticComponent::Footer,
        "article" => SemanticComponent::Article,
        "section" => SemanticComponent::Section,
        "aside" => SemanticComponent::Aside,
        "search" => SemanticComponent::Search,
        "details" => SemanticComponent::Disclosure,
        "summary" => SemanticComponent::DisclosureSummary,
        "figure" => SemanticComponent::Figure,
        "figcaption" => SemanticComponent::FigureCaption,
        "dl" => SemanticComponent::DescriptionList,
        "dt" => SemanticComponent::DescriptionTerm,
        "dd" => SemanticComponent::DescriptionDetails,
        "img" | "picture" => SemanticComponent::Image,
        "audio" | "video" => SemanticComponent::Media,
        "canvas" => SemanticComponent::Canvas,
        "embed" | "iframe" | "object" | "source" | "track" | "param" => {
            SemanticComponent::EmbeddedContent
        }
        "table" => SemanticComponent::Table,
        "thead" | "tbody" | "tfoot" | "colgroup" => SemanticComponent::TableSection,
        "tr" => SemanticComponent::TableRow,
        "td" | "th" => SemanticComponent::TableCell,
        "col" => SemanticComponent::TableColumn,
        "caption" => SemanticComponent::TableCaption,
        "dialog" => SemanticComponent::Dialog,
        "menu" => SemanticComponent::Menu,
        "hr" => SemanticComponent::Separator,
        "meter" => SemanticComponent::Meter,
        "progress" => SemanticComponent::ProgressBar,
        "fieldset" => SemanticComponent::FieldSet,
        "output" => SemanticComponent::Output,
        "form" => SemanticComponent::Form,
        "a" => component_for_anchor(attributes),
        "map" => SemanticComponent::ImageMap,
        "area" => SemanticComponent::ImageMapArea,
        tag if is_text_html_tag(tag) => SemanticComponent::Text,
        _ => SemanticComponent::Group,
    })
}

pub fn component_for_intrinsic_tag(
    tag: &str,
    attributes: &BTreeMap<String, String>,
) -> Option<SemanticComponent> {
    component_for_html_tag(tag, attributes).or_else(|| {
        if is_custom_element(tag) {
            Some(SemanticComponent::Group)
        } else {
            None
        }
    })
}

fn component_for_input_type(input_type: Option<&str>) -> SemanticComponent {
    match input_type
        .unwrap_or("text")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "button" | "submit" | "reset" | "image" => SemanticComponent::Button,
        "checkbox" => SemanticComponent::Checkbox,
        "radio" => SemanticComponent::Radio,
        "range" => SemanticComponent::Slider,
        _ => SemanticComponent::Input,
    }
}

fn component_for_anchor(attributes: &BTreeMap<String, String>) -> SemanticComponent {
    if attributes
        .get("href")
        .map(String::as_str)
        .is_some_and(|value| !value.trim().is_empty())
    {
        SemanticComponent::Link
    } else {
        SemanticComponent::Group
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
            Some(SemanticComponent::Checkbox)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "range".into())])),
            Some(SemanticComponent::Slider)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "submit".into())])),
            Some(SemanticComponent::Button)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "reset".into())])),
            Some(SemanticComponent::Button)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "button".into())])),
            Some(SemanticComponent::Button)
        );
        assert_eq!(
            component_for_html_tag("input", &BTreeMap::from([("type".into(), "image".into())])),
            Some(SemanticComponent::Button)
        );
    }

    #[test]
    fn maps_form_grouping_and_value_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("form", &attributes),
            Some(SemanticComponent::Form)
        );
        assert_eq!(
            component_for_html_tag("fieldset", &attributes),
            Some(SemanticComponent::FieldSet)
        );
        assert_eq!(
            component_for_html_tag("legend", &attributes),
            Some(SemanticComponent::Legend)
        );
        assert_eq!(
            component_for_html_tag("optgroup", &attributes),
            Some(SemanticComponent::OptionGroup)
        );
        assert_eq!(
            component_for_html_tag("output", &attributes),
            Some(SemanticComponent::Output)
        );
        assert_eq!(
            component_for_html_tag("meter", &attributes),
            Some(SemanticComponent::Meter)
        );
        assert_eq!(
            component_for_html_tag("progress", &attributes),
            Some(SemanticComponent::ProgressBar)
        );
    }

    #[test]
    fn maps_link_and_image_map_tags_to_native_semantics() {
        let empty_attributes = BTreeMap::new();
        let href_attributes = BTreeMap::from([("href".to_string(), "/docs".to_string())]);

        assert_eq!(
            component_for_html_tag("a", &href_attributes),
            Some(SemanticComponent::Link)
        );
        assert_eq!(
            component_for_html_tag("a", &empty_attributes),
            Some(SemanticComponent::Group)
        );
        assert_eq!(
            component_for_html_tag("map", &empty_attributes),
            Some(SemanticComponent::ImageMap)
        );
        assert_eq!(
            component_for_html_tag("area", &empty_attributes),
            Some(SemanticComponent::ImageMapArea)
        );
    }

    #[test]
    fn maps_document_metadata_template_and_slot_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("html", &attributes),
            Some(SemanticComponent::Document)
        );
        assert_eq!(
            component_for_html_tag("head", &attributes),
            Some(SemanticComponent::DocumentHead)
        );
        assert_eq!(
            component_for_html_tag("body", &attributes),
            Some(SemanticComponent::DocumentBody)
        );
        assert_eq!(
            component_for_html_tag("title", &attributes),
            Some(SemanticComponent::DocumentTitle)
        );
        assert_eq!(
            component_for_html_tag("base", &attributes),
            Some(SemanticComponent::Metadata)
        );
        assert_eq!(
            component_for_html_tag("meta", &attributes),
            Some(SemanticComponent::Metadata)
        );
        assert_eq!(
            component_for_html_tag("link", &attributes),
            Some(SemanticComponent::ResourceLink)
        );
        assert_eq!(
            component_for_html_tag("style", &attributes),
            Some(SemanticComponent::StyleSheet)
        );
        assert_eq!(
            component_for_html_tag("script", &attributes),
            Some(SemanticComponent::Script)
        );
        assert_eq!(
            component_for_html_tag("noscript", &attributes),
            Some(SemanticComponent::Script)
        );
        assert_eq!(
            component_for_html_tag("template", &attributes),
            Some(SemanticComponent::Template)
        );
        assert_eq!(
            component_for_html_tag("slot", &attributes),
            Some(SemanticComponent::Slot)
        );
        assert_eq!(
            component_for_html_tag("hgroup", &attributes),
            Some(SemanticComponent::HeadingGroup)
        );
    }

    #[test]
    fn maps_ruby_annotation_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("ruby", &attributes),
            Some(SemanticComponent::Ruby)
        );
        assert_eq!(
            component_for_html_tag("rb", &attributes),
            Some(SemanticComponent::RubyBase)
        );
        assert_eq!(
            component_for_html_tag("rt", &attributes),
            Some(SemanticComponent::RubyText)
        );
        assert_eq!(
            component_for_html_tag("rp", &attributes),
            Some(SemanticComponent::RubyParenthesis)
        );
        assert_eq!(
            component_for_html_tag("rtc", &attributes),
            Some(SemanticComponent::RubyTextContainer)
        );
    }

    #[test]
    fn maps_text_annotation_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("abbr", &attributes),
            Some(SemanticComponent::Abbreviation)
        );
        assert_eq!(
            component_for_html_tag("acronym", &attributes),
            Some(SemanticComponent::Abbreviation)
        );
        assert_eq!(
            component_for_html_tag("cite", &attributes),
            Some(SemanticComponent::Citation)
        );
        assert_eq!(
            component_for_html_tag("dfn", &attributes),
            Some(SemanticComponent::Definition)
        );
        assert_eq!(
            component_for_html_tag("data", &attributes),
            Some(SemanticComponent::DataValue)
        );
        assert_eq!(
            component_for_html_tag("ins", &attributes),
            Some(SemanticComponent::InsertedText)
        );
        assert_eq!(
            component_for_html_tag("del", &attributes),
            Some(SemanticComponent::DeletedText)
        );
        assert_eq!(
            component_for_html_tag("mark", &attributes),
            Some(SemanticComponent::MarkedText)
        );
        assert_eq!(
            component_for_html_tag("time", &attributes),
            Some(SemanticComponent::Time)
        );
    }

    #[test]
    fn maps_phrasing_text_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("em", &attributes),
            Some(SemanticComponent::Emphasis)
        );
        assert_eq!(
            component_for_html_tag("strong", &attributes),
            Some(SemanticComponent::StrongText)
        );
        assert_eq!(
            component_for_html_tag("code", &attributes),
            Some(SemanticComponent::Code)
        );
        assert_eq!(
            component_for_html_tag("kbd", &attributes),
            Some(SemanticComponent::KeyboardInput)
        );
        assert_eq!(
            component_for_html_tag("samp", &attributes),
            Some(SemanticComponent::SampleOutput)
        );
        assert_eq!(
            component_for_html_tag("var", &attributes),
            Some(SemanticComponent::Variable)
        );
        assert_eq!(
            component_for_html_tag("q", &attributes),
            Some(SemanticComponent::InlineQuote)
        );
        assert_eq!(
            component_for_html_tag("sub", &attributes),
            Some(SemanticComponent::Subscript)
        );
        assert_eq!(
            component_for_html_tag("sup", &attributes),
            Some(SemanticComponent::Superscript)
        );
        assert_eq!(
            component_for_html_tag("small", &attributes),
            Some(SemanticComponent::SmallText)
        );
        assert_eq!(
            component_for_html_tag("b", &attributes),
            Some(SemanticComponent::BoldText)
        );
        assert_eq!(
            component_for_html_tag("i", &attributes),
            Some(SemanticComponent::ItalicText)
        );
        assert_eq!(
            component_for_html_tag("s", &attributes),
            Some(SemanticComponent::StruckText)
        );
        assert_eq!(
            component_for_html_tag("strike", &attributes),
            Some(SemanticComponent::StruckText)
        );
        assert_eq!(
            component_for_html_tag("u", &attributes),
            Some(SemanticComponent::UnderlinedText)
        );
        assert_eq!(
            component_for_html_tag("bdi", &attributes),
            Some(SemanticComponent::BidirectionalIsolate)
        );
        assert_eq!(
            component_for_html_tag("bdo", &attributes),
            Some(SemanticComponent::BidirectionalOverride)
        );
    }

    #[test]
    fn maps_flow_and_legacy_text_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("p", &attributes),
            Some(SemanticComponent::Paragraph)
        );
        assert_eq!(
            component_for_html_tag("pre", &attributes),
            Some(SemanticComponent::PreformattedText)
        );
        assert_eq!(
            component_for_html_tag("listing", &attributes),
            Some(SemanticComponent::PreformattedText)
        );
        assert_eq!(
            component_for_html_tag("plaintext", &attributes),
            Some(SemanticComponent::PreformattedText)
        );
        assert_eq!(
            component_for_html_tag("xmp", &attributes),
            Some(SemanticComponent::PreformattedText)
        );
        assert_eq!(
            component_for_html_tag("blockquote", &attributes),
            Some(SemanticComponent::BlockQuote)
        );
        assert_eq!(
            component_for_html_tag("address", &attributes),
            Some(SemanticComponent::ContactAddress)
        );
        assert_eq!(
            component_for_html_tag("br", &attributes),
            Some(SemanticComponent::LineBreak)
        );
        assert_eq!(
            component_for_html_tag("wbr", &attributes),
            Some(SemanticComponent::WordBreakOpportunity)
        );
        assert_eq!(
            component_for_html_tag("nobr", &attributes),
            Some(SemanticComponent::NoBreakText)
        );
        assert_eq!(
            component_for_html_tag("center", &attributes),
            Some(SemanticComponent::CenteredText)
        );
        assert_eq!(
            component_for_html_tag("font", &attributes),
            Some(SemanticComponent::FontText)
        );
        assert_eq!(
            component_for_html_tag("basefont", &attributes),
            Some(SemanticComponent::FontText)
        );
        assert_eq!(
            component_for_html_tag("big", &attributes),
            Some(SemanticComponent::BigText)
        );
        assert_eq!(
            component_for_html_tag("tt", &attributes),
            Some(SemanticComponent::TeletypeText)
        );
        assert_eq!(
            component_for_html_tag("dir", &attributes),
            Some(SemanticComponent::ListBox)
        );
    }

    #[test]
    fn maps_remaining_legacy_and_foreign_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("applet", &attributes),
            Some(SemanticComponent::Applet)
        );
        assert_eq!(
            component_for_html_tag("bgsound", &attributes),
            Some(SemanticComponent::BackgroundSound)
        );
        assert_eq!(
            component_for_html_tag("frame", &attributes),
            Some(SemanticComponent::Frame)
        );
        assert_eq!(
            component_for_html_tag("frameset", &attributes),
            Some(SemanticComponent::FrameSet)
        );
        assert_eq!(
            component_for_html_tag("noembed", &attributes),
            Some(SemanticComponent::NoEmbedFallback)
        );
        assert_eq!(
            component_for_html_tag("noframes", &attributes),
            Some(SemanticComponent::NoFramesFallback)
        );
        assert_eq!(
            component_for_html_tag("marquee", &attributes),
            Some(SemanticComponent::Marquee)
        );
        assert_eq!(
            component_for_html_tag("math", &attributes),
            Some(SemanticComponent::Math)
        );
        assert_eq!(
            component_for_html_tag("nextid", &attributes),
            Some(SemanticComponent::NextId)
        );
        assert_eq!(
            component_for_html_tag("selectedcontent", &attributes),
            Some(SemanticComponent::SelectedContent)
        );
    }

    #[test]
    fn maps_embedded_media_and_table_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("img", &attributes),
            Some(SemanticComponent::Image)
        );
        assert_eq!(
            component_for_html_tag("picture", &attributes),
            Some(SemanticComponent::Image)
        );
        assert_eq!(
            component_for_html_tag("video", &attributes),
            Some(SemanticComponent::Media)
        );
        assert_eq!(
            component_for_html_tag("audio", &attributes),
            Some(SemanticComponent::Media)
        );
        assert_eq!(
            component_for_html_tag("canvas", &attributes),
            Some(SemanticComponent::Canvas)
        );
        assert_eq!(
            component_for_html_tag("iframe", &attributes),
            Some(SemanticComponent::EmbeddedContent)
        );
        assert_eq!(
            component_for_html_tag("table", &attributes),
            Some(SemanticComponent::Table)
        );
        assert_eq!(
            component_for_html_tag("tbody", &attributes),
            Some(SemanticComponent::TableSection)
        );
        assert_eq!(
            component_for_html_tag("tr", &attributes),
            Some(SemanticComponent::TableRow)
        );
        assert_eq!(
            component_for_html_tag("td", &attributes),
            Some(SemanticComponent::TableCell)
        );
        assert_eq!(
            component_for_html_tag("col", &attributes),
            Some(SemanticComponent::TableColumn)
        );
        assert_eq!(
            component_for_html_tag("caption", &attributes),
            Some(SemanticComponent::TableCaption)
        );
    }

    #[test]
    fn maps_sectioning_landmark_and_heading_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("h1", &attributes),
            Some(SemanticComponent::Heading)
        );
        assert_eq!(
            component_for_html_tag("main", &attributes),
            Some(SemanticComponent::Main)
        );
        assert_eq!(
            component_for_html_tag("nav", &attributes),
            Some(SemanticComponent::Navigation)
        );
        assert_eq!(
            component_for_html_tag("header", &attributes),
            Some(SemanticComponent::Header)
        );
        assert_eq!(
            component_for_html_tag("footer", &attributes),
            Some(SemanticComponent::Footer)
        );
        assert_eq!(
            component_for_html_tag("article", &attributes),
            Some(SemanticComponent::Article)
        );
        assert_eq!(
            component_for_html_tag("section", &attributes),
            Some(SemanticComponent::Section)
        );
        assert_eq!(
            component_for_html_tag("aside", &attributes),
            Some(SemanticComponent::Aside)
        );
        assert_eq!(
            component_for_html_tag("search", &attributes),
            Some(SemanticComponent::Search)
        );
    }

    #[test]
    fn maps_disclosure_figure_and_description_list_tags_to_native_semantics() {
        let attributes = BTreeMap::new();

        assert_eq!(
            component_for_html_tag("details", &attributes),
            Some(SemanticComponent::Disclosure)
        );
        assert_eq!(
            component_for_html_tag("summary", &attributes),
            Some(SemanticComponent::DisclosureSummary)
        );
        assert_eq!(
            component_for_html_tag("figure", &attributes),
            Some(SemanticComponent::Figure)
        );
        assert_eq!(
            component_for_html_tag("figcaption", &attributes),
            Some(SemanticComponent::FigureCaption)
        );
        assert_eq!(
            component_for_html_tag("dl", &attributes),
            Some(SemanticComponent::DescriptionList)
        );
        assert_eq!(
            component_for_html_tag("dt", &attributes),
            Some(SemanticComponent::DescriptionTerm)
        );
        assert_eq!(
            component_for_html_tag("dd", &attributes),
            Some(SemanticComponent::DescriptionDetails)
        );
    }
}
