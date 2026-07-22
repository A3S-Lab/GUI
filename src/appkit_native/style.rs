use super::*;

pub(super) fn config_rect(
    config: &NativeWidgetConfig,
    default_width: f64,
    default_height: f64,
) -> NSRect {
    NSRect::new(
        NSPoint::new(0.0, 0.0),
        config_size(config, default_width, default_height),
    )
}

pub(super) fn config_rect_for_orientation(
    config: &NativeWidgetConfig,
    orientation: Orientation,
    horizontal_width: f64,
    horizontal_height: f64,
    vertical_width: f64,
    vertical_height: f64,
) -> NSRect {
    let (width, height) = match orientation {
        Orientation::Horizontal => (horizontal_width, horizontal_height),
        Orientation::Vertical => (vertical_width, vertical_height),
    };
    config_rect(config, width, height)
}

pub(super) fn separator_size(orientation: Orientation) -> NSSize {
    match orientation {
        Orientation::Horizontal => NSSize::new(160.0, 1.0),
        Orientation::Vertical => NSSize::new(1.0, 160.0),
    }
}

pub(super) fn slider_size_for_orientation(
    config: &NativeWidgetConfig,
    orientation: Orientation,
) -> NSSize {
    config_rect_for_orientation(config, orientation, 180.0, 24.0, 24.0, 180.0).size
}

pub(super) fn apply_slider_orientation(slider: &NSSlider, orientation: Orientation) {
    slider.setVertical(matches!(orientation, Orientation::Vertical));
}

pub(super) fn config_size(
    config: &NativeWidgetConfig,
    default_width: f64,
    default_height: f64,
) -> NSSize {
    let size = config.portable_style.native_size_constraints();
    let width = size.width.unwrap_or(default_width);
    let height = size.height.unwrap_or(default_height);
    NSSize::new(width, height)
}

pub(super) fn size_constraint(
    view: &NSView,
    attribute: NSLayoutAttribute,
    relation: NSLayoutRelation,
    constant: f64,
) -> Retained<NSLayoutConstraint> {
    unsafe {
        NSLayoutConstraint::constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
            view.as_super().as_super().as_super(),
            attribute,
            relation,
            None,
            NSLayoutAttribute::NotAnAttribute,
            1.0,
            constant,
        )
    }
}

pub(super) fn config_text_input_size(config: &NativeWidgetConfig) -> NSSize {
    let sizing = AppKitTextInputSizing::from_config(config);
    let width = sizing
        .explicit_width
        .or_else(|| sizing.hinted_width())
        .unwrap_or(APPKIT_TEXT_INPUT_DEFAULT_WIDTH);
    let height = sizing
        .explicit_height
        .or_else(|| sizing.hinted_height())
        .unwrap_or(APPKIT_TEXT_INPUT_DEFAULT_HEIGHT);
    NSSize::new(width, height)
}

pub(super) fn apply_text_field_hints(text_field: &NSTextField, hints: AppKitTextInputHints) {
    if let Some(enabled) = hints.automatic_text_completion_enabled {
        text_field.setAutomaticTextCompletionEnabled(enabled);
    }
    text_field.setAllowsCharacterPickerTouchBarItem(hints.character_picker_enabled);
    set_spell_checking_trait(text_field, hints.spell_checking);
    set_autocorrection_trait(text_field, hints.autocorrection);
    set_text_replacement_trait(text_field, hints.text_replacement);
    set_text_completion_trait(text_field, hints.text_completion);
    set_inline_prediction_trait(text_field, hints.inline_prediction);
}

pub(super) fn set_spell_checking_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setSpellCheckingType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setSpellCheckingType: value];
        }
    }
}

pub(super) fn set_autocorrection_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setAutocorrectionType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setAutocorrectionType: value];
        }
    }
}

pub(super) fn set_text_replacement_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setTextReplacementType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setTextReplacementType: value];
        }
    }
}

pub(super) fn set_text_completion_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setTextCompletionType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setTextCompletionType: value];
        }
    }
}

