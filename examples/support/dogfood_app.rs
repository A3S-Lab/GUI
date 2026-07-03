use a3s_gui::{ActionInvocation, GuiError, GuiResult, UiFrame};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct DogfoodState {
    pub title: String,
    pub notes: String,
    pub priority: String,
    pub stage: String,
    pub assignee: String,
    pub completed: bool,
    pub design_reviewed: bool,
    pub tests_reviewed: bool,
    pub docs_updated: bool,
    pub review_open: bool,
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
            assignee: "Ada".to_string(),
            completed: false,
            design_reviewed: false,
            tests_reviewed: false,
            docs_updated: false,
            review_open: false,
            estimate: 6.0,
            saves: 0,
            last_event: "Ready".to_string(),
        }
    }
}

impl DogfoodState {
    pub fn review_ready(&self) -> bool {
        self.design_reviewed
            && self.tests_reviewed
            && self.docs_updated
            && !self.title.trim().is_empty()
    }

    fn review_count(&self) -> u8 {
        u8::from(self.design_reviewed) + u8::from(self.tests_reviewed) + u8::from(self.docs_updated)
    }

    fn review_status(&self) -> String {
        if self.review_ready() {
            "Ready to complete review".to_string()
        } else {
            format!("{}/3 review checks complete", self.review_count())
        }
    }
}

