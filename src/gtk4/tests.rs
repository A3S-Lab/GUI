use super::*;
use crate::backend::CommandExecutingHost;
use crate::compiler::CompiledRsxNode;
use crate::geometry::Orientation;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformAdapter};
use crate::runtime::GuiRuntime;
use crate::style::{OverflowMode, StyleLength};

#[test]
fn gtk4_widget_driver_reparents_children_and_removes_subtrees() {
    let mut driver = Gtk4WidgetDriver::default();
    let root = HostNodeId::new(1);
    let child = HostNodeId::new(2);
    let grandchild = HostNodeId::new(3);
    let second = HostNodeId::new(4);
    let container = Gtk4Adapter.blueprint(&NativeElement::new("container", NativeRole::View));
    let button = Gtk4Adapter.blueprint(&NativeElement::new("button", NativeRole::Button));

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
fn gtk4_handle_adapter_clears_previous_parent_on_reparent() {
    let mut driver = Gtk4HandleDriver::default();
    let first = HostNodeId::new(1);
    let second = HostNodeId::new(2);
    let child = HostNodeId::new(3);
    let container = Gtk4Adapter.blueprint(&NativeElement::new("container", NativeRole::View));
    let button = Gtk4Adapter.blueprint(&NativeElement::new("button", NativeRole::Button));

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
fn gtk4_executor_consumes_compiled_semantic_ui_commands() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"isDisabled": true, "events": {"onPress": "saveProfile"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();

    assert_eq!(object.kind, Gtk4WidgetKind::Button);
    assert_eq!(object.label.as_deref(), Some("Save"));
    assert_eq!(object.action.as_deref(), Some("saveProfile"));
    assert!(object.control_state.disabled);
}

#[test]
fn gtk4_executor_consumes_compiled_semantic_ui_toolbar_commands() {
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
    let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();
    let child = runtime
        .host()
        .executor()
        .driver()
        .object(object.children[0])
        .unwrap();

    assert_eq!(object.kind, Gtk4WidgetKind::ToolbarBox);
    assert_eq!(
        object.control_state.orientation,
        Some(crate::geometry::Orientation::Horizontal)
    );
    assert_eq!(child.kind, Gtk4WidgetKind::Button);
    assert_eq!(child.action.as_deref(), Some("saveDocument"));
}

#[test]
fn gtk4_executor_consumes_compiled_semantic_ui_dialog_commands() {
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
    let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();
    let child = runtime
        .host()
        .executor()
        .driver()
        .object(object.children[0])
        .unwrap();

    assert_eq!(object.kind, Gtk4WidgetKind::Dialog);
    assert_eq!(object.label.as_deref(), Some("Preferences"));
    assert_eq!(child.kind, Gtk4WidgetKind::Button);
    assert_eq!(child.action.as_deref(), Some("closePreferences"));
}

#[test]
fn gtk4_executor_consumes_compiled_semantic_ui_popover_commands() {
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
    let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();
    let child = runtime
        .host()
        .executor()
        .driver()
        .object(object.children[0])
        .unwrap();

    assert_eq!(object.kind, Gtk4WidgetKind::Popover);
    assert_eq!(object.label.as_deref(), Some("Actions"));
    assert_eq!(child.kind, Gtk4WidgetKind::Button);
    assert_eq!(child.action.as_deref(), Some("archiveItem"));
}

#[test]
fn gtk4_executor_consumes_compiled_semantic_ui_menu_commands() {
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
    let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let object = runtime.host().executor().driver().object(root_id).unwrap();
    let item = runtime
        .host()
        .executor()
        .driver()
        .object(object.children[0])
        .unwrap();

    assert_eq!(object.kind, Gtk4WidgetKind::Menu);
    assert_eq!(item.kind, Gtk4WidgetKind::MenuItem);
    assert_eq!(item.label.as_deref(), Some("Open"));
    assert_eq!(item.value.as_deref(), Some("open"));
    assert_eq!(item.action.as_deref(), Some("openFile"));
}

#[test]
fn gtk4_handle_adapter_stores_thread_bound_native_handles() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"isDisabled": true, "events": {"onPress": "saveProfile"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4HandleCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let handle = runtime.host().executor().driver().handle(root_id).unwrap();
    let state = handle.state();

    assert_eq!(state.kind, Gtk4WidgetKind::Button);
    assert_eq!(state.label.as_deref(), Some("Save"));
    assert_eq!(state.action.as_deref(), Some("saveProfile"));
    assert!(state.control_state.disabled);
    assert!(!state.config.enabled);
    assert!(state
        .applied_setters
        .contains(&NativeWidgetSetter::SetEnabled(false)));
    assert!(state
        .applied_setters
        .contains(&NativeWidgetSetter::SetLabel(Some("Save".to_string()))));
}

#[test]
fn gtk4_handle_adapter_applies_update_setters() {
    let first: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"events": {"onPress": "saveProfile"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
            }
            "#,
    )
    .unwrap();
    let second: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"isDisabled": true, "events": {"onPress": "saveProfile"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Saved"}]
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4HandleCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&first).unwrap();
    runtime.render_compiled(&second).unwrap();
    let handle = runtime.host().executor().driver().handle(root_id).unwrap();
    let state = handle.state();

    assert_eq!(state.label.as_deref(), Some("Saved"));
    assert!(!state.config.enabled);
    assert!(state
        .applied_setters
        .contains(&NativeWidgetSetter::SetLabel(Some("Saved".to_string()))));
    assert!(state
        .applied_setters
        .contains(&NativeWidgetSetter::SetEnabled(false)));
}

#[test]
fn gtk4_handle_adapter_clears_removed_textarea_sizing_on_rerender() {
    let mut driver = Gtk4HandleDriver::default();
    let id = HostNodeId::new(1);
    let limited = Gtk4Adapter.blueprint(
        &NativeElement::new("notes", NativeRole::TextField).with_props(
            NativeProps::new()
                .metadata(crate::html::HTML_TAG_METADATA_KEY, "textarea")
                .rows(Some(6))
                .cols(Some(48)),
        ),
    );
    let unlimited = Gtk4Adapter.blueprint(
        &NativeElement::new("notes", NativeRole::TextField).with_props(
            NativeProps::new().metadata(crate::html::HTML_TAG_METADATA_KEY, "textarea"),
        ),
    );

    driver.create_widget(id, &limited).unwrap();
    let initial_setter_count = {
        let handle = driver.handle(id).unwrap();
        let state = handle.state();
        assert_eq!(state.config.rows, Some(6));
        assert_eq!(state.config.cols, Some(48));
        state.applied_setters.len()
    };

    driver.update_widget(id, &unlimited).unwrap();

    let handle = driver.handle(id).unwrap();
    let state = handle.state();
    let update_setters = &state.applied_setters[initial_setter_count..];

    assert_eq!(state.config.rows, None);
    assert_eq!(state.config.cols, None);
    assert!(update_setters.contains(&NativeWidgetSetter::SetRows(None)));
    assert!(update_setters.contains(&NativeWidgetSetter::SetCols(None)));
}

#[test]
fn gtk4_scroll_handle_adapter_applies_rerender_style_setters() {
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
    let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4HandleCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&first).unwrap();
    let initial_setter_count = {
        let handle = runtime.host().executor().driver().handle(root_id).unwrap();
        let state = handle.state();
        assert_eq!(state.kind, Gtk4WidgetKind::ScrolledWindow);
        state.applied_setters.len()
    };

    runtime.render_compiled(&second).unwrap();
    let handle = runtime.host().executor().driver().handle(root_id).unwrap();
    let state = handle.state();
    let update_setters = &state.applied_setters[initial_setter_count..];

    assert_eq!(state.kind, Gtk4WidgetKind::ScrolledWindow);
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
