use super::*;

mod core;
mod filters;
mod media;
mod motion;
mod ring_grid;
mod transform;
mod values;

pub(super) use core::*;
pub(super) use filters::*;
pub(super) use media::*;
pub(super) use motion::*;
pub(super) use ring_grid::*;
pub(super) use transform::*;
pub(super) use values::*;

pub(super) fn tailwind_utility_declarations(class: &str) -> BTreeMap<String, String> {
    let mut declarations = BTreeMap::new();
    if let Some(arbitrary) = class
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        if let Some((property, value)) = arbitrary.split_once(':') {
            declarations.insert(
                normalize_css_property_name(property),
                tailwind_arbitrary_value(value.trim()),
            );
        }
        return declarations;
    }
    if let Some(radius) = tailwind_radius_declarations(class) {
        declarations.extend(radius);
        return declarations;
    }
    if let Some(screen_reader) = tailwind_screen_reader_declarations(class) {
        declarations.extend(screen_reader);
        return declarations;
    }
    if class == "truncate" {
        declarations.insert("overflow".to_string(), "hidden".to_string());
        declarations.insert("text-overflow".to_string(), "ellipsis".to_string());
        declarations.insert("white-space".to_string(), "nowrap".to_string());
        return declarations;
    }
    if class == "break-normal" {
        declarations.insert("overflow-wrap".to_string(), "normal".to_string());
        declarations.insert("word-break".to_string(), "normal".to_string());
        return declarations;
    }
    if let Some((property, value)) = tailwind_content_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some(line_clamp) = tailwind_line_clamp_declarations(class) {
        declarations.extend(line_clamp);
        return declarations;
    }
    if let Some(font_features) = tailwind_font_feature_declarations(class) {
        declarations.extend(font_features);
        return declarations;
    }
    if class == "antialiased" {
        declarations.insert(
            "-webkit-font-smoothing".to_string(),
            "antialiased".to_string(),
        );
        declarations.insert(
            "-moz-osx-font-smoothing".to_string(),
            "grayscale".to_string(),
        );
        return declarations;
    }
    if class == "subpixel-antialiased" {
        declarations.insert("-webkit-font-smoothing".to_string(), "auto".to_string());
        declarations.insert("-moz-osx-font-smoothing".to_string(), "auto".to_string());
        return declarations;
    }
    if let Some(container) = tailwind_container_declarations(class) {
        declarations.extend(container);
        return declarations;
    }
    if let Some(motion) = tailwind_motion_declarations(class) {
        declarations.extend(motion);
        return declarations;
    }
    if let Some(interaction) = tailwind_interaction_declarations(class) {
        declarations.extend(interaction);
        return declarations;
    }
    if let Some(svg) = tailwind_svg_presentation_declarations(class) {
        declarations.extend(svg);
        return declarations;
    }
    if let Some(formatting) = tailwind_formatting_declarations(class) {
        declarations.extend(formatting);
        return declarations;
    }
    if let Some(space) = tailwind_space_declarations(class) {
        declarations.extend(space);
        return declarations;
    }
    if let Some(divide) = tailwind_divide_declarations(class) {
        declarations.extend(divide);
        return declarations;
    }
    if let Some(transform) = tailwind_transform_declarations(class) {
        declarations.extend(transform);
        return declarations;
    }
    if let Some(filter) = tailwind_filter_declarations(class) {
        declarations.extend(filter);
        return declarations;
    }
    if let Some(backdrop_filter) = tailwind_backdrop_filter_declarations(class) {
        declarations.extend(backdrop_filter);
        return declarations;
    }
    if let Some((property, value)) = tailwind_blend_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some((property, value)) = tailwind_fragmentation_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some((property, value)) = tailwind_mask_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    let declaration = match class {
        "inline" => Some(("display", "inline".to_string())),
        "inline-block" => Some(("display", "inline-block".to_string())),
        "block" => Some(("display", "block".to_string())),
        "flow-root" => Some(("display", "flow-root".to_string())),
        "flex" => Some(("display", "flex".to_string())),
        "inline-flex" => Some(("display", "inline-flex".to_string())),
        "grid" => Some(("display", "grid".to_string())),
        "inline-grid" => Some(("display", "inline-grid".to_string())),
        "contents" => Some(("display", "contents".to_string())),
        "list-item" => Some(("display", "list-item".to_string())),
        "table" => Some(("display", "table".to_string())),
        "inline-table" => Some(("display", "inline-table".to_string())),
        "table-caption" => Some(("display", "table-caption".to_string())),
        "table-cell" => Some(("display", "table-cell".to_string())),
        "table-column" => Some(("display", "table-column".to_string())),
        "table-column-group" => Some(("display", "table-column-group".to_string())),
        "table-footer-group" => Some(("display", "table-footer-group".to_string())),
        "table-header-group" => Some(("display", "table-header-group".to_string())),
        "table-row-group" => Some(("display", "table-row-group".to_string())),
        "table-row" => Some(("display", "table-row".to_string())),
        "hidden" => Some(("display", "none".to_string())),
        "static" => Some(("position", "static".to_string())),
        "fixed" => Some(("position", "fixed".to_string())),
        "absolute" => Some(("position", "absolute".to_string())),
        "relative" => Some(("position", "relative".to_string())),
        "sticky" => Some(("position", "sticky".to_string())),
        "flex-row" => Some(("flex-direction", "row".to_string())),
        "flex-row-reverse" => Some(("flex-direction", "row-reverse".to_string())),
        "flex-col" => Some(("flex-direction", "column".to_string())),
        "flex-col-reverse" => Some(("flex-direction", "column-reverse".to_string())),
        "flex-wrap" => Some(("flex-wrap", "wrap".to_string())),
        "flex-nowrap" => Some(("flex-wrap", "nowrap".to_string())),
        "flex-wrap-reverse" => Some(("flex-wrap", "wrap-reverse".to_string())),
        "items-start" => Some(("align-items", "flex-start".to_string())),
        "items-center" => Some(("align-items", "center".to_string())),
        "items-end" => Some(("align-items", "flex-end".to_string())),
        "items-stretch" => Some(("align-items", "stretch".to_string())),
        "items-baseline" => Some(("align-items", "baseline".to_string())),
        "items-normal" => Some(("align-items", "normal".to_string())),
        "content-normal" => Some(("align-content", "normal".to_string())),
        "content-center" => Some(("align-content", "center".to_string())),
        "content-start" => Some(("align-content", "flex-start".to_string())),
        "content-end" => Some(("align-content", "flex-end".to_string())),
        "content-between" => Some(("align-content", "space-between".to_string())),
        "content-around" => Some(("align-content", "space-around".to_string())),
        "content-evenly" => Some(("align-content", "space-evenly".to_string())),
        "content-baseline" => Some(("align-content", "baseline".to_string())),
        "content-stretch" => Some(("align-content", "stretch".to_string())),
        "self-auto" => Some(("align-self", "auto".to_string())),
        "self-start" => Some(("align-self", "flex-start".to_string())),
        "self-center" => Some(("align-self", "center".to_string())),
        "self-end" => Some(("align-self", "flex-end".to_string())),
        "self-stretch" => Some(("align-self", "stretch".to_string())),
        "self-baseline" => Some(("align-self", "baseline".to_string())),
        "justify-normal" => Some(("justify-content", "normal".to_string())),
        "justify-start" => Some(("justify-content", "flex-start".to_string())),
        "justify-center" => Some(("justify-content", "center".to_string())),
        "justify-end" => Some(("justify-content", "flex-end".to_string())),
        "justify-between" => Some(("justify-content", "space-between".to_string())),
        "justify-around" => Some(("justify-content", "space-around".to_string())),
        "justify-evenly" => Some(("justify-content", "space-evenly".to_string())),
        "justify-stretch" => Some(("justify-content", "stretch".to_string())),
        "justify-items-normal" => Some(("justify-items", "normal".to_string())),
        "justify-items-start" => Some(("justify-items", "flex-start".to_string())),
        "justify-items-center" => Some(("justify-items", "center".to_string())),
        "justify-items-end" => Some(("justify-items", "flex-end".to_string())),
        "justify-items-stretch" => Some(("justify-items", "stretch".to_string())),
        "justify-self-auto" => Some(("justify-self", "auto".to_string())),
        "justify-self-start" => Some(("justify-self", "flex-start".to_string())),
        "justify-self-center" => Some(("justify-self", "center".to_string())),
        "justify-self-end" => Some(("justify-self", "flex-end".to_string())),
        "justify-self-stretch" => Some(("justify-self", "stretch".to_string())),
        "place-content-center" => Some(("place-content", "center".to_string())),
        "place-content-start" => Some(("place-content", "start".to_string())),
        "place-content-end" => Some(("place-content", "end".to_string())),
        "place-content-between" => Some(("place-content", "space-between".to_string())),
        "place-content-around" => Some(("place-content", "space-around".to_string())),
        "place-content-evenly" => Some(("place-content", "space-evenly".to_string())),
        "place-content-baseline" => Some(("place-content", "baseline".to_string())),
        "place-content-stretch" => Some(("place-content", "stretch".to_string())),
        "place-items-start" => Some(("place-items", "start".to_string())),
        "place-items-center" => Some(("place-items", "center".to_string())),
        "place-items-end" => Some(("place-items", "end".to_string())),
        "place-items-baseline" => Some(("place-items", "baseline".to_string())),
        "place-items-stretch" => Some(("place-items", "stretch".to_string())),
        "place-self-auto" => Some(("place-self", "auto".to_string())),
        "place-self-start" => Some(("place-self", "start".to_string())),
        "place-self-center" => Some(("place-self", "center".to_string())),
        "place-self-end" => Some(("place-self", "end".to_string())),
        "place-self-stretch" => Some(("place-self", "stretch".to_string())),
        "flex-1" => Some(("flex", "1".to_string())),
        "flex-auto" => Some(("flex", "auto".to_string())),
        "flex-initial" => Some(("flex", "0 auto".to_string())),
        "flex-none" => Some(("flex", "none".to_string())),
        "basis-auto" => Some(("flex-basis", "auto".to_string())),
        "basis-full" => Some(("flex-basis", "100%".to_string())),
        "grow" => Some(("flex-grow", "1".to_string())),
        "grow-0" => Some(("flex-grow", "0".to_string())),
        "shrink" => Some(("flex-shrink", "1".to_string())),
        "shrink-0" => Some(("flex-shrink", "0".to_string())),
        "order-first" => Some(("order", "-9999".to_string())),
        "order-last" => Some(("order", "9999".to_string())),
        "order-none" => Some(("order", "0".to_string())),
        "overflow-visible" => Some(("overflow", "visible".to_string())),
        "overflow-hidden" => Some(("overflow", "hidden".to_string())),
        "overflow-scroll" => Some(("overflow", "scroll".to_string())),
        "overflow-auto" => Some(("overflow", "auto".to_string())),
        "overflow-clip" => Some(("overflow", "clip".to_string())),
        "overflow-x-visible" => Some(("overflow-x", "visible".to_string())),
        "overflow-x-hidden" => Some(("overflow-x", "hidden".to_string())),
        "overflow-x-scroll" => Some(("overflow-x", "scroll".to_string())),
        "overflow-x-auto" => Some(("overflow-x", "auto".to_string())),
        "overflow-x-clip" => Some(("overflow-x", "clip".to_string())),
        "overflow-y-visible" => Some(("overflow-y", "visible".to_string())),
        "overflow-y-hidden" => Some(("overflow-y", "hidden".to_string())),
        "overflow-y-scroll" => Some(("overflow-y", "scroll".to_string())),
        "overflow-y-auto" => Some(("overflow-y", "auto".to_string())),
        "overflow-y-clip" => Some(("overflow-y", "clip".to_string())),
        "visible" => Some(("visibility", "visible".to_string())),
        "invisible" => Some(("visibility", "hidden".to_string())),
        "collapse" => Some(("visibility", "collapse".to_string())),
        "font-thin" => Some(("font-weight", "100".to_string())),
        "font-extralight" => Some(("font-weight", "200".to_string())),
        "font-light" => Some(("font-weight", "300".to_string())),
        "font-normal" => Some(("font-weight", "400".to_string())),
        "font-medium" => Some(("font-weight", "500".to_string())),
        "font-semibold" => Some(("font-weight", "600".to_string())),
        "font-bold" => Some(("font-weight", "700".to_string())),
        "font-extrabold" => Some(("font-weight", "800".to_string())),
        "font-black" => Some(("font-weight", "900".to_string())),
        "font-sans" => Some((
            "font-family",
            "ui-sans-serif, system-ui, sans-serif".to_string(),
        )),
        "font-serif" => Some(("font-family", "ui-serif, Georgia, serif".to_string())),
        "font-mono" => Some(("font-family", "ui-monospace, monospace".to_string())),
        "italic" => Some(("font-style", "italic".to_string())),
        "not-italic" => Some(("font-style", "normal".to_string())),
        "text-left" => Some(("text-align", "left".to_string())),
        "text-center" => Some(("text-align", "center".to_string())),
        "text-right" => Some(("text-align", "right".to_string())),
        "text-justify" => Some(("text-align", "justify".to_string())),
        "text-start" => Some(("text-align", "start".to_string())),
        "text-end" => Some(("text-align", "end".to_string())),
        "text-wrap" => Some(("text-wrap", "wrap".to_string())),
        "text-nowrap" => Some(("text-wrap", "nowrap".to_string())),
        "text-balance" => Some(("text-wrap", "balance".to_string())),
        "text-pretty" => Some(("text-wrap", "pretty".to_string())),
        "uppercase" => Some(("text-transform", "uppercase".to_string())),
        "lowercase" => Some(("text-transform", "lowercase".to_string())),
        "capitalize" => Some(("text-transform", "capitalize".to_string())),
        "normal-case" => Some(("text-transform", "none".to_string())),
        "underline" => Some(("text-decoration-line", "underline".to_string())),
        "overline" => Some(("text-decoration-line", "overline".to_string())),
        "line-through" => Some(("text-decoration-line", "line-through".to_string())),
        "no-underline" => Some(("text-decoration-line", "none".to_string())),
        "decoration-solid" => Some(("text-decoration-style", "solid".to_string())),
        "decoration-double" => Some(("text-decoration-style", "double".to_string())),
        "decoration-dotted" => Some(("text-decoration-style", "dotted".to_string())),
        "decoration-dashed" => Some(("text-decoration-style", "dashed".to_string())),
        "decoration-wavy" => Some(("text-decoration-style", "wavy".to_string())),
        "decoration-auto" => Some(("text-decoration-thickness", "auto".to_string())),
        "decoration-from-font" => Some(("text-decoration-thickness", "from-font".to_string())),
        "underline-offset-auto" => Some(("text-underline-offset", "auto".to_string())),
        "text-ellipsis" => Some(("text-overflow", "ellipsis".to_string())),
        "text-clip" => Some(("text-overflow", "clip".to_string())),
        "bg-fixed" => Some(("background-attachment", "fixed".to_string())),
        "bg-local" => Some(("background-attachment", "local".to_string())),
        "bg-scroll" => Some(("background-attachment", "scroll".to_string())),
        "bg-clip-border" => Some(("background-clip", "border-box".to_string())),
        "bg-clip-padding" => Some(("background-clip", "padding-box".to_string())),
        "bg-clip-content" => Some(("background-clip", "content-box".to_string())),
        "bg-clip-text" => Some(("background-clip", "text".to_string())),
        "bg-origin-border" => Some(("background-origin", "border-box".to_string())),
        "bg-origin-padding" => Some(("background-origin", "padding-box".to_string())),
        "bg-origin-content" => Some(("background-origin", "content-box".to_string())),
        "bg-cover" => Some(("background-size", "cover".to_string())),
        "bg-contain" => Some(("background-size", "contain".to_string())),
        "bg-auto" => Some(("background-size", "auto".to_string())),
        "bg-center" => Some(("background-position", "center".to_string())),
        "bg-top" => Some(("background-position", "top".to_string())),
        "bg-bottom" => Some(("background-position", "bottom".to_string())),
        "bg-left" => Some(("background-position", "left".to_string())),
        "bg-left-top" => Some(("background-position", "left top".to_string())),
        "bg-left-bottom" => Some(("background-position", "left bottom".to_string())),
        "bg-right" => Some(("background-position", "right".to_string())),
        "bg-right-top" => Some(("background-position", "right top".to_string())),
        "bg-right-bottom" => Some(("background-position", "right bottom".to_string())),
        "bg-no-repeat" => Some(("background-repeat", "no-repeat".to_string())),
        "bg-repeat" => Some(("background-repeat", "repeat".to_string())),
        "bg-repeat-x" => Some(("background-repeat", "repeat-x".to_string())),
        "bg-repeat-y" => Some(("background-repeat", "repeat-y".to_string())),
        "bg-repeat-round" => Some(("background-repeat", "round".to_string())),
        "bg-repeat-space" => Some(("background-repeat", "space".to_string())),
        "bg-none" => Some(("background-image", "none".to_string())),
        "object-contain" => Some(("object-fit", "contain".to_string())),
        "object-cover" => Some(("object-fit", "cover".to_string())),
        "object-fill" => Some(("object-fit", "fill".to_string())),
        "object-none" => Some(("object-fit", "none".to_string())),
        "object-scale-down" => Some(("object-fit", "scale-down".to_string())),
        "object-bottom" => Some(("object-position", "bottom".to_string())),
        "object-center" => Some(("object-position", "center".to_string())),
        "object-left" => Some(("object-position", "left".to_string())),
        "object-left-bottom" => Some(("object-position", "left bottom".to_string())),
        "object-left-top" => Some(("object-position", "left top".to_string())),
        "object-right" => Some(("object-position", "right".to_string())),
        "object-right-bottom" => Some(("object-position", "right bottom".to_string())),
        "object-right-top" => Some(("object-position", "right top".to_string())),
        "object-top" => Some(("object-position", "top".to_string())),
        "list-inside" => Some(("list-style-position", "inside".to_string())),
        "list-outside" => Some(("list-style-position", "outside".to_string())),
        "list-none" => Some(("list-style-type", "none".to_string())),
        "list-disc" => Some(("list-style-type", "disc".to_string())),
        "list-decimal" => Some(("list-style-type", "decimal".to_string())),
        "columns-auto" => Some(("columns", "auto".to_string())),
        "whitespace-normal" => Some(("white-space", "normal".to_string())),
        "whitespace-nowrap" => Some(("white-space", "nowrap".to_string())),
        "whitespace-pre" => Some(("white-space", "pre".to_string())),
        "whitespace-pre-line" => Some(("white-space", "pre-line".to_string())),
        "whitespace-pre-wrap" => Some(("white-space", "pre-wrap".to_string())),
        "whitespace-break-spaces" => Some(("white-space", "break-spaces".to_string())),
        "break-words" => Some(("overflow-wrap", "break-word".to_string())),
        "wrap-break-word" => Some(("overflow-wrap", "break-word".to_string())),
        "wrap-anywhere" => Some(("overflow-wrap", "anywhere".to_string())),
        "wrap-normal" => Some(("overflow-wrap", "normal".to_string())),
        "break-all" => Some(("word-break", "break-all".to_string())),
        "break-keep" => Some(("word-break", "keep-all".to_string())),
        "hyphens-none" => Some(("hyphens", "none".to_string())),
        "hyphens-manual" => Some(("hyphens", "manual".to_string())),
        "hyphens-auto" => Some(("hyphens", "auto".to_string())),
        "border" => Some(("border-width", "1px".to_string())),
        "border-solid" => Some(("border-style", "solid".to_string())),
        "border-dashed" => Some(("border-style", "dashed".to_string())),
        "border-dotted" => Some(("border-style", "dotted".to_string())),
        "border-double" => Some(("border-style", "double".to_string())),
        "border-hidden" => Some(("border-style", "hidden".to_string())),
        "border-none" => Some(("border-style", "none".to_string())),
        "outline" => Some(("outline-width", "1px".to_string())),
        "outline-none" => Some(("outline", "2px solid transparent".to_string())),
        "outline-hidden" => Some(("outline-style", "none".to_string())),
        "outline-solid" => Some(("outline-style", "solid".to_string())),
        "outline-dashed" => Some(("outline-style", "dashed".to_string())),
        "outline-dotted" => Some(("outline-style", "dotted".to_string())),
        "outline-double" => Some(("outline-style", "double".to_string())),
        "shadow" => Some((
            "box-shadow",
            "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-xs" => Some(("box-shadow", "0 1px rgb(0 0 0 / 0.05)".to_string())),
        "shadow-sm" => Some(("box-shadow", "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string())),
        "shadow-md" => Some((
            "box-shadow",
            "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-lg" => Some((
            "box-shadow",
            "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-xl" => Some((
            "box-shadow",
            "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-2xl" => Some((
            "box-shadow",
            "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
        )),
        "shadow-inner" => Some((
            "box-shadow",
            "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string(),
        )),
        "shadow-none" => Some(("box-shadow", "none".to_string())),
        "transform" => Some(("transform", "translateZ(0)".to_string())),
        "transform-none" => Some(("transform", "none".to_string())),
        "filter" => Some(("filter", "var(--tw-filter)".to_string())),
        "filter-none" => Some(("filter", "none".to_string())),
        "backdrop-filter" => Some(("backdrop-filter", "var(--tw-backdrop-filter)".to_string())),
        "backdrop-filter-none" => Some(("backdrop-filter", "none".to_string())),
        "pointer-events-auto" => Some(("pointer-events", "auto".to_string())),
        "pointer-events-none" => Some(("pointer-events", "none".to_string())),
        "select-auto" => Some(("user-select", "auto".to_string())),
        "select-text" => Some(("user-select", "text".to_string())),
        "select-none" => Some(("user-select", "none".to_string())),
        "select-all" => Some(("user-select", "all".to_string())),
        "resize-none" => Some(("resize", "none".to_string())),
        "resize" => Some(("resize", "both".to_string())),
        "resize-x" => Some(("resize", "horizontal".to_string())),
        "resize-y" => Some(("resize", "vertical".to_string())),
        "aspect-auto" => Some(("aspect-ratio", "auto".to_string())),
        "aspect-square" => Some(("aspect-ratio", "1 / 1".to_string())),
        "aspect-video" => Some(("aspect-ratio", "16 / 9".to_string())),
        "rounded" => Some(("border-radius", "4px".to_string())),
        "rounded-none" => Some(("border-radius", "0px".to_string())),
        "rounded-xs" => Some(("border-radius", "2px".to_string())),
        "rounded-sm" => Some(("border-radius", "4px".to_string())),
        "rounded-md" => Some(("border-radius", "6px".to_string())),
        "rounded-lg" => Some(("border-radius", "8px".to_string())),
        "rounded-xl" => Some(("border-radius", "12px".to_string())),
        "rounded-2xl" => Some(("border-radius", "16px".to_string())),
        "rounded-3xl" => Some(("border-radius", "24px".to_string())),
        "rounded-4xl" => Some(("border-radius", "32px".to_string())),
        "rounded-full" => Some(("border-radius", "calc(infinity * 1px)".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return declarations;
    }
    if let Some((property, value)) = tailwind_media_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_edge_utility(class, "scroll-m") {
        insert_edge_declarations(&mut declarations, "scroll-margin", edges, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_logical_edge_utility(class, "scroll-m", true) {
        insert_logical_edge_declaration(&mut declarations, "scroll-margin", edges, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_edge_utility(class, "scroll-p") {
        insert_edge_declarations(&mut declarations, "scroll-padding", edges, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_logical_edge_utility(class, "scroll-p", false) {
        insert_logical_edge_declaration(&mut declarations, "scroll-padding", edges, value);
        return declarations;
    }
    if let Some(text_size) = tailwind_text_size_declarations(class) {
        declarations.extend(text_size);
        return declarations;
    }
    if let Some(value) = class
        .strip_prefix("wrap-")
        .and_then(tailwind_arbitrary_or_custom_var)
    {
        declarations.insert("overflow-wrap".to_string(), value);
        return declarations;
    }
    if let Some((property, value)) = tailwind_visual_effect_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some(ring) = tailwind_ring_declarations(class) {
        declarations.extend(ring);
        return declarations;
    }
    if let Some((property, value)) = tailwind_grid_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some((properties, value)) = tailwind_inset_utility(class) {
        insert_position_declarations(&mut declarations, properties, value);
        return declarations;
    }
    if let Some((properties, value)) = tailwind_logical_inset_utility(class) {
        insert_logical_position_declaration(&mut declarations, properties, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_border_width_utility(class) {
        insert_border_width_declarations(&mut declarations, edges, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_logical_border_width_utility(class) {
        insert_logical_border_width_declaration(&mut declarations, edges, value);
        return declarations;
    }
    if let Some(border_color) = tailwind_border_color_declarations(class) {
        declarations.extend(border_color);
        return declarations;
    }
    if let Some(size) = tailwind_size_declarations(class) {
        declarations.extend(size);
        return declarations;
    }
    if let Some((property, value)) = tailwind_prefixed_declaration(class) {
        declarations.insert(property, value);
    } else if let Some((edges, value)) = tailwind_edge_utility(class, "p") {
        insert_edge_declarations(&mut declarations, "padding", edges, value);
    } else if let Some((edges, value)) = tailwind_logical_edge_utility(class, "p", false) {
        insert_logical_edge_declaration(&mut declarations, "padding", edges, value);
    } else if let Some((edges, value)) = tailwind_edge_utility(class, "m") {
        insert_edge_declarations(&mut declarations, "margin", edges, value);
    } else if let Some((edges, value)) = tailwind_logical_edge_utility(class, "m", true) {
        insert_logical_edge_declaration(&mut declarations, "margin", edges, value);
    }
    declarations
}
