use super::*;

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

fn tailwind_prefixed_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class.strip_prefix("w-").and_then(tailwind_length) {
        Some(("width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("h-").and_then(tailwind_length) {
        Some(("height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("min-w-").and_then(tailwind_length) {
        Some(("min-width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("min-h-").and_then(tailwind_length) {
        Some(("min-height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("max-w-").and_then(tailwind_length) {
        Some(("max-width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("max-h-").and_then(tailwind_length) {
        Some(("max-height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-").and_then(tailwind_length) {
        Some(("gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-x-").and_then(tailwind_length) {
        Some(("column-gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-y-").and_then(tailwind_length) {
        Some(("row-gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("opacity-").and_then(tailwind_opacity) {
        Some(("opacity".to_string(), trim_float(value)))
    } else if let Some(value) = tailwind_z_index(class) {
        Some(("z-index".to_string(), value))
    } else if let Some(value) = class.strip_prefix("flex-").and_then(tailwind_flex_value) {
        Some(("flex".to_string(), value))
    } else if let Some(value) = class.strip_prefix("basis-").and_then(tailwind_basis_value) {
        Some(("flex-basis".to_string(), value))
    } else if let Some(value) = class.strip_prefix("grow-").and_then(tailwind_number_token) {
        Some(("flex-grow".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("shrink-")
        .and_then(tailwind_number_token)
    {
        Some(("flex-shrink".to_string(), value))
    } else if let Some(value) = tailwind_order_value(class) {
        Some(("order".to_string(), value))
    } else if let Some(value) = class.strip_prefix("bg-").and_then(tailwind_color_css) {
        Some(("background-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("border-").and_then(tailwind_color_css) {
        Some(("border-color".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("accent-")
        .and_then(tailwind_accent_color_css)
    {
        Some(("accent-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("caret-").and_then(tailwind_color_css) {
        Some(("caret-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("font-").and_then(tailwind_font_family) {
        Some(("font-family".to_string(), value))
    } else if let Some(value) = tailwind_letter_spacing(class) {
        Some(("letter-spacing".to_string(), value))
    } else if let Some((property, value)) = tailwind_decoration_declaration(class) {
        Some((property, value))
    } else if let Some(value) = class
        .strip_prefix("underline-offset-")
        .and_then(tailwind_underline_offset)
    {
        Some(("text-underline-offset".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("leading-")
        .and_then(tailwind_line_height)
    {
        Some(("line-height".to_string(), value))
    } else if let Some(value) = tailwind_text_indent(class) {
        Some(("text-indent".to_string(), value))
    } else if let Some(value) = class.strip_prefix("text-").and_then(tailwind_color_css) {
        Some(("color".to_string(), value))
    } else {
        None
    }
}

fn tailwind_size_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let value = class.strip_prefix("size-").and_then(tailwind_length)?;
    let value = style_length_css(value);
    let mut declarations = BTreeMap::new();
    declarations.insert("width".to_string(), value.clone());
    declarations.insert("height".to_string(), value);
    Some(declarations)
}

fn tailwind_content_declaration(class: &str) -> Option<(String, String)> {
    if class == "content-none" {
        return Some(("content".to_string(), "none".to_string()));
    }
    if let Some(value) = class
        .strip_prefix("content-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "content".to_string(),
            tailwind_arbitrary_content_value(value),
        ));
    }
    class
        .strip_prefix("content-")
        .and_then(tailwind_custom_var)
        .map(|value| ("content".to_string(), value))
}

fn tailwind_screen_reader_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "sr-only" => {
            declarations.insert("position".to_string(), "absolute".to_string());
            declarations.insert("width".to_string(), "1px".to_string());
            declarations.insert("height".to_string(), "1px".to_string());
            declarations.insert("padding".to_string(), "0".to_string());
            declarations.insert("margin".to_string(), "-1px".to_string());
            declarations.insert("overflow".to_string(), "hidden".to_string());
            declarations.insert("clip".to_string(), "rect(0, 0, 0, 0)".to_string());
            declarations.insert("white-space".to_string(), "nowrap".to_string());
            declarations.insert("border-width".to_string(), "0".to_string());
        }
        "not-sr-only" => {
            declarations.insert("position".to_string(), "static".to_string());
            declarations.insert("width".to_string(), "auto".to_string());
            declarations.insert("height".to_string(), "auto".to_string());
            declarations.insert("padding".to_string(), "0".to_string());
            declarations.insert("margin".to_string(), "0".to_string());
            declarations.insert("overflow".to_string(), "visible".to_string());
            declarations.insert("clip".to_string(), "auto".to_string());
            declarations.insert("white-space".to_string(), "normal".to_string());
            declarations.insert("border-width".to_string(), "0".to_string());
        }
        _ => return None,
    }
    Some(declarations)
}

fn tailwind_svg_presentation_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if let Some(value) = class
        .strip_prefix("fill-")
        .and_then(tailwind_svg_paint_value)
    {
        declarations.insert("fill".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("stroke-") {
        let (property, value) = tailwind_stroke_declaration(value)?;
        declarations.insert(property, value);
        return Some(declarations);
    }
    None
}

fn tailwind_svg_paint_value(value: &str) -> Option<String> {
    if value == "none" {
        return Some("none".to_string());
    }
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_color_css(value).or_else(|| tailwind_custom_var(value))
}

fn tailwind_stroke_declaration(value: &str) -> Option<(String, String)> {
    if let Some(value) = tailwind_typed_custom_var(value, "length") {
        return Some(("stroke-width".to_string(), value));
    }
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(("stroke".to_string(), value));
    }
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if is_likely_stroke_width_value(&value) {
            return Some(("stroke-width".to_string(), value));
        }
        return Some(("stroke".to_string(), value));
    }
    if let Ok(width) = value.parse::<f64>() {
        return Some(("stroke-width".to_string(), trim_float(width)));
    }
    tailwind_svg_paint_value(value).map(|value| ("stroke".to_string(), value))
}

fn is_likely_stroke_width_value(value: &str) -> bool {
    !value.trim().starts_with("var(") && parse_length(value).is_some()
}

fn tailwind_radius_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let suffix = class.strip_prefix("rounded")?;
    let (physical, logical, value) = if suffix.is_empty() {
        (Some(CornerSelection::All), None, "sm")
    } else {
        let suffix = suffix.strip_prefix('-')?;
        if let Some(value) = suffix.strip_prefix("ss-") {
            (None, Some(LogicalCornerSelection::StartStart), value)
        } else if let Some(value) = suffix.strip_prefix("se-") {
            (None, Some(LogicalCornerSelection::StartEnd), value)
        } else if let Some(value) = suffix.strip_prefix("ee-") {
            (None, Some(LogicalCornerSelection::EndEnd), value)
        } else if let Some(value) = suffix.strip_prefix("es-") {
            (None, Some(LogicalCornerSelection::EndStart), value)
        } else if let Some(value) = suffix.strip_prefix("s-") {
            (None, Some(LogicalCornerSelection::Start), value)
        } else if let Some(value) = suffix.strip_prefix("e-") {
            (None, Some(LogicalCornerSelection::End), value)
        } else if let Some(value) = suffix.strip_prefix("tl-") {
            (Some(CornerSelection::TopLeft), None, value)
        } else if let Some(value) = suffix.strip_prefix("tr-") {
            (Some(CornerSelection::TopRight), None, value)
        } else if let Some(value) = suffix.strip_prefix("br-") {
            (Some(CornerSelection::BottomRight), None, value)
        } else if let Some(value) = suffix.strip_prefix("bl-") {
            (Some(CornerSelection::BottomLeft), None, value)
        } else if let Some(value) = suffix.strip_prefix("t-") {
            (Some(CornerSelection::Top), None, value)
        } else if let Some(value) = suffix.strip_prefix("r-") {
            (Some(CornerSelection::Right), None, value)
        } else if let Some(value) = suffix.strip_prefix("b-") {
            (Some(CornerSelection::Bottom), None, value)
        } else if let Some(value) = suffix.strip_prefix("l-") {
            (Some(CornerSelection::Left), None, value)
        } else {
            (Some(CornerSelection::All), None, suffix)
        }
    };
    let radius = CornerRadius::circular(tailwind_radius_value(value)?);
    if let Some(selection) = physical {
        insert_corner_radius_declarations(&mut declarations, selection, radius);
    } else if let Some(selection) = logical {
        insert_logical_corner_radius_declarations(&mut declarations, selection, radius);
    }
    Some(declarations)
}

fn tailwind_radius_value(value: &str) -> Option<StyleLength> {
    if let Some(value) = tailwind_typed_custom_var(value, "length") {
        return Some(StyleLength::Css(value));
    }
    if let Some(variable) = tailwind_custom_var(value) {
        return Some(StyleLength::Css(variable));
    }
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
    }
    match value {
        "none" => Some(StyleLength::Points(0.0)),
        "xs" => Some(StyleLength::Points(2.0)),
        "sm" => Some(StyleLength::Points(4.0)),
        "md" => Some(StyleLength::Points(6.0)),
        "lg" => Some(StyleLength::Points(8.0)),
        "xl" => Some(StyleLength::Points(12.0)),
        "2xl" => Some(StyleLength::Points(16.0)),
        "3xl" => Some(StyleLength::Points(24.0)),
        "4xl" => Some(StyleLength::Points(32.0)),
        "full" => Some(StyleLength::Css("calc(infinity * 1px)".to_string())),
        _ if is_tailwind_identifier(value) => {
            Some(StyleLength::Css(format!("var(--radius-{value})")))
        }
        _ => None,
    }
}

fn tailwind_formatting_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let declaration = match class {
        "box-border" => Some(("box-sizing", "border-box".to_string())),
        "box-content" => Some(("box-sizing", "content-box".to_string())),
        "box-decoration-slice" => Some(("box-decoration-break", "slice".to_string())),
        "box-decoration-clone" => Some(("box-decoration-break", "clone".to_string())),
        "isolate" => Some(("isolation", "isolate".to_string())),
        "isolation-auto" => Some(("isolation", "auto".to_string())),
        "float-right" => Some(("float", "right".to_string())),
        "float-left" => Some(("float", "left".to_string())),
        "float-start" => Some(("float", "inline-start".to_string())),
        "float-end" => Some(("float", "inline-end".to_string())),
        "float-none" => Some(("float", "none".to_string())),
        "clear-right" => Some(("clear", "right".to_string())),
        "clear-left" => Some(("clear", "left".to_string())),
        "clear-both" => Some(("clear", "both".to_string())),
        "clear-start" => Some(("clear", "inline-start".to_string())),
        "clear-end" => Some(("clear", "inline-end".to_string())),
        "clear-none" => Some(("clear", "none".to_string())),
        "align-baseline" => Some(("vertical-align", "baseline".to_string())),
        "align-top" => Some(("vertical-align", "top".to_string())),
        "align-middle" => Some(("vertical-align", "middle".to_string())),
        "align-bottom" => Some(("vertical-align", "bottom".to_string())),
        "align-text-top" => Some(("vertical-align", "text-top".to_string())),
        "align-text-bottom" => Some(("vertical-align", "text-bottom".to_string())),
        "align-sub" => Some(("vertical-align", "sub".to_string())),
        "align-super" => Some(("vertical-align", "super".to_string())),
        "table-auto" => Some(("table-layout", "auto".to_string())),
        "table-fixed" => Some(("table-layout", "fixed".to_string())),
        "border-collapse" => Some(("border-collapse", "collapse".to_string())),
        "border-separate" => Some(("border-collapse", "separate".to_string())),
        "caption-top" => Some(("caption-side", "top".to_string())),
        "caption-bottom" => Some(("caption-side", "bottom".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("align-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "vertical-align".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("align-").and_then(tailwind_custom_var) {
        declarations.insert("vertical-align".to_string(), value);
        return Some(declarations);
    }
    if let Some((axis, value)) = tailwind_border_spacing_declaration(class) {
        insert_tailwind_border_spacing_declarations(&mut declarations, axis, value);
        return Some(declarations);
    }
    None
}

fn tailwind_space_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "space-x-reverse" => {
            declarations.insert("--tw-space-x-reverse".to_string(), "1".to_string());
            return Some(declarations);
        }
        "space-y-reverse" => {
            declarations.insert("--tw-space-y-reverse".to_string(), "1".to_string());
            return Some(declarations);
        }
        _ => {}
    }
    let (axis, value) = if let Some(value) = class.strip_prefix("space-x-") {
        ("x", value)
    } else if let Some(value) = class.strip_prefix("space-y-") {
        ("y", value)
    } else if let Some(value) = class.strip_prefix("-space-x-") {
        ("x", value)
    } else if let Some(value) = class.strip_prefix("-space-y-") {
        ("y", value)
    } else {
        return None;
    };
    let mut length = tailwind_length(value)?;
    if class.starts_with("-space-") {
        length = negate_style_length(length)?;
    }
    declarations.insert(format!("--tw-space-{axis}-reverse"), "0".to_string());
    declarations.insert(format!("space-{axis}"), style_length_css(length));
    Some(declarations)
}

fn tailwind_divide_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "divide-x" => {
            declarations.insert("--tw-divide-x-reverse".to_string(), "0".to_string());
            declarations.insert("divide-x-width".to_string(), "1px".to_string());
            return Some(declarations);
        }
        "divide-y" => {
            declarations.insert("--tw-divide-y-reverse".to_string(), "0".to_string());
            declarations.insert("divide-y-width".to_string(), "1px".to_string());
            return Some(declarations);
        }
        "divide-x-reverse" => {
            declarations.insert("--tw-divide-x-reverse".to_string(), "1".to_string());
            return Some(declarations);
        }
        "divide-y-reverse" => {
            declarations.insert("--tw-divide-y-reverse".to_string(), "1".to_string());
            return Some(declarations);
        }
        "divide-solid" => {
            declarations.insert("divide-style".to_string(), "solid".to_string());
            return Some(declarations);
        }
        "divide-dashed" => {
            declarations.insert("divide-style".to_string(), "dashed".to_string());
            return Some(declarations);
        }
        "divide-dotted" => {
            declarations.insert("divide-style".to_string(), "dotted".to_string());
            return Some(declarations);
        }
        "divide-double" => {
            declarations.insert("divide-style".to_string(), "double".to_string());
            return Some(declarations);
        }
        "divide-hidden" => {
            declarations.insert("divide-style".to_string(), "hidden".to_string());
            return Some(declarations);
        }
        "divide-none" => {
            declarations.insert("divide-style".to_string(), "none".to_string());
            return Some(declarations);
        }
        _ => {}
    }
    if let Some(value) = class.strip_prefix("divide-x-") {
        declarations.insert("--tw-divide-x-reverse".to_string(), "0".to_string());
        declarations.insert(
            "divide-x-width".to_string(),
            style_length_css(tailwind_divide_width(value)?),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("divide-y-") {
        declarations.insert("--tw-divide-y-reverse".to_string(), "0".to_string());
        declarations.insert(
            "divide-y-width".to_string(),
            style_length_css(tailwind_divide_width(value)?),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("divide-") {
        declarations.insert(
            "divide-color".to_string(),
            tailwind_divide_color_value(value)?,
        );
        return Some(declarations);
    }
    None
}

fn tailwind_divide_width(value: &str) -> Option<StyleLength> {
    if let Some(value) = tailwind_typed_custom_var(value, "length") {
        return Some(StyleLength::Css(value));
    }
    tailwind_border_width(value)
}

fn tailwind_divide_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_border_color_value(value)
}

fn tailwind_container_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let (base, name) = class.split_once('/').unwrap_or((class, ""));
    let container_type = match base {
        "@container" => "inline-size",
        "@container-size" => "size",
        "@container-normal" => "normal",
        _ => return None,
    };
    let mut declarations = BTreeMap::new();
    declarations.insert("container-type".to_string(), container_type.to_string());
    if !name.is_empty() {
        declarations.insert("container-name".to_string(), tailwind_container_name(name));
    }
    Some(declarations)
}

fn tailwind_container_name(value: &str) -> String {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        tailwind_arbitrary_value(arbitrary)
    } else {
        value.to_string()
    }
}

fn tailwind_border_spacing_declaration(class: &str) -> Option<(SpacingAxis, String)> {
    let (axis, value) = if let Some(value) = class.strip_prefix("border-spacing-x-") {
        (SpacingAxis::X, value)
    } else if let Some(value) = class.strip_prefix("border-spacing-y-") {
        (SpacingAxis::Y, value)
    } else if let Some(value) = class.strip_prefix("border-spacing-") {
        (SpacingAxis::Both, value)
    } else {
        return None;
    };
    Some((axis, tailwind_spacing_value(value)?))
}

#[derive(Debug, Clone, Copy)]
enum SpacingAxis {
    Both,
    X,
    Y,
}

fn insert_tailwind_border_spacing_declarations(
    declarations: &mut BTreeMap<String, String>,
    axis: SpacingAxis,
    value: String,
) {
    match axis {
        SpacingAxis::Both => {
            declarations.insert("--tw-border-spacing-x".to_string(), value.clone());
            declarations.insert("--tw-border-spacing-y".to_string(), value);
        }
        SpacingAxis::X => {
            declarations.insert("--tw-border-spacing-x".to_string(), value);
        }
        SpacingAxis::Y => {
            declarations.insert("--tw-border-spacing-y".to_string(), value);
        }
    }
    declarations.insert(
        "border-spacing".to_string(),
        "var(--tw-border-spacing-x) var(--tw-border-spacing-y)".to_string(),
    );
}

fn tailwind_spacing_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value).or_else(|| tailwind_length(value).map(style_length_css))
}

fn tailwind_transform_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let declaration = match class {
        "transform-none" => Some(("transform", "none".to_string())),
        "transform-gpu" => Some(("transform", tailwind_transform_pipeline(true))),
        "transform-cpu" | "transform" => Some(("transform", tailwind_transform_pipeline(false))),
        "transform-flat" => Some(("transform-style", "flat".to_string())),
        "transform-3d" => Some(("transform-style", "preserve-3d".to_string())),
        "backface-visible" => Some(("backface-visibility", "visible".to_string())),
        "backface-hidden" => Some(("backface-visibility", "hidden".to_string())),
        "perspective-none" => Some(("perspective", "none".to_string())),
        "perspective-dramatic" => Some(("perspective", "100px".to_string())),
        "perspective-near" => Some(("perspective", "300px".to_string())),
        "perspective-normal" => Some(("perspective", "500px".to_string())),
        "perspective-midrange" => Some(("perspective", "800px".to_string())),
        "perspective-distant" => Some(("perspective", "1200px".to_string())),
        "origin-center" => Some(("transform-origin", "center".to_string())),
        "origin-top" => Some(("transform-origin", "top".to_string())),
        "origin-top-right" => Some(("transform-origin", "top right".to_string())),
        "origin-right" => Some(("transform-origin", "right".to_string())),
        "origin-bottom-right" => Some(("transform-origin", "bottom right".to_string())),
        "origin-bottom" => Some(("transform-origin", "bottom".to_string())),
        "origin-bottom-left" => Some(("transform-origin", "bottom left".to_string())),
        "origin-left" => Some(("transform-origin", "left".to_string())),
        "origin-top-left" => Some(("transform-origin", "top left".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return Some(declarations);
    }

    if let Some(value) = class
        .strip_prefix("transform-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("transform".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("transform-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("transform".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("origin-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "transform-origin".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("origin-").and_then(tailwind_custom_var) {
        declarations.insert("transform-origin".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("perspective-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("perspective".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("perspective-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("perspective".to_string(), value);
        return Some(declarations);
    }
    if let Some((axis, value)) = tailwind_translate_declaration(class) {
        insert_tailwind_axis_declarations(
            &mut declarations,
            "translate",
            "--tw-translate",
            axis,
            value,
            "0",
        );
        return Some(declarations);
    }
    if let Some((axis, value)) = tailwind_scale_declaration(class) {
        insert_tailwind_axis_declarations(
            &mut declarations,
            "scale",
            "--tw-scale",
            axis,
            value,
            "100%",
        );
        return Some(declarations);
    }
    if let Some(value) = tailwind_rotate_declaration(class) {
        declarations.insert("--tw-rotate".to_string(), value);
        declarations.insert("rotate".to_string(), "var(--tw-rotate)".to_string());
        return Some(declarations);
    }
    if let Some((property, value)) = tailwind_transform_function_declaration(class) {
        declarations.insert(property, value);
        declarations.insert("transform".to_string(), tailwind_transform_pipeline(false));
        return Some(declarations);
    }
    None
}

fn insert_tailwind_axis_declarations(
    declarations: &mut BTreeMap<String, String>,
    property: &str,
    variable_prefix: &str,
    axis: TransformAxis,
    value: String,
    default_value: &str,
) {
    match axis {
        TransformAxis::All => {
            declarations.insert(format!("{variable_prefix}-x"), value.clone());
            declarations.insert(format!("{variable_prefix}-y"), value);
            declarations.insert(
                property.to_string(),
                format!("var({variable_prefix}-x) var({variable_prefix}-y)"),
            );
        }
        TransformAxis::X => {
            declarations.insert(format!("{variable_prefix}-x"), value);
            declarations.insert(
                property.to_string(),
                format!("var({variable_prefix}-x) var({variable_prefix}-y, {default_value})"),
            );
        }
        TransformAxis::Y => {
            declarations.insert(format!("{variable_prefix}-y"), value);
            declarations.insert(
                property.to_string(),
                format!("var({variable_prefix}-x, {default_value}) var({variable_prefix}-y)"),
            );
        }
        TransformAxis::Z => {
            declarations.insert(format!("{variable_prefix}-z"), value);
            declarations.insert(
                property.to_string(),
                format!(
                    "var({variable_prefix}-x, {default_value}) var({variable_prefix}-y, {default_value}) var({variable_prefix}-z)"
                ),
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TransformAxis {
    All,
    X,
    Y,
    Z,
}

fn tailwind_translate_declaration(class: &str) -> Option<(TransformAxis, String)> {
    let (negative, class) = strip_negative_prefix(class);
    let (axis, value) = if let Some(value) = class.strip_prefix("translate-x-") {
        (TransformAxis::X, value)
    } else if let Some(value) = class.strip_prefix("translate-y-") {
        (TransformAxis::Y, value)
    } else if let Some(value) = class.strip_prefix("translate-z-") {
        (TransformAxis::Z, value)
    } else if let Some(value) = class.strip_prefix("translate-") {
        (TransformAxis::All, value)
    } else {
        return None;
    };
    Some((axis, tailwind_signed_length_value(value, negative)?))
}

fn tailwind_scale_declaration(class: &str) -> Option<(TransformAxis, String)> {
    let (negative, class) = strip_negative_prefix(class);
    let (axis, value) = if let Some(value) = class.strip_prefix("scale-x-") {
        (TransformAxis::X, value)
    } else if let Some(value) = class.strip_prefix("scale-y-") {
        (TransformAxis::Y, value)
    } else if let Some(value) = class.strip_prefix("scale-z-") {
        (TransformAxis::Z, value)
    } else if let Some(value) = class.strip_prefix("scale-") {
        (TransformAxis::All, value)
    } else {
        return None;
    };
    Some((axis, tailwind_signed_scale_value(value, negative)?))
}

fn tailwind_rotate_declaration(class: &str) -> Option<String> {
    let (negative, class) = strip_negative_prefix(class);
    let value = class.strip_prefix("rotate-")?;
    tailwind_signed_angle_value(value, negative)
}

fn tailwind_transform_function_declaration(class: &str) -> Option<(String, String)> {
    let (negative, class) = strip_negative_prefix(class);
    if let Some(value) = class.strip_prefix("rotate-x-") {
        return Some((
            "--tw-rotate-x".to_string(),
            format!("rotateX({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("rotate-y-") {
        return Some((
            "--tw-rotate-y".to_string(),
            format!("rotateY({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("rotate-z-") {
        return Some((
            "--tw-rotate-z".to_string(),
            format!("rotateZ({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("skew-x-") {
        return Some((
            "--tw-skew-x".to_string(),
            format!("skewX({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("skew-y-") {
        return Some((
            "--tw-skew-y".to_string(),
            format!("skewY({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    None
}

fn strip_negative_prefix(value: &str) -> (bool, &str) {
    if let Some(value) = value.strip_prefix('-') {
        (true, value)
    } else {
        (false, value)
    }
}

fn tailwind_signed_length_value(value: &str, negative: bool) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        if value == "full" {
            Some("100%".to_string())
        } else {
            tailwind_length(value).map(style_length_css)
        }
    })?;
    Some(if negative {
        negate_css_value(&value)
    } else {
        value
    })
}

fn tailwind_signed_scale_value(value: &str, negative: bool) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}%", trim_float(value)))
    })?;
    Some(if negative {
        negate_css_value(&value)
    } else {
        value
    })
}

fn tailwind_signed_angle_value(value: &str, negative: bool) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}deg", trim_float(value)))
    })?;
    Some(if negative {
        negate_css_value(&value)
    } else {
        value
    })
}

fn negate_css_value(value: &str) -> String {
    if value.starts_with("var(") || value.starts_with("calc(") {
        format!("calc({value} * -1)")
    } else if let Some(number) = value.strip_prefix('-') {
        number.to_string()
    } else {
        format!("-{value}")
    }
}

fn tailwind_transform_pipeline(gpu: bool) -> String {
    let pipeline = "var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)";
    if gpu {
        format!("translateZ(0) {pipeline}")
    } else {
        pipeline.to_string()
    }
}

fn tailwind_filter_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if class == "filter-none" {
        declarations.insert("filter".to_string(), "none".to_string());
        return Some(declarations);
    }
    if class == "filter" {
        declarations.insert("filter".to_string(), tailwind_filter_pipeline());
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("filter".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("filter-").and_then(tailwind_custom_var) {
        declarations.insert("filter".to_string(), value);
        return Some(declarations);
    }
    if let Some((property, value)) = tailwind_filter_component_declaration(class, "") {
        declarations.insert(property, value);
        declarations.insert("filter".to_string(), tailwind_filter_pipeline());
        return Some(declarations);
    }
    None
}

fn tailwind_backdrop_filter_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if matches!(class, "backdrop-filter-none" | "backdrop-none") {
        declarations.insert("backdrop-filter".to_string(), "none".to_string());
        return Some(declarations);
    }
    if matches!(class, "backdrop-filter" | "backdrop") {
        declarations.insert(
            "backdrop-filter".to_string(),
            tailwind_backdrop_filter_pipeline(),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-[")
        .or_else(|| class.strip_prefix("backdrop-["))
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "backdrop-filter".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-")
        .or_else(|| class.strip_prefix("backdrop-"))
        .and_then(tailwind_custom_var)
    {
        declarations.insert("backdrop-filter".to_string(), value);
        return Some(declarations);
    }
    if let Some((property, value)) = tailwind_filter_component_declaration(class, "backdrop-") {
        declarations.insert(property, value);
        declarations.insert(
            "backdrop-filter".to_string(),
            tailwind_backdrop_filter_pipeline(),
        );
        return Some(declarations);
    }
    None
}

fn tailwind_blend_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class.strip_prefix("mix-blend-") {
        return tailwind_blend_mode_value(value, true)
            .map(|value| ("mix-blend-mode".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("bg-blend-") {
        return tailwind_blend_mode_value(value, false)
            .map(|value| ("background-blend-mode".to_string(), value));
    }
    None
}

fn tailwind_blend_mode_value(value: &str, include_plus_modes: bool) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    let known = matches!(
        value,
        "normal"
            | "multiply"
            | "screen"
            | "overlay"
            | "darken"
            | "lighten"
            | "color-dodge"
            | "color-burn"
            | "hard-light"
            | "soft-light"
            | "difference"
            | "exclusion"
            | "hue"
            | "saturation"
            | "color"
            | "luminosity"
    ) || (include_plus_modes && matches!(value, "plus-darker" | "plus-lighter"));
    known.then(|| value.to_string())
}

fn tailwind_fragmentation_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class.strip_prefix("break-before-") {
        return tailwind_break_value(value).map(|value| ("break-before".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("break-after-") {
        return tailwind_break_value(value).map(|value| ("break-after".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("break-inside-") {
        return tailwind_break_inside_value(value).map(|value| ("break-inside".to_string(), value));
    }
    None
}

fn tailwind_break_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    matches!(
        value,
        "auto"
            | "avoid"
            | "all"
            | "avoid-page"
            | "avoid-column"
            | "page"
            | "left"
            | "right"
            | "recto"
            | "verso"
            | "column"
    )
    .then(|| value.to_string())
}

fn tailwind_break_inside_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    matches!(value, "auto" | "avoid" | "avoid-page" | "avoid-column").then(|| value.to_string())
}

fn tailwind_mask_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("mask-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        if let Some((property, value)) = tailwind_mask_arbitrary_property(value) {
            return Some((property, value));
        }
        return Some(("mask-image".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class.strip_prefix("mask-").and_then(tailwind_custom_var) {
        return Some(("mask-image".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-image-") {
        return Some(("mask-image".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-size-") {
        return Some(("mask-size".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-position-") {
        return Some(("mask-position".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-repeat-") {
        return Some(("mask-repeat".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-origin-") {
        return Some(("mask-origin".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-clip-") {
        return Some(("mask-clip".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-composite-") {
        return Some(("mask-composite".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-mode-") {
        return Some(("mask-mode".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-type-") {
        return Some(("mask-type".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-border-") {
        return Some(("mask-border".to_string(), value));
    }

    let declaration = match class {
        "mask-none" => Some(("mask-image", "none")),
        "mask-alpha" => Some(("mask-mode", "alpha")),
        "mask-luminance" => Some(("mask-mode", "luminance")),
        "mask-match" => Some(("mask-mode", "match-source")),
        "mask-type-alpha" => Some(("mask-type", "alpha")),
        "mask-type-luminance" => Some(("mask-type", "luminance")),
        "mask-auto" => Some(("mask-size", "auto")),
        "mask-cover" => Some(("mask-size", "cover")),
        "mask-contain" => Some(("mask-size", "contain")),
        "mask-repeat" => Some(("mask-repeat", "repeat")),
        "mask-no-repeat" => Some(("mask-repeat", "no-repeat")),
        "mask-repeat-x" => Some(("mask-repeat", "repeat-x")),
        "mask-repeat-y" => Some(("mask-repeat", "repeat-y")),
        "mask-repeat-space" => Some(("mask-repeat", "space")),
        "mask-repeat-round" => Some(("mask-repeat", "round")),
        "mask-center" => Some(("mask-position", "center")),
        "mask-top" => Some(("mask-position", "top")),
        "mask-right" => Some(("mask-position", "right")),
        "mask-bottom" => Some(("mask-position", "bottom")),
        "mask-left" => Some(("mask-position", "left")),
        "mask-top-left" | "mask-left-top" => Some(("mask-position", "top left")),
        "mask-top-right" | "mask-right-top" => Some(("mask-position", "top right")),
        "mask-bottom-right" | "mask-right-bottom" => Some(("mask-position", "bottom right")),
        "mask-bottom-left" | "mask-left-bottom" => Some(("mask-position", "bottom left")),
        "mask-origin-border" => Some(("mask-origin", "border-box")),
        "mask-origin-padding" => Some(("mask-origin", "padding-box")),
        "mask-origin-content" => Some(("mask-origin", "content-box")),
        "mask-origin-fill" => Some(("mask-origin", "fill-box")),
        "mask-origin-stroke" => Some(("mask-origin", "stroke-box")),
        "mask-origin-view" => Some(("mask-origin", "view-box")),
        "mask-clip-border" => Some(("mask-clip", "border-box")),
        "mask-clip-padding" => Some(("mask-clip", "padding-box")),
        "mask-clip-content" => Some(("mask-clip", "content-box")),
        "mask-clip-fill" => Some(("mask-clip", "fill-box")),
        "mask-clip-stroke" => Some(("mask-clip", "stroke-box")),
        "mask-clip-view" => Some(("mask-clip", "view-box")),
        "mask-no-clip" => Some(("mask-clip", "no-clip")),
        "mask-add" => Some(("mask-composite", "add")),
        "mask-subtract" => Some(("mask-composite", "subtract")),
        "mask-intersect" => Some(("mask-composite", "intersect")),
        "mask-exclude" => Some(("mask-composite", "exclude")),
        _ => None,
    }?;
    Some((declaration.0.to_string(), declaration.1.to_string()))
}

fn tailwind_mask_arbitrary_property(value: &str) -> Option<(String, String)> {
    let (name, value) = value.split_once(':')?;
    let property = match name {
        "image" => "mask-image",
        "mode" => "mask-mode",
        "repeat" => "mask-repeat",
        "position" => "mask-position",
        "size" => "mask-size",
        "origin" => "mask-origin",
        "clip" => "mask-clip",
        "composite" => "mask-composite",
        "type" => "mask-type",
        "border" => "mask-border",
        "border-source" => "mask-border-source",
        "border-mode" => "mask-border-mode",
        "border-slice" => "mask-border-slice",
        "border-width" => "mask-border-width",
        "border-outset" => "mask-border-outset",
        "border-repeat" => "mask-border-repeat",
        _ => return None,
    };
    Some((property.to_string(), tailwind_arbitrary_value(value)))
}

fn tailwind_mask_prefixed_value(class: &str, prefix: &str) -> Option<String> {
    class
        .strip_prefix(prefix)
        .and_then(tailwind_arbitrary_or_custom_var)
}

fn tailwind_filter_component_declaration(class: &str, prefix: &str) -> Option<(String, String)> {
    let class = class.strip_prefix(prefix)?;
    let variable_prefix = if prefix.is_empty() {
        "--tw"
    } else {
        "--tw-backdrop"
    };
    if let Some(value) = class.strip_prefix("blur") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_blur_value(value)?;
        return Some((format!("{variable_prefix}-blur"), value));
    }
    if let Some(value) = class.strip_prefix("brightness-") {
        let value = tailwind_percent_filter_value(value, "brightness")?;
        return Some((format!("{variable_prefix}-brightness"), value));
    }
    if let Some(value) = class.strip_prefix("contrast-") {
        let value = tailwind_percent_filter_value(value, "contrast")?;
        return Some((format!("{variable_prefix}-contrast"), value));
    }
    if prefix.is_empty() {
        if let Some(value) = class.strip_prefix("drop-shadow") {
            let value = tailwind_optional_suffix(value)?;
            let value = tailwind_drop_shadow_value(value)?;
            return Some(("--tw-drop-shadow".to_string(), value));
        }
    }
    if let Some(value) = class.strip_prefix("grayscale") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_binary_filter_value(value, "grayscale")?;
        return Some((format!("{variable_prefix}-grayscale"), value));
    }
    if let Some(value) = class.strip_prefix("hue-rotate-") {
        let (negative, value) = strip_negative_prefix(value);
        let value = tailwind_signed_angle_value(value, negative)?;
        return Some((
            format!("{variable_prefix}-hue-rotate"),
            format!("hue-rotate({value})"),
        ));
    }
    if let Some(value) = class.strip_prefix("-hue-rotate-") {
        let value = tailwind_signed_angle_value(value, true)?;
        return Some((
            format!("{variable_prefix}-hue-rotate"),
            format!("hue-rotate({value})"),
        ));
    }
    if let Some(value) = class.strip_prefix("invert") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_binary_filter_value(value, "invert")?;
        return Some((format!("{variable_prefix}-invert"), value));
    }
    if prefix == "backdrop-" {
        if let Some(value) = class.strip_prefix("opacity-") {
            let value = tailwind_percent_filter_value(value, "opacity")?;
            return Some(("--tw-backdrop-opacity".to_string(), value));
        }
    }
    if let Some(value) = class.strip_prefix("saturate-") {
        let value = tailwind_percent_filter_value(value, "saturate")?;
        return Some((format!("{variable_prefix}-saturate"), value));
    }
    if let Some(value) = class.strip_prefix("sepia") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_binary_filter_value(value, "sepia")?;
        return Some((format!("{variable_prefix}-sepia"), value));
    }
    None
}

fn tailwind_optional_suffix(value: &str) -> Option<&str> {
    if value.is_empty() {
        Some("DEFAULT")
    } else {
        value.strip_prefix('-')
    }
}

fn tailwind_blur_value(value: &str) -> Option<String> {
    match value {
        "DEFAULT" => Some("blur(8px)".to_string()),
        "none" => Some(String::new()),
        "xs" => Some("blur(4px)".to_string()),
        "sm" => Some("blur(8px)".to_string()),
        "md" => Some("blur(12px)".to_string()),
        "lg" => Some("blur(16px)".to_string()),
        "xl" => Some("blur(24px)".to_string()),
        "2xl" => Some("blur(40px)".to_string()),
        "3xl" => Some("blur(64px)".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value).map(|value| format!("blur({value})")),
    }
}

fn tailwind_percent_filter_value(value: &str, function: &str) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}%", trim_float(value)))
    })?;
    Some(format!("{function}({value})"))
}

fn tailwind_binary_filter_value(value: &str, function: &str) -> Option<String> {
    let value = match value {
        "DEFAULT" => "100%".to_string(),
        "0" => "0%".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)?,
    };
    Some(format!("{function}({value})"))
}

fn tailwind_drop_shadow_value(value: &str) -> Option<String> {
    let shadow = match value {
        "DEFAULT" => "0 1px 2px rgb(0 0 0 / 0.1), 0 1px 1px rgb(0 0 0 / 0.06)".to_string(),
        "xs" => "0 1px 1px rgb(0 0 0 / 0.05)".to_string(),
        "sm" => "0 1px 2px rgb(0 0 0 / 0.15)".to_string(),
        "md" => "0 3px 3px rgb(0 0 0 / 0.12)".to_string(),
        "lg" => "0 4px 4px rgb(0 0 0 / 0.15)".to_string(),
        "xl" => "0 9px 7px rgb(0 0 0 / 0.1)".to_string(),
        "2xl" => "0 25px 25px rgb(0 0 0 / 0.15)".to_string(),
        "none" => "0 0 #0000".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)?,
    };
    Some(format!("drop-shadow({shadow})"))
}

pub(super) fn tailwind_filter_pipeline() -> String {
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-drop-shadow) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia)"
        .to_string()
}

fn tailwind_backdrop_filter_pipeline() -> String {
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)"
        .to_string()
}

fn tailwind_motion_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "transition" => {
            declarations.insert(
                "transition-property".to_string(),
                "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter".to_string(),
            );
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-all" => {
            declarations.insert("transition-property".to_string(), "all".to_string());
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-colors" => {
            declarations.insert(
                "transition-property".to_string(),
                "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke".to_string(),
            );
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-opacity" => {
            declarations.insert("transition-property".to_string(), "opacity".to_string());
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-shadow" => {
            declarations.insert("transition-property".to_string(), "box-shadow".to_string());
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-transform" => {
            declarations.insert(
                "transition-property".to_string(),
                "transform, translate, scale, rotate".to_string(),
            );
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-none" => {
            declarations.insert("transition-property".to_string(), "none".to_string());
            return Some(declarations);
        }
        "transition-discrete" => {
            declarations.insert(
                "transition-behavior".to_string(),
                "allow-discrete".to_string(),
            );
            return Some(declarations);
        }
        "transition-normal" => {
            declarations.insert("transition-behavior".to_string(), "normal".to_string());
            return Some(declarations);
        }
        "animate-spin" => {
            declarations.insert(
                "animation".to_string(),
                "spin 1s linear infinite".to_string(),
            );
            return Some(declarations);
        }
        "animate-ping" => {
            declarations.insert(
                "animation".to_string(),
                "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            );
            return Some(declarations);
        }
        "animate-pulse" => {
            declarations.insert(
                "animation".to_string(),
                "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            );
            return Some(declarations);
        }
        "animate-bounce" => {
            declarations.insert("animation".to_string(), "bounce 1s infinite".to_string());
            return Some(declarations);
        }
        "animate-none" => {
            declarations.insert("animation".to_string(), "none".to_string());
            return Some(declarations);
        }
        _ => {}
    }

    if let Some(value) = class
        .strip_prefix("transition-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "transition-property".to_string(),
            tailwind_arbitrary_value(value),
        );
        insert_tailwind_default_transition(&mut declarations);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("transition-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("transition-property".to_string(), value);
        insert_tailwind_default_transition(&mut declarations);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("duration-")
        .and_then(tailwind_time_value)
    {
        declarations.insert("transition-duration".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("delay-").and_then(tailwind_time_value) {
        declarations.insert("transition-delay".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("ease-").and_then(tailwind_easing_value) {
        declarations.insert("transition-timing-function".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("animate-")
        .and_then(tailwind_animation_value)
    {
        declarations.insert("animation".to_string(), value);
        return Some(declarations);
    }

    None
}

fn insert_tailwind_default_transition(declarations: &mut BTreeMap<String, String>) {
    declarations.insert(
        "transition-timing-function".to_string(),
        "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
    );
    declarations.insert("transition-duration".to_string(), "150ms".to_string());
}

fn tailwind_time_value(value: &str) -> Option<String> {
    if value == "initial" {
        return Some("initial".to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    value.parse::<f64>().ok().map(|value| {
        if value == 0.0 {
            "0ms".to_string()
        } else {
            format!("{}ms", trim_float(value))
        }
    })
}

fn tailwind_easing_value(value: &str) -> Option<String> {
    match value {
        "linear" => Some("linear".to_string()),
        "in" => Some("cubic-bezier(0.4, 0, 1, 1)".to_string()),
        "out" => Some("cubic-bezier(0, 0, 0.2, 1)".to_string()),
        "in-out" => Some("cubic-bezier(0.4, 0, 0.2, 1)".to_string()),
        "initial" => Some("initial".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value),
    }
}

fn tailwind_animation_value(value: &str) -> Option<String> {
    match value {
        "none" => Some("none".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value),
    }
}

fn tailwind_interaction_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let declaration = match class {
        "scheme-normal" => Some(("color-scheme", "normal".to_string())),
        "scheme-dark" => Some(("color-scheme", "dark".to_string())),
        "scheme-light" => Some(("color-scheme", "light".to_string())),
        "scheme-light-dark" => Some(("color-scheme", "light dark".to_string())),
        "scheme-only-dark" => Some(("color-scheme", "only dark".to_string())),
        "scheme-only-light" => Some(("color-scheme", "only light".to_string())),
        "forced-color-adjust-auto" => Some(("forced-color-adjust", "auto".to_string())),
        "forced-color-adjust-none" => Some(("forced-color-adjust", "none".to_string())),
        "field-sizing-fixed" => Some(("field-sizing", "fixed".to_string())),
        "field-sizing-content" => Some(("field-sizing", "content".to_string())),
        "appearance-none" => Some(("appearance", "none".to_string())),
        "appearance-auto" => Some(("appearance", "auto".to_string())),
        "will-change-auto" => Some(("will-change", "auto".to_string())),
        "will-change-scroll" => Some(("will-change", "scroll-position".to_string())),
        "will-change-contents" => Some(("will-change", "contents".to_string())),
        "will-change-transform" => Some(("will-change", "transform".to_string())),
        "scrollbar-gutter-auto" => Some(("scrollbar-gutter", "auto".to_string())),
        "scrollbar-gutter-stable" => Some(("scrollbar-gutter", "stable".to_string())),
        "scrollbar-gutter-both" => Some(("scrollbar-gutter", "stable both-edges".to_string())),
        "scrollbar-auto" => Some(("scrollbar-width", "auto".to_string())),
        "scrollbar-thin" => Some(("scrollbar-width", "thin".to_string())),
        "scrollbar-none" => Some(("scrollbar-width", "none".to_string())),
        "scroll-auto" => Some(("scroll-behavior", "auto".to_string())),
        "scroll-smooth" => Some(("scroll-behavior", "smooth".to_string())),
        "snap-none" => Some(("scroll-snap-type", "none".to_string())),
        "snap-x" => Some((
            "scroll-snap-type",
            "x var(--tw-scroll-snap-strictness)".to_string(),
        )),
        "snap-y" => Some((
            "scroll-snap-type",
            "y var(--tw-scroll-snap-strictness)".to_string(),
        )),
        "snap-both" => Some((
            "scroll-snap-type",
            "both var(--tw-scroll-snap-strictness)".to_string(),
        )),
        "snap-mandatory" => Some(("--tw-scroll-snap-strictness", "mandatory".to_string())),
        "snap-proximity" => Some(("--tw-scroll-snap-strictness", "proximity".to_string())),
        "snap-start" => Some(("scroll-snap-align", "start".to_string())),
        "snap-end" => Some(("scroll-snap-align", "end".to_string())),
        "snap-center" => Some(("scroll-snap-align", "center".to_string())),
        "snap-align-none" => Some(("scroll-snap-align", "none".to_string())),
        "snap-normal" => Some(("scroll-snap-stop", "normal".to_string())),
        "snap-always" => Some(("scroll-snap-stop", "always".to_string())),
        "overscroll-auto" => Some(("overscroll-behavior", "auto".to_string())),
        "overscroll-contain" => Some(("overscroll-behavior", "contain".to_string())),
        "overscroll-none" => Some(("overscroll-behavior", "none".to_string())),
        "overscroll-x-auto" => Some(("overscroll-behavior-x", "auto".to_string())),
        "overscroll-x-contain" => Some(("overscroll-behavior-x", "contain".to_string())),
        "overscroll-x-none" => Some(("overscroll-behavior-x", "none".to_string())),
        "overscroll-y-auto" => Some(("overscroll-behavior-y", "auto".to_string())),
        "overscroll-y-contain" => Some(("overscroll-behavior-y", "contain".to_string())),
        "overscroll-y-none" => Some(("overscroll-behavior-y", "none".to_string())),
        "touch-auto" => Some(("touch-action", "auto".to_string())),
        "touch-none" => Some(("touch-action", "none".to_string())),
        "touch-pan-x" => Some(("touch-action", "pan-x".to_string())),
        "touch-pan-left" => Some(("touch-action", "pan-left".to_string())),
        "touch-pan-right" => Some(("touch-action", "pan-right".to_string())),
        "touch-pan-y" => Some(("touch-action", "pan-y".to_string())),
        "touch-pan-up" => Some(("touch-action", "pan-up".to_string())),
        "touch-pan-down" => Some(("touch-action", "pan-down".to_string())),
        "touch-pinch-zoom" => Some(("touch-action", "pinch-zoom".to_string())),
        "touch-manipulation" => Some(("touch-action", "manipulation".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("appearance-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("appearance".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("will-change-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("will-change".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("will-change-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("will-change".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("touch-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("touch-action".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scheme-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("color-scheme".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("scheme-").and_then(tailwind_custom_var) {
        declarations.insert("color-scheme".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("forced-color-adjust-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "forced-color-adjust".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("forced-color-adjust-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("forced-color-adjust".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("field-sizing-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("field-sizing".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("field-sizing-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("field-sizing".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-thumb-")
        .and_then(tailwind_scrollbar_color_value)
    {
        declarations.insert("--tw-scrollbar-thumb".to_string(), value);
        declarations.insert(
            "scrollbar-color".to_string(),
            tailwind_scrollbar_color_pipeline().to_string(),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-track-")
        .and_then(tailwind_scrollbar_color_value)
    {
        declarations.insert("--tw-scrollbar-track".to_string(), value);
        declarations.insert(
            "scrollbar-color".to_string(),
            tailwind_scrollbar_color_pipeline().to_string(),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-gutter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "scrollbar-gutter".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-gutter-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("scrollbar-gutter".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "scrollbar-width".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("scrollbar-width".to_string(), value);
        return Some(declarations);
    }
    None
}

fn tailwind_scrollbar_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_color_css(value).or_else(|| tailwind_custom_var(value))
}

pub(super) fn tailwind_scrollbar_color_pipeline() -> &'static str {
    "var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)"
}

fn tailwind_media_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("bg-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        if let Some((hint, hinted_value)) = value.split_once(':') {
            if let Some(property) = match hint {
                "color" => Some("background-color"),
                "image" | "url" => Some("background-image"),
                "length" | "size" => Some("background-size"),
                "position" => Some("background-position"),
                _ => None,
            } {
                return Some((property.to_string(), tailwind_arbitrary_value(hinted_value)));
            }
        }
        let value = tailwind_arbitrary_value(value);
        let property = if is_css_background_image_value(&value) {
            "background-image"
        } else if parse_color(&value).is_some() {
            "background-color"
        } else if is_background_position_value(&value) {
            "background-position"
        } else {
            "background-size"
        };
        return Some((property.to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("object-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "object-position".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if class == "list-image-none" {
        return Some(("list-style-image".to_string(), "none".to_string()));
    }
    if let Some(value) = class
        .strip_prefix("list-image-")
        .and_then(tailwind_arbitrary_or_custom_var)
    {
        return Some(("list-style-image".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("list-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "list-style-type".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("columns-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("columns".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class.strip_prefix("columns-") {
        if let Some(value) = tailwind_columns_value(value) {
            return Some(("columns".to_string(), value));
        }
    }
    None
}

fn is_css_background_image_value(value: &str) -> bool {
    matches!(
        value.split_once('(').map(|(name, _)| name.trim()),
        Some(
            "url"
                | "image"
                | "image-set"
                | "linear-gradient"
                | "radial-gradient"
                | "conic-gradient"
                | "repeating-linear-gradient"
                | "repeating-radial-gradient"
                | "repeating-conic-gradient"
        )
    ) && value.ends_with(')')
}

fn is_background_position_value(value: &str) -> bool {
    let parts = value.split_whitespace().collect::<Vec<_>>();
    !parts.is_empty()
        && parts.iter().all(|part| {
            matches!(*part, "top" | "right" | "bottom" | "left" | "center")
                || parse_length(part).is_some()
        })
}

fn tailwind_columns_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_container_width(value).map(ToString::to_string))
        .or_else(|| value.parse::<u16>().ok().map(|value| value.to_string()))
}

fn tailwind_flex_value(value: &str) -> Option<String> {
    match value {
        "auto" => Some("auto".to_string()),
        "initial" => Some("0 auto".to_string()),
        "none" => Some("none".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| tailwind_fraction(value).map(|value| format!("calc({value} * 100%)")))
            .or_else(|| value.parse::<f64>().ok().map(trim_float)),
    }
}

fn tailwind_basis_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_container_width(value).map(ToString::to_string))
        .or_else(|| tailwind_length(value).map(style_length_css))
}

fn tailwind_number_token(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value).or_else(|| value.parse::<f64>().ok().map(trim_float))
}

fn tailwind_order_value(class: &str) -> Option<String> {
    let negative = class.starts_with("-order-");
    let value = if negative {
        class.strip_prefix("-order-")?
    } else {
        class.strip_prefix("order-")?
    };
    let value = match value {
        "first" if !negative => "-9999".to_string(),
        "last" if !negative => "9999".to_string(),
        "none" if !negative => "0".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| value.parse::<i32>().ok().map(|value| value.to_string()))?,
    };
    Some(if negative {
        format!("calc({value} * -1)")
    } else {
        value
    })
}

fn tailwind_fraction(value: &str) -> Option<String> {
    let (numerator, denominator) = value.split_once('/')?;
    let numerator = numerator.parse::<f64>().ok()?;
    let denominator = denominator.parse::<f64>().ok()?;
    if denominator == 0.0 {
        None
    } else {
        Some(trim_float(numerator / denominator))
    }
}

fn tailwind_container_width(value: &str) -> Option<&'static str> {
    match value {
        "3xs" => Some("16rem"),
        "2xs" => Some("18rem"),
        "xs" => Some("20rem"),
        "sm" => Some("24rem"),
        "md" => Some("28rem"),
        "lg" => Some("32rem"),
        "xl" => Some("36rem"),
        "2xl" => Some("42rem"),
        "3xl" => Some("48rem"),
        "4xl" => Some("56rem"),
        "5xl" => Some("64rem"),
        "6xl" => Some("72rem"),
        "7xl" => Some("80rem"),
        _ => None,
    }
}

fn tailwind_font_family(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
}

fn tailwind_letter_spacing(class: &str) -> Option<String> {
    let negative = class.starts_with("-tracking-");
    let value = if negative {
        class.strip_prefix("-tracking-")?
    } else {
        class.strip_prefix("tracking-")?
    };
    let value = match value {
        "tighter" => "-0.05em".to_string(),
        "tight" => "-0.025em".to_string(),
        "normal" => "0em".to_string(),
        "wide" => "0.025em".to_string(),
        "wider" => "0.05em".to_string(),
        "widest" => "0.1em".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| parse_length(value).map(style_length_css))?,
    };
    Some(if negative {
        format!("calc({value} * -1)")
    } else {
        value
    })
}

fn tailwind_decoration_declaration(class: &str) -> Option<(String, String)> {
    let value = class.strip_prefix("decoration-")?;
    if let Some(value) = tailwind_decoration_thickness(value) {
        return Some(("text-decoration-thickness".to_string(), value));
    }
    tailwind_color_css(value).map(|value| ("text-decoration-color".to_string(), value))
}

fn tailwind_decoration_thickness(value: &str) -> Option<String> {
    match value {
        "auto" => Some("auto".to_string()),
        "from-font" => Some("from-font".to_string()),
        _ => tailwind_border_width(value).map(style_length_css),
    }
}

fn tailwind_underline_offset(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_border_width(value).map(style_length_css))
}

fn tailwind_visual_effect_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("shadow-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("box-shadow".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("outline".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-offset-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "outline-offset".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("cursor-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("cursor".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("aspect-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("aspect-ratio".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("transform-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("transform".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("filter".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "backdrop-filter".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("outline-offset-")
        .and_then(tailwind_length)
    {
        return Some(("outline-offset".to_string(), style_length_css(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-")
        .and_then(tailwind_border_width)
    {
        return Some(("outline-width".to_string(), style_length_css(value)));
    }
    if let Some(value) = class.strip_prefix("outline-").and_then(tailwind_color_css) {
        return Some(("outline-color".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("cursor-") {
        if is_tailwind_cursor(value) {
            return Some(("cursor".to_string(), value.to_string()));
        }
    }
    if let Some(value) = class.strip_prefix("aspect-") {
        if let Some((width, height)) = value.split_once('/') {
            if width.parse::<f64>().is_ok() && height.parse::<f64>().is_ok() {
                return Some(("aspect-ratio".to_string(), format!("{width} / {height}")));
            }
        }
    }
    if let Some(value) = tailwind_transform_declaration(class) {
        return Some(("transform".to_string(), value));
    }
    None
}

fn tailwind_ring_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "ring" => {
            insert_tailwind_ring_width_declarations(&mut declarations, false, "1px".to_string());
            return Some(declarations);
        }
        "ring-inset" => {
            declarations.insert("--tw-ring-inset".to_string(), "inset".to_string());
            declarations.insert(
                "box-shadow".to_string(),
                tailwind_box_shadow_pipeline().to_string(),
            );
            return Some(declarations);
        }
        "inset-ring" => {
            insert_tailwind_ring_width_declarations(&mut declarations, true, "1px".to_string());
            return Some(declarations);
        }
        _ => {}
    }
    if let Some(value) = class.strip_prefix("ring-") {
        if let Some(width) = tailwind_ring_width(value) {
            insert_tailwind_ring_width_declarations(&mut declarations, false, width);
            return Some(declarations);
        }
        declarations.insert(
            "--tw-ring-color".to_string(),
            tailwind_ring_color_value(value)?,
        );
        declarations.insert(
            "box-shadow".to_string(),
            tailwind_box_shadow_pipeline().to_string(),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("inset-ring-") {
        if let Some(width) = tailwind_ring_width(value) {
            insert_tailwind_ring_width_declarations(&mut declarations, true, width);
            return Some(declarations);
        }
        declarations.insert(
            "--tw-inset-ring-color".to_string(),
            tailwind_ring_color_value(value)?,
        );
        declarations.insert(
            "box-shadow".to_string(),
            tailwind_box_shadow_pipeline().to_string(),
        );
        return Some(declarations);
    }
    None
}

fn insert_tailwind_ring_width_declarations(
    declarations: &mut BTreeMap<String, String>,
    inset: bool,
    width: String,
) {
    let property = if inset {
        "--tw-inset-ring-shadow"
    } else {
        "--tw-ring-shadow"
    };
    let prefix = if inset { "inset " } else { "" };
    declarations.insert(property.to_string(), format!("{prefix}0 0 0 {width}"));
    declarations.insert(
        "box-shadow".to_string(),
        tailwind_box_shadow_pipeline().to_string(),
    );
}

fn tailwind_ring_width(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    value
        .parse::<f64>()
        .ok()
        .map(|value| format!("{}px", trim_float(value)))
}

fn tailwind_ring_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_border_color_value(value)
}

pub(super) fn tailwind_box_shadow_pipeline() -> &'static str {
    "var(--tw-inset-ring-shadow), var(--tw-ring-shadow)"
}

pub(super) fn compose_tailwind_ring_shadow(
    shadow: &str,
    color: Option<&str>,
    force_inset: bool,
) -> String {
    let mut shadow = shadow.trim().to_string();
    if force_inset && !shadow.starts_with("inset ") {
        shadow = format!("inset {shadow}");
    }
    let Some(color) = color.map(str::trim).filter(|color| !color.is_empty()) else {
        return shadow;
    };
    if shadow.contains(color) {
        shadow
    } else {
        format!("{shadow} {color}")
    }
}

fn tailwind_grid_declaration(class: &str) -> Option<(String, String)> {
    let declaration = match class {
        "grid-flow-row" => Some(("grid-auto-flow", "row".to_string())),
        "grid-flow-col" => Some(("grid-auto-flow", "column".to_string())),
        "grid-flow-dense" => Some(("grid-auto-flow", "dense".to_string())),
        "grid-flow-row-dense" => Some(("grid-auto-flow", "row dense".to_string())),
        "grid-flow-col-dense" => Some(("grid-auto-flow", "column dense".to_string())),
        "auto-cols-auto" => Some(("grid-auto-columns", "auto".to_string())),
        "auto-cols-min" => Some(("grid-auto-columns", "min-content".to_string())),
        "auto-cols-max" => Some(("grid-auto-columns", "max-content".to_string())),
        "auto-cols-fr" => Some(("grid-auto-columns", "minmax(0, 1fr)".to_string())),
        "auto-rows-auto" => Some(("grid-auto-rows", "auto".to_string())),
        "auto-rows-min" => Some(("grid-auto-rows", "min-content".to_string())),
        "auto-rows-max" => Some(("grid-auto-rows", "max-content".to_string())),
        "auto-rows-fr" => Some(("grid-auto-rows", "minmax(0, 1fr)".to_string())),
        "col-auto" => Some(("grid-column", "auto".to_string())),
        "col-span-full" => Some(("grid-column", "1 / -1".to_string())),
        "row-auto" => Some(("grid-row", "auto".to_string())),
        "row-span-full" => Some(("grid-row", "1 / -1".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        return Some((property.to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("grid-cols-")
        .and_then(tailwind_grid_track_list)
    {
        return Some(("grid-template-columns".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("grid-rows-")
        .and_then(tailwind_grid_track_list)
    {
        return Some(("grid-template-rows".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("auto-cols-")
        .and_then(tailwind_grid_auto_track)
    {
        return Some(("grid-auto-columns".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("auto-rows-")
        .and_then(tailwind_grid_auto_track)
    {
        return Some(("grid-auto-rows".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("col-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("grid-column".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("row-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("grid-row".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class.strip_prefix("col-").and_then(tailwind_custom_var) {
        return Some(("grid-column".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("row-").and_then(tailwind_custom_var) {
        return Some(("grid-row".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("col-span-").and_then(tailwind_grid_line) {
        return Some((
            "grid-column".to_string(),
            format!("span {value} / span {value}"),
        ));
    }
    if let Some(value) = class.strip_prefix("row-span-").and_then(tailwind_grid_line) {
        return Some((
            "grid-row".to_string(),
            format!("span {value} / span {value}"),
        ));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "col-start-") {
        return Some(("grid-column-start".to_string(), value));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "col-end-") {
        return Some(("grid-column-end".to_string(), value));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "row-start-") {
        return Some(("grid-row-start".to_string(), value));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "row-end-") {
        return Some(("grid-row-end".to_string(), value));
    }
    None
}

fn tailwind_grid_track_list(value: &str) -> Option<String> {
    if matches!(value, "none" | "subgrid") {
        return Some(value.to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    let count = value.parse::<u16>().ok()?;
    if count == 0 {
        return None;
    }
    Some(format!("repeat({count}, minmax(0, 1fr))"))
}

fn tailwind_grid_auto_track(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    match value {
        "auto" => Some("auto".to_string()),
        "min" => Some("min-content".to_string()),
        "max" => Some("max-content".to_string()),
        "fr" => Some("minmax(0, 1fr)".to_string()),
        _ => None,
    }
}

fn tailwind_grid_line_utility(class: &str, prefix: &str) -> Option<String> {
    if let Some(value) = class.strip_prefix(prefix).and_then(tailwind_grid_line) {
        return Some(value);
    }
    let negative_prefix = format!("-{prefix}");
    let value = class
        .strip_prefix(&negative_prefix)
        .and_then(tailwind_grid_line)?;
    Some(format!("calc({value} * -1)"))
}

fn tailwind_grid_line(value: &str) -> Option<String> {
    if value == "auto" {
        return Some("auto".to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    value.parse::<u16>().ok().map(|value| value.to_string())
}

fn insert_edge_declarations(
    declarations: &mut BTreeMap<String, String>,
    prefix: &str,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert(prefix.to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert(format!("{prefix}-inline"), value);
        }
        EdgeSelection::Y => {
            declarations.insert(format!("{prefix}-block"), value);
        }
        EdgeSelection::Top => {
            declarations.insert(format!("{prefix}-top"), value);
        }
        EdgeSelection::Right => {
            declarations.insert(format!("{prefix}-right"), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert(format!("{prefix}-bottom"), value);
        }
        EdgeSelection::Left => {
            declarations.insert(format!("{prefix}-left"), value);
        }
    }
}

fn insert_position_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert("inset".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("inset-inline".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("inset-block".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("top".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("right".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("bottom".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("left".to_string(), value);
        }
    }
}

fn insert_logical_edge_declaration(
    declarations: &mut BTreeMap<String, String>,
    prefix: &str,
    edges: LogicalEdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    let property = match edges {
        LogicalEdgeSelection::Block => format!("{prefix}-block"),
        LogicalEdgeSelection::Inline => format!("{prefix}-inline"),
        LogicalEdgeSelection::BlockStart => format!("{prefix}-block-start"),
        LogicalEdgeSelection::BlockEnd => format!("{prefix}-block-end"),
        LogicalEdgeSelection::InlineStart => format!("{prefix}-inline-start"),
        LogicalEdgeSelection::InlineEnd => format!("{prefix}-inline-end"),
    };
    declarations.insert(property, value);
}

fn insert_logical_position_declaration(
    declarations: &mut BTreeMap<String, String>,
    edges: LogicalEdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    let property = match edges {
        LogicalEdgeSelection::Block => "inset-block",
        LogicalEdgeSelection::Inline => "inset-inline",
        LogicalEdgeSelection::BlockStart => "inset-block-start",
        LogicalEdgeSelection::BlockEnd => "inset-block-end",
        LogicalEdgeSelection::InlineStart => "inset-inline-start",
        LogicalEdgeSelection::InlineEnd => "inset-inline-end",
    };
    declarations.insert(property.to_string(), value);
}

fn insert_corner_radius_declarations(
    declarations: &mut BTreeMap<String, String>,
    corners: CornerSelection,
    radius: CornerRadius,
) {
    let value = corner_radius_css(radius);
    match corners {
        CornerSelection::All => {
            declarations.insert("border-radius".to_string(), value);
        }
        CornerSelection::Top => {
            declarations.insert("border-top-left-radius".to_string(), value.clone());
            declarations.insert("border-top-right-radius".to_string(), value);
        }
        CornerSelection::Right => {
            declarations.insert("border-top-right-radius".to_string(), value.clone());
            declarations.insert("border-bottom-right-radius".to_string(), value);
        }
        CornerSelection::Bottom => {
            declarations.insert("border-bottom-right-radius".to_string(), value.clone());
            declarations.insert("border-bottom-left-radius".to_string(), value);
        }
        CornerSelection::Left => {
            declarations.insert("border-top-left-radius".to_string(), value.clone());
            declarations.insert("border-bottom-left-radius".to_string(), value);
        }
        CornerSelection::TopLeft => {
            declarations.insert("border-top-left-radius".to_string(), value);
        }
        CornerSelection::TopRight => {
            declarations.insert("border-top-right-radius".to_string(), value);
        }
        CornerSelection::BottomRight => {
            declarations.insert("border-bottom-right-radius".to_string(), value);
        }
        CornerSelection::BottomLeft => {
            declarations.insert("border-bottom-left-radius".to_string(), value);
        }
    }
}

fn insert_logical_corner_radius_declarations(
    declarations: &mut BTreeMap<String, String>,
    corners: LogicalCornerSelection,
    radius: CornerRadius,
) {
    let value = corner_radius_css(radius);
    match corners {
        LogicalCornerSelection::Start => {
            declarations.insert("border-start-start-radius".to_string(), value.clone());
            declarations.insert("border-end-start-radius".to_string(), value);
        }
        LogicalCornerSelection::End => {
            declarations.insert("border-start-end-radius".to_string(), value.clone());
            declarations.insert("border-end-end-radius".to_string(), value);
        }
        LogicalCornerSelection::StartStart => {
            declarations.insert("border-start-start-radius".to_string(), value);
        }
        LogicalCornerSelection::StartEnd => {
            declarations.insert("border-start-end-radius".to_string(), value);
        }
        LogicalCornerSelection::EndEnd => {
            declarations.insert("border-end-end-radius".to_string(), value);
        }
        LogicalCornerSelection::EndStart => {
            declarations.insert("border-end-start-radius".to_string(), value);
        }
    }
}

fn insert_border_color_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: String,
) {
    match edges {
        EdgeSelection::All => {
            declarations.insert("border-color".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("border-inline-color".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("border-block-color".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("border-top-color".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("border-right-color".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("border-bottom-color".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("border-left-color".to_string(), value);
        }
    }
}

fn insert_logical_border_color_declaration(
    declarations: &mut BTreeMap<String, String>,
    edges: LogicalEdgeSelection,
    value: String,
) {
    let property = match edges {
        LogicalEdgeSelection::Block => "border-block-color",
        LogicalEdgeSelection::Inline => "border-inline-color",
        LogicalEdgeSelection::BlockStart => "border-block-start-color",
        LogicalEdgeSelection::BlockEnd => "border-block-end-color",
        LogicalEdgeSelection::InlineStart => "border-inline-start-color",
        LogicalEdgeSelection::InlineEnd => "border-inline-end-color",
    };
    declarations.insert(property.to_string(), value);
}

fn insert_logical_border_width_declaration(
    declarations: &mut BTreeMap<String, String>,
    edges: LogicalEdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    let property = match edges {
        LogicalEdgeSelection::Block => "border-block-width",
        LogicalEdgeSelection::Inline => "border-inline-width",
        LogicalEdgeSelection::BlockStart => "border-block-start-width",
        LogicalEdgeSelection::BlockEnd => "border-block-end-width",
        LogicalEdgeSelection::InlineStart => "border-inline-start-width",
        LogicalEdgeSelection::InlineEnd => "border-inline-end-width",
    };
    declarations.insert(property.to_string(), value);
}

fn insert_border_width_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert("border-width".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("border-inline-width".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("border-block-width".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("border-top-width".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("border-right-width".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("border-bottom-width".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("border-left-width".to_string(), value);
        }
    }
}

fn style_length_css(value: StyleLength) -> String {
    match value {
        StyleLength::Points(value) => format!("{}px", trim_float(value)),
        StyleLength::Percent(value) => format!("{}%", trim_float(value)),
        StyleLength::Auto => "auto".to_string(),
        StyleLength::Css(value) => value,
    }
}

fn corner_radius_css(radius: CornerRadius) -> String {
    let horizontal = style_length_css(radius.horizontal);
    if let Some(vertical) = radius.vertical {
        format!("{horizontal} {}", style_length_css(vertical))
    } else {
        horizontal
    }
}

fn trim_float(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn is_tailwind_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
}

fn is_tailwind_cursor(value: &str) -> bool {
    matches!(
        value,
        "auto"
            | "default"
            | "pointer"
            | "wait"
            | "text"
            | "move"
            | "help"
            | "not-allowed"
            | "none"
            | "context-menu"
            | "progress"
            | "cell"
            | "crosshair"
            | "vertical-text"
            | "alias"
            | "copy"
            | "no-drop"
            | "grab"
            | "grabbing"
            | "all-scroll"
            | "col-resize"
            | "row-resize"
            | "n-resize"
            | "e-resize"
            | "s-resize"
            | "w-resize"
            | "ne-resize"
            | "nw-resize"
            | "se-resize"
            | "sw-resize"
            | "ew-resize"
            | "ns-resize"
            | "nesw-resize"
            | "nwse-resize"
            | "zoom-in"
            | "zoom-out"
    )
}

fn tailwind_transform_declaration(class: &str) -> Option<String> {
    if let Some(suffix) = class.strip_prefix("rotate-") {
        if let Some(value) = tailwind_rotate_value(suffix) {
            return Some(format!("rotate({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-rotate-") {
        if let Some(value) = tailwind_rotate_value(suffix) {
            return Some(format!("rotate(-{value})"));
        }
    }
    if let Some(value) = class.strip_prefix("scale-").and_then(tailwind_scale_value) {
        return Some(format!("scale({value})"));
    }
    if let Some(value) = class
        .strip_prefix("scale-x-")
        .and_then(tailwind_scale_value)
    {
        return Some(format!("scaleX({value})"));
    }
    if let Some(value) = class
        .strip_prefix("scale-y-")
        .and_then(tailwind_scale_value)
    {
        return Some(format!("scaleY({value})"));
    }
    if let Some(suffix) = class.strip_prefix("translate-x-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateX({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-translate-x-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateX(-{value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("translate-y-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateY({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-translate-y-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateY(-{value})"));
        }
    }
    None
}

fn tailwind_rotate_value(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    Some(format!("{}deg", trim_float(value.parse::<f64>().ok()?)))
}

fn tailwind_scale_value(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    Some(trim_float(value.parse::<f64>().ok()? / 100.0))
}

fn tailwind_translate_value(value: &str) -> Option<String> {
    tailwind_length(value).map(style_length_css)
}

fn tailwind_text_size_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if let Some(arbitrary) = class
        .strip_prefix("text-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if parse_length(&value).is_some() {
            declarations.insert("font-size".to_string(), value);
            return Some(declarations);
        }
        return None;
    }
    let (font_size, line_height) = match class {
        "text-xs" => ("0.75rem", "1rem"),
        "text-sm" => ("0.875rem", "1.25rem"),
        "text-base" => ("1rem", "1.5rem"),
        "text-lg" => ("1.125rem", "1.75rem"),
        "text-xl" => ("1.25rem", "1.75rem"),
        "text-2xl" => ("1.5rem", "2rem"),
        "text-3xl" => ("1.875rem", "2.25rem"),
        "text-4xl" => ("2.25rem", "2.5rem"),
        "text-5xl" => ("3rem", "1"),
        "text-6xl" => ("3.75rem", "1"),
        "text-7xl" => ("4.5rem", "1"),
        "text-8xl" => ("6rem", "1"),
        "text-9xl" => ("8rem", "1"),
        _ => return None,
    };
    declarations.insert("font-size".to_string(), font_size.to_string());
    declarations.insert("line-height".to_string(), line_height.to_string());
    Some(declarations)
}

fn tailwind_font_feature_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if let Some((property, value)) = tailwind_font_variant_numeric_declaration(class) {
        declarations.insert(property, value);
        if class != "normal-nums" {
            declarations.insert(
                "font-variant-numeric".to_string(),
                tailwind_font_variant_numeric_pipeline(),
            );
        }
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("font-stretch-")
        .and_then(tailwind_font_stretch_value)
    {
        declarations.insert("font-stretch".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("font-features-")
        .and_then(tailwind_arbitrary_or_custom_var)
    {
        declarations.insert("font-feature-settings".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("tab-").and_then(tailwind_tab_size_value) {
        declarations.insert("tab-size".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = tailwind_text_shadow_value(class) {
        declarations.insert("text-shadow".to_string(), value);
        return Some(declarations);
    }
    None
}

fn tailwind_font_variant_numeric_declaration(class: &str) -> Option<(String, String)> {
    let declaration = match class {
        "normal-nums" => ("font-variant-numeric", "normal"),
        "ordinal" => ("--tw-ordinal", "ordinal"),
        "slashed-zero" => ("--tw-slashed-zero", "slashed-zero"),
        "lining-nums" => ("--tw-numeric-figure", "lining-nums"),
        "oldstyle-nums" => ("--tw-numeric-figure", "oldstyle-nums"),
        "proportional-nums" => ("--tw-numeric-spacing", "proportional-nums"),
        "tabular-nums" => ("--tw-numeric-spacing", "tabular-nums"),
        "diagonal-fractions" => ("--tw-numeric-fraction", "diagonal-fractions"),
        "stacked-fractions" => ("--tw-numeric-fraction", "stacked-fractions"),
        _ => return None,
    };
    Some((declaration.0.to_string(), declaration.1.to_string()))
}

fn tailwind_font_variant_numeric_pipeline() -> String {
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)".to_string()
}

fn tailwind_font_stretch_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    match value {
        "ultra-condensed" | "extra-condensed" | "condensed" | "semi-condensed" | "normal"
        | "semi-expanded" | "expanded" | "extra-expanded" | "ultra-expanded" => {
            Some(value.to_string())
        }
        _ => value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}%", trim_float(value))),
    }
}

fn tailwind_tab_size_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| value.parse::<u32>().ok().map(|value| value.to_string()))
}

fn tailwind_text_shadow_value(class: &str) -> Option<String> {
    if class == "text-shadow-none" {
        return Some("none".to_string());
    }
    let value = class.strip_prefix("text-shadow-")?;
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    if is_tailwind_identifier(value) {
        Some(format!("var(--text-shadow-{value})"))
    } else {
        None
    }
}

fn tailwind_line_clamp_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let value = class.strip_prefix("line-clamp-")?;
    let mut declarations = BTreeMap::new();
    if value == "none" {
        declarations.insert("overflow".to_string(), "visible".to_string());
        declarations.insert("display".to_string(), "block".to_string());
        declarations.insert("-webkit-box-orient".to_string(), "horizontal".to_string());
        declarations.insert("-webkit-line-clamp".to_string(), "unset".to_string());
        return Some(declarations);
    }
    let value = tailwind_line_clamp_value(value)?;
    declarations.insert("overflow".to_string(), "hidden".to_string());
    declarations.insert("display".to_string(), "-webkit-box".to_string());
    declarations.insert("-webkit-box-orient".to_string(), "vertical".to_string());
    declarations.insert("-webkit-line-clamp".to_string(), value);
    Some(declarations)
}

fn tailwind_line_clamp_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "number") {
        return Some(value);
    }
    if let Some(value) = tailwind_custom_var(value) {
        return Some(value);
    }
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    value.parse::<u32>().ok().map(|value| value.to_string())
}

pub(super) fn tailwind_length(value: &str) -> Option<StyleLength> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
    }
    if let Some(variable) = tailwind_custom_var(value) {
        return Some(StyleLength::Css(variable));
    }
    if value == "full" {
        return Some(StyleLength::Percent(100.0));
    }
    if value == "screen" {
        return Some(StyleLength::Percent(100.0));
    }
    if value == "auto" {
        return Some(StyleLength::Auto);
    }
    if value == "px" {
        return Some(StyleLength::Points(1.0));
    }
    if let Some((numerator, denominator)) = value.split_once('/') {
        let numerator = numerator.parse::<f64>().ok()?;
        let denominator = denominator.parse::<f64>().ok()?;
        if denominator != 0.0 {
            return Some(StyleLength::Percent((numerator / denominator) * 100.0));
        }
    }
    let value = value.parse::<f64>().ok()?;
    Some(StyleLength::Points(value * 4.0))
}

fn tailwind_line_height(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    match value {
        "none" => Some("1".to_string()),
        "tight" => Some("1.25".to_string()),
        "snug" => Some("1.375".to_string()),
        "normal" => Some("1.5".to_string()),
        "relaxed" => Some("1.625".to_string()),
        "loose" => Some("2".to_string()),
        _ => tailwind_length(value).map(style_length_css),
    }
}

fn tailwind_text_indent(class: &str) -> Option<String> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let value = class.strip_prefix("indent-")?;
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some(style_length_css(length))
}

pub(super) fn tailwind_opacity(value: &str) -> Option<f64> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return arbitrary.parse::<f64>().ok();
    }
    value.parse::<f64>().ok().map(|value| value / 100.0)
}

pub(super) fn tailwind_color(value: &str) -> Option<StyleColor> {
    let (value, opacity) = split_tailwind_color_opacity(value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let color = parse_color(&tailwind_arbitrary_value(arbitrary))?;
        return Some(apply_tailwind_color_opacity(color, opacity));
    }
    let color = match value {
        "black" => parse_color("#000"),
        "white" => parse_color("#fff"),
        "transparent" => Some(StyleColor::Keyword("transparent".to_string())),
        "current" => Some(StyleColor::Keyword("currentColor".to_string())),
        "inherit" => Some(StyleColor::Keyword("inherit".to_string())),
        other if is_tailwind_palette_color(other) => Some(StyleColor::Keyword(other.to_string())),
        _ => None,
    }?;
    Some(apply_tailwind_color_opacity(color, opacity))
}

fn tailwind_accent_color_css(value: &str) -> Option<String> {
    if value == "auto" {
        Some("auto".to_string())
    } else {
        tailwind_color_css(value)
    }
}

fn tailwind_color_css(value: &str) -> Option<String> {
    let (value, opacity) = split_tailwind_color_opacity(value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if let Some(color) = parse_color(&value) {
            return Some(style_color_css(&apply_tailwind_color_opacity(
                color, opacity,
            )));
        }
        return Some(apply_tailwind_keyword_opacity(value, opacity));
    }
    let color = match value {
        "black" => parse_color("#000")
            .map(|color| style_color_css(&apply_tailwind_color_opacity(color, opacity))),
        "white" => parse_color("#fff")
            .map(|color| style_color_css(&apply_tailwind_color_opacity(color, opacity))),
        "transparent" => Some("transparent".to_string()),
        "current" => Some("currentColor".to_string()),
        "inherit" => Some("inherit".to_string()),
        other if is_tailwind_palette_color(other) => Some(other.to_string()),
        _ => None,
    }?;
    Some(match value {
        "black" | "white" => color,
        _ => apply_tailwind_keyword_opacity(color, opacity),
    })
}

fn split_tailwind_color_opacity(value: &str) -> (&str, Option<&str>) {
    let mut bracket_depth = 0usize;
    for (index, ch) in value.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '/' if bracket_depth == 0 => return (&value[..index], Some(&value[index + 1..])),
            _ => {}
        }
    }
    (value, None)
}

fn apply_tailwind_color_opacity(color: StyleColor, opacity: Option<&str>) -> StyleColor {
    let Some(alpha) = opacity.and_then(tailwind_opacity_alpha) else {
        return color;
    };
    match color {
        StyleColor::Rgba {
            red, green, blue, ..
        } => StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        },
        StyleColor::Function(value) => {
            StyleColor::Function(apply_tailwind_function_opacity(value, opacity))
        }
        StyleColor::Keyword(value) => {
            StyleColor::Keyword(apply_tailwind_keyword_opacity(value, opacity))
        }
    }
}

fn apply_tailwind_function_opacity(value: String, opacity: Option<&str>) -> String {
    let Some(percent) = opacity.and_then(tailwind_opacity_percent) else {
        return value;
    };
    format!("color-mix(in srgb, {value} {percent}, transparent)")
}

fn apply_tailwind_keyword_opacity(value: String, opacity: Option<&str>) -> String {
    let Some(opacity) = opacity else {
        return value;
    };
    let Some(percent) = tailwind_opacity_percent(opacity) else {
        return value;
    };
    if value == "transparent" {
        value
    } else {
        format!("{value} / {percent}")
    }
}

fn tailwind_opacity_alpha(value: &str) -> Option<u8> {
    let opacity = tailwind_opacity(value)?;
    Some((opacity.clamp(0.0, 1.0) * 255.0).round() as u8)
}

fn tailwind_opacity_percent(value: &str) -> Option<String> {
    let opacity = tailwind_opacity(value)?;
    Some(format!("{}%", trim_float(opacity.clamp(0.0, 1.0) * 100.0)))
}

fn style_color_css(color: &StyleColor) -> String {
    match color {
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        } if *alpha < 255 => {
            let alpha = trim_float((*alpha as f64 / 255.0 * 100.0).round() / 100.0);
            format!("rgba({red}, {green}, {blue}, {alpha})")
        }
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha: _,
        } => format!("rgb({red}, {green}, {blue})"),
        StyleColor::Function(value) => value.clone(),
        StyleColor::Keyword(value) => value.clone(),
    }
}

fn tailwind_z_index(class: &str) -> Option<String> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let value = class.strip_prefix("z-")?;
    if value == "auto" {
        return Some("auto".to_string());
    }
    let value = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .map(tailwind_arbitrary_value)
        .unwrap_or_else(|| value.to_string());
    if negative {
        Some(format!("-{value}"))
    } else {
        Some(value)
    }
}

fn is_tailwind_palette_color(value: &str) -> bool {
    let Some((name, shade)) = value.rsplit_once('-') else {
        return false;
    };
    matches!(
        name,
        "slate"
            | "gray"
            | "zinc"
            | "neutral"
            | "stone"
            | "red"
            | "orange"
            | "amber"
            | "yellow"
            | "lime"
            | "green"
            | "emerald"
            | "teal"
            | "cyan"
            | "sky"
            | "blue"
            | "indigo"
            | "violet"
            | "purple"
            | "fuchsia"
            | "pink"
            | "rose"
    ) && matches!(
        shade,
        "50" | "100" | "200" | "300" | "400" | "500" | "600" | "700" | "800" | "900" | "950"
    )
}

fn tailwind_inset_utility(class: &str) -> Option<(EdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let (edges, value) = if let Some(value) = class.strip_prefix("inset-x-") {
        (EdgeSelection::X, value)
    } else if let Some(value) = class.strip_prefix("inset-y-") {
        (EdgeSelection::Y, value)
    } else if let Some(value) = class.strip_prefix("inset-") {
        (EdgeSelection::All, value)
    } else if let Some(value) = class.strip_prefix("top-") {
        (EdgeSelection::Top, value)
    } else if let Some(value) = class.strip_prefix("right-") {
        (EdgeSelection::Right, value)
    } else if let Some(value) = class.strip_prefix("bottom-") {
        (EdgeSelection::Bottom, value)
    } else if let Some(value) = class.strip_prefix("left-") {
        (EdgeSelection::Left, value)
    } else {
        return None;
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

fn tailwind_logical_inset_utility(class: &str) -> Option<(LogicalEdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let (edges, value) = if let Some(value) = class.strip_prefix("start-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if let Some(value) = class.strip_prefix("end-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else if let Some(value) = class.strip_prefix("inset-s-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if let Some(value) = class.strip_prefix("inset-e-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else if let Some(value) = class.strip_prefix("inset-bs-") {
        (LogicalEdgeSelection::BlockStart, value)
    } else if let Some(value) = class.strip_prefix("inset-be-") {
        (LogicalEdgeSelection::BlockEnd, value)
    } else if let Some(value) = class.strip_prefix("inset-is-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if let Some(value) = class.strip_prefix("inset-ie-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else {
        return None;
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

fn tailwind_border_width_utility(class: &str) -> Option<(EdgeSelection, StyleLength)> {
    let suffix = class.strip_prefix("border")?;
    if suffix.is_empty() {
        return Some((EdgeSelection::All, StyleLength::Points(1.0)));
    }
    let suffix = suffix.strip_prefix('-')?;
    let (edges, value) = if suffix == "x" {
        (EdgeSelection::X, "1")
    } else if let Some(value) = suffix.strip_prefix("x-") {
        (EdgeSelection::X, value)
    } else if suffix == "y" {
        (EdgeSelection::Y, "1")
    } else if let Some(value) = suffix.strip_prefix("y-") {
        (EdgeSelection::Y, value)
    } else if suffix == "t" {
        (EdgeSelection::Top, "1")
    } else if let Some(value) = suffix.strip_prefix("t-") {
        (EdgeSelection::Top, value)
    } else if suffix == "r" {
        (EdgeSelection::Right, "1")
    } else if let Some(value) = suffix.strip_prefix("r-") {
        (EdgeSelection::Right, value)
    } else if suffix == "b" {
        (EdgeSelection::Bottom, "1")
    } else if let Some(value) = suffix.strip_prefix("b-") {
        (EdgeSelection::Bottom, value)
    } else if suffix == "l" {
        (EdgeSelection::Left, "1")
    } else if let Some(value) = suffix.strip_prefix("l-") {
        (EdgeSelection::Left, value)
    } else {
        (EdgeSelection::All, suffix)
    };
    Some((edges, tailwind_border_width(value)?))
}

fn tailwind_logical_border_width_utility(
    class: &str,
) -> Option<(LogicalEdgeSelection, StyleLength)> {
    let suffix = class.strip_prefix("border-")?;
    let (edges, value) = if suffix == "s" {
        (LogicalEdgeSelection::InlineStart, "1")
    } else if let Some(value) = suffix.strip_prefix("s-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if suffix == "e" {
        (LogicalEdgeSelection::InlineEnd, "1")
    } else if let Some(value) = suffix.strip_prefix("e-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else if suffix == "bs" {
        (LogicalEdgeSelection::BlockStart, "1")
    } else if let Some(value) = suffix.strip_prefix("bs-") {
        (LogicalEdgeSelection::BlockStart, value)
    } else if suffix == "be" {
        (LogicalEdgeSelection::BlockEnd, "1")
    } else if let Some(value) = suffix.strip_prefix("be-") {
        (LogicalEdgeSelection::BlockEnd, value)
    } else if suffix == "is" {
        (LogicalEdgeSelection::InlineStart, "1")
    } else if let Some(value) = suffix.strip_prefix("is-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if suffix == "ie" {
        (LogicalEdgeSelection::InlineEnd, "1")
    } else if let Some(value) = suffix.strip_prefix("ie-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else {
        return None;
    };
    Some((edges, tailwind_border_width(value)?))
}

fn tailwind_border_color_declarations(class: &str) -> Option<BTreeMap<String, String>> {
    let suffix = class.strip_prefix("border-")?;
    let mut declarations = BTreeMap::new();
    if let Some((edges, value)) = tailwind_border_color_edge_value(suffix) {
        let color = tailwind_border_color_value(value)?;
        insert_border_color_declarations(&mut declarations, edges, color);
        return Some(declarations);
    }
    if let Some((edges, value)) = tailwind_logical_border_color_edge_value(suffix) {
        let color = tailwind_border_color_value(value)?;
        insert_logical_border_color_declaration(&mut declarations, edges, color);
        return Some(declarations);
    }
    let color = tailwind_border_color_value(suffix)?;
    declarations.insert("border-color".to_string(), color);
    Some(declarations)
}

fn tailwind_border_color_edge_value(value: &str) -> Option<(EdgeSelection, &str)> {
    if let Some(value) = value.strip_prefix("x-") {
        Some((EdgeSelection::X, value))
    } else if let Some(value) = value.strip_prefix("y-") {
        Some((EdgeSelection::Y, value))
    } else if let Some(value) = value.strip_prefix("t-") {
        Some((EdgeSelection::Top, value))
    } else if let Some(value) = value.strip_prefix("r-") {
        Some((EdgeSelection::Right, value))
    } else if let Some(value) = value.strip_prefix("b-") {
        Some((EdgeSelection::Bottom, value))
    } else if let Some(value) = value.strip_prefix("l-") {
        Some((EdgeSelection::Left, value))
    } else {
        None
    }
}

fn tailwind_logical_border_color_edge_value(value: &str) -> Option<(LogicalEdgeSelection, &str)> {
    if let Some(value) = value.strip_prefix("s-") {
        Some((LogicalEdgeSelection::InlineStart, value))
    } else if let Some(value) = value.strip_prefix("e-") {
        Some((LogicalEdgeSelection::InlineEnd, value))
    } else if let Some(value) = value.strip_prefix("bs-") {
        Some((LogicalEdgeSelection::BlockStart, value))
    } else if let Some(value) = value.strip_prefix("be-") {
        Some((LogicalEdgeSelection::BlockEnd, value))
    } else if let Some(value) = value.strip_prefix("is-") {
        Some((LogicalEdgeSelection::InlineStart, value))
    } else if let Some(value) = value.strip_prefix("ie-") {
        Some((LogicalEdgeSelection::InlineEnd, value))
    } else {
        None
    }
}

fn tailwind_border_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    if let Some(value) = tailwind_custom_var(value) {
        return Some(value);
    }
    tailwind_color_css(value)
}

fn tailwind_border_width(value: &str) -> Option<StyleLength> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
    }
    if value == "px" {
        return Some(StyleLength::Points(1.0));
    }
    value.parse::<f64>().ok().map(StyleLength::Points)
}

fn negate_style_length(value: StyleLength) -> Option<StyleLength> {
    match value {
        StyleLength::Points(value) => Some(StyleLength::Points(-value)),
        StyleLength::Percent(value) => Some(StyleLength::Percent(-value)),
        StyleLength::Css(value) => Some(StyleLength::Css(format!("calc({value} * -1)"))),
        StyleLength::Auto => None,
    }
}

pub(super) fn tailwind_edge_utility(
    class: &str,
    prefix: &str,
) -> Option<(EdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let suffix = class.strip_prefix(prefix)?;
    let (edges, value) = match suffix.as_bytes() {
        [b'-', ..] => (EdgeSelection::All, &suffix[1..]),
        [b'x', b'-', ..] => (EdgeSelection::X, &suffix[2..]),
        [b'y', b'-', ..] => (EdgeSelection::Y, &suffix[2..]),
        [b't', b'-', ..] => (EdgeSelection::Top, &suffix[2..]),
        [b'r', b'-', ..] => (EdgeSelection::Right, &suffix[2..]),
        [b'b', b'-', ..] => (EdgeSelection::Bottom, &suffix[2..]),
        [b'l', b'-', ..] => (EdgeSelection::Left, &suffix[2..]),
        _ => return None,
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

fn tailwind_logical_edge_utility(
    class: &str,
    prefix: &str,
    allow_negative: bool,
) -> Option<(LogicalEdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    if negative && !allow_negative {
        return None;
    }
    let class = class.strip_prefix('-').unwrap_or(class);
    let suffix = class.strip_prefix(prefix)?;
    let (edges, value) = match suffix.as_bytes() {
        [b's', b'-', ..] => (LogicalEdgeSelection::InlineStart, &suffix[2..]),
        [b'e', b'-', ..] => (LogicalEdgeSelection::InlineEnd, &suffix[2..]),
        [b'b', b's', b'-', ..] => (LogicalEdgeSelection::BlockStart, &suffix[3..]),
        [b'b', b'e', b'-', ..] => (LogicalEdgeSelection::BlockEnd, &suffix[3..]),
        [b'i', b's', b'-', ..] => (LogicalEdgeSelection::InlineStart, &suffix[3..]),
        [b'i', b'e', b'-', ..] => (LogicalEdgeSelection::InlineEnd, &suffix[3..]),
        _ => return None,
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}
