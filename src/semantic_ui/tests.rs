use super::*;
use crate::accessibility::{AccessibilityNode, AccessibilityRole};
use crate::geometry::Orientation;
use crate::host::{HeadlessHost, HostOperation};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{native_widget_name, NativeBackendKind};
use crate::renderer::Renderer;

#[test]
fn maps_button_to_native_button_with_accessibility_label() {
    let aria = SemanticElement::new("save", SemanticComponent::Button).with_props(
        SemanticProps::new()
            .label("Save")
            .action("save")
            .labelled_by("save-label")
            .pressed("false")
            .has_popup("menu"),
    );

    let native = SemanticMapper::new().map(&aria).unwrap();

    assert_eq!(native.role, NativeRole::Button);
    assert_eq!(native.props.label.as_deref(), Some("Save"));
    assert_eq!(native.props.action.as_deref(), Some("save"));

    let accessibility = AccessibilityNode::from_native(&native);
    assert_eq!(accessibility.role, AccessibilityRole::Button);
    assert_eq!(accessibility.label.as_deref(), Some("Save"));
    assert_eq!(
        accessibility.relationships.labelled_by.as_deref(),
        Some("save-label")
    );
    assert_eq!(accessibility.state.pressed.as_deref(), Some("false"));
    assert_eq!(accessibility.state.has_popup.as_deref(), Some("menu"));
}

#[test]
fn aria_label_becomes_native_accessibility_label_without_visible_text() {
    let aria = SemanticElement::new("save", SemanticComponent::Button).with_props(
        SemanticProps::new()
            .dom_attribute("aria-label", "Save profile")
            .on_press("saveProfile"),
    );

    let native = SemanticMapper::new().map(&aria).unwrap();
    let accessibility = AccessibilityNode::from_native(&native);

    assert_eq!(native.role, NativeRole::Button);
    assert_eq!(native.props.label, None);
    assert_eq!(
        native.props.accessibility_label.as_deref(),
        Some("Save profile")
    );
    assert_eq!(native.props.action.as_deref(), Some("saveProfile"));
    assert_eq!(accessibility.label.as_deref(), Some("Save profile"));
}

#[test]
fn aria_label_overrides_descendant_text_for_container_name() {
    let aria = SemanticElement::new("preferences", SemanticComponent::Dialog)
        .with_props(SemanticProps::new().dom_attribute("aria-label", "Preferences"))
        .child(
            SemanticElement::new("close", SemanticComponent::Button)
                .child(SemanticElement::text("close-text", "Close")),
        );

    let native = SemanticMapper::new().map(&aria).unwrap();

    assert_eq!(native.role, NativeRole::Dialog);
    assert_eq!(native.props.label, None);
    assert_eq!(
        native.props.accessibility_label.as_deref(),
        Some("Preferences")
    );
    assert_eq!(native.children[0].props.label.as_deref(), Some("Close"));
}

#[test]
fn aria_label_overrides_visible_text_only_for_accessibility() {
    let aria = SemanticElement::new("save", SemanticComponent::Button)
        .with_props(SemanticProps::new().dom_attribute("aria-label", "Save profile"))
        .child(SemanticElement::text("save-text", "Save"));

    let native = SemanticMapper::new().map(&aria).unwrap();
    let accessibility = AccessibilityNode::from_native(&native);

    assert_eq!(native.props.label.as_deref(), Some("Save"));
    assert_eq!(
        native.props.accessibility_label.as_deref(),
        Some("Save profile")
    );
    assert_eq!(accessibility.label.as_deref(), Some("Save profile"));
}

