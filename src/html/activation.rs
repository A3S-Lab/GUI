use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlActivationProps {
    pub command: Option<String>,
    pub command_for: Option<String>,
    pub popover_target: Option<String>,
    pub popover_target_action: Option<String>,
}

impl HtmlActivationProps {
    pub fn command(mut self, command: impl Into<String>) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn command_for(mut self, command_for: impl Into<String>) -> Self {
        self.command_for = Some(command_for.into());
        self
    }

    pub fn popover_target(mut self, popover_target: impl Into<String>) -> Self {
        self.popover_target = Some(popover_target.into());
        self
    }

    pub fn popover_target_action(mut self, popover_target_action: impl Into<String>) -> Self {
        self.popover_target_action = Some(popover_target_action.into());
        self
    }
}
