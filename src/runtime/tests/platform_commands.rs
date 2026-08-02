use super::super::*;
use crate::platform::{HeadlessAdapter, PlatformCommand, PlatformPlanningHost};

#[test]
fn runtime_renders_compiled_rsx_to_native_command_stream() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "form",
              "tag": "form",
              "props": {"className": "profile-form"},
              "children": [
                {
                  "kind": "element",
                  "key": "email",
                  "tag": "TextField",
                  "children": [
                    {"kind": "element", "key": "label", "tag": "Label", "children": [
                      {"kind": "text", "key": "label-text", "value": "Email"}
                    ]},
                    {"kind": "element", "key": "input", "tag": "Input", "props": {
                      "placeholder": "you@example.com",
                      "events": {"onChange": "setEmail"}
                    }}
                  ]
                },
                {
                  "kind": "element",
                  "key": "save",
                  "tag": "Button",
                  "props": {"events": {"onPress": "saveProfile"}},
                  "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
                }
              ]
            }
            "#,
    )
    .unwrap();
    let host = PlatformPlanningHost::new(HeadlessAdapter);
    let mut runtime = GuiRuntime::new(host);

    runtime.render_compiled(&compiled).unwrap();

    let commands = runtime.host().commands();
    assert!(commands.iter().any(|command| matches!(
        command,
        PlatformCommand::Create {
            blueprint,
            ..
        } if blueprint.widget_class == "a3s_gui::HeadlessNode"
            && blueprint.label.as_deref() == Some("Email")
    )));
    assert!(commands.iter().any(|command| matches!(
        command,
        PlatformCommand::Create {
            blueprint,
            ..
        } if blueprint.widget_class == "a3s_gui::HeadlessNode"
            && blueprint.action.as_deref() == Some("saveProfile")
    )));
}
