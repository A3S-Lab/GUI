use super::*;
use std::collections::BTreeMap;

use crate::accessibility::{
    AccessibilityDescriptionProps, AccessibilityRelationshipProps, AccessibilityRole,
    AccessibilityStateProps, AccessibilityStructureProps,
};
use crate::geometry::Orientation;
use crate::host::HostNodeId;
use crate::html::{
    HtmlActivationProps, HtmlCollectionProps, HtmlDialogProps, HtmlFormAssociationProps,
    HtmlMicrodataProps, HtmlResourcePolicyProps, HtmlShadowProps, HtmlTextAnnotationProps,
};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::renderer::Renderer;
use crate::web::WebProps;

#[test]
fn appkit_blueprint_targets_native_button_not_webview() {
    let element = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new().label("Save").web(
            WebProps::new()
                .class_name("primary")
                .style("backgroundColor", "#663399")
                .on_click("saveDocument"),
        ),
    );

    let blueprint = AppKitAdapter.blueprint(&element);

    assert_eq!(blueprint.widget_class, "NSButton");
    assert_eq!(blueprint.accessibility_role, AccessibilityRole::Button);
    assert_eq!(blueprint.action.as_deref(), Some("saveDocument"));
    assert_eq!(blueprint.class_name.as_deref(), Some("primary"));
    assert_eq!(
        blueprint.style.get("backgroundColor").map(String::as_str),
        Some("#663399")
    );
    assert!(blueprint.portable_style.background_color.is_some());
}

#[test]
fn appkit_blueprint_targets_native_listbox_not_webview() {
    let list_box = NativeElement::new("projects", NativeRole::ListBox)
        .child(NativeElement::new("a3s", NativeRole::ListBoxItem));
    let item = &list_box.children[0];

    assert_eq!(
        AppKitAdapter.blueprint(&list_box).widget_class,
        "NSScrollView+NSStackView"
    );
    assert_eq!(
        AppKitAdapter.blueprint(item).widget_class,
        "NSButton(list-row)"
    );
}

#[test]
fn toolbar_blueprint_targets_native_container_controls_not_webview() {
    let element = NativeElement::new("tools", NativeRole::Toolbar)
        .with_props(NativeProps::new().orientation(Orientation::Horizontal));

    assert_eq!(
        AppKitAdapter.blueprint(&element).widget_class,
        "NSStackView(toolbar)"
    );
    assert_eq!(
        WinUiAdapter.blueprint(&element).widget_class,
        "Microsoft.UI.Xaml.Controls.StackPanel(toolbar)"
    );
    assert_eq!(
        Gtk4Adapter.blueprint(&element).widget_class,
        "gtk::Box(toolbar)"
    );
}

#[test]
fn dialog_blueprint_targets_native_dialog_controls_not_webview() {
    let element =
        NativeElement::new("preferences", NativeRole::Dialog).with_props(NativeProps::new());

    assert_eq!(AppKitAdapter.blueprint(&element).widget_class, "NSPanel");
    assert_eq!(
        WinUiAdapter.blueprint(&element).widget_class,
        "Microsoft.UI.Xaml.Controls.ContentDialog"
    );
    assert_eq!(Gtk4Adapter.blueprint(&element).widget_class, "gtk::Dialog");
}

#[test]
fn widget_config_preserves_html_dialog_hints_and_visibility() {
    let native_dialog =
        NativeElement::new("preferences", NativeRole::Dialog).with_props(NativeProps::new());
    let open_html_dialog = NativeElement::new("settings", NativeRole::Dialog)
        .with_props(NativeProps::new().html_dialog(HtmlDialogProps::default().open(true)));
    let closed_html_dialog = NativeElement::new("help", NativeRole::Dialog)
        .with_props(NativeProps::new().html_dialog(HtmlDialogProps::default().open(false)));

    let native_config = Gtk4Adapter.blueprint(&native_dialog).config();
    let open_config = Gtk4Adapter.blueprint(&open_html_dialog).config();
    let closed_config = Gtk4Adapter.blueprint(&closed_html_dialog).config();
    let closed_setters = closed_config.create_setters();

    assert!(native_config.visible);
    assert_eq!(native_config.html_dialog.open, None);
    assert!(open_config.visible);
    assert_eq!(open_config.html_dialog.open, Some(true));
    assert!(!closed_config.visible);
    assert_eq!(closed_config.html_dialog.open, Some(false));
    assert!(closed_setters.contains(&NativeWidgetSetter::SetVisible(false)));
    assert!(closed_setters.contains(&NativeWidgetSetter::SetHtmlDialog(
        HtmlDialogProps::default().open(false)
    )));
}

#[test]
fn popover_blueprint_targets_native_overlay_controls_not_webview() {
    let element =
        NativeElement::new("actions-popover", NativeRole::Popover).with_props(NativeProps::new());

    assert_eq!(AppKitAdapter.blueprint(&element).widget_class, "NSPopover");
    assert_eq!(
        WinUiAdapter.blueprint(&element).widget_class,
        "Microsoft.UI.Xaml.Controls.ToolTip"
    );
    assert_eq!(Gtk4Adapter.blueprint(&element).widget_class, "gtk::Popover");
}

#[test]
fn menu_blueprint_targets_native_menu_controls_not_webview() {
    let menu = NativeElement::new("file-menu", NativeRole::Menu)
        .child(NativeElement::new("open", NativeRole::MenuItem));
    let item = &menu.children[0];

    assert_eq!(AppKitAdapter.blueprint(&menu).widget_class, "NSMenu");
    assert_eq!(AppKitAdapter.blueprint(item).widget_class, "NSMenuItem");
    assert_eq!(
        WinUiAdapter.blueprint(&menu).widget_class,
        "Microsoft.UI.Xaml.Controls.StackPanel(menu)"
    );
    assert_eq!(
        WinUiAdapter.blueprint(item).widget_class,
        "Microsoft.UI.Xaml.Controls.Button(menu-item)"
    );
    assert_eq!(Gtk4Adapter.blueprint(&menu).widget_class, "gio::Menu");
    assert_eq!(Gtk4Adapter.blueprint(item).widget_class, "gio::MenuItem");
}

#[test]
fn same_ir_targets_winui_and_gtk_native_controls() {
    let element = NativeElement::new("email", NativeRole::TextField)
        .with_props(NativeProps::new().label("Email"));

    assert_eq!(
        WinUiAdapter.blueprint(&element).widget_class,
        "Microsoft.UI.Xaml.Controls.TextBox"
    );
    assert_eq!(Gtk4Adapter.blueprint(&element).widget_class, "gtk::Entry");
}

