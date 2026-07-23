use super::*;
use crate::backend::CommandExecutingHost;
use crate::compiler::CompiledRsxNode;
use crate::geometry::Orientation;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{PlatformAdapter, WinUiAdapter};
use crate::runtime::GuiRuntime;
use crate::style::{OverflowMode, StyleLength};

#[test]
fn winui_tree_items_use_list_view_item_contract() {
    assert_eq!(
        WinUiWidgetKind::from_widget_kind(NativeWidgetKind::TreeItem),
        WinUiWidgetKind::ListViewItem
    );
}

#[test]
fn winui_max_length_value_maps_protocol_limits_to_winui_contract() {
    assert_eq!(winui_max_length_value(None), 0);
    assert_eq!(winui_max_length_value(Some(64)), 64);
    assert_eq!(winui_max_length_value(Some(i32::MAX as u32)), i32::MAX);
    assert_eq!(winui_max_length_value(Some(u32::MAX)), i32::MAX);
}

#[test]
fn winui_truncate_to_max_length_limits_unicode_scalar_values() {
    assert_eq!(winui_truncate_to_max_length("abcdef", Some(3)), "abc");
    assert_eq!(winui_truncate_to_max_length("aé日b", Some(3)), "aé日");
    assert_eq!(winui_truncate_to_max_length("abc", None), "abc");
    assert_eq!(winui_truncate_to_max_length("abc", Some(0)), "");
}

#[test]
fn winui_text_input_hints_disable_prediction_for_structured_fields() {
    let config = WinUiAdapter
        .blueprint(
            &NativeElement::new("field", NativeRole::TextField).with_props(
                NativeProps::new()
                    .input_type("email")
                    .autocomplete("on")
                    .spell_check(Some(true)),
            ),
        )
        .config();

    assert_eq!(
        winui_text_input_hints(&config),
        WinUiTextInputHints {
            spellcheck_enabled: Some(true),
            text_prediction_enabled: Some(false),
            prevent_keyboard_display_on_programmatic_focus: false,
            color_font_enabled: true,
        }
    );
}

#[test]
fn winui_text_input_hints_track_web_completion_and_keyboard_hints() {
    let config = WinUiAdapter
        .blueprint(
            &NativeElement::new("field", NativeRole::TextField).with_props(
                NativeProps::new()
                    .autocomplete("on")
                    .auto_correct("off")
                    .input_mode("none"),
            ),
        )
        .config();

    assert_eq!(
        winui_text_input_hints(&config),
        WinUiTextInputHints {
            spellcheck_enabled: Some(false),
            text_prediction_enabled: Some(false),
            prevent_keyboard_display_on_programmatic_focus: true,
            color_font_enabled: false,
        }
    );

    let config = WinUiAdapter
        .blueprint(
            &NativeElement::new("field", NativeRole::TextField)
                .with_props(NativeProps::new().autocomplete("on")),
        )
        .config();

    assert_eq!(
        winui_text_input_hints(&config).text_prediction_enabled,
        Some(true)
    );
}

#[test]
fn winui_widget_driver_reparents_children_and_removes_subtrees() {
    let mut driver = WinUiWidgetDriver::default();
    let root = HostNodeId::new(1);
    let child = HostNodeId::new(2);
    let grandchild = HostNodeId::new(3);
    let second = HostNodeId::new(4);
    let container = WinUiAdapter.blueprint(&NativeElement::new("container", NativeRole::View));
    let button = WinUiAdapter.blueprint(&NativeElement::new("button", NativeRole::Button));

    driver.create_widget(root, &container).unwrap();
    driver.create_widget(child, &container).unwrap();
    driver.create_widget(grandchild, &button).unwrap();
    driver.create_widget(second, &container).unwrap();
    driver.insert_child(root, child, 0).unwrap();
    driver.insert_child(child, grandchild, 0).unwrap();
    driver.insert_child(second, child, 0).unwrap();

    assert!(driver.object(root).unwrap().children.is_empty());
    assert_eq!(driver.object(second).unwrap().children, vec![child]);
    let error = driver.insert_child(child, second, 0).unwrap_err();
    assert!(error.to_string().contains("would create a cycle"));

    driver.set_root_widget(second).unwrap();
    driver.remove_widget(second).unwrap();

    assert!(driver.root().is_none());
    assert!(driver.object(root).is_some());
    assert!(driver.object(second).is_none());
    assert!(driver.object(child).is_none());
    assert!(driver.object(grandchild).is_none());
    assert_eq!(driver.objects().len(), 1);
}

