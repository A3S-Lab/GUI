use super::support::*;

#[test]
fn active_interaction_variants_overlay_the_base_style_in_source_order() {
    let style = PortableStyle::from_web(
        &WebProps::new()
            .class_name("opacity-50 hover:opacity-75 focus-visible:opacity-90 active:opacity-25"),
    );

    let resolved = style.resolve_active_variants(["hover", "focus-visible", "active"]);

    assert_eq!(resolved.opacity, Some(0.25));
    assert_eq!(style.opacity, Some(0.5));
    assert_eq!(
        style.variant_declaration_order,
        [
            ("hover".to_string(), "opacity".to_string()),
            ("focus-visible".to_string(), "opacity".to_string()),
            ("active".to_string(), "opacity".to_string()),
        ]
    );
}

#[test]
fn repeated_variant_properties_keep_declaration_level_source_order() {
    let repeated = PortableStyle::from_web(
        &WebProps::new().class_name("hover:opacity-75 active:opacity-25 hover:opacity-90"),
    );
    assert_eq!(
        repeated
            .resolve_active_variants(["hover", "active"])
            .opacity,
        Some(0.9)
    );

    let unrelated_tail = PortableStyle::from_web(
        &WebProps::new().class_name("hover:opacity-75 active:opacity-25 hover:text-red-500"),
    );
    assert_eq!(
        unrelated_tail
            .resolve_active_variants(["hover", "active"])
            .opacity,
        Some(0.25)
    );
}

#[test]
fn inactive_and_unknown_variants_do_not_change_the_resolved_style() {
    let style = PortableStyle::from_web(
        &WebProps::new().class_name("opacity-50 hover:opacity-75 md:opacity-100"),
    );

    let resolved = style.resolve_active_variants(["focus-visible"]);

    assert_eq!(resolved.opacity, Some(0.5));
    assert_eq!(resolved.variant_declarations, style.variant_declarations);
}

#[test]
fn interaction_requirements_cover_pseudo_and_react_aria_data_variants() {
    let style = PortableStyle::from_web(&WebProps::new().class_name(
        "hover:opacity-75 active:opacity-50 focus-visible:opacity-100 \
         focus-within:opacity-90 data-[pressed=true]:opacity-25 \
         data-[hovered=true]:opacity-80 data-[long-pressed=true]:opacity-60 \
         data-[moving=true]:opacity-70 \
         data-[focus-visible-within=true]:opacity-100",
    ));
    let requirements = style.interaction_requirements();

    assert!(requirements.hover);
    assert!(requirements.press);
    assert!(requirements.long_press);
    assert!(requirements.movement);
    assert!(requirements.keyboard_modality);
    assert!(requirements.focus_within);
}

#[test]
fn focus_visible_within_requires_focus_boundaries_and_keyboard_modality() {
    let requirements = PortableStyle::from_web(
        &WebProps::new().class_name("data-[focus-visible-within=true]:opacity-100"),
    )
    .interaction_requirements();

    assert!(requirements.keyboard_modality);
    assert!(requirements.focus_within);
}