pub(super) fn set_inline_prediction_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setInlinePredictionType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setInlinePredictionType: value];
        }
    }
}

pub(super) fn appkit_text_input_trait_value(value: AppKitTextInputTrait) -> NSInteger {
    match value {
        AppKitTextInputTrait::Default => 0,
        AppKitTextInputTrait::No => 1,
        AppKitTextInputTrait::Yes => 2,
    }
}

pub(super) fn apply_widget_portable_style(
    widget: &AppKitOsWidget,
    kind: AppKitWidgetKind,
    style: &PortableStyle,
) {
    match widget {
        AppKitOsWidget::Window(window) => {
            if let Some(color) = style
                .background_color
                .as_ref()
                .and_then(ns_color_from_style_color)
            {
                window.setBackgroundColor(Some(&color));
            }
            if let Some(content_view) = window.contentView() {
                apply_view_portable_style(&content_view, style);
            }
        }
        AppKitOsWidget::Panel(panel) => {
            if let Some(color) = style
                .background_color
                .as_ref()
                .and_then(ns_color_from_style_color)
            {
                panel.as_super().setBackgroundColor(Some(&color));
            }
            if let Some(content_view) = panel.as_super().contentView() {
                apply_view_portable_style(&content_view, style);
            }
        }
        AppKitOsWidget::Popover(state) => {
            apply_view_portable_style(&state.content_view, style);
        }
        AppKitOsWidget::StackView(stack_view) => {
            apply_stack_view_portable_visuals(stack_view, style);
        }
        AppKitOsWidget::ScrollView(state) => {
            apply_scroll_view_portable_visuals(state, style);
        }
        AppKitOsWidget::Button(button) => {
            apply_button_portable_style(button, style);
        }
        AppKitOsWidget::TextField(text_field) => {
            apply_text_field_portable_style(text_field, kind, style);
        }
        AppKitOsWidget::SearchField(text_field) => {
            apply_text_field_portable_style(text_field.as_super(), kind, style);
        }
        AppKitOsWidget::SecureTextField(text_field) => {
            apply_text_field_portable_style(text_field.as_super(), kind, style);
        }
        _ => {
            if let Some(view) = widget.as_view() {
                apply_view_portable_style(view, style);
            }
            if let Some(control) = widget.as_control() {
                apply_control_typography(control, style);
            }
        }
    }
}

pub(super) fn apply_stack_view_portable_visuals(stack_view: &NSStackView, style: &PortableStyle) {
    apply_view_portable_style(stack_view.as_super(), style);
    if let Some(edge_insets) = ns_edge_insets_from_style(&style.padding) {
        stack_view.setEdgeInsets(edge_insets);
    }
}

pub(super) fn apply_scroll_view_portable_visuals(
    state: &AppKitScrollViewState,
    style: &PortableStyle,
) {
    apply_view_portable_style(state.scroll_view.as_super(), style);
    apply_view_portable_style(state.stack_view.as_super(), style);
    if let Some(color) = style
        .background_color
        .as_ref()
        .and_then(ns_color_from_style_color)
    {
        state.scroll_view.setDrawsBackground(true);
        state.scroll_view.setBackgroundColor(&color);
        let clip_view = state.scroll_view.contentView();
        clip_view.setDrawsBackground(true);
        clip_view.setBackgroundColor(&color);
    }
    if let Some(edge_insets) = ns_edge_insets_from_style(&style.padding) {
        state.scroll_view.setContentInsets(edge_insets);
        state.stack_view.setEdgeInsets(edge_insets);
    }
}

pub(super) fn apply_list_view_portable_visuals(
    scroll_view: &NSScrollView,
    state: &AppKitListViewState,
    style: &PortableStyle,
) {
    apply_view_portable_style(scroll_view.as_super(), style);
    apply_view_portable_style(state.stack_view.as_super(), style);
    if let Some(color) = style
        .background_color
        .as_ref()
        .and_then(ns_color_from_style_color)
    {
        scroll_view.setDrawsBackground(true);
        scroll_view.setBackgroundColor(&color);
        let clip_view = scroll_view.contentView();
        clip_view.setDrawsBackground(true);
        clip_view.setBackgroundColor(&color);
    }
    scroll_view.setContentInsets(zero_edge_insets());
    state
        .stack_view
        .setEdgeInsets(ns_edge_insets_from_style(&style.padding).unwrap_or_else(zero_edge_insets));
}