#[test]
fn winui_handle_adapter_clears_previous_parent_on_reparent() {
    let mut driver = WinUiHandleDriver::default();
    let first = HostNodeId::new(1);
    let second = HostNodeId::new(2);
    let child = HostNodeId::new(3);
    let container = WinUiAdapter.blueprint(&NativeElement::new("container", NativeRole::View));
    let button = WinUiAdapter.blueprint(&NativeElement::new("button", NativeRole::Button));

    driver.create_widget(first, &container).unwrap();
    driver.create_widget(second, &container).unwrap();
    driver.create_widget(child, &button).unwrap();
    driver.insert_child(first, child, 0).unwrap();
    driver.insert_child(second, child, 0).unwrap();

    assert_eq!(driver.children(first), Some([].as_slice()));
    assert_eq!(driver.children(second), Some([child].as_slice()));
    assert!(driver.handle(first).unwrap().state().children.is_empty());
    assert_eq!(driver.handle(second).unwrap().state().children, vec![child]);
}

#[test]
fn winui_executor_consumes_compiled_semantic_ui_commands() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "email",
              "tag": "TextField",
              "props": {"isRequired": true, "isInvalid": true},
              "children": [
                {"kind": "element", "key": "label", "tag": "Label", "children": [
                  {"kind": "text", "key": "label-text", "value": "Email"}
                ]},
                {"kind": "element", "key": "input", "tag": "Input", "props": {
                  "placeholder": "you@example.com",
                  "events": {"onChange": "setEmail"}
                }}
              ]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();

    assert_eq!(object.kind, WinUiWidgetKind::TextBox);
    assert_eq!(object.label.as_deref(), Some("Email"));
    assert_eq!(object.action.as_deref(), Some("setEmail"));
    assert_eq!(
        object.control_state.placeholder.as_deref(),
        Some("you@example.com")
    );
    assert!(object.control_state.required);
    assert!(object.control_state.invalid);
}

#[test]
fn winui_executor_consumes_compiled_semantic_ui_toolbar_commands() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "tools",
              "tag": "Toolbar",
              "props": {"aria-orientation": "horizontal"},
              "children": [
                {
                  "kind": "element",
                  "key": "save",
                  "tag": "Button",
                  "props": {"events": {"onPress": "saveDocument"}},
                  "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
                }
              ]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();
    let child = runtime
        .host()
        .executor()
        .driver()
        .object(object.children[0])
        .unwrap();

    assert_eq!(object.kind, WinUiWidgetKind::CommandBar);
    assert_eq!(
        object.control_state.orientation,
        Some(crate::geometry::Orientation::Horizontal)
    );
    assert_eq!(child.kind, WinUiWidgetKind::Button);
    assert_eq!(child.action.as_deref(), Some("saveDocument"));
}

#[test]
fn winui_executor_consumes_compiled_semantic_ui_dialog_commands() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "preferences",
              "tag": "Dialog",
              "props": {"aria-label": "Preferences"},
              "children": [
                {
                  "kind": "element",
                  "key": "close",
                  "tag": "Button",
                  "props": {"events": {"onPress": "closePreferences"}},
                  "children": [{"kind": "text", "key": "close-text", "value": "Close"}]
                }
              ]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();
    let child = runtime
        .host()
        .executor()
        .driver()
        .object(object.children[0])
        .unwrap();

    assert_eq!(object.kind, WinUiWidgetKind::ContentDialog);
    assert_eq!(object.label.as_deref(), Some("Preferences"));
    assert_eq!(child.kind, WinUiWidgetKind::Button);
    assert_eq!(child.action.as_deref(), Some("closePreferences"));
}

#[test]
fn winui_executor_consumes_compiled_semantic_ui_popover_commands() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "actions-popover",
              "tag": "Popover",
              "props": {"aria-label": "Actions"},
              "children": [
                {
                  "kind": "element",
                  "key": "archive",
                  "tag": "Button",
                  "props": {"events": {"onPress": "archiveItem"}},
                  "children": [{"kind": "text", "key": "archive-text", "value": "Archive"}]
                }
              ]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();
    let child = runtime
        .host()
        .executor()
        .driver()
        .object(object.children[0])
        .unwrap();

    assert_eq!(object.kind, WinUiWidgetKind::ToolTip);
    assert_eq!(object.label.as_deref(), Some("Actions"));
    assert_eq!(child.kind, WinUiWidgetKind::Button);
    assert_eq!(child.action.as_deref(), Some("archiveItem"));
}

#[test]
fn winui_executor_consumes_compiled_semantic_ui_menu_commands() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "file-menu",
              "tag": "Menu",
              "children": [
                {
                  "kind": "element",
                  "key": "open",
                  "tag": "MenuItem",
                  "props": {"value": "open", "events": {"onPress": "openFile"}},
                  "children": [{"kind": "text", "key": "open-text", "value": "Open"}]
                }
              ]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();
    let item = runtime
        .host()
        .executor()
        .driver()
        .object(object.children[0])
        .unwrap();

    assert_eq!(object.kind, WinUiWidgetKind::MenuPanel);
    assert_eq!(item.kind, WinUiWidgetKind::MenuItemButton);
    assert_eq!(item.label.as_deref(), Some("Open"));
    assert_eq!(item.value.as_deref(), Some("open"));
    assert_eq!(item.action.as_deref(), Some("openFile"));
}