#[test]
fn tree_mapping_flattens_nested_items_without_losing_hierarchy() {
    let tree = SemanticElement::new("files", SemanticComponent::Tree)
        .with_props(SemanticProps::new().label("Files"))
        .child(
            SemanticElement::new("documents", SemanticComponent::TreeItem)
                .with_props(SemanticProps::new().text_value("Documents"))
                .child(
                    SemanticElement::new("documents-content", SemanticComponent::Group)
                        .child(SemanticElement::text("documents-label", "Documents")),
                )
                .child(
                    SemanticElement::new("resume", SemanticComponent::TreeItem)
                        .with_props(SemanticProps::new().text_value("Resume"))
                        .child(SemanticElement::text("resume-label", "Resume")),
                ),
        )
        .child(
            SemanticElement::new("photos", SemanticComponent::TreeItem)
                .child(SemanticElement::text("photos-label", "Photos")),
        );

    let native = SemanticMapper::new().map(&tree).unwrap();

    assert_eq!(native.role, NativeRole::Tree);
    assert_eq!(native.children.len(), 3);
    assert_eq!(native.children[0].key.as_str(), "documents");
    assert_eq!(native.children[1].key.as_str(), "resume");
    assert_eq!(native.children[2].key.as_str(), "photos");
    assert_eq!(native.children[0].props.label.as_deref(), Some("Documents"));
    assert_eq!(native.children[0].props.expanded, Some(false));
    assert_eq!(
        native.children[0]
            .props
            .metadata
            .get("data-tree-level")
            .map(String::as_str),
        Some("1")
    );
    assert_eq!(
        native.children[1]
            .props
            .metadata
            .get("data-tree-parent-key")
            .map(String::as_str),
        Some("documents")
    );
    assert_eq!(
        native.children[1].props.accessibility_structure.level,
        Some(2)
    );
    assert!(native.children.iter().all(|item| item.children.is_empty()));
}

#[test]
fn folds_text_field_label_and_input_into_one_native_control() {
    let aria = SemanticElement::new("email-field", SemanticComponent::TextField)
        .child(SemanticElement::text("email-label", "Email"))
        .child(
            SemanticElement::new("email-input", SemanticComponent::Input).with_props(
                SemanticProps::new()
                    .placeholder("you@example.com")
                    .value("a@b.c"),
            ),
        );

    let native = SemanticMapper::new().map(&aria).unwrap();

    assert_eq!(native.role, NativeRole::TextField);
    assert_eq!(native.props.label.as_deref(), Some("Email"));
    assert_eq!(native.props.placeholder.as_deref(), Some("you@example.com"));
    assert_eq!(native.props.value.as_deref(), Some("a@b.c"));
    assert!(native.children.is_empty());
}

#[test]
fn folded_text_field_inherits_input_web_events_and_style() {
    let aria = SemanticElement::new("email-field", SemanticComponent::TextField)
        .child(SemanticElement::text("email-label", "Email"))
        .child(
            SemanticElement::new("email-input", SemanticComponent::Input).with_props(
                SemanticProps::new()
                    .on_change("setEmail")
                    .style("minWidth", "280")
                    .dom_attribute("data-testid", "email-input"),
            ),
        );

    let native = SemanticMapper::new().map(&aria).unwrap();

    assert_eq!(native.role, NativeRole::TextField);
    assert_eq!(native.props.action.as_deref(), Some("setEmail"));
    assert_eq!(
        native.props.web.events.get("onChange").map(String::as_str),
        Some("setEmail")
    );
    assert_eq!(
        native.props.web.style.get("minWidth").map(String::as_str),
        Some("280")
    );
    assert_eq!(
        native.props.metadata.get("data-testid").map(String::as_str),
        Some("email-input")
    );
}

#[test]
fn maps_select_listbox_items_to_native_options() {
    let aria = SemanticElement::new("project", SemanticComponent::Select)
        .child(
            SemanticElement::new("project-label", SemanticComponent::Label)
                .with_props(SemanticProps::new().text_value("Project")),
        )
        .child(
            SemanticElement::new("project-options", SemanticComponent::ListBox)
                .child(
                    SemanticElement::new("a3s", SemanticComponent::ListBoxItem)
                        .with_props(SemanticProps::new().text_value("A3S").selected(true)),
                )
                .child(
                    SemanticElement::new("other", SemanticComponent::ListBoxItem)
                        .with_props(SemanticProps::new().text_value("Other")),
                ),
        );

    let native = SemanticMapper::new().map(&aria).unwrap();

    assert_eq!(native.role, NativeRole::Select);
    assert_eq!(native.props.label.as_deref(), Some("Project"));
    assert_eq!(native.children.len(), 2);
    assert_eq!(native.children[0].role, NativeRole::ListBoxItem);
    assert_eq!(native.children[0].props.label.as_deref(), Some("A3S"));
    assert!(native.children[0].props.selected);
}