pub(super) fn zero_edge_insets() -> NSEdgeInsets {
    NSEdgeInsets {
        top: 0.0,
        left: 0.0,
        bottom: 0.0,
        right: 0.0,
    }
}

pub(super) fn apply_text_field_portable_style(
    text_field: &NSTextField,
    kind: AppKitWidgetKind,
    style: &PortableStyle,
) {
    apply_control_typography(text_field.as_super(), style);
    if let Some(alignment) = style.text_align.map(appkit_text_alignment) {
        unsafe {
            let _: () = msg_send![text_field, setAlignment: alignment];
        }
    }
    if let Some(color) = style.color.as_ref().and_then(ns_color_from_style_color) {
        text_field.setTextColor(Some(&color));
    }
    if let Some(background) = style
        .background_color
        .as_ref()
        .and_then(ns_color_from_style_color)
    {
        text_field.setDrawsBackground(true);
        text_field.setBackgroundColor(Some(&background));
    }
    if kind == AppKitWidgetKind::Label {
        text_field.setBordered(false);
    }
    apply_view_portable_style(text_field.as_super().as_super(), style);
}

pub(super) fn appkit_text_alignment(alignment: TextAlign) -> NSInteger {
    match alignment {
        TextAlign::Start => 0,
        TextAlign::End => 1,
        TextAlign::Center => 2,
        TextAlign::Justify => 3,
    }
}

pub(super) fn apply_button_portable_style(button: &NSButton, style: &PortableStyle) {
    apply_control_typography(button.as_super(), style);
    if let Some(color) = style.color.as_ref().and_then(ns_color_from_style_color) {
        button.setContentTintColor(Some(&color));
    }
    if style_has_custom_chrome(style) {
        button.setBordered(false);
    }
    button.as_super().sizeToFit();
    apply_control_padding(button.as_super(), style);
    apply_view_portable_style(button.as_super().as_super(), style);
}

pub(super) fn apply_control_typography(control: &NSControl, style: &PortableStyle) {
    if style.font_size.is_none() && style.font_weight.is_none() && style.font_family.is_none() {
        return;
    }
    let font_size = style_font_size_points(style).unwrap_or_else(|| {
        control
            .font()
            .as_ref()
            .map(|font| font.pointSize())
            .unwrap_or(13.0)
    });
    let font = ns_font_from_style(style, font_size);
    control.setFont(Some(&font));
}

pub(super) fn apply_control_padding(control: &NSControl, style: &PortableStyle) {
    let Some(edge_insets) = ns_edge_insets_from_style(&style.padding) else {
        return;
    };
    let view = control.as_super();
    let current = view.frame().size;
    let width = (current.width + edge_insets.left + edge_insets.right).max(current.width);
    let height = (current.height + edge_insets.top + edge_insets.bottom).max(current.height);
    view.setFrameSize(NSSize::new(width, height));
}

pub(super) fn apply_view_portable_style(view: &NSView, style: &PortableStyle) {
    if !style_has_custom_chrome(style) && style.opacity.is_none() {
        return;
    }
    view.setWantsLayer(true);
    let Some(layer) = view.layer() else {
        return;
    };
    if let Some(color) = style
        .background_color
        .as_ref()
        .and_then(ns_color_from_style_color)
    {
        let cg_color = color.CGColor();
        layer.setBackgroundColor(Some(&cg_color));
    }
    if let Some(opacity) = style.opacity {
        layer.setOpacity(opacity.clamp(0.0, 1.0) as f32);
    }
    if let Some(radius) = border_radius_points(style) {
        layer.setCornerRadius(radius);
        layer.setMasksToBounds(radius > 0.0);
    }
    if let Some(width) = border_width_points(style) {
        layer.setBorderWidth(width);
        let color = first_border_color(style)
            .and_then(ns_color_from_style_color)
            .unwrap_or_else(neutral_border_color);
        let cg_color = color.CGColor();
        layer.setBorderColor(Some(&cg_color));
    }
    view.setNeedsDisplay(true);
}

