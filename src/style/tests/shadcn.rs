use super::support::*;

fn variant<'a>(style: &'a PortableStyle, key: &str, property: &str) -> Option<&'a str> {
    style
        .variant_declarations
        .get(key)
        .and_then(|styles| styles.get(property))
        .map(String::as_str)
}

#[test]
fn parses_design_button_component_class_contract() {
    let web = WebProps::new().class_name(
        "inline-flex h-10 items-center justify-center gap-2 whitespace-nowrap rounded-md \
         border border-primary bg-primary px-[18px] py-2 text-sm font-medium leading-none \
         text-on-primary transition-colors disabled:pointer-events-none \
         disabled:bg-surface-strong disabled:text-muted-soft [&_svg]:pointer-events-none \
         [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none \
         active:bg-primary-active focus-visible:ring-[3px] focus-visible:ring-ring/50 \
         aria-invalid:border-semantic-error has-[>svg]:px-4",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.display, Some(DisplayMode::InlineFlex));
    assert_eq!(style.align_items, Some(AlignItems::Center));
    assert_eq!(style.justify_content, Some(JustifyContent::Center));
    assert_eq!(style.gap, Some(StyleLength::Points(8.0)));
    assert_eq!(style.white_space, Some(WhiteSpaceMode::NoWrap));
    assert_eq!(style.border_radius, Some(StyleLength::Points(8.0)));
    assert_eq!(style.font_weight, Some(FontWeight::Number(500)));
    assert_eq!(style.flex_shrink.as_deref(), Some("0"));
    assert_eq!(style.height, Some(StyleLength::Points(40.0)));
    assert_eq!(style.padding.left, Some(StyleLength::Points(18.0)));
    assert_eq!(style.padding.right, Some(StyleLength::Points(18.0)));
    assert_eq!(style.padding.top, Some(StyleLength::Points(8.0)));
    assert_eq!(style.padding.bottom, Some(StyleLength::Points(8.0)));
    assert_eq!(
        style.background_color,
        Some(StyleColor::Rgba {
            red: 0x00,
            green: 0x00,
            blue: 0x00,
            alpha: 255,
        })
    );
    assert_eq!(
        style.color,
        Some(StyleColor::Rgba {
            red: 0xff,
            green: 0xff,
            blue: 0xff,
            alpha: 255,
        })
    );
    assert_eq!(
        style.transition_property.as_deref(),
        Some("color, background-color, border-color, outline-color, text-decoration-color, fill, stroke")
    );
    assert_eq!(
        style.transition_duration,
        Some(StyleTime::Milliseconds(150.0))
    );
    assert_eq!(
        style.declarations.get("outline").map(String::as_str),
        Some("2px solid transparent")
    );
    assert!(style.custom_properties.get("--tw-shadow").is_none());

    assert_eq!(variant(&style, "disabled", "pointer-events"), Some("none"));
    assert_eq!(
        variant(&style, "disabled", "background-color"),
        Some("rgb(240, 240, 243)")
    );
    assert_eq!(
        variant(&style, "disabled", "color"),
        Some("rgb(204, 204, 204)")
    );
    assert_eq!(variant(&style, "[& svg]", "pointer-events"), Some("none"));
    assert_eq!(variant(&style, "[& svg]", "flex-shrink"), Some("0"));
    assert_eq!(
        variant(&style, "[& svg:not([class*='size-'])]", "width"),
        Some("16px")
    );
    assert_eq!(
        variant(&style, "[& svg:not([class*='size-'])]", "height"),
        Some("16px")
    );
    assert_eq!(
        variant(&style, "focus-visible", "--tw-ring-color"),
        Some("rgba(23, 23, 23, 0.5)")
    );
    assert_eq!(
        variant(&style, "focus-visible", "--tw-ring-shadow"),
        Some("0 0 0 3px")
    );
    assert_eq!(
        variant(&style, "aria-invalid", "border-color"),
        Some("rgb(235, 142, 144)")
    );
    assert_eq!(
        variant(&style, "active", "background-color"),
        Some("rgb(26, 26, 26)")
    );
    assert_eq!(
        variant(&style, "has-[>svg]", "padding-inline"),
        Some("16px")
    );
}

