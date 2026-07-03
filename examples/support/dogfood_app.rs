use a3s_gui::{ActionInvocation, GuiError, GuiResult, UiFrame};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct DogfoodState {
    pub title: String,
    pub notes: String,
    pub priority: String,
    pub stage: String,
    pub completed: bool,
    pub estimate: f64,
    pub saves: u32,
    pub last_event: String,
}

impl Default for DogfoodState {
    fn default() -> Self {
        Self {
            title: "Ship native GUI dogfood".to_string(),
            notes: "Finish the native task editor pass and verify the reducer loop.".to_string(),
            priority: "High".to_string(),
            stage: "Build".to_string(),
            completed: false,
            estimate: 6.0,
            saves: 0,
            last_event: "Ready".to_string(),
        }
    }
}

pub fn dogfood_frame(state: &DogfoodState, frame_id: &str, title: &str) -> GuiResult<UiFrame> {
    serde_json::from_value(json!({
        "frameId": frame_id,
        "window": {
            "title": title,
            "width": 720,
            "height": 560,
            "minWidth": 540,
            "minHeight": 420
        },
        "actions": [
            {"id": "updateTitle", "label": "Update title"},
            {"id": "updateNotes", "label": "Update notes"},
            {"id": "setPriority", "label": "Set priority"},
            {"id": "setStage", "label": "Set stage"},
            {"id": "setCompleted", "label": "Set completed"},
            {"id": "setEstimate", "label": "Set estimate"},
            {"id": "saveWork", "label": "Save work"},
            {"id": "handleShortcut", "label": "Handle shortcut"}
        ],
        "root": {
            "kind": "element",
            "key": "dogfood-root",
            "tag": "Toolbar",
            "props": {
                "orientation": "vertical",
                "events": {"onKeyDown": "handleShortcut"},
                "style": {
                    "writingMode": "horizontal-tb",
                    "inlineSize": 680,
                    "blockSize": 500,
                    "minInlineSize": 480,
                    "minBlockSize": 360,
                    "gap": 10
                }
            },
            "children": [
                text("summary", format!(
                    "{} | {} | {:.0}h | {} | saves {}",
                    state.priority,
                    state.stage,
                    state.estimate,
                    if state.completed { "done" } else { "active" },
                    state.saves
                )),
                text("title-label", "Task"),
                {
                    "kind": "element",
                    "key": "title",
                    "tag": "input",
                    "props": {
                        "inputType": "text",
                        "value": state.title,
                        "placeholder": "Task title",
                        "attributes": {"autoFocus": "true"},
                        "size": 48,
                        "maxLength": 96,
                        "events": {
                            "onInput": "updateTitle",
                            "onChange": "updateTitle",
                            "onKeyDown": "handleShortcut"
                        },
                        "style": {"inlineSize": 620}
                    }
                },
                text("notes-label", "Notes"),
                {
                    "kind": "element",
                    "key": "notes",
                    "tag": "textarea",
                    "props": {
                        "value": state.notes,
                        "placeholder": "Notes",
                        "rows": 4,
                        "cols": 54,
                        "maxLength": 240,
                        "events": {"onInput": "updateNotes", "onChange": "updateNotes"},
                        "style": {"inlineSize": 620, "blockSize": 96}
                    }
                },
                {
                    "kind": "element",
                    "key": "decision-row",
                    "tag": "Toolbar",
                    "props": {
                        "orientation": "horizontal",
                        "style": {"gap": 12, "inlineSize": 620}
                    },
                    "children": [
                        priority_select(state),
                        completed_switch(state),
                        estimate_slider(state)
                    ]
                },
                stage_tabs(state),
                {
                    "kind": "element",
                    "key": "save",
                    "tag": "Button",
                    "props": {
                        "label": "Save",
                        "events": {"onPress": "saveWork"}
                    }
                },
                text("last-event", format!("Last event: {}", state.last_event))
            ]
        }
    }))
    .map_err(|error| GuiError::invalid_tree(format!("invalid dogfood frame: {error}")))
}