pub(super) fn style_has_custom_chrome(style: &PortableStyle) -> bool {
    style.background_color.is_some()
        || style.border_radius.is_some()
        || style.border_color.is_some()
        || border_width_points(style).is_some()
}

pub(super) fn ns_edge_insets_from_style(edge_insets: &EdgeInsets) -> Option<NSEdgeInsets> {
    let top = edge_length_points(edge_insets.top.as_ref());
    let right = edge_length_points(edge_insets.right.as_ref());
    let bottom = edge_length_points(edge_insets.bottom.as_ref());
    let left = edge_length_points(edge_insets.left.as_ref());
    if top.is_none() && right.is_none() && bottom.is_none() && left.is_none() {
        return None;
    }
    Some(NSEdgeInsets {
        top: top.unwrap_or(0.0),
        left: left.unwrap_or(0.0),
        bottom: bottom.unwrap_or(0.0),
        right: right.unwrap_or(0.0),
    })
}

pub(super) fn edge_length_points(length: Option<&StyleLength>) -> Option<f64> {
    length
        .and_then(StyleLength::points)
        .filter(|value| value.is_finite() && *value >= 0.0)
}

pub(super) fn border_width_points(style: &PortableStyle) -> Option<f64> {
    let mut width = 0.0_f64;
    let mut found = false;
    for length in [
        style.border_width.top.as_ref(),
        style.border_width.right.as_ref(),
        style.border_width.bottom.as_ref(),
        style.border_width.left.as_ref(),
        style.logical_border_width.block_start.as_ref(),
        style.logical_border_width.block_end.as_ref(),
        style.logical_border_width.inline_start.as_ref(),
        style.logical_border_width.inline_end.as_ref(),
    ] {
        if let Some(points) = edge_length_points(length) {
            found = true;
            width = width.max(points);
        }
    }
    found.then_some(width)
}

pub(super) fn border_radius_points(style: &PortableStyle) -> Option<f64> {
    style
        .border_radius
        .as_ref()
        .and_then(StyleLength::points)
        .filter(|value| value.is_finite() && *value >= 0.0)
}

pub(super) fn style_font_size_points(style: &PortableStyle) -> Option<f64> {
    style
        .font_size
        .as_ref()
        .and_then(StyleLength::points)
        .filter(|value| value.is_finite() && *value > 0.0)
}

pub(super) fn ns_font_from_style(style: &PortableStyle, font_size: f64) -> Retained<NSFont> {
    let mono = style
        .font_family
        .as_deref()
        .is_some_and(|family| family.to_ascii_lowercase().contains("mono"));
    if mono {
        if let Some(font) = NSFont::userFixedPitchFontOfSize(font_size) {
            return font;
        }
    }
    if font_weight_is_bold(style.font_weight.as_ref()) {
        NSFont::boldSystemFontOfSize(font_size)
    } else {
        NSFont::systemFontOfSize(font_size)
    }
}

pub(super) fn font_weight_is_bold(weight: Option<&FontWeight>) -> bool {
    match weight {
        Some(FontWeight::Number(value)) => *value >= 600,
        Some(FontWeight::Keyword(value)) => {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "bold" | "bolder"
            )
        }
        None => false,
    }
}

pub(super) fn first_border_color(style: &PortableStyle) -> Option<&StyleColor> {
    style
        .border_color
        .as_ref()
        .or(style.border_colors.top.as_ref())
        .or(style.border_colors.right.as_ref())
        .or(style.border_colors.bottom.as_ref())
        .or(style.border_colors.left.as_ref())
        .or(style.logical_border_colors.block_start.as_ref())
        .or(style.logical_border_colors.block_end.as_ref())
        .or(style.logical_border_colors.inline_start.as_ref())
        .or(style.logical_border_colors.inline_end.as_ref())
}

