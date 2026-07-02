use super::*;
use std::collections::BTreeMap;

use crate::accessibility::AccessibilityRole;
use crate::geometry::Orientation;
use crate::host::HostNodeId;
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
fn widget_config_diff_reports_changed_native_setters() {
    let first = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Volume")
            .value("50")
            .range(Some(0.0), Some(100.0), Some(50.0))
            .step(Some(5.0))
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
        NativeWidgetSetter::SetCurrent(Some(50.0)),
        NativeWidgetSetter::SetStep(Some(5.0)),
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
                    .step(Some(10.0)),
            ),
        )
        .config();
    let mut replayed = before.clone();

    apply_widget_setters(&mut replayed, &before.diff(&after).setters());

    assert_eq!(replayed.label.as_deref(), Some("Muted"));
    assert!(!replayed.enabled);
    assert_eq!(replayed.current, Some(0.0));
    assert_eq!(replayed.step, Some(10.0));
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