#[test]
fn maps_checkbox_and_switch_to_typed_toggle_roles() {
    let checkbox = SemanticElement::new("accept", SemanticComponent::Checkbox).with_props(
        SemanticProps::new()
            .text_value("Accept terms")
            .checked(true)
            .on_change("setAccepted"),
    );
    let switch = SemanticElement::new("notifications", SemanticComponent::Switch).with_props(
        SemanticProps::new()
            .text_value("Notifications")
            .checked(false)
            .on_change("setNotifications"),
    );

    let checkbox = SemanticMapper::new().map(&checkbox).unwrap();
    let switch = SemanticMapper::new().map(&switch).unwrap();

    assert_eq!(checkbox.role, NativeRole::Checkbox);
    assert_eq!(checkbox.props.label.as_deref(), Some("Accept terms"));
    assert_eq!(checkbox.props.checked, Some(true));
    assert_eq!(checkbox.props.action.as_deref(), Some("setAccepted"));
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, checkbox.role),
        "a3s_gui::HeadlessNode"
    );

    assert_eq!(switch.role, NativeRole::Switch);
    assert_eq!(switch.props.checked, Some(false));
    assert_eq!(switch.props.action.as_deref(), Some("setNotifications"));
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, switch.role),
        "a3s_gui::HeadlessNode"
    );
}

#[test]
fn maps_radio_group_to_typed_radio_roles() {
    let aria = SemanticElement::new("theme", SemanticComponent::RadioGroup)
        .with_props(
            SemanticProps::new()
                .label("Theme")
                .orientation(Orientation::Vertical)
                .on_change("setTheme"),
        )
        .child(
            SemanticElement::new("light", SemanticComponent::Radio)
                .with_props(SemanticProps::new().text_value("Light").value("light")),
        )
        .child(
            SemanticElement::new("dark", SemanticComponent::Radio).with_props(
                SemanticProps::new()
                    .text_value("Dark")
                    .value("dark")
                    .selected(true),
            ),
        );

    let native = SemanticMapper::new().map(&aria).unwrap();

    assert_eq!(native.role, NativeRole::RadioGroup);
    assert_eq!(native.props.label.as_deref(), Some("Theme"));
    assert_eq!(native.props.action.as_deref(), Some("setTheme"));
    assert_eq!(native.props.orientation, Some(Orientation::Vertical));
    assert_eq!(native.children.len(), 2);
    assert_eq!(native.children[1].role, NativeRole::Radio);
    assert_eq!(native.children[1].props.label.as_deref(), Some("Dark"));
    assert_eq!(native.children[1].props.value.as_deref(), Some("dark"));
    assert!(native.children[1].props.selected);
    assert_eq!(native.children[1].props.checked, Some(true));
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, native.role),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, native.children[1].role),
        "a3s_gui::HeadlessNode"
    );
}

#[test]
fn folds_tabs_tablist_and_panels_into_semantic_tab_items() {
    let aria = SemanticElement::new("settings", SemanticComponent::Tabs)
        .with_props(SemanticProps::new().on_selection_change("setTab"))
        .child(
            SemanticElement::new("settings-tabs", SemanticComponent::TabList)
                .child(
                    SemanticElement::new("profile-tab", SemanticComponent::Tab)
                        .with_props(SemanticProps::new().text_value("Profile").selected(true)),
                )
                .child(
                    SemanticElement::new("billing-tab", SemanticComponent::Tab)
                        .with_props(SemanticProps::new().text_value("Billing")),
                ),
        )
        .child(
            SemanticElement::new("profile-panel", SemanticComponent::TabPanel)
                .child(SemanticElement::text("profile-title", "Profile settings")),
        )
        .child(
            SemanticElement::new("billing-panel", SemanticComponent::TabPanel)
                .child(SemanticElement::text("billing-title", "Billing settings")),
        );

    let native = SemanticMapper::new().map(&aria).unwrap();

    assert_eq!(native.role, NativeRole::Tabs);
    assert_eq!(
        native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setTab")
    );
    assert_eq!(native.children.len(), 2);
    assert_eq!(native.children[0].role, NativeRole::Tab);
    assert_eq!(native.children[0].props.label.as_deref(), Some("Profile"));
    assert!(native.children[0].props.selected);
    assert_eq!(native.children[0].children.len(), 1);
    assert_eq!(native.children[0].children[0].role, NativeRole::TabPanel);
    assert_eq!(
        native.children[0].children[0].children[0].role,
        NativeRole::Text
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, native.role),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, native.children[0].role),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, native.role),
        "a3s_gui::HeadlessNode"
    );
}

