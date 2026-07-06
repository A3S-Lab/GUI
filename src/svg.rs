use crate::semantic_ui::SemanticComponent;

pub const SVG_TAG_METADATA_KEY: &str = "data-a3s-svg-tag";

pub const SVG_ELEMENTS: &[&str] = &[
    "animate",
    "animateMotion",
    "animateTransform",
    "circle",
    "clipPath",
    "defs",
    "desc",
    "discard",
    "ellipse",
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDistantLight",
    "feDropShadow",
    "feFlood",
    "feFuncA",
    "feFuncB",
    "feFuncG",
    "feFuncR",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMergeNode",
    "feMorphology",
    "feOffset",
    "fePointLight",
    "feSpecularLighting",
    "feSpotLight",
    "feTile",
    "feTurbulence",
    "filter",
    "foreignObject",
    "g",
    "hatch",
    "hatchpath",
    "image",
    "line",
    "linearGradient",
    "marker",
    "mask",
    "metadata",
    "mpath",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialGradient",
    "rect",
    "set",
    "solidcolor",
    "stop",
    "svg",
    "switch",
    "symbol",
    "text",
    "textPath",
    "title",
    "tspan",
    "use",
    "view",
];

pub fn is_svg_element(tag: &str) -> bool {
    canonical_svg_tag(tag).is_some()
}

pub fn canonical_svg_tag(tag: &str) -> Option<&'static str> {
    let tag = tag.trim();
    SVG_ELEMENTS
        .iter()
        .copied()
        .find(|candidate| candidate.eq_ignore_ascii_case(tag))
}

pub fn component_for_svg_tag(tag: &str) -> Option<SemanticComponent> {
    let tag = canonical_svg_tag(tag)?;
    Some(if is_text_svg_tag(tag) {
        SemanticComponent::Text
    } else {
        SemanticComponent::Group
    })
}

fn is_text_svg_tag(tag: &str) -> bool {
    matches!(tag, "desc" | "text" | "textPath" | "title" | "tspan")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_svg_elements_case_insensitively() {
        assert!(is_svg_element("path"));
        assert!(is_svg_element("lineargradient"));
        assert!(is_svg_element("feGaussianBlur"));
        assert!(!is_svg_element("not-svg"));
    }

    #[test]
    fn maps_svg_text_elements_to_text_semantics() {
        assert_eq!(component_for_svg_tag("text"), Some(SemanticComponent::Text));
        assert_eq!(
            component_for_svg_tag("path"),
            Some(SemanticComponent::Group)
        );
    }
}