#[test]
fn parses_design_input_component_class_contract() {
    let web = WebProps::new().class_name(
        "h-11 w-full min-w-0 rounded-md border border-hairline-strong bg-canvas px-4 py-3 \
         text-sm text-ink transition-colors outline-none selection:bg-primary \
         selection:text-on-primary file:inline-flex file:h-7 file:border-0 \
         file:bg-transparent file:text-sm file:font-medium file:text-ink \
         placeholder:text-mute disabled:pointer-events-none disabled:cursor-not-allowed \
         disabled:bg-surface-strong disabled:text-muted-soft md:text-sm focus-visible:border-ink \
         focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-semantic-error",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.height, Some(StyleLength::Points(44.0)));
    assert_eq!(style.width, Some(StyleLength::Percent(100.0)));
    assert_eq!(style.min_width, Some(StyleLength::Points(0.0)));
    assert_eq!(style.border_radius, Some(StyleLength::Points(8.0)));
    assert_eq!(style.border_width.top, Some(StyleLength::Points(1.0)));
    assert_eq!(
        style.border_color,
        Some(StyleColor::Rgba {
            red: 0xdc,
            green: 0xde,
            blue: 0xe0,
            alpha: 255,
        })
    );
    assert_eq!(
        style.background_color,
        Some(StyleColor::Rgba {
            red: 0xff,
            green: 0xff,
            blue: 0xff,
            alpha: 255,
        })
    );
    assert_eq!(style.padding.left, Some(StyleLength::Points(16.0)));
    assert_eq!(style.padding.top, Some(StyleLength::Points(12.0)));
    assert_eq!(style.font_size, Some(StyleLength::Points(14.0)));
    assert_eq!(
        style.transition_property.as_deref(),
        Some("color, background-color, border-color, outline-color, text-decoration-color, fill, stroke")
    );
    assert_eq!(
        variant(&style, "selection", "background-color"),
        Some("rgb(0, 0, 0)")
    );
    assert_eq!(
        variant(&style, "selection", "color"),
        Some("rgb(255, 255, 255)")
    );
    assert_eq!(variant(&style, "file", "display"), Some("inline-flex"));
    assert_eq!(variant(&style, "file", "height"), Some("28px"));
    assert_eq!(variant(&style, "file", "border-width"), Some("0px"));
    assert_eq!(
        variant(&style, "placeholder", "color"),
        Some("rgb(153, 153, 153)")
    );
    assert_eq!(variant(&style, "disabled", "cursor"), Some("not-allowed"));
    assert_eq!(
        variant(&style, "disabled", "background-color"),
        Some("rgb(240, 240, 243)")
    );
    assert_eq!(
        variant(&style, "disabled", "color"),
        Some("rgb(204, 204, 204)")
    );
    assert_eq!(variant(&style, "md", "font-size"), Some("0.875rem"));
    assert_eq!(
        variant(&style, "focus-visible", "--tw-ring-color"),
        Some("rgba(23, 23, 23, 0.5)")
    );
}