pub(super) fn ns_color_from_style_color(color: &StyleColor) -> Option<Retained<NSColor>> {
    match color {
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        } => Some(ns_color(*red, *green, *blue, *alpha)),
        StyleColor::Keyword(keyword) => match keyword.trim().to_ascii_lowercase().as_str() {
            "black" => Some(ns_color(0, 0, 0, 255)),
            "white" => Some(ns_color(255, 255, 255, 255)),
            "transparent" => Some(ns_color(0, 0, 0, 0)),
            _ => None,
        },
        StyleColor::Function(_) => None,
    }
}

pub(super) fn neutral_border_color() -> Retained<NSColor> {
    ns_color(229, 229, 229, 255)
}

pub(super) fn ns_color(red: u8, green: u8, blue: u8, alpha: u8) -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(
        f64::from(red) / 255.0,
        f64::from(green) / 255.0,
        f64::from(blue) / 255.0,
        f64::from(alpha) / 255.0,
    )
}

pub(super) fn apply_window_portable_style(window: &NSWindow, style: &crate::style::PortableStyle) {
    let size = style.native_size_constraints();
    if size.width.is_some() || size.height.is_some() {
        let current = window
            .contentView()
            .map(|view| view.frame().size)
            .unwrap_or_else(|| window.contentLayoutRect().size);
        window.setContentSize(NSSize::new(
            size.width.unwrap_or(current.width),
            size.height.unwrap_or(current.height),
        ));
    }

    if size.min_width.is_some() || size.min_height.is_some() {
        let current = window.minSize();
        window.setMinSize(NSSize::new(
            size.min_width.unwrap_or(current.width),
            size.min_height.unwrap_or(current.height),
        ));
    }

    if size.max_width.is_some() || size.max_height.is_some() {
        let current = window.maxSize();
        window.setMaxSize(NSSize::new(
            size.max_width.unwrap_or(current.width),
            size.max_height.unwrap_or(current.height),
        ));
    }
}

pub(super) fn activate_current_application() {
    let options = {
        let mut options = NSApplicationActivationOptions::ActivateAllWindows;
        #[allow(deprecated)]
        {
            options.insert(NSApplicationActivationOptions::ActivateIgnoringOtherApps);
        }
        options
    };
    let _ = NSRunningApplication::currentApplication().activateWithOptions(options);
}

pub(super) fn apply_scroll_view_layout(state: &AppKitScrollViewState, style: &PortableStyle) {
    state
        .scroll_view
        .setHasVerticalScroller(appkit_vertical_scroll_enabled_for_style(style));
    state
        .scroll_view
        .setHasHorizontalScroller(appkit_horizontal_scroll_enabled_for_style(style));
    apply_stack_view_layout(&state.stack_view, style, None);

    let size = style.native_size_constraints();
    if size.width.is_some() || size.height.is_some() {
        let current = state.stack_view.frame().size;
        state.stack_view.setFrameSize(NSSize::new(
            size.width.unwrap_or(current.width.max(120.0)),
            size.height.unwrap_or(current.height.max(32.0)),
        ));
    }
    scroll_view_to_top(state);
}

pub(super) fn apply_list_view_layout(
    scroll_view: &NSScrollView,
    state: &AppKitListViewState,
    style: &PortableStyle,
) {
    scroll_view.setHasVerticalScroller(appkit_vertical_scroll_enabled_for_style(style));
    scroll_view.setHasHorizontalScroller(false);
    scroll_view.setAutohidesScrollers(true);
    apply_stack_view_layout(&state.stack_view, style, Some(Orientation::Vertical));
    apply_list_view_portable_visuals(scroll_view, state, style);
    resize_list_view_document(scroll_view, state, style);
}

