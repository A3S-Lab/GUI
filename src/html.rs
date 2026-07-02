use std::collections::BTreeMap;

use crate::react_aria::AriaComponent;

pub const HTML_TAG_METADATA_KEY: &str = "data-a3s-html-tag";

pub const HTML_ELEMENTS: &[&str] = &[
    "a",
    "abbr",
    "acronym",
    "address",
    "applet",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "basefont",
    "bdi",
    "bdo",
    "bgsound",
    "big",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "center",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "font",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "link",
    "listing",
    "main",
    "map",
    "mark",
    "marquee",
    "math",
    "menu",
    "meta",
    "meter",
    "nav",
    "nextid",
    "nobr",
    "noembed",
    "noframes",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "plaintext",
    "pre",
    "progress",
    "q",
    "rb",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "search",
    "section",
    "select",
    "selectedcontent",
    "slot",
    "small",
    "source",
    "span",
    "strike",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "svg",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "tt",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
    "xmp",
];

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
        "ul" | "ol" | "datalist" => AriaComponent::ListBox,
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
    matches!(
        tag,
        "abbr"
            | "acronym"
            | "address"
            | "b"
            | "bdi"
            | "bdo"
            | "big"
            | "blockquote"
            | "br"
            | "caption"
            | "center"
            | "cite"
            | "code"
            | "data"
            | "del"
            | "dfn"
            | "em"
            | "font"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "i"
            | "ins"
            | "kbd"
            | "mark"
            | "nobr"
            | "p"
            | "pre"
            | "q"
            | "s"
            | "samp"
            | "small"
            | "span"
            | "strike"
            | "strong"
            | "sub"
            | "sup"
            | "time"
            | "tt"
            | "u"
            | "var"
            | "wbr"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