pub fn dogfood_frame(state: &DogfoodState, frame_id: &str, title: &str) -> GuiResult<UiFrame> {
    serde_json::from_value(json!({
        "frameId": frame_id,
        "window": {
            "title": title,
            "width": 760,
            "height": 680,
            "minWidth": 540,
            "minHeight": 500
        },
        "actions": [
            {"id": "updateTitle", "label": "Update title"},
            {"id": "updateNotes", "label": "Update notes"},
            {"id": "setPriority", "label": "Set priority"},
            {"id": "setAssignee", "label": "Set assignee"},
            {"id": "setStage", "label": "Set stage"},
            {"id": "setCompleted", "label": "Set completed"},
            {"id": "setEstimate", "label": "Set estimate"},
            {"id": "setDesignReviewed", "label": "Set design reviewed"},
            {"id": "setTestsReviewed", "label": "Set tests reviewed"},
            {"id": "setDocsUpdated", "label": "Set docs updated"},
            {"id": "requestReview", "label": "Request review"},
            {"id": "closeReview", "label": "Close review"},
            {"id": "finishReview", "label": "Finish review"},
            {"id": "reopenWork", "label": "Reopen work"},
            {"id": "saveWork", "label": "Save work"},
            {"id": "handleShortcut", "label": "Handle shortcut"},
            {"id": "handleShortcutRelease", "label": "Handle shortcut release"}
        ],
        "root": {
            "kind": "element",
            "key": "dogfood-root",
            "tag": "Toolbar",
            "props": {
                "orientation": "vertical",
                "events": {
                    "onKeyDown": "handleShortcut",
                    "onKeyUp": "handleShortcutRelease"
                },
                "style": {
                    "writingMode": "horizontal-tb",
                    "inlineSize": 700,
                    "blockSize": 620,
                    "minInlineSize": 480,
                    "minBlockSize": 420,
                    "gap": 10
                }
            },
            "children": [
                workflow_menu(state),
                text("summary", format!(
                    "{} | {} | {} | {:.0}h | {}/3 review | {} | saves {}",
                    state.priority,
                    state.stage,
                    state.assignee,
                    state.estimate,
                    state.review_count(),
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
                        "isRequired": true,
                        "isInvalid": state.title.trim().is_empty(),
                        "attributes": {"autoFocus": "true"},
                        "size": 48,
                        "maxLength": 96,
                        "events": {
                            "onInput": "updateTitle",
                            "onChange": "updateTitle",
                            "onKeyDown": "handleShortcut"
                        },
                        "style": {"inlineSize": 640}
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
                        "style": {"inlineSize": 640, "blockSize": 96}
                    }
                },
                {
                    "kind": "element",
                    "key": "assignment-row",
                    "tag": "Toolbar",
                    "props": {
                        "orientation": "horizontal",
                        "style": {"gap": 12, "inlineSize": 640}
                    },
                    "children": [
                        priority_select(state),
                        assignee_select(state),
                        estimate_slider(state)
                    ]
                },
                stage_tabs(state),
                review_panel(state),
                {
                    "kind": "element",
                    "key": "action-row",
                    "tag": "Toolbar",
                    "props": {
                        "orientation": "horizontal",
                        "style": {"gap": 12, "inlineSize": 640}
                    },
                    "children": [
                        completed_switch(state),
                        button("save", "Save", "saveWork", false),
                        button("request-review", "Request review", "requestReview", false),
                        button(
                            "finish-review",
                            "Complete review",
                            "finishReview",
                            !state.review_ready()
                        )
                    ]
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
        "setAssignee" => {
            state.assignee = value_or(invocation, "Ada");
            state.last_event = format!("Assigned to {}", state.assignee);
        }
        "setStage" => {
            state.stage = value_or(invocation, "Build");
            state.last_event = format!("Stage {}", state.stage);
        }
        "setCompleted" => {
            state.completed = bool_value(invocation);
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
        "setDesignReviewed" => {
            state.design_reviewed = bool_value(invocation);
            state.last_event = review_item_event("Design review", state.design_reviewed);
        }
        "setTestsReviewed" => {
            state.tests_reviewed = bool_value(invocation);
            state.last_event = review_item_event("Tests", state.tests_reviewed);
        }
        "setDocsUpdated" => {
            state.docs_updated = bool_value(invocation);
            state.last_event = review_item_event("Docs", state.docs_updated);
        }
        "requestReview" => {
            state.review_open = true;
            state.stage = "Review".to_string();
            state.last_event = "Review requested".to_string();
        }
        "closeReview" => {
            state.review_open = false;
            state.last_event = "Review dialog closed".to_string();
        }
        "finishReview" => {
            if state.review_ready() {
                state.completed = true;
                state.review_open = false;
                state.stage = "Done".to_string();
                state.last_event = "Review completed".to_string();
            } else {
                state.review_open = true;
                state.last_event = "Review checklist incomplete".to_string();
            }
        }
        "reopenWork" => {
            state.completed = false;
            state.review_open = false;
            state.stage = "Build".to_string();
            state.design_reviewed = false;
            state.tests_reviewed = false;
            state.docs_updated = false;
            state.last_event = "Reopened work".to_string();
        }
        "saveWork" => {
            state.saves += 1;
            state.last_event = "Saved".to_string();
        }
        "handleShortcut" => apply_shortcut(state, invocation.value.as_deref()),
        "handleShortcutRelease" => apply_shortcut_release(state, invocation.value.as_deref()),
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
        "r" | "meta+r" | "control+r" | "ctrl+r" => {
            state.review_open = true;
            state.stage = "Review".to_string();
            state.last_event = "Review requested from shortcut".to_string();
        }
        "escape" => {
            if state.review_open {
                state.review_open = false;
                state.last_event = "Review dialog closed".to_string();
            } else {
                state.stage = "Review".to_string();
                state.last_event = "Moved to review".to_string();
            }
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

fn apply_shortcut_release(state: &mut DogfoodState, key: Option<&str>) {
    let key = key.unwrap_or_default().trim();
    state.last_event = if key.is_empty() {
        "Released shortcut".to_string()
    } else {
        format!("Released {key}")
    };
}

fn value_or(invocation: &ActionInvocation, fallback: &str) -> String {
    invocation
        .value
        .clone()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

fn bool_value(invocation: &ActionInvocation) -> bool {
    invocation.value.as_deref() == Some("true")
}

fn review_item_event(label: &str, checked: bool) -> String {
    if checked {
        format!("{label} checked")
    } else {
        format!("{label} unchecked")
    }
}

fn workflow_menu(state: &DogfoodState) -> Value {
    json!({
        "kind": "element",
        "key": "workflow-menu",
        "tag": "Menu",
        "props": {"label": "Workflow"},
        "children": [
            menu_item("menu-save", "Save", "saveWork", false),
            menu_item("menu-review", "Request review", "requestReview", false),
            menu_item("menu-finish", "Complete review", "finishReview", !state.review_ready()),
            menu_item("menu-reopen", "Reopen work", "reopenWork", state.stage == "Build")
        ]
    })
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

fn assignee_select(state: &DogfoodState) -> Value {
    json!({
        "kind": "element",
        "key": "assignee",
        "tag": "Select",
        "props": {
            "label": "Assignee",
            "value": state.assignee,
            "events": {"onSelectionChange": "setAssignee"}
        },
        "children": [
            option("assignee-ada", "Ada", state.assignee == "Ada"),
            option("assignee-grace", "Grace", state.assignee == "Grace"),
            option("assignee-linus", "Linus", state.assignee == "Linus")
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
            "isDisabled": state.review_open,
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

fn review_panel(state: &DogfoodState) -> Value {
    json!({
        "kind": "element",
        "key": "review-panel",
        "tag": "Toolbar",
        "props": {
            "orientation": "vertical",
            "style": {"gap": 8, "inlineSize": 640}
        },
        "children": [
            text("review-label", "Review checklist"),
            review_checkbox(
                "review-design",
                "Design reviewed",
                state.design_reviewed,
                "setDesignReviewed"
            ),
            review_checkbox(
                "review-tests",
                "Tests pass",
                state.tests_reviewed,
                "setTestsReviewed"
            ),
            review_checkbox(
                "review-docs",
                "Docs updated",
                state.docs_updated,
                "setDocsUpdated"
            ),
            {
                "kind": "element",
                "key": "review-status",
                "tag": "input",
                "props": {
                    "label": "Review status",
                    "inputType": "text",
                    "value": state.review_status(),
                    "isReadOnly": true,
                    "style": {"inlineSize": 360}
                }
            },
            review_dialog(state)
        ]
    })
}

fn review_checkbox(key: &str, label: &str, checked: bool, action: &str) -> Value {
    json!({
        "kind": "element",
        "key": key,
        "tag": "Checkbox",
        "props": {
            "label": label,
            "isChecked": checked,
            "events": {"onChange": action}
        }
    })
}

fn review_dialog(state: &DogfoodState) -> Value {
    let attributes = if state.review_open {
        json!({"open": ""})
    } else {
        json!({})
    };

    json!({
        "kind": "element",
        "key": "review-dialog",
        "tag": "dialog",
        "props": {
            "label": "Review gate",
            "attributes": attributes,
            "style": {"inlineSize": 520}
        },
        "children": [
            text("review-dialog-title", "Review gate"),
            text("review-dialog-status", state.review_status()),
            button(
                "dialog-finish-review",
                "Complete review",
                "finishReview",
                !state.review_ready()
            ),
            button("dialog-close-review", "Close", "closeReview", false)
        ]
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
            "style": {"inlineSize": 640}
        },
        "children": [
            {
                "kind": "element",
                "key": "stage-tab-list",
                "tag": "TabList",
                "children": [
                    tab("stage-plan", "Plan", state.stage == "Plan"),
                    tab("stage-build", "Build", state.stage == "Build"),
                    tab("stage-review", "Review", state.stage == "Review"),
                    tab("stage-done", "Done", state.stage == "Done")
                ]
            },
            panel("panel-plan", "Plan the task."),
            panel("panel-build", "Build and verify the task."),
            panel("panel-review", "Review the finished task."),
            panel("panel-done", "The task is ready to ship.")
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

fn menu_item(key: &str, label: &str, action: &str, disabled: bool) -> Value {
    json!({
        "kind": "element",
        "key": key,
        "tag": "MenuItem",
        "props": {
            "label": label,
            "value": action,
            "isDisabled": disabled,
            "events": {"onPress": action}
        }
    })
}

fn button(key: &str, label: &str, action: &str, disabled: bool) -> Value {
    json!({
        "kind": "element",
        "key": key,
        "tag": "Button",
        "props": {
            "label": label,
            "isDisabled": disabled,
            "events": {"onPress": action}
        }
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