#[test]
fn maps_menu_and_menu_items_to_native_menu_roles() {
    let aria = SemanticElement::new("file-menu", SemanticComponent::Menu)
        .child(
            SemanticElement::new("open", SemanticComponent::MenuItem).with_props(
                SemanticProps::new()
                    .text_value("Open")
                    .value("open")
                    .on_press("openFile"),
            ),
        )
        .child(
            SemanticElement::new("recent", SemanticComponent::MenuItem)
                .with_props(SemanticProps::new().text_value("Recent")),
        );

    let native = SemanticMapper::new().map(&aria).unwrap();

    assert_eq!(native.role, NativeRole::Menu);
    assert_eq!(native.children.len(), 2);
    assert_eq!(native.children[0].role, NativeRole::MenuItem);
    assert_eq!(native.children[0].props.label.as_deref(), Some("Open"));
    assert_eq!(native.children[0].props.value.as_deref(), Some("open"));
    assert_eq!(native.children[0].props.action.as_deref(), Some("openFile"));
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, native.role),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, native.children[0].role),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, native.children[0].role),
        "a3s_gui::HeadlessNode"
    );
}

#[test]
fn accepts_web_compatible_props_and_normalizes_primary_action() {
    let aria = SemanticElement::new("save", SemanticComponent::Button).with_props(
        SemanticProps::new()
            .label("Save")
            .class_name("primary")
            .style("backgroundColor", "rebeccapurple")
            .dom_attribute("aria-label", "Save document")
            .dom_attribute("data-testid", "save-button")
            .on_click("saveDocument"),
    );

    let native = SemanticMapper::new().map(&aria).unwrap();

    assert_eq!(native.props.action.as_deref(), Some("saveDocument"));
    assert_eq!(native.props.web.class_name.as_deref(), Some("primary"));
    assert_eq!(
        native
            .props
            .web
            .style
            .get("backgroundColor")
            .map(String::as_str),
        Some("rebeccapurple")
    );
    assert_eq!(
        native.props.metadata.get("aria-label").map(String::as_str),
        Some("Save document")
    );
    assert_eq!(
        native.props.metadata.get("data-testid").map(String::as_str),
        Some("save-button")
    );
}

#[test]
fn renderer_updates_native_node_without_remounting_same_key_and_role() {
    let first =
        NativeElement::new("save", NativeRole::Button).with_props(NativeProps::new().label("Save"));
    let second = NativeElement::new("save", NativeRole::Button)
        .with_props(NativeProps::new().label("Saved"));
    let mut renderer = Renderer::new();
    let mut host = HeadlessHost::default();

    let first_id = renderer.render(&first, &mut host).unwrap();
    host.clear_operations();
    let second_id = renderer.render(&second, &mut host).unwrap();

    assert_eq!(first_id, second_id);
    assert!(host
        .operations()
        .iter()
        .any(|operation| matches!(operation, HostOperation::Update { id, .. } if *id == first_id)));
    assert!(!host
        .operations()
        .iter()
        .any(|operation| matches!(operation, HostOperation::Create { .. })));
}

#[test]
fn semantic_roles_share_one_nonvisual_planning_class() {
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Button),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::TextField),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Select),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Separator),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Separator),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Separator),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Heading),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Navigation),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Main),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Image),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Media),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Canvas),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Table),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::TableCell),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::TableCaption),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::DisclosureSummary),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::FigureCaption),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::DescriptionDetails),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::FieldSet),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Meter),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::OptionGroup),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Link),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::ImageMapArea),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::ImageMap),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::DocumentTitle),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::ResourceLink),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Slot),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::RubyBase),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::RubyTextContainer),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::RubyText),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Abbreviation),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::MarkedText),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Time),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Code),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::InlineQuote),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(
            NativeBackendKind::Headless,
            NativeRole::BidirectionalOverride
        ),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Paragraph),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::PreformattedText),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(
            NativeBackendKind::Headless,
            NativeRole::WordBreakOpportunity
        ),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::FrameSet),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::Math),
        "a3s_gui::HeadlessNode"
    );
    assert_eq!(
        native_widget_name(NativeBackendKind::Headless, NativeRole::SelectedContent),
        "a3s_gui::HeadlessNode"
    );
}
