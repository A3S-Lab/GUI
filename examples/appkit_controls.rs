#[cfg(target_os = "macos")]
mod appkit_controls {
    use a3s_gui::{ActionInvocation, AppKitRuntimeApp, UiFrame};
    use serde_json::{json, Value};

    #[derive(Debug, Clone, PartialEq)]
    struct ControlsState {
        name: String,
        notifications: bool,
        volume: f64,
        theme: String,
        tab: String,
        saves: u32,
    }

    impl Default for ControlsState {
        fn default() -> Self {
            Self {
                name: "Ada".to_string(),
                notifications: true,
                volume: 35.0,
                theme: "Compact".to_string(),
                tab: "Profile".to_string(),
                saves: 0,
            }
        }
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut app =
            AppKitRuntimeApp::appkit(ControlsState::default(), controls_frame, controls_reduce)?;
        app.render()?;
        app.run_appkit()?;
        println!("controls smoke closed with state: {:?}", app.state());
        Ok(())
    }

    fn controls_frame(state: &ControlsState) -> a3s_gui::GuiResult<UiFrame> {
        serde_json::from_value(json!({
        "frameId": "appkit-controls",
        "window": {
            "title": "A3S AppKit Controls",
            "width": 520,
            "height": 520,
            "minWidth": 420,
            "minHeight": 420
        },
        "actions": [
            {"id": "setName", "label": "Set name"},
            {"id": "setNotifications", "label": "Set notifications"},
            {"id": "setVolume", "label": "Set volume"},
            {"id": "setTheme", "label": "Set theme"},
            {"id": "setTab", "label": "Set tab"},
            {"id": "saveProfile", "label": "Save profile"}
        ],
        "root": {
            "kind": "element",
            "key": "controls",
            "tag": "Toolbar",
            "props": {
                "orientation": "vertical",
                "style": {"width": 480, "height": 480}
            },
            "children": [
                text("summary", format!(
                    "Name: {} | Notifications: {} | Volume: {:.0} | Theme: {} | Tab: {} | Saves: {}",
                    state.name,
                    state.notifications,
                    state.volume,
                    state.theme,
                    state.tab,
                    state.saves
                )),
                text("name-label", "Name"),
                {
                    "kind": "element",
                    "key": "name",
                    "tag": "input",
                    "props": {
                        "inputType": "text",
                        "value": state.name,
                        "placeholder": "Type a name",
                        "events": {"onInput": "setName", "onChange": "setName"}
                    }
                },
                text("notifications-label", "Notifications"),
                {
                    "kind": "element",
                    "key": "notifications",
                    "tag": "Switch",
                    "props": {
                        "isChecked": state.notifications,
                        "events": {"onChange": "setNotifications"}
                    }
                },
                text("volume-label", format!("Volume {:.0}", state.volume)),
                {
                    "kind": "element",
                    "key": "volume",
                    "tag": "Slider",
                    "props": {
                        "label": "Volume",
                        "minValue": 0,
                        "maxValue": 100,
                        "valueNumber": state.volume,
                        "stepValue": 5,
                        "events": {"onChange": "setVolume"}
                    }
                },
                text("theme-label", "Theme"),
                {
                    "kind": "element",
                    "key": "theme",
                    "tag": "Select",
                    "props": {
                        "label": "Theme",
                        "value": state.theme,
                        "events": {"onSelectionChange": "setTheme"}
                    },
                    "children": [
                        {
                            "kind": "element",
                            "key": "theme-compact",
                            "tag": "ListBoxItem",
                            "props": {
                                "value": "Compact",
                                "textValue": "Compact",
                                "isSelected": state.theme == "Compact"
                            }
                        },
                        {
                            "kind": "element",
                            "key": "theme-comfortable",
                            "tag": "ListBoxItem",
                            "props": {
                                "value": "Comfortable",
                                "textValue": "Comfortable",
                                "isSelected": state.theme == "Comfortable"
                            }
                        }
                    ]
                },
                {
                    "kind": "element",
                    "key": "tabs",
                    "tag": "Tabs",
                    "props": {
                        "label": "Sections",
                        "events": {"onSelectionChange": "setTab"}
                    },
                    "children": [
                        {
                            "kind": "element",
                            "key": "tab-list",
                            "tag": "TabList",
                            "children": [
                                {
                                    "kind": "element",
                                    "key": "profile-tab",
                                    "tag": "Tab",
                                    "props": {
                                        "textValue": "Profile",
                                        "isSelected": state.tab == "Profile"
                                    }
                                },
                                {
                                    "kind": "element",
                                    "key": "activity-tab",
                                    "tag": "Tab",
                                    "props": {
                                        "textValue": "Activity",
                                        "isSelected": state.tab == "Activity"
                                    }
                                }
                            ]
                        },
                        {
                            "kind": "element",
                            "key": "profile-panel",
                            "tag": "TabPanel",
                            "children": [text("profile-copy", "Edit the profile controls above.")]
                        },
                        {
                            "kind": "element",
                            "key": "activity-panel",
                            "tag": "TabPanel",
                            "children": [text("activity-copy", "Use this panel to verify tab selection events.")]
                        }
                    ]
                },
                {
                    "kind": "element",
                    "key": "save",
                    "tag": "Button",
                    "props": {
                        "label": "Save",
                        "events": {"onPress": "saveProfile"}
                    }
                },
                text("close-note", "Close the window to stop the smoke app.")
            ]
        }
    }))
    .map_err(|error| a3s_gui::GuiError::invalid_tree(format!("invalid controls frame: {error}")))
    }

    fn text(key: &str, label: impl Into<String>) -> Value {
        json!({
            "kind": "element",
            "key": key,
            "tag": "Text",
            "props": {"label": label.into()}
        })
    }

    fn controls_reduce(
        state: &mut ControlsState,
        invocation: &ActionInvocation,
    ) -> a3s_gui::GuiResult<()> {
        match invocation.action.as_str() {
            "setName" => {
                state.name = invocation.value.clone().unwrap_or_default();
            }
            "setNotifications" => {
                state.notifications = invocation.value.as_deref() == Some("true");
            }
            "setVolume" => {
                state.volume = invocation
                    .value
                    .as_deref()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|error| a3s_gui::GuiError::host(format!("invalid volume: {error}")))?;
            }
            "setTheme" => {
                state.theme = invocation
                    .value
                    .clone()
                    .unwrap_or_else(|| "Compact".to_string());
            }
            "setTab" => {
                state.tab = invocation
                    .value
                    .clone()
                    .unwrap_or_else(|| "Profile".to_string());
            }
            "saveProfile" => {
                state.saves += 1;
            }
            other => {
                return Err(a3s_gui::GuiError::host(format!(
                    "unexpected action {other}"
                )));
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    appkit_controls::run()
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("appkit_controls requires macOS.");
}