#[test]
fn parses_design_card_component_class_contract() {
    let card = PortableStyle::from_web(&WebProps::new().class_name(
        "flex flex-col gap-4 rounded-lg border border-hairline-strong bg-canvas p-6 text-ink",
    ));

    assert_eq!(card.display, Some(DisplayMode::Flex));
    assert_eq!(
        card.flex_direction,
        Some(crate::geometry::Orientation::Vertical)
    );
    assert_eq!(card.gap, Some(StyleLength::Points(16.0)));
    assert_eq!(card.border_radius, Some(StyleLength::Points(12.0)));
    assert_eq!(card.border_width.top, Some(StyleLength::Points(1.0)));
    assert_eq!(
        card.border_color,
        Some(StyleColor::Rgba {
            red: 0xdc,
            green: 0xde,
            blue: 0xe0,
            alpha: 255,
        })
    );
    assert_eq!(
        card.background_color,
        Some(StyleColor::Rgba {
            red: 0xff,
            green: 0xff,
            blue: 0xff,
            alpha: 255,
        })
    );
    assert_eq!(card.padding.top, Some(StyleLength::Points(24.0)));
    assert_eq!(card.padding.bottom, Some(StyleLength::Points(24.0)));
    assert_eq!(card.font_weight, None);
    assert!(card.box_shadow.is_none());

    let header = PortableStyle::from_web(&WebProps::new().class_name(
        "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 \
         has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-4",
    ));

    assert_eq!(header.container_type, Some(ContainerType::InlineSize));
    assert_eq!(header.container_name.as_deref(), Some("card-header"));
    assert_eq!(header.display, Some(DisplayMode::Grid));
    assert_eq!(header.grid_auto_rows.as_deref(), Some("min-content"));
    assert_eq!(header.grid_template_rows.as_deref(), Some("auto auto"));
    assert_eq!(header.align_items, Some(AlignItems::Start));
    assert_eq!(
        variant(
            &header,
            "has-data-[slot=card-action]",
            "grid-template-columns"
        ),
        Some("1fr auto")
    );
    assert_eq!(
        variant(&header, "[.border-b]", "padding-bottom"),
        Some("16px")
    );

    let action = PortableStyle::from_web(
        &WebProps::new()
            .class_name("col-start-2 row-span-2 row-start-1 self-start justify-self-end"),
    );
    assert_eq!(action.grid_column_start.as_deref(), Some("2"));
    assert_eq!(action.grid_row.as_deref(), Some("span 2 / span 2"));
    assert_eq!(action.grid_row_start.as_deref(), Some("1"));
    assert_eq!(action.align_self, Some(SelfAlignment::Start));
    assert_eq!(action.justify_self, Some(SelfAlignment::End));
}

#[test]
fn parses_design_dialog_component_class_contract() {
    let overlay = PortableStyle::from_web(&WebProps::new().class_name(
        "fixed inset-0 z-50 bg-black/50 data-[state=closed]:animate-out \
         data-[state=closed]:fade-out-0 data-[state=open]:animate-in \
         data-[state=open]:fade-in-0",
    ));

    assert_eq!(overlay.position, Some(PositionMode::Fixed));
    assert_eq!(overlay.inset.top, Some(StyleLength::Points(0.0)));
    assert_eq!(overlay.z_index, Some(50));
    assert_eq!(
        overlay.background_color,
        Some(StyleColor::Rgba {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 128,
        })
    );
    assert_eq!(
        variant(&overlay, "data-[state=closed]", "animation-name"),
        Some("exit")
    );
    assert_eq!(
        variant(&overlay, "data-[state=closed]", "--tw-exit-opacity"),
        Some("0")
    );
    assert_eq!(
        variant(&overlay, "data-[state=open]", "animation-name"),
        Some("enter")
    );
    assert_eq!(
        variant(&overlay, "data-[state=open]", "--tw-enter-opacity"),
        Some("0")
    );

    let content = PortableStyle::from_web(&WebProps::new().class_name(
        "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] \
         translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-canvas \
         p-6 text-ink duration-200 outline-none data-[state=closed]:animate-out \
         data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 \
         data-[state=open]:animate-in data-[state=open]:fade-in-0 \
         data-[state=open]:zoom-in-95 sm:max-w-lg",
    ));

    assert_eq!(content.position, Some(PositionMode::Fixed));
    assert_eq!(content.inset.top, Some(StyleLength::Percent(50.0)));
    assert_eq!(content.inset.left, Some(StyleLength::Percent(50.0)));
    assert_eq!(content.display, Some(DisplayMode::Grid));
    assert_eq!(content.width, Some(StyleLength::Percent(100.0)));
    assert_eq!(
        content.max_width,
        Some(StyleLength::Css("calc(100%-2rem)".to_string()))
    );
    assert_eq!(content.translate.as_deref(), Some("-50% -50%"));
    assert_eq!(content.gap, Some(StyleLength::Points(16.0)));
    assert_eq!(
        content.animation_duration,
        Some(StyleTime::Milliseconds(200.0))
    );
    assert_eq!(
        variant(&content, "data-[state=closed]", "--tw-exit-scale"),
        Some("0.95")
    );
    assert_eq!(
        variant(&content, "data-[state=open]", "--tw-enter-scale"),
        Some("0.95")
    );
    assert_eq!(variant(&content, "sm", "max-width"), Some("32rem"));
}

