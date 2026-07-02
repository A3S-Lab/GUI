use super::support::*;

#[test]
fn parses_css_masking_properties() {
    let web = WebProps::new()
        .style("clipPath", "circle(50% at center)")
        .style("mask", "url(mask.svg) center / contain no-repeat")
        .style("-webkitMaskImage", "linear-gradient(black, transparent)")
        .style("maskMode", "luminance")
        .style("maskRepeat", "no-repeat")
        .style("maskPosition", "center")
        .style("maskSize", "cover")
        .style("maskOrigin", "border-box")
        .style("maskClip", "content-box")
        .style("maskComposite", "exclude")
        .style("maskType", "alpha")
        .style("maskBorder", "url(border.svg) 30 fill / 10px")
        .style("maskBorderSource", "url(border.svg)")
        .style("maskBorderMode", "luminance")
        .style("maskBorderSlice", "30 fill")
        .style("maskBorderWidth", "10px")
        .style("maskBorderOutset", "2px")
        .style("maskBorderRepeat", "round");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.clip_path.as_deref(), Some("circle(50% at center)"));
    assert_eq!(
        style.mask.as_deref(),
        Some("url(mask.svg) center / contain no-repeat")
    );
    assert_eq!(
        style.mask_image.as_deref(),
        Some("linear-gradient(black, transparent)")
    );
    assert_eq!(style.mask_mode.as_deref(), Some("luminance"));
    assert_eq!(style.mask_repeat.as_deref(), Some("no-repeat"));
    assert_eq!(style.mask_position.as_deref(), Some("center"));
    assert_eq!(style.mask_size.as_deref(), Some("cover"));
    assert_eq!(style.mask_origin.as_deref(), Some("border-box"));
    assert_eq!(style.mask_clip.as_deref(), Some("content-box"));
    assert_eq!(style.mask_composite.as_deref(), Some("exclude"));
    assert_eq!(style.mask_type.as_deref(), Some("alpha"));
    assert_eq!(
        style.mask_border.as_deref(),
        Some("url(border.svg) 30 fill / 10px")
    );
    assert_eq!(style.mask_border_source.as_deref(), Some("url(border.svg)"));
    assert_eq!(style.mask_border_mode.as_deref(), Some("luminance"));
    assert_eq!(style.mask_border_slice.as_deref(), Some("30 fill"));
    assert_eq!(style.mask_border_width.as_deref(), Some("10px"));
    assert_eq!(style.mask_border_outset.as_deref(), Some("2px"));
    assert_eq!(style.mask_border_repeat.as_deref(), Some("round"));
    assert!(!style.unsupported.contains_key("clip-path"));
    assert!(!style.unsupported.contains_key("-webkit-mask-image"));
    assert!(!style.unsupported.contains_key("mask-border"));
}

#[test]
fn parses_tailwind_mask_utilities() {
    let web = WebProps::new().class_name(
        "mask-[url(/mask.svg)] mask-cover mask-no-repeat mask-center \
             mask-origin-content mask-clip-padding mask-add mask-alpha \
             mask-type-luminance hover:mask-size-[50%_50%] \
             md:mask-[position:30%_50%,70%_50%] focus:mask-(--mask-image)",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.mask_image.as_deref(), Some("url(/mask.svg)"));
    assert_eq!(style.mask_size.as_deref(), Some("cover"));
    assert_eq!(style.mask_repeat.as_deref(), Some("no-repeat"));
    assert_eq!(style.mask_position.as_deref(), Some("center"));
    assert_eq!(style.mask_origin.as_deref(), Some("content-box"));
    assert_eq!(style.mask_clip.as_deref(), Some("padding-box"));
    assert_eq!(style.mask_composite.as_deref(), Some("add"));
    assert_eq!(style.mask_mode.as_deref(), Some("alpha"));
    assert_eq!(style.mask_type.as_deref(), Some("luminance"));
    assert_eq!(
        style.declarations.get("mask-image").map(String::as_str),
        Some("url(/mask.svg)")
    );
    assert_eq!(
        style.declarations.get("mask-composite").map(String::as_str),
        Some("add")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("mask-size"))
            .map(String::as_str),
        Some("50% 50%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("mask-position"))
            .map(String::as_str),
        Some("30% 50%,70% 50%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("mask-image"))
            .map(String::as_str),
        Some("var(--mask-image)")
    );
}