pub fn dogfood_reduce(state: &mut DogfoodState, invocation: &ActionInvocation) -> GuiResult<()> {
    match invocation.action.as_str() {
        "updateTitle" => {
            state.title = invocation.value.clone().unwrap_or_default();
            state.last_event = "Updated title".to_string();
        }
        "updateNotes" => {
            state.notes = invocation.value.clone().unwrap_or_default();
            state.last_event = "Updated notes".to_string();
        }
        "setPriority" => {
            state.priority = value_or(invocation, "Normal");
            state.last_event = format!("Priority {}", state.priority);
        }
        "setStage" => {
            state.stage = value_or(invocation, "Build");
            state.last_event = format!("Stage {}", state.stage);
        }
        "setCompleted" => {
            state.completed = invocation.value.as_deref() == Some("true");
            state.last_event = if state.completed {
                "Completed"
            } else {
                "Reopened"
            }
            .to_string();
        }
        "setEstimate" => {
            state.estimate = invocation
                .value
                .as_deref()
                .unwrap_or("0")
                .parse()
                .map_err(|error| GuiError::host(format!("invalid estimate: {error}")))?;
            state.last_event = format!("Estimate {:.0}h", state.estimate);
        }
        "saveWork" => {
            state.saves += 1;
            state.last_event = "Saved".to_string();
        }
        "handleShortcut" => apply_shortcut(state, invocation.value.as_deref()),
        other => return Err(GuiError::host(format!("unexpected action {other}"))),
    }
    Ok(())
}

fn apply_shortcut(state: &mut DogfoodState, key: Option<&str>) {
    let key = key.unwrap_or_default().trim();
    match key.to_ascii_lowercase().as_str() {
        "enter" | "return" | "meta+s" | "control+s" | "ctrl+s" => {
            state.saves += 1;
            state.last_event = "Saved from shortcut".to_string();
        }
        "escape" => {
            state.stage = "Review".to_string();
            state.last_event = "Moved to review".to_string();
        }
        " " | "space" | "spacebar" => {
            state.completed = !state.completed;
            state.last_event = if state.completed {
                "Completed from shortcut"
            } else {
                "Reopened from shortcut"
            }
            .to_string();
        }
        "" => {
            state.last_event = "Shortcut without key".to_string();
        }
        other => {
            state.last_event = format!("Key {other}");
        }
    }
}

fn value_or(invocation: &ActionInvocation, fallback: &str) -> String {
    invocation
        .value
        .clone()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

fn priority_select(state: &DogfoodState) -> Value {
    json!({
        "kind": "element",
        "key": "priority",
        "tag": "Select",
        "props": {
            "label": "Priority",
            "value": state.priority,
            "events": {"onSelectionChange": "setPriority"}
        },
        "children": [
            option("priority-low", "Low", state.priority == "Low"),
            option("priority-normal", "Normal", state.priority == "Normal"),
            option("priority-high", "High", state.priority == "High")
        ]
    })
}

fn completed_switch(state: &DogfoodState) -> Value {
    json!({
        "kind": "element",
        "key": "completed",
        "tag": "Switch",
        "props": {
            "label": "Completed",
            "isChecked": state.completed,
            "events": {"onChange": "setCompleted"}
        }
    })
}

fn estimate_slider(state: &DogfoodState) -> Value {
    json!({
        "kind": "element",
        "key": "estimate",
        "tag": "Slider",
        "props": {
            "label": "Estimate",
            "minValue": 1,
            "maxValue": 12,
            "valueNumber": state.estimate,
            "stepValue": 1,
            "events": {"onChange": "setEstimate"}
        }
    })
}

fn stage_tabs(state: &DogfoodState) -> Value {
    json!({
        "kind": "element",
        "key": "stage-tabs",
        "tag": "Tabs",
        "props": {
            "label": "Stage",
            "events": {"onSelectionChange": "setStage"},
            "style": {"inlineSize": 620}
        },
        "children": [
            {
                "kind": "element",
                "key": "stage-tab-list",
                "tag": "TabList",
                "children": [
                    tab("stage-plan", "Plan", state.stage == "Plan"),
                    tab("stage-build", "Build", state.stage == "Build"),
                    tab("stage-review", "Review", state.stage == "Review")
                ]
            },
            panel("panel-plan", "Plan the task."),
            panel("panel-build", "Build and verify the task."),
            panel("panel-review", "Review the finished task.")
        ]
    })
}

fn option(key: &str, value: &str, selected: bool) -> Value {
    json!({
        "kind": "element",
        "key": key,
        "tag": "ListBoxItem",
        "props": {
            "value": value,
            "textValue": value,
            "isSelected": selected
        }
    })
}

fn tab(key: &str, value: &str, selected: bool) -> Value {
    json!({
        "kind": "element",
        "key": key,
        "tag": "Tab",
        "props": {
            "textValue": value,
            "isSelected": selected
        }
    })
}

fn panel(key: &str, label: &str) -> Value {
    json!({
        "kind": "element",
        "key": key,
        "tag": "TabPanel",
        "children": [text(&format!("{key}-text"), label)]
    })
}

fn text(key: &str, label: impl Into<String>) -> Value {
    json!({
        "kind": "element",
        "key": key,
        "tag": "Text",
        "props": {"label": label.into()}
    })
}