pub(super) fn resize_list_view_document(
    scroll_view: &NSScrollView,
    state: &AppKitListViewState,
    style: &PortableStyle,
) {
    let constraints = style.native_size_constraints();
    let current = scroll_view.as_super().frame().size;
    let width = constraints
        .width
        .or(constraints.min_width)
        .unwrap_or_else(|| current.width.max(APPKIT_LIST_DEFAULT_WIDTH));
    let implicit_height = implicit_list_view_height(state, style);
    let height = constraints.height.unwrap_or_else(|| {
        constraints
            .min_height
            .map(|min_height| implicit_height.max(min_height))
            .unwrap_or(implicit_height)
    });
    let height = constraints
        .max_height
        .map(|max_height| height.min(max_height))
        .unwrap_or(height)
        .max(APPKIT_LIST_MIN_HEIGHT);
    let document_height = implicit_height.max(height);
    let document_width = width.max(APPKIT_LIST_DEFAULT_WIDTH + horizontal_padding_points(style));
    scroll_view
        .as_super()
        .setFrameSize(NSSize::new(width, height));
    state
        .stack_view
        .setFrameSize(NSSize::new(document_width, document_height));
    scroll_view_to_top_view(scroll_view, state.stack_view.as_super());
}

pub(super) fn implicit_list_view_height(state: &AppKitListViewState, style: &PortableStyle) -> f64 {
    let arranged_count = state.stack_view.arrangedSubviews().count();
    let rows = state.rows.borrow();
    let row_count = rows.len().max(arranged_count);
    let rows_height = if rows.is_empty() {
        APPKIT_LIST_MIN_HEIGHT
    } else {
        rows.iter()
            .map(|row| {
                row.button_view()
                    .frame()
                    .size
                    .height
                    .max(APPKIT_LIST_ROW_HEIGHT)
            })
            .sum::<f64>()
    };
    let rows_height = if row_count > rows.len() {
        rows_height + ((row_count - rows.len()) as f64 * APPKIT_LIST_ROW_HEIGHT)
    } else {
        rows_height
    };
    (rows_height + vertical_padding_points(style)).max(APPKIT_LIST_MIN_HEIGHT)
}

pub(super) fn vertical_padding_points(style: &PortableStyle) -> f64 {
    edge_length_points(style.padding.top.as_ref()).unwrap_or(0.0)
        + edge_length_points(style.padding.bottom.as_ref()).unwrap_or(0.0)
}

pub(super) fn horizontal_padding_points(style: &PortableStyle) -> f64 {
    edge_length_points(style.padding.left.as_ref()).unwrap_or(0.0)
        + edge_length_points(style.padding.right.as_ref()).unwrap_or(0.0)
}

pub(super) fn row_width_from_style(style: &PortableStyle) -> f64 {
    let constraints = style.native_size_constraints();
    constraints
        .width
        .or(constraints.min_width)
        .unwrap_or(APPKIT_LIST_DEFAULT_WIDTH)
        .max(80.0)
}

pub(super) fn row_height_from_style(style: &PortableStyle) -> f64 {
    let constraints = style.native_size_constraints();
    let padded_height = 17.0 + vertical_padding_points(style);
    constraints
        .height
        .or(constraints.min_height)
        .unwrap_or(padded_height.max(APPKIT_LIST_ROW_HEIGHT))
        .max(APPKIT_LIST_MIN_HEIGHT)
}

pub(super) fn apply_stack_view_layout(
    stack_view: &NSStackView,
    style: &PortableStyle,
    orientation: Option<Orientation>,
) {
    let orientation = orientation.or(style.flex_direction);
    if let Some(orientation) = orientation {
        stack_view.setOrientation(appkit_stack_orientation(orientation));
    }
    stack_view.setDistribution(NSStackViewDistribution::Fill);
    stack_view.setAlignment(appkit_stack_alignment(
        orientation.unwrap_or_else(|| orientation_from_appkit_stack(stack_view)),
    ));
    let gap = style
        .gap
        .as_ref()
        .and_then(StyleLength::points)
        .unwrap_or(0.0);
    unsafe {
        let _: () = msg_send![stack_view, setSpacing: gap];
    }
}

