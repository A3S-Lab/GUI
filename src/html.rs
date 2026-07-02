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
        "label" | "legend" => AriaComponent::Label,
        "input" => component_for_input_type(attributes.get("type").map(String::as_str)),
        "textarea" => AriaComponent::Input,
        "select" => AriaComponent::Select,
        "option" => AriaComponent::ListBoxItem,
        "ul" | "ol" | "datalist" => AriaComponent::ListBox,
        "li" => AriaComponent::ListBoxItem,
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
        "meter" | "progress" => AriaComponent::ProgressBar,
        "form" => AriaComponent::Form,
        "summary" => AriaComponent::Button,
        "a" | "area" => AriaComponent::Button,
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
            | "dd"
            | "del"
            | "dfn"
            | "dt"
            | "em"
            | "figcaption"
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
            | "output"
            | "p"
            | "pre"
            | "q"
            | "rb"
            | "rp"
            | "rt"
            | "rtc"
            | "ruby"
            | "s"
            | "samp"
            | "small"
            | "span"
            | "strike"
            | "strong"
            | "sub"
            | "sup"
            | "time"
            | "title"
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
}