#[test]
fn blueprint_preserves_react_aria_control_state_for_native_adapters() {
    let element = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Volume")
            .placeholder("0-100")
            .disabled(true)
            .required(true)
            .invalid(true)
            .selected(true)
            .checked(false)
            .expanded(true)
            .orientation(Orientation::Horizontal)
            .range(Some(0.0), Some(100.0), Some(50.0))
            .step(Some(5.0)),
    );

    let blueprint = Gtk4Adapter.blueprint(&element);

    assert_eq!(
        blueprint.control_state.placeholder.as_deref(),
        Some("0-100")
    );
    assert!(blueprint.control_state.disabled);
    assert!(blueprint.control_state.required);
    assert!(blueprint.control_state.invalid);
    assert!(blueprint.control_state.selected);
    assert_eq!(blueprint.control_state.checked, Some(false));
    assert_eq!(blueprint.control_state.expanded, Some(true));
    assert_eq!(
        blueprint.control_state.orientation,
        Some(Orientation::Horizontal)
    );
    assert_eq!(blueprint.control_state.min, Some(0.0));
    assert_eq!(blueprint.control_state.max, Some(100.0));
    assert_eq!(blueprint.control_state.current, Some(50.0));
    assert_eq!(blueprint.control_state.step, Some(5.0));
}