pub(super) fn table_stack_orientation(role: crate::native::NativeRole) -> Option<Orientation> {
    match role {
        crate::native::NativeRole::TableSection => Some(Orientation::Vertical),
        crate::native::NativeRole::TableRow
        | crate::native::NativeRole::TableCell
        | crate::native::NativeRole::TableColumn => Some(Orientation::Horizontal),
        _ => None,
    }
}

pub(super) fn table_stack_minimum_size(role: crate::native::NativeRole) -> Option<(f64, f64)> {
    match role {
        crate::native::NativeRole::TableSection => Some((240.0, 32.0)),
        crate::native::NativeRole::TableRow => Some((240.0, 36.0)),
        crate::native::NativeRole::TableCell | crate::native::NativeRole::TableColumn => {
            Some((96.0, 36.0))
        }
        _ => None,
    }
}

pub(super) fn stack_arranged_insert_index(
    stack_view: &NSStackView,
    requested: usize,
) -> GuiResult<NSInteger> {
    let existing = stack_view.arrangedSubviews().count();
    let index = stack_insert_index(existing, requested);
    index
        .try_into()
        .map_err(|_| GuiError::host("AppKit stack view child insertion index overflow"))
}

pub(super) fn stack_insert_index(existing: usize, requested: usize) -> usize {
    requested.min(existing)
}

pub(super) fn orientation_from_appkit_stack(stack_view: &NSStackView) -> Orientation {
    if stack_view.orientation() == NSUserInterfaceLayoutOrientation::Vertical {
        Orientation::Vertical
    } else {
        Orientation::Horizontal
    }
}

pub(super) fn appkit_stack_alignment(orientation: Orientation) -> NSLayoutAttribute {
    match orientation {
        Orientation::Horizontal => NSLayoutAttribute::Top,
        Orientation::Vertical => NSLayoutAttribute::Leading,
    }
}

pub(super) fn appkit_vertical_scroll_enabled_for_style(style: &PortableStyle) -> bool {
    scroll_enabled(style.overflow_y)
        || scroll_enabled(style.overflow_block)
        || (!scroll_enabled(style.overflow_x) && !scroll_enabled(style.overflow_inline))
}

pub(super) fn appkit_horizontal_scroll_enabled_for_style(style: &PortableStyle) -> bool {
    scroll_enabled(style.overflow_x) || scroll_enabled(style.overflow_inline)
}

pub(super) fn scroll_enabled(value: Option<OverflowMode>) -> bool {
    matches!(value, Some(OverflowMode::Auto | OverflowMode::Scroll))
}

pub(super) fn appkit_stack_orientation(
    orientation: Orientation,
) -> NSUserInterfaceLayoutOrientation {
    match orientation {
        Orientation::Horizontal => NSUserInterfaceLayoutOrientation::Horizontal,
        Orientation::Vertical => NSUserInterfaceLayoutOrientation::Vertical,
    }
}

pub(super) fn configure_scroll_document(scroll_view: &NSScrollView, stack_view: &NSStackView) {
    stack_view
        .as_super()
        .setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
    scroll_view.setDocumentView(Some(stack_view.as_super()));
}

pub(super) fn scroll_view_to_top(state: &AppKitScrollViewState) {
    scroll_view_to_top_view(&state.scroll_view, state.stack_view.as_super());
}

pub(super) fn scroll_view_to_top_view(scroll_view: &NSScrollView, document_view: &NSView) {
    let clip_view = scroll_view.contentView();
    let top_y = if document_view.isFlipped() {
        0.0
    } else {
        let clip_height = clip_view.bounds().size.height.max(0.0);
        let document_height = document_view.frame().size.height.max(0.0);
        (document_height - clip_height).max(0.0)
    };
    clip_view.scrollToPoint(NSPoint::new(0.0, top_y));
    scroll_view.reflectScrolledClipView(&clip_view);
}