#[test]
fn winui_handle_adapter_stores_thread_bound_native_handles() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "email",
              "tag": "TextField",
              "props": {"isRequired": true},
              "children": [
                {"kind": "element", "key": "label", "tag": "Label", "children": [
                  {"kind": "text", "key": "label-text", "value": "Email"}
                ]},
                {"kind": "element", "key": "input", "tag": "Input", "props": {
                  "placeholder": "you@example.com",
                  "events": {"onChange": "setEmail"}
                }}
              ]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(WinUiAdapter, WinUiHandleCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let handle = runtime.host().executor().driver().handle(root_id).unwrap();
    let state = handle.state();

    assert_eq!(state.kind, WinUiWidgetKind::TextBox);
    assert_eq!(state.label.as_deref(), Some("Email"));
    assert_eq!(state.action.as_deref(), Some("setEmail"));
    assert_eq!(
        state.control_state.placeholder.as_deref(),
        Some("you@example.com")
    );
    assert!(state.control_state.required);
    assert!(state.config.required);
    assert_eq!(state.config.placeholder.as_deref(), Some("you@example.com"));
    assert!(state
        .applied_setters
        .contains(&NativeWidgetSetter::SetRequired(true)));
    assert!(state
        .applied_setters
        .contains(&NativeWidgetSetter::SetPlaceholder(Some(
            "you@example.com".to_string()
        ))));
}

#[test]
fn winui_handle_adapter_clears_removed_text_max_length_on_rerender() {
    let mut driver = WinUiHandleDriver::default();
    let id = HostNodeId::new(1);
    let limited = WinUiAdapter.blueprint(
        &NativeElement::new("notes", NativeRole::TextField)
            .with_props(NativeProps::new().label("Notes").max_length(Some(8))),
    );
    let unlimited = WinUiAdapter.blueprint(
        &NativeElement::new("notes", NativeRole::TextField)
            .with_props(NativeProps::new().label("Notes")),
    );

    driver.create_widget(id, &limited).unwrap();
    let initial_setter_count = {
        let handle = driver.handle(id).unwrap();
        let state = handle.state();
        assert_eq!(state.config.max_length, Some(8));
        assert!(state
            .applied_setters
            .contains(&NativeWidgetSetter::SetMaxLength(Some(8))));
        state.applied_setters.len()
    };

    driver.update_widget(id, &unlimited).unwrap();

    let handle = driver.handle(id).unwrap();
    let state = handle.state();
    let update_setters = &state.applied_setters[initial_setter_count..];

    assert_eq!(state.config.max_length, None);
    assert_eq!(update_setters, [NativeWidgetSetter::SetMaxLength(None)]);
}

#[test]
fn winui_scroll_handle_adapter_applies_rerender_style_setters() {
    let first: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "shell",
              "tag": "Toolbar",
              "props": {
                "orientation": "vertical",
                "style": {"overflowY": "auto", "gap": 8, "inlineSize": 320}
              },
              "children": [{"kind": "text", "key": "summary", "value": "Ready"}]
            }
            "#,
    )
    .unwrap();
    let second: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "shell",
              "tag": "Toolbar",
              "props": {
                "orientation": "horizontal",
                "style": {"overflowX": "scroll", "overflowY": "auto", "gap": 12, "inlineSize": 420}
              },
              "children": [{"kind": "text", "key": "summary", "value": "Ready"}]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(WinUiAdapter, WinUiHandleCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&first).unwrap();
    let initial_setter_count = {
        let handle = runtime.host().executor().driver().handle(root_id).unwrap();
        let state = handle.state();
        assert_eq!(state.kind, WinUiWidgetKind::ScrollViewer);
        state.applied_setters.len()
    };

    runtime.render_compiled(&second).unwrap();
    let handle = runtime.host().executor().driver().handle(root_id).unwrap();
    let state = handle.state();
    let update_setters = &state.applied_setters[initial_setter_count..];

    assert_eq!(state.kind, WinUiWidgetKind::ScrollViewer);
    assert!(
        update_setters.contains(&NativeWidgetSetter::SetOrientation(Some(
            Orientation::Horizontal
        )))
    );
    assert!(update_setters.iter().any(|setter| matches!(
        setter,
        NativeWidgetSetter::SetPortableStyle(style)
            if style.overflow_x == Some(OverflowMode::Scroll)
                && style.gap.as_ref().and_then(StyleLength::points) == Some(12.0)
    )));
}