#[test]
fn widget_config_normalizes_blueprint_for_native_setters() {
    let element = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Volume")
            .value("50")
            .placeholder("0-100")
            .disabled(true)
            .required(true)
            .invalid(true)
            .orientation(Orientation::Horizontal)
            .range(Some(0.0), Some(100.0), Some(50.0))
            .step(Some(5.0))
            .metadata("data-testid", "volume-slider")
            .web(
                WebProps::new()
                    .class_name("range")
                    .style("display", "none")
                    .style("minWidth", "240")
                    .on_change("setVolume"),
            ),
    );

    let blueprint = WinUiAdapter.blueprint(&element);
    let config = blueprint.config();

    assert_eq!(config.widget_class, "Microsoft.UI.Xaml.Controls.Slider");
    assert_eq!(config.accessibility_role, AccessibilityRole::Slider);
    assert_eq!(config.label.as_deref(), Some("Volume"));
    assert_eq!(config.value.as_deref(), Some("50"));
    assert_eq!(config.placeholder.as_deref(), Some("0-100"));
    assert!(!config.enabled);
    assert!(!config.visible);
    assert!(config.required);
    assert!(config.invalid);
    assert_eq!(config.orientation, Some(Orientation::Horizontal));
    assert_eq!(config.min, Some(0.0));
    assert_eq!(config.max, Some(100.0));
    assert_eq!(config.current, Some(50.0));
    assert_eq!(config.step, Some(5.0));
    assert_eq!(config.class_name.as_deref(), Some("range"));
    assert_eq!(
        config
            .portable_style
            .min_width
            .as_ref()
            .and_then(|value| value.points()),
        Some(240.0)
    );
    assert_eq!(
        config.events.get("onChange").map(String::as_str),
        Some("setVolume")
    );
    assert_eq!(
        config.metadata.get("data-testid").map(String::as_str),
        Some("volume-slider")
    );

    let setters = config.create_setters();
    assert!(setters.contains(&NativeWidgetSetter::SetAccessibilityRole(
        AccessibilityRole::Slider
    )));
    assert!(setters.contains(&NativeWidgetSetter::SetLabel(Some("Volume".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetEnabled(false)));
    assert!(setters.contains(&NativeWidgetSetter::SetVisible(false)));
    assert!(setters.contains(&NativeWidgetSetter::SetPlaceholder(Some(
        "0-100".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetMinimum(Some(0.0))));
    assert!(setters.contains(&NativeWidgetSetter::SetMaximum(Some(100.0))));
    assert!(setters.contains(&NativeWidgetSetter::SetCurrent(Some(50.0))));
    assert!(setters.contains(&NativeWidgetSetter::SetStep(Some(5.0))));
}

#[test]
fn widget_config_preserves_html_form_control_hints() {
    let element = NativeElement::new("email", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Email")
            .read_only(true)
            .multiple(true)
            .auto_focus(true)
            .autocomplete("email")
            .input_mode("email")
            .enter_key_hint("send")
            .auto_capitalize("sentences")
            .auto_correct("on")
            .virtual_keyboard_policy("manual")
            .pattern(".+@example\\.com")
            .min_length(Some(3))
            .max_length(Some(64))
            .rows(Some(6))
            .cols(Some(40))
            .size(Some(32))
            .name("email")
            .form("profile-form")
            .input_type("email")
            .accept("image/*")
            .capture("environment")
            .alt("Profile photo")
            .src("/photo.png")
            .list("email-options")
            .dirname("email.dir")
            .form_action("/profiles")
            .form_enctype("multipart/form-data")
            .form_method("post")
            .form_target("_blank")
            .form_no_validate(true),
    );

    let config = AppKitAdapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert!(config.read_only);
    assert!(config.multiple);
    assert!(config.auto_focus);
    assert_eq!(config.autocomplete.as_deref(), Some("email"));
    assert_eq!(config.input_mode.as_deref(), Some("email"));
    assert_eq!(config.enter_key_hint.as_deref(), Some("send"));
    assert_eq!(config.auto_capitalize.as_deref(), Some("sentences"));
    assert_eq!(config.auto_correct.as_deref(), Some("on"));
    assert_eq!(config.virtual_keyboard_policy.as_deref(), Some("manual"));
    assert_eq!(config.pattern.as_deref(), Some(".+@example\\.com"));
    assert_eq!(config.min_length, Some(3));
    assert_eq!(config.max_length, Some(64));
    assert_eq!(config.rows, Some(6));
    assert_eq!(config.cols, Some(40));
    assert_eq!(config.size, Some(32));
    assert_eq!(config.name.as_deref(), Some("email"));
    assert_eq!(config.form.as_deref(), Some("profile-form"));
    assert_eq!(config.input_type.as_deref(), Some("email"));
    assert_eq!(config.accept.as_deref(), Some("image/*"));
    assert_eq!(config.capture.as_deref(), Some("environment"));
    assert_eq!(config.alt.as_deref(), Some("Profile photo"));
    assert_eq!(config.src.as_deref(), Some("/photo.png"));
    assert_eq!(config.list.as_deref(), Some("email-options"));
    assert_eq!(config.dirname.as_deref(), Some("email.dir"));
    assert_eq!(config.form_action.as_deref(), Some("/profiles"));
    assert_eq!(config.form_enctype.as_deref(), Some("multipart/form-data"));
    assert_eq!(config.form_method.as_deref(), Some("post"));
    assert_eq!(config.form_target.as_deref(), Some("_blank"));
    assert!(config.form_no_validate);
    assert!(setters.contains(&NativeWidgetSetter::SetReadOnly(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetMultiple(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetAutoFocus(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetAutocomplete(Some(
        "email".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetInputMode(Some("email".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetEnterKeyHint(Some(
        "send".to_string()
    ))));
    assert!(
        setters.contains(&NativeWidgetSetter::SetAutoCapitalize(Some(
            "sentences".to_string()
        )))
    );
    assert!(setters.contains(&NativeWidgetSetter::SetAutoCorrect(Some("on".to_string()))));
    assert!(
        setters.contains(&NativeWidgetSetter::SetVirtualKeyboardPolicy(Some(
            "manual".to_string()
        )))
    );
    assert!(setters.contains(&NativeWidgetSetter::SetPattern(Some(
        ".+@example\\.com".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetMinLength(Some(3))));
    assert!(setters.contains(&NativeWidgetSetter::SetMaxLength(Some(64))));
    assert!(setters.contains(&NativeWidgetSetter::SetRows(Some(6))));
    assert!(setters.contains(&NativeWidgetSetter::SetCols(Some(40))));
    assert!(setters.contains(&NativeWidgetSetter::SetSize(Some(32))));
    assert!(setters.contains(&NativeWidgetSetter::SetName(Some("email".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetForm(Some(
        "profile-form".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetInputType(Some("email".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetAccept(Some("image/*".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetCapture(Some(
        "environment".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetAlt(Some(
        "Profile photo".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetSrc(Some("/photo.png".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetList(Some(
        "email-options".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetDirname(Some(
        "email.dir".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetFormAction(Some(
        "/profiles".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetFormEnctype(Some(
        "multipart/form-data".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetFormMethod(Some("post".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetFormTarget(Some(
        "_blank".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetFormNoValidate(true)));
}

#[test]
fn widget_config_preserves_html_global_hints() {
    let element = NativeElement::new("panel", NativeRole::Section).with_props(
        NativeProps::new()
            .label("Panel")
            .title("Profile summary")
            .hidden(true)
            .lang("en-US")
            .dir("rtl")
            .tab_index(Some(-1))
            .explicit_role("region")
            .access_key("p")
            .content_editable("plaintext-only")
            .draggable("true")
            .spell_check(Some(false))
            .translate(Some(false))
            .inert(true)
            .popover("auto")
            .anchor("profile-card-anchor")
            .custom_element_is("profile-card")
            .nonce("nonce-1")
            .html_shadow(
                HtmlShadowProps::default()
                    .slot_name("summary")
                    .part("panel header")
                    .export_parts("header: panel-header"),
            )
            .html_microdata(
                HtmlMicrodataProps::default()
                    .item_scope(true)
                    .item_prop("profile")
                    .item_type("https://schema.org/ProfilePage")
                    .item_id("https://example.test/profiles/1")
                    .item_ref("profile-name profile-email"),
            ),
    );

    let config = AppKitAdapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert_eq!(config.title.as_deref(), Some("Profile summary"));
    assert!(config.hidden);
    assert!(!config.visible);
    assert_eq!(config.lang.as_deref(), Some("en-US"));
    assert_eq!(config.dir.as_deref(), Some("rtl"));
    assert_eq!(config.tab_index, Some(-1));
    assert_eq!(config.explicit_role.as_deref(), Some("region"));
    assert_eq!(config.access_key.as_deref(), Some("p"));
    assert_eq!(config.content_editable.as_deref(), Some("plaintext-only"));
    assert_eq!(config.draggable.as_deref(), Some("true"));
    assert_eq!(config.spell_check, Some(false));
    assert_eq!(config.translate, Some(false));
    assert!(config.inert);
    assert_eq!(config.popover.as_deref(), Some("auto"));
    assert_eq!(config.anchor.as_deref(), Some("profile-card-anchor"));
    assert_eq!(config.custom_element_is.as_deref(), Some("profile-card"));
    assert_eq!(config.nonce.as_deref(), Some("nonce-1"));
    assert_eq!(config.html_shadow.slot_name.as_deref(), Some("summary"));
    assert_eq!(config.html_shadow.part.as_deref(), Some("panel header"));
    assert_eq!(
        config.html_shadow.export_parts.as_deref(),
        Some("header: panel-header")
    );
    assert!(config.html_microdata.item_scope);
    assert_eq!(config.html_microdata.item_prop.as_deref(), Some("profile"));
    assert_eq!(
        config.html_microdata.item_type.as_deref(),
        Some("https://schema.org/ProfilePage")
    );
    assert_eq!(
        config.html_microdata.item_id.as_deref(),
        Some("https://example.test/profiles/1")
    );
    assert_eq!(
        config.html_microdata.item_ref.as_deref(),
        Some("profile-name profile-email")
    );
    assert!(setters.contains(&NativeWidgetSetter::SetTitle(Some(
        "Profile summary".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetHidden(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetLang(Some("en-US".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetDir(Some("rtl".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetTabIndex(Some(-1))));
    assert!(setters.contains(&NativeWidgetSetter::SetExplicitRole(Some(
        "region".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetAccessKey(Some("p".to_string()))));
    assert!(
        setters.contains(&NativeWidgetSetter::SetContentEditable(Some(
            "plaintext-only".to_string()
        )))
    );
    assert!(setters.contains(&NativeWidgetSetter::SetDraggable(Some("true".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetSpellCheck(Some(false))));
    assert!(setters.contains(&NativeWidgetSetter::SetTranslate(Some(false))));
    assert!(setters.contains(&NativeWidgetSetter::SetInert(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetPopover(Some("auto".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetAnchor(Some(
        "profile-card-anchor".to_string()
    ))));
    assert!(
        setters.contains(&NativeWidgetSetter::SetCustomElementIs(Some(
            "profile-card".to_string()
        )))
    );
    assert!(setters.contains(&NativeWidgetSetter::SetNonce(Some("nonce-1".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetHtmlShadow(
        HtmlShadowProps::default()
            .slot_name("summary")
            .part("panel header")
            .export_parts("header: panel-header")
    )));
    assert!(setters.contains(&NativeWidgetSetter::SetHtmlMicrodata(
        HtmlMicrodataProps::default()
            .item_scope(true)
            .item_prop("profile")
            .item_type("https://schema.org/ProfilePage")
            .item_id("https://example.test/profiles/1")
            .item_ref("profile-name profile-email")
    )));
}

#[test]
fn widget_config_preserves_accessibility_relationship_hints() {
    let relationships = AccessibilityRelationshipProps::default()
        .labelled_by("profile-title")
        .described_by("profile-help")
        .controls("profile-panel")
        .active_descendant("profile-row-1");
    let element = NativeElement::new("profile", NativeRole::Section)
        .with_props(NativeProps::new().accessibility_relationships(relationships.clone()));

    let config = Gtk4Adapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert_eq!(config.accessibility_relationships, relationships);
    assert!(
        setters.contains(&NativeWidgetSetter::SetAccessibilityRelationships(
            relationships
        ))
    );
}

#[test]
fn widget_config_preserves_accessibility_description_hints() {
    let description = AccessibilityDescriptionProps::default()
        .description("Volume in percent")
        .role_description("volume slider")
        .key_shortcuts("Alt+ArrowUp")
        .value_text("Half volume");
    let element = NativeElement::new("volume", NativeRole::Slider)
        .with_props(NativeProps::new().accessibility_description(description.clone()));

    let config = WinUiAdapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert_eq!(config.accessibility_description, description);
    assert!(
        setters.contains(&NativeWidgetSetter::SetAccessibilityDescription(
            description
        ))
    );
}

#[test]
fn widget_config_preserves_accessibility_structure_hints() {
    let structure = AccessibilityStructureProps::default()
        .level(Some(2))
        .position_in_set(Some(3))
        .set_size(Some(10))
        .row_count(Some(20))
        .row_index(Some(4))
        .row_span(Some(2))
        .column_count(Some(6))
        .column_index(Some(5))
        .column_span(Some(3))
        .row_index_text("Row four")
        .column_index_text("Column five")
        .sort("ascending");
    let element = NativeElement::new("metric-cell", NativeRole::TableCell).with_props(
        NativeProps::new()
            .accessibility_level(Some(2))
            .accessibility_position_in_set(Some(3))
            .accessibility_set_size(Some(10))
            .accessibility_row_count(Some(20))
            .accessibility_row_index(Some(4))
            .accessibility_row_span(Some(2))
            .accessibility_column_count(Some(6))
            .accessibility_column_index(Some(5))
            .accessibility_column_span(Some(3))
            .accessibility_row_index_text("Row four")
            .accessibility_column_index_text("Column five")
            .accessibility_sort("ascending"),
    );

    let config = Gtk4Adapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert_eq!(config.accessibility_structure, structure);
    assert!(setters.contains(&NativeWidgetSetter::SetAccessibilityStructure(structure)));
}

#[test]
fn widget_config_preserves_accessibility_state_hints() {
    let state = AccessibilityStateProps::default()
        .hidden(Some(true))
        .autocomplete("list")
        .multiline(Some(true))
        .current("page")
        .has_popup("dialog")
        .pressed("mixed")
        .live("polite")
        .atomic(Some(true))
        .busy(Some(false))
        .relevant("additions text")
        .modal(Some(true));
    let element = NativeElement::new("profile", NativeRole::Dialog).with_props(
        NativeProps::new()
            .accessibility_hidden(Some(true))
            .accessibility_autocomplete("list")
            .accessibility_multiline(Some(true))
            .current("page")
            .has_popup("dialog")
            .pressed("mixed")
            .live("polite")
            .atomic(Some(true))
            .busy(Some(false))
            .relevant("additions text")
            .modal(Some(true)),
    );

    let config = AppKitAdapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert!(config.visible);
    assert_eq!(config.accessibility_state, state);
    assert!(setters.contains(&NativeWidgetSetter::SetAccessibilityState(state)));
}

#[test]
fn widget_config_preserves_html_media_and_resource_hints() {
    let element = NativeElement::new("hero", NativeRole::Image).with_props(
        NativeProps::new()
            .label("Hero")
            .alt("Hero")
            .href("/gallery")
            .src("/hero.png")
            .srcset("/hero.png 1x, /hero@2x.png 2x")
            .sizes("100vw")
            .media("(min-width: 48rem)")
            .resource_type("image/png")
            .intrinsic_width(Some(640))
            .intrinsic_height(Some(360))
            .loading("lazy")
            .decoding("async")
            .fetch_priority("high")
            .cross_origin("anonymous")
            .referrer_policy("no-referrer")
            .poster("/poster.png")
            .controls(true)
            .autoplay(true)
            .loop_playback(true)
            .muted(true)
            .plays_inline(true)
            .preload("metadata")
            .track_kind("captions")
            .srclang("en")
            .track_label("English")
            .default_track(true),
    );

    let config = Gtk4Adapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert_eq!(config.alt.as_deref(), Some("Hero"));
    assert_eq!(config.href.as_deref(), Some("/gallery"));
    assert_eq!(config.src.as_deref(), Some("/hero.png"));
    assert_eq!(
        config.srcset.as_deref(),
        Some("/hero.png 1x, /hero@2x.png 2x")
    );
    assert_eq!(config.sizes.as_deref(), Some("100vw"));
    assert_eq!(config.media.as_deref(), Some("(min-width: 48rem)"));
    assert_eq!(config.resource_type.as_deref(), Some("image/png"));
    assert_eq!(config.intrinsic_width, Some(640));
    assert_eq!(config.intrinsic_height, Some(360));
    assert_eq!(config.loading.as_deref(), Some("lazy"));
    assert_eq!(config.decoding.as_deref(), Some("async"));
    assert_eq!(config.fetch_priority.as_deref(), Some("high"));
    assert_eq!(config.cross_origin.as_deref(), Some("anonymous"));
    assert_eq!(config.referrer_policy.as_deref(), Some("no-referrer"));
    assert_eq!(config.poster.as_deref(), Some("/poster.png"));
    assert!(config.controls);
    assert!(config.autoplay);
    assert!(config.loop_playback);
    assert!(config.muted);
    assert!(config.plays_inline);
    assert_eq!(config.preload.as_deref(), Some("metadata"));
    assert_eq!(config.track_kind.as_deref(), Some("captions"));
    assert_eq!(config.srclang.as_deref(), Some("en"));
    assert_eq!(config.track_label.as_deref(), Some("English"));
    assert!(config.default_track);
    assert!(setters.contains(&NativeWidgetSetter::SetAlt(Some("Hero".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetHref(Some("/gallery".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetSrc(Some("/hero.png".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetSrcset(Some(
        "/hero.png 1x, /hero@2x.png 2x".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetIntrinsicWidth(Some(640))));
    assert!(setters.contains(&NativeWidgetSetter::SetIntrinsicHeight(Some(360))));
    assert!(setters.contains(&NativeWidgetSetter::SetControls(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetAutoplay(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetLoopPlayback(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetMuted(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetPlaysInline(true)));
    assert!(setters.contains(&NativeWidgetSetter::SetDefaultTrack(true)));
}

#[test]
fn widget_config_preserves_html_collection_hints() {
    let table_cell = NativeElement::new("metric-cell", NativeRole::TableCell).with_props(
        NativeProps::new()
            .column_span(Some(2))
            .row_span(Some(3))
            .headers("quarter revenue")
            .scope("colgroup")
            .cell_abbr("Rev"),
    );
    let list = NativeElement::new("steps", NativeRole::ListBox).with_props(
        NativeProps::new()
            .list_start(Some(5))
            .list_reversed(true)
            .list_type("A")
            .list_item_value(Some(7)),
    );

    let table_config = Gtk4Adapter.blueprint(&table_cell).config();
    let list_config = Gtk4Adapter.blueprint(&list).config();
    let table_setters = table_config.create_setters();
    let list_setters = list_config.create_setters();

    let expected_table_collection = HtmlCollectionProps::default()
        .column_span(Some(2))
        .row_span(Some(3))
        .headers("quarter revenue")
        .scope("colgroup")
        .cell_abbr("Rev");
    let expected_list_collection = HtmlCollectionProps::default()
        .list_start(Some(5))
        .list_reversed(true)
        .list_type("A")
        .list_item_value(Some(7));

    assert_eq!(table_config.html_collection, expected_table_collection);
    assert_eq!(list_config.html_collection, expected_list_collection);
    assert!(
        table_setters.contains(&NativeWidgetSetter::SetHtmlCollection(
            expected_table_collection
        ))
    );
    assert!(
        list_setters.contains(&NativeWidgetSetter::SetHtmlCollection(
            expected_list_collection
        ))
    );
}

#[test]
fn widget_config_preserves_html_activation_hints() {
    let activation = HtmlActivationProps::default()
        .command("show-modal")
        .command_for("settings-dialog")
        .popover_target("settings-popover")
        .popover_target_action("show");
    let element = NativeElement::new("settings", NativeRole::Button)
        .with_props(NativeProps::new().html_activation(activation.clone()));

    let config = Gtk4Adapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert_eq!(config.html_activation, activation);
    assert!(setters.contains(&NativeWidgetSetter::SetHtmlActivation(activation)));
}

#[test]
fn widget_config_preserves_html_text_annotation_hints() {
    let text_annotation = HtmlTextAnnotationProps::default()
        .cite("https://example.test/change")
        .date_time("2026-07-02T09:00:00Z");
    let element = NativeElement::new("change", NativeRole::InsertedText)
        .with_props(NativeProps::new().html_text_annotation(text_annotation.clone()));

    let config = Gtk4Adapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert_eq!(config.html_text_annotation, text_annotation);
    assert!(setters.contains(&NativeWidgetSetter::SetHtmlTextAnnotation(text_annotation)));
}

#[test]
fn widget_config_preserves_html_form_association_hints() {
    let form_association = HtmlFormAssociationProps::default()
        .label_for("email")
        .output_for("price quantity")
        .meter_low(Some(25.0))
        .meter_high(Some(90.0))
        .meter_optimum(Some(75.0));
    let element = NativeElement::new("quota", NativeRole::Meter)
        .with_props(NativeProps::new().html_form_association(form_association.clone()));

    let config = Gtk4Adapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert_eq!(config.html_form_association, form_association);
    assert!(
        setters.contains(&NativeWidgetSetter::SetHtmlFormAssociation(
            form_association
        ))
    );
}

#[test]
fn widget_config_preserves_html_resource_policy_hints() {
    let resource_policy = HtmlResourcePolicyProps::default()
        .target("_blank")
        .download("guide.pdf")
        .ping("/analytics")
        .rel("noopener")
        .href_lang("en")
        .link_as("image")
        .integrity("sha384-resource")
        .blocking("render")
        .nonce("nonce-1")
        .image_srcset("/hero.avif 1x")
        .image_sizes("100vw")
        .resource_disabled(true)
        .async_script(true)
        .defer_script(true)
        .no_module(true)
        .frame_name("preview")
        .frame_allow("fullscreen")
        .frame_allow_fullscreen(true)
        .frame_sandbox("allow-scripts")
        .frame_srcdoc("<p>Preview</p>");
    let element = NativeElement::new("preload", NativeRole::ResourceLink)
        .with_props(NativeProps::new().html_resource_policy(resource_policy.clone()));

    let config = Gtk4Adapter.blueprint(&element).config();
    let setters = config.create_setters();

    assert_eq!(config.html_resource_policy, resource_policy);
    assert!(setters.contains(&NativeWidgetSetter::SetHtmlResourcePolicy(resource_policy)));
}

#[test]
fn widget_config_diff_reports_changed_native_setters() {
    let first = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Volume")
            .value("50")
            .range(Some(0.0), Some(100.0), Some(50.0))
            .step(Some(5.0))
            .name("volume")
            .anchor("volume-anchor")
            .nonce("nonce-1")
            .form_action("/volume")
            .web(
                WebProps::new()
                    .style("display", "flex")
                    .on_change("setVolume"),
            ),
    );
    let second = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Muted")
            .value("0")
            .disabled(true)
            .range(Some(0.0), Some(100.0), Some(0.0))
            .step(Some(10.0))
            .name("mute")
            .anchor("mute-anchor")
            .nonce("nonce-2")
            .form_action("/mute")
            .form_no_validate(true)
            .web(
                WebProps::new()
                    .style("display", "none")
                    .on_change("setVolume"),
            ),
    );

    let before = Gtk4Adapter.blueprint(&first).config();
    let after = Gtk4Adapter.blueprint(&second).config();
    let unchanged = before.diff(&before);
    let patch = before.diff(&after);

    assert!(unchanged.is_empty());
    assert_eq!(
        patch.label.as_ref().map(|change| change.after.as_deref()),
        Some(Some("Muted"))
    );
    assert_eq!(
        patch.value.as_ref().map(|change| change.after.as_deref()),
        Some(Some("0"))
    );
    assert_eq!(
        patch.enabled.as_ref().map(|change| change.after),
        Some(false)
    );
    assert_eq!(
        patch.visible.as_ref().map(|change| change.after),
        Some(false)
    );
    assert_eq!(
        patch.current.as_ref().map(|change| change.after),
        Some(Some(0.0))
    );
    assert_eq!(
        patch.step.as_ref().map(|change| change.after),
        Some(Some(10.0))
    );
    assert_eq!(
        patch.name.as_ref().map(|change| change.after.as_deref()),
        Some(Some("mute"))
    );
    assert_eq!(
        patch.anchor.as_ref().map(|change| change.after.as_deref()),
        Some(Some("mute-anchor"))
    );
    assert_eq!(
        patch.nonce.as_ref().map(|change| change.after.as_deref()),
        Some(Some("nonce-2"))
    );
    assert_eq!(
        patch
            .form_action
            .as_ref()
            .map(|change| change.after.as_deref()),
        Some(Some("/mute"))
    );
    assert_eq!(
        patch.form_no_validate.as_ref().map(|change| change.after),
        Some(true)
    );
    assert!(patch.min.is_none());
    assert!(patch.max.is_none());
    assert!(patch.events.is_none());

    let setters = patch.setters();
    assert!(setters.contains(&NativeWidgetSetter::SetLabel(Some("Muted".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetValue(Some("0".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetEnabled(false)));
    assert!(setters.contains(&NativeWidgetSetter::SetVisible(false)));
    assert!(setters.contains(&NativeWidgetSetter::SetCurrent(Some(0.0))));
    assert!(setters.contains(&NativeWidgetSetter::SetStep(Some(10.0))));
    assert!(setters.contains(&NativeWidgetSetter::SetName(Some("mute".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetAnchor(Some(
        "mute-anchor".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetNonce(Some("nonce-2".to_string()))));
    assert!(setters.contains(&NativeWidgetSetter::SetFormAction(Some(
        "/mute".to_string()
    ))));
    assert!(setters.contains(&NativeWidgetSetter::SetFormNoValidate(true)));
    assert!(!setters.contains(&NativeWidgetSetter::SetMinimum(Some(0.0))));
    assert!(!setters.contains(&NativeWidgetSetter::SetMaximum(Some(100.0))));
    assert!(!setters
        .iter()
        .any(|setter| matches!(setter, NativeWidgetSetter::SetEvents(_))));
}

#[test]
fn native_widget_setters_round_trip_as_json() {
    let setters = vec![
        NativeWidgetSetter::SetLabel(Some("Save".to_string())),
        NativeWidgetSetter::SetEnabled(false),
        NativeWidgetSetter::SetReadOnly(true),
        NativeWidgetSetter::SetCurrent(Some(50.0)),
        NativeWidgetSetter::SetStep(Some(5.0)),
        NativeWidgetSetter::SetAutocomplete(Some("email".to_string())),
        NativeWidgetSetter::SetEnterKeyHint(Some("send".to_string())),
        NativeWidgetSetter::SetAccessibilityRelationships(
            AccessibilityRelationshipProps::default()
                .labelled_by("save-label")
                .controls("save-panel"),
        ),
        NativeWidgetSetter::SetAccessibilityDescription(
            AccessibilityDescriptionProps::default()
                .description("Save changes")
                .role_description("primary button")
                .key_shortcuts("Meta+S")
                .value_text("Ready"),
        ),
        NativeWidgetSetter::SetAccessibilityStructure(
            AccessibilityStructureProps::default()
                .level(Some(1))
                .position_in_set(Some(2))
                .set_size(Some(5))
                .row_count(Some(20))
                .row_index(Some(4))
                .row_span(Some(2))
                .column_count(Some(6))
                .column_index(Some(3))
                .column_span(Some(1))
                .row_index_text("Row four")
                .column_index_text("Column three")
                .sort("descending"),
        ),
        NativeWidgetSetter::SetAccessibilityState(
            AccessibilityStateProps::default()
                .hidden(Some(false))
                .autocomplete("inline")
                .multiline(Some(false))
                .current("page")
                .has_popup("menu")
                .pressed("false")
                .live("polite"),
        ),
        NativeWidgetSetter::SetAnchor(Some("profile-card-anchor".to_string())),
        NativeWidgetSetter::SetCustomElementIs(Some("profile-card".to_string())),
        NativeWidgetSetter::SetNonce(Some("nonce-1".to_string())),
        NativeWidgetSetter::SetEvents(BTreeMap::from([(
            "onPress".to_string(),
            "saveProfile".to_string(),
        )])),
    ];

    let json = serde_json::to_string(&setters).unwrap();
    let decoded: Vec<NativeWidgetSetter> = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded, setters);
    assert!(json.contains(r#""type":"setLabel""#));
    assert!(json.contains(r#""type":"setEnabled""#));
    assert!(json.contains(r#""type":"setCurrent""#));
    assert!(json.contains(r#""type":"setStep""#));
    assert!(json.contains(r#""type":"setEnterKeyHint""#));
    assert!(json.contains(r#""type":"setAccessibilityRelationships""#));
    assert!(json.contains(r#""type":"setAccessibilityDescription""#));
    assert!(json.contains(r#""type":"setAccessibilityStructure""#));
    assert!(json.contains(r#""type":"setAccessibilityState""#));
    assert!(json.contains(r#""autocomplete":"inline""#));
    assert!(json.contains(r#""multiline":false"#));
    assert!(json.contains(r#""type":"setAnchor""#));
    assert!(json.contains(r#""type":"setCustomElementIs""#));
    assert!(json.contains(r#""type":"setNonce""#));
    assert!(json.contains(r#""onPress":"saveProfile""#));
}

#[test]
fn widget_setters_replay_into_native_config() {
    let before = Gtk4Adapter
        .blueprint(
            &NativeElement::new("volume", NativeRole::Slider).with_props(
                NativeProps::new()
                    .label("Volume")
                    .range(Some(0.0), Some(100.0), Some(50.0))
                    .step(Some(5.0)),
            ),
        )
        .config();
    let after = Gtk4Adapter
        .blueprint(
            &NativeElement::new("volume", NativeRole::Slider).with_props(
                NativeProps::new()
                    .label("Muted")
                    .disabled(true)
                    .range(Some(0.0), Some(100.0), Some(0.0))
                    .step(Some(10.0))
                    .enter_key_hint("done")
                    .auto_capitalize("words")
                    .auto_correct("off")
                    .virtual_keyboard_policy("auto")
                    .accessibility_relationships(
                        AccessibilityRelationshipProps::default()
                            .labelled_by("volume-label")
                            .described_by("volume-help")
                            .controls("volume-output"),
                    )
                    .accessibility_description(
                        AccessibilityDescriptionProps::default()
                            .description("Volume in percent")
                            .role_description("volume slider")
                            .key_shortcuts("Alt+ArrowUp")
                            .value_text("Muted"),
                    )
                    .accessibility_structure(
                        AccessibilityStructureProps::default()
                            .level(Some(2))
                            .position_in_set(Some(1))
                            .set_size(Some(3))
                            .row_count(Some(20))
                            .row_index(Some(4))
                            .row_span(Some(2))
                            .column_count(Some(6))
                            .column_index(Some(5))
                            .column_span(Some(3))
                            .row_index_text("Row four")
                            .column_index_text("Column five")
                            .sort("other"),
                    )
                    .accessibility_state(
                        AccessibilityStateProps::default()
                            .hidden(Some(true))
                            .autocomplete("both")
                            .multiline(Some(true))
                            .current("step")
                            .has_popup("listbox")
                            .pressed("true")
                            .live("assertive")
                            .atomic(Some(true))
                            .busy(Some(false))
                            .relevant("all")
                            .modal(Some(true)),
                    )
                    .anchor("profile-card-anchor")
                    .custom_element_is("profile-card")
                    .nonce("nonce-1")
                    .html_activation(
                        HtmlActivationProps::default()
                            .command("show-modal")
                            .command_for("settings-dialog"),
                    )
                    .html_text_annotation(
                        HtmlTextAnnotationProps::default()
                            .cite("https://example.test/change")
                            .date_time("2026-07-02T09:00:00Z"),
                    )
                    .html_dialog(HtmlDialogProps::default().open(true))
                    .html_shadow(HtmlShadowProps::default().slot_name("summary"))
                    .html_microdata(
                        HtmlMicrodataProps::default()
                            .item_scope(true)
                            .item_prop("profile"),
                    )
                    .html_form_association(
                        HtmlFormAssociationProps::default()
                            .meter_low(Some(25.0))
                            .meter_high(Some(90.0)),
                    ),
            ),
        )
        .config();
    let mut replayed = before.clone();

    apply_widget_setters(&mut replayed, &before.diff(&after).setters());

    assert_eq!(replayed.label.as_deref(), Some("Muted"));
    assert!(!replayed.enabled);
    assert_eq!(replayed.current, Some(0.0));
    assert_eq!(replayed.step, Some(10.0));
    assert_eq!(replayed.enter_key_hint.as_deref(), Some("done"));
    assert_eq!(replayed.auto_capitalize.as_deref(), Some("words"));
    assert_eq!(replayed.auto_correct.as_deref(), Some("off"));
    assert_eq!(replayed.virtual_keyboard_policy.as_deref(), Some("auto"));
    assert_eq!(
        replayed.accessibility_relationships.labelled_by.as_deref(),
        Some("volume-label")
    );
    assert_eq!(
        replayed.accessibility_relationships.described_by.as_deref(),
        Some("volume-help")
    );
    assert_eq!(
        replayed.accessibility_relationships.controls.as_deref(),
        Some("volume-output")
    );
    assert_eq!(
        replayed.accessibility_description.description.as_deref(),
        Some("Volume in percent")
    );
    assert_eq!(
        replayed
            .accessibility_description
            .role_description
            .as_deref(),
        Some("volume slider")
    );
    assert_eq!(
        replayed.accessibility_description.key_shortcuts.as_deref(),
        Some("Alt+ArrowUp")
    );
    assert_eq!(
        replayed.accessibility_description.value_text.as_deref(),
        Some("Muted")
    );
    assert_eq!(replayed.accessibility_structure.level, Some(2));
    assert_eq!(replayed.accessibility_structure.position_in_set, Some(1));
    assert_eq!(replayed.accessibility_structure.set_size, Some(3));
    assert_eq!(replayed.accessibility_structure.row_count, Some(20));
    assert_eq!(replayed.accessibility_structure.row_index, Some(4));
    assert_eq!(replayed.accessibility_structure.row_span, Some(2));
    assert_eq!(replayed.accessibility_structure.column_count, Some(6));
    assert_eq!(replayed.accessibility_structure.column_index, Some(5));
    assert_eq!(replayed.accessibility_structure.column_span, Some(3));
    assert_eq!(
        replayed.accessibility_structure.row_index_text.as_deref(),
        Some("Row four")
    );
    assert_eq!(
        replayed
            .accessibility_structure
            .column_index_text
            .as_deref(),
        Some("Column five")
    );
    assert_eq!(
        replayed.accessibility_structure.sort.as_deref(),
        Some("other")
    );
    assert_eq!(replayed.accessibility_state.hidden, Some(true));
    assert_eq!(
        replayed.accessibility_state.autocomplete.as_deref(),
        Some("both")
    );
    assert_eq!(replayed.accessibility_state.multiline, Some(true));
    assert_eq!(
        replayed.accessibility_state.current.as_deref(),
        Some("step")
    );
    assert_eq!(
        replayed.accessibility_state.has_popup.as_deref(),
        Some("listbox")
    );
    assert_eq!(
        replayed.accessibility_state.pressed.as_deref(),
        Some("true")
    );
    assert_eq!(
        replayed.accessibility_state.live.as_deref(),
        Some("assertive")
    );
    assert_eq!(replayed.accessibility_state.atomic, Some(true));
    assert_eq!(replayed.accessibility_state.busy, Some(false));
    assert_eq!(
        replayed.accessibility_state.relevant.as_deref(),
        Some("all")
    );
    assert_eq!(replayed.accessibility_state.modal, Some(true));
    assert_eq!(replayed.anchor.as_deref(), Some("profile-card-anchor"));
    assert_eq!(replayed.custom_element_is.as_deref(), Some("profile-card"));
    assert_eq!(replayed.nonce.as_deref(), Some("nonce-1"));
    assert_eq!(
        replayed.html_activation.command.as_deref(),
        Some("show-modal")
    );
    assert_eq!(
        replayed.html_activation.command_for.as_deref(),
        Some("settings-dialog")
    );
    assert_eq!(
        replayed.html_text_annotation.cite.as_deref(),
        Some("https://example.test/change")
    );
    assert_eq!(
        replayed.html_text_annotation.date_time.as_deref(),
        Some("2026-07-02T09:00:00Z")
    );
    assert_eq!(replayed.html_dialog.open, Some(true));
    assert_eq!(replayed.html_shadow.slot_name.as_deref(), Some("summary"));
    assert!(replayed.html_microdata.item_scope);
    assert_eq!(
        replayed.html_microdata.item_prop.as_deref(),
        Some("profile")
    );
    assert_eq!(replayed.html_form_association.meter_low, Some(25.0));
    assert_eq!(replayed.html_form_association.meter_high, Some(90.0));
    assert_eq!(replayed, after);
}

#[test]
fn renderer_can_drive_platform_planning_host_directly() {
    let tree = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save").action("saveDocument")),
        )
        .child(
            NativeElement::new("email", NativeRole::TextField)
                .with_props(NativeProps::new().label("Email")),
        );
    let mut renderer = Renderer::new();
    let mut host = PlatformPlanningHost::new(WinUiAdapter);

    let root_id = renderer.render(&tree, &mut host).unwrap();
    let root = host.node(root_id).unwrap();
    let child_widgets: Vec<_> = root
        .children
        .iter()
        .map(|id| host.node(*id).unwrap().blueprint.widget_class.as_str())
        .collect();

    assert_eq!(
        root.blueprint.widget_class,
        "Microsoft.UI.Xaml.Controls.StackPanel"
    );
    assert_eq!(
        child_widgets,
        vec![
            "Microsoft.UI.Xaml.Controls.Button",
            "Microsoft.UI.Xaml.Controls.TextBox"
        ]
    );
    assert!(host.commands().iter().any(|command| matches!(
        command,
        PlatformCommand::Create {
            blueprint,
            ..
        } if blueprint.widget_class == "Microsoft.UI.Xaml.Controls.Button"
    )));
}

#[test]
fn platform_planning_host_updates_blueprint_on_rerender() {
    let first = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .web(WebProps::new().style("minWidth", "120").on_click("save")),
    );
    let second = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new().label("Saved").web(
            WebProps::new()
                .style("minWidth", "160")
                .on_press("saveAgain"),
        ),
    );
    let mut renderer = Renderer::new();
    let mut host = PlatformPlanningHost::new(AppKitAdapter);

    let first_id = renderer.render(&first, &mut host).unwrap();
    let second_id = renderer.render(&second, &mut host).unwrap();

    assert_eq!(first_id, second_id);
    let blueprint = &host.node(second_id).unwrap().blueprint;
    assert_eq!(blueprint.label.as_deref(), Some("Saved"));
    assert_eq!(blueprint.action.as_deref(), Some("saveAgain"));
    assert_eq!(
        blueprint
            .portable_style
            .min_width
            .as_ref()
            .and_then(|value| value.points()),
        Some(160.0)
    );
    assert!(host.commands().iter().any(|command| matches!(
        command,
        PlatformCommand::Update {
            id,
            blueprint,
        } if *id == second_id && blueprint.label.as_deref() == Some("Saved")
    )));
}

#[test]
fn command_stream_records_native_remove_and_reorder() {
    let first = NativeElement::new("root", NativeRole::View)
        .child(NativeElement::new("a", NativeRole::Button))
        .child(NativeElement::new("b", NativeRole::Button));
    let second = NativeElement::new("root", NativeRole::View)
        .child(NativeElement::new("b", NativeRole::Button))
        .child(NativeElement::new("c", NativeRole::Button));
    let mut renderer = Renderer::new();
    let mut host = PlatformPlanningHost::new(Gtk4Adapter);

    let root_id = renderer.render(&first, &mut host).unwrap();
    host.clear_commands();
    renderer.render(&second, &mut host).unwrap();

    assert!(host.commands().iter().any(|command| matches!(
        command,
        PlatformCommand::InsertChild {
            parent,
            index: 0,
            ..
        } if *parent == root_id
    )));
    assert!(host
        .commands()
        .iter()
        .any(|command| matches!(command, PlatformCommand::Remove { .. })));
    assert!(host.commands().iter().any(|command| matches!(
        command,
        PlatformCommand::Create {
            blueprint,
            ..
        } if blueprint.widget_class == "gtk::Button"
    )));
}

#[test]
fn platform_commands_round_trip_as_native_backend_json() {
    let element = NativeElement::new("email", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Email")
            .value("a@b.c")
            .placeholder("you@example.com")
            .disabled(true)
            .required(true)
            .invalid(true)
            .range(Some(0.0), Some(100.0), Some(50.0))
            .step(Some(5.0))
            .web(
                WebProps::new()
                    .style("minWidth", "280")
                    .attribute("data-testid", "email-input")
                    .on_change("setEmail"),
            ),
    );
    let command = PlatformCommand::Create {
        id: HostNodeId::new(42),
        blueprint: Gtk4Adapter.blueprint(&element),
    };

    let json = serde_json::to_string(&command).unwrap();
    let decoded: PlatformCommand = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded, command);
    assert!(json.contains(r#""type":"create""#));
    assert!(json.contains(r#""backend":"gtk4""#));
    assert!(json.contains(r#""widgetClass":"gtk::Entry""#));
    assert!(json.contains(r#""role":"textField""#));
    assert!(json.contains(r#""accessibilityRole":"textField""#));
    assert!(json.contains(r#""controlState""#));
    assert!(json.contains(r#""placeholder":"you@example.com""#));
    assert!(json.contains(r#""disabled":true"#));
    assert!(json.contains(r#""onChange":"setEmail""#));
    let PlatformCommand::Create { blueprint, .. } = decoded else {
        unreachable!("decoded command should remain a create command");
    };
    assert_eq!(
        blueprint.control_state.placeholder.as_deref(),
        Some("you@example.com")
    );
    assert!(blueprint.control_state.disabled);
    assert!(blueprint.control_state.required);
    assert!(blueprint.control_state.invalid);
    assert_eq!(blueprint.control_state.min, Some(0.0));
    assert_eq!(blueprint.control_state.max, Some(100.0));
    assert_eq!(blueprint.control_state.current, Some(50.0));
    assert_eq!(blueprint.control_state.step, Some(5.0));
}