#[test]
fn parses_design_dropdown_menu_component_class_contract() {
    let content = PortableStyle::from_web(&WebProps::new().class_name(
        "z-50 max-h-(--radix-dropdown-menu-content-available-height) min-w-[8rem] \
         origin-(--radix-dropdown-menu-content-transform-origin) overflow-x-hidden \
         overflow-y-auto rounded-md border bg-canvas p-1 text-ink \
         data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 \
         data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 \
         data-[state=closed]:animate-out data-[state=closed]:fade-out-0 \
         data-[state=closed]:zoom-out-95 data-[state=open]:animate-in \
         data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
    ));

    assert_eq!(content.z_index, Some(50));
    assert_eq!(
        content.max_height,
        Some(StyleLength::Css(
            "var(--radix-dropdown-menu-content-available-height)".to_string()
        ))
    );
    assert_eq!(content.min_width, Some(StyleLength::Points(128.0)));
    assert_eq!(
        content.transform_origin.as_deref(),
        Some("var(--radix-dropdown-menu-content-transform-origin)")
    );
    assert_eq!(content.overflow_x, Some(OverflowMode::Hidden));
    assert_eq!(content.overflow_y, Some(OverflowMode::Auto));
    assert_eq!(content.border_radius, Some(StyleLength::Points(8.0)));
    assert_eq!(
        content.background_color,
        Some(StyleColor::Rgba {
            red: 0xff,
            green: 0xff,
            blue: 0xff,
            alpha: 255,
        })
    );
    assert_eq!(
        variant(&content, "data-[side=bottom]", "--tw-enter-translate-y"),
        Some("-8px")
    );
    assert_eq!(
        variant(&content, "data-[side=left]", "--tw-enter-translate-x"),
        Some("8px")
    );
    assert_eq!(
        variant(&content, "data-[side=right]", "--tw-enter-translate-x"),
        Some("-8px")
    );
    assert_eq!(
        variant(&content, "data-[side=top]", "--tw-enter-translate-y"),
        Some("8px")
    );

    let item = PortableStyle::from_web(&WebProps::new().class_name(
        "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm \
         outline-hidden select-none focus:bg-surface-strong focus:text-ink \
         data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[inset]:pl-8 \
         data-[variant=destructive]:text-semantic-error \
         data-[variant=destructive]:focus:bg-semantic-error/10 \
         data-[variant=destructive]:focus:text-semantic-error \
         dark:data-[variant=destructive]:focus:bg-semantic-error/20 \
         [&_svg]:pointer-events-none [&_svg]:shrink-0 \
         [&_svg:not([class*='size-'])]:size-4 \
         [&_svg:not([class*='text-'])]:text-body",
    ));

    assert_eq!(item.position, Some(PositionMode::Relative));
    assert_eq!(item.cursor.as_deref(), Some("default"));
    assert_eq!(item.user_select, Some(UserSelect::None));
    assert_eq!(
        variant(&item, "focus", "background-color"),
        Some("rgb(240, 240, 243)")
    );
    assert_eq!(
        variant(&item, "data-[disabled]", "pointer-events"),
        Some("none")
    );
    assert_eq!(variant(&item, "data-[inset]", "padding-left"), Some("32px"));
    assert_eq!(
        variant(
            &item,
            "data-[variant=destructive]:focus",
            "background-color"
        ),
        Some("rgba(235, 142, 144, 0.1)")
    );
    assert_eq!(
        variant(
            &item,
            "dark:data-[variant=destructive]:focus",
            "background-color"
        ),
        Some("rgba(235, 142, 144, 0.2)")
    );
    assert_eq!(
        variant(&item, "[& svg:not([class*='text-'])]", "color"),
        Some("rgb(96, 100, 108)")
    );
}
