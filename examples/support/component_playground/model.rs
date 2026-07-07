use a3s_gui::GuiResult;

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentPlaygroundState {
    pub interaction_count: u32,
    pub query: String,
    pub selected_value: String,
    pub active_section: String,
    pub overlay_open: bool,
    pub last_event: String,
}

impl Default for ComponentPlaygroundState {
    fn default() -> Self {
        Self {
            interaction_count: 0,
            query: "native gui".to_string(),
            selected_value: "compact".to_string(),
            active_section: "foundation".to_string(),
            overlay_open: false,
            last_event: "Ready".to_string(),
        }
    }
}

impl ComponentPlaygroundState {
    pub fn record(&mut self, label: &str) {
        self.interaction_count += 1;
        self.last_event = label.to_string();
    }

    pub fn set_value(&mut self, value: String) -> GuiResult<()> {
        self.interaction_count += 1;
        if !value.is_empty() {
            self.query = value.clone();
            self.selected_value = value;
        }
        self.last_event = "Value changed".to_string();
        Ok(())
    }

    pub fn close_overlay(&mut self) {
        self.overlay_open = false;
        self.record("Overlay closed");
    }

    pub fn open_overlay(&mut self) {
        self.overlay_open = true;
        self.record("Overlay opened");
    }

    pub fn set_section(&mut self, section: &str) {
        self.active_section = section.to_string();
        self.record(&format!("Opened {section}"));
    }
}
