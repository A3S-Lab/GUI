use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{GuiError, GuiResult};

mod collection;
mod layout;
mod mounted;
mod typeahead;

pub use collection::{CollectionItem, KeyedCollection};
pub use layout::CollectionLayoutSnapshot;
pub use mounted::MountedSelectionRegistry;
pub(crate) use mounted::{
    apply_item_selection_props, apply_item_tree_props, validate_native_collection_keys,
    MountedSelectionUpdate,
};

pub(crate) const COLLECTION_ACTION_METADATA_KEY: &str = "data-a3s-collection-action";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CollectionKey(String);

impl CollectionKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CollectionKey {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CollectionKey {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for CollectionKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SelectionMode {
    None,
    #[default]
    Single,
    Multiple,
}

impl SelectionMode {
    pub fn from_name(value: Option<impl AsRef<str>>) -> Self {
        match value
            .map(|value| value.as_ref().to_ascii_lowercase())
            .as_deref()
        {
            Some("none") => Self::None,
            Some("multiple") => Self::Multiple,
            _ => Self::Single,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }

    pub(crate) fn from_option(value: Option<impl Into<String>>) -> Self {
        Self::from_name(value.map(Into::into))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SelectionBehavior {
    Toggle,
    #[default]
    Replace,
}

impl SelectionBehavior {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Toggle => "toggle",
            Self::Replace => "replace",
        }
    }

    pub fn from_name(value: Option<impl AsRef<str>>) -> Option<Self> {
        match value
            .map(|value| value.as_ref().to_ascii_lowercase())
            .as_deref()
        {
            Some("toggle") => Some(Self::Toggle),
            Some("replace") => Some(Self::Replace),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DisabledBehavior {
    Selection,
    #[default]
    All,
}

impl DisabledBehavior {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Selection => "selection",
            Self::All => "all",
        }
    }

    pub fn from_name(value: Option<impl AsRef<str>>) -> Option<Self> {
        match value
            .map(|value| value.as_ref().to_ascii_lowercase())
            .as_deref()
        {
            Some("selection") => Some(Self::Selection),
            Some("all") => Some(Self::All),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EscapeKeyBehavior {
    #[default]
    ClearSelection,
    None,
}

impl EscapeKeyBehavior {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClearSelection => "clearSelection",
            Self::None => "none",
        }
    }

    pub fn from_name(value: Option<impl AsRef<str>>) -> Self {
        match value
            .map(|value| value.as_ref().to_ascii_lowercase())
            .as_deref()
        {
            Some("none") => Self::None,
            _ => Self::ClearSelection,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    All,
    Keys(BTreeSet<CollectionKey>),
}

impl Default for Selection {
    fn default() -> Self {
        Self::empty()
    }
}

impl Selection {
    pub fn empty() -> Self {
        Self::Keys(BTreeSet::new())
    }

    pub fn keys(keys: impl IntoIterator<Item = CollectionKey>) -> Self {
        Self::Keys(keys.into_iter().collect())
    }

    pub fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Keys(keys) if keys.is_empty())
    }

    pub fn explicit_keys(&self) -> Option<&BTreeSet<CollectionKey>> {
        match self {
            Self::All => None,
            Self::Keys(keys) => Some(keys),
        }
    }
}

impl Serialize for Selection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Keys(keys) => keys.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Selection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum SelectionRepresentation {
            All(String),
            Keys(BTreeSet<CollectionKey>),
        }

        match SelectionRepresentation::deserialize(deserializer)? {
            SelectionRepresentation::All(value) if value.eq_ignore_ascii_case("all") => {
                Ok(Self::All)
            }
            SelectionRepresentation::All(value) => Ok(Self::keys([CollectionKey::new(value)])),
            SelectionRepresentation::Keys(keys) => Ok(Self::Keys(keys)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionManager {
    mode: SelectionMode,
    behavior: SelectionBehavior,
    disabled_behavior: DisabledBehavior,
    disallow_empty_selection: bool,
    selection: Selection,
    ordered_keys: Vec<CollectionKey>,
    known_keys: BTreeSet<CollectionKey>,
    item_disabled_keys: BTreeSet<CollectionKey>,
    disabled_keys: BTreeSet<CollectionKey>,
    anchor_key: Option<CollectionKey>,
    focused_key: Option<CollectionKey>,
    is_focused: bool,
}

impl SelectionManager {
    pub fn new(mode: SelectionMode) -> Self {
        Self {
            mode,
            behavior: SelectionBehavior::Replace,
            disabled_behavior: DisabledBehavior::All,
            disallow_empty_selection: false,
            selection: Selection::empty(),
            ordered_keys: Vec::new(),
            known_keys: BTreeSet::new(),
            item_disabled_keys: BTreeSet::new(),
            disabled_keys: BTreeSet::new(),
            anchor_key: None,
            focused_key: None,
            is_focused: false,
        }
    }

    pub fn sync_collection<T>(&mut self, collection: &KeyedCollection<T>) {
        self.ordered_keys = collection.keys().cloned().collect();
        self.known_keys = self.ordered_keys.iter().cloned().collect();
        self.item_disabled_keys = collection
            .items()
            .iter()
            .filter(|item| item.disabled)
            .map(|item| item.key.clone())
            .collect();
        if self
            .anchor_key
            .as_ref()
            .is_some_and(|key| !self.known_keys.contains(key))
        {
            self.anchor_key = None;
        }
        if self
            .focused_key
            .as_ref()
            .is_some_and(|key| !self.known_keys.contains(key))
        {
            self.focused_key = None;
        }
    }

    pub fn mode(&self) -> SelectionMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: SelectionMode) {
        self.mode = mode;
        if mode == SelectionMode::None {
            self.selection = Selection::empty();
            self.anchor_key = None;
        } else if mode == SelectionMode::Single {
            self.collapse_to_first_selected();
        }
    }

    pub fn behavior(&self) -> SelectionBehavior {
        self.behavior
    }

    pub fn set_selection_behavior(&mut self, behavior: SelectionBehavior) {
        self.behavior = behavior;
    }

    pub fn disabled_behavior(&self) -> DisabledBehavior {
        self.disabled_behavior
    }

    pub fn set_disabled_behavior(&mut self, behavior: DisabledBehavior) {
        self.disabled_behavior = behavior;
    }

    pub fn set_disallow_empty_selection(&mut self, disallow: bool) {
        self.disallow_empty_selection = disallow;
    }

    pub fn disallow_empty_selection(&self) -> bool {
        self.disallow_empty_selection
    }

    pub fn set_disabled_keys(&mut self, keys: impl IntoIterator<Item = CollectionKey>) {
        self.disabled_keys = keys.into_iter().collect();
    }

    pub fn focused_key(&self) -> Option<&CollectionKey> {
        self.focused_key.as_ref()
    }

    pub fn set_focused_key(&mut self, key: Option<CollectionKey>) -> bool {
        if key
            .as_ref()
            .is_some_and(|key| !self.known_keys.contains(key) || self.is_disabled(key))
        {
            return false;
        }
        if self.focused_key == key {
            return false;
        }
        self.focused_key = key;
        true
    }

    pub fn first_focusable_key(&self) -> Option<&CollectionKey> {
        self.ordered_keys.iter().find(|key| !self.is_disabled(key))
    }

    pub fn last_focusable_key(&self) -> Option<&CollectionKey> {
        self.ordered_keys
            .iter()
            .rev()
            .find(|key| !self.is_disabled(key))
    }

    pub fn next_focusable_key(
        &self,
        from: Option<&CollectionKey>,
        wrap: bool,
    ) -> Option<&CollectionKey> {
        let Some(from_index) = from.and_then(|from| {
            self.ordered_keys
                .iter()
                .position(|candidate| candidate == from)
        }) else {
            return self.first_focusable_key();
        };
        self.ordered_keys[from_index.saturating_add(1)..]
            .iter()
            .find(|key| !self.is_disabled(key))
            .or_else(|| wrap.then(|| self.first_focusable_key()).flatten())
    }

    pub fn previous_focusable_key(
        &self,
        from: Option<&CollectionKey>,
        wrap: bool,
    ) -> Option<&CollectionKey> {
        let Some(from_index) = from.and_then(|from| {
            self.ordered_keys
                .iter()
                .position(|candidate| candidate == from)
        }) else {
            return self.last_focusable_key();
        };
        self.ordered_keys[..from_index]
            .iter()
            .rev()
            .find(|key| !self.is_disabled(key))
            .or_else(|| wrap.then(|| self.last_focusable_key()).flatten())
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn set_focused(&mut self, focused: bool) -> bool {
        if self.is_focused == focused {
            return false;
        }
        self.is_focused = focused;
        true
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn set_selection(&mut self, selection: Selection) -> GuiResult<bool> {
        let normalized = self.normalize_selection(selection)?;
        if normalized == self.selection {
            return Ok(false);
        }
        if self.disallow_empty_selection && normalized.is_empty() && !self.selection.is_empty() {
            return Ok(false);
        }
        self.selection = normalized;
        self.anchor_key = self.first_selected_key().cloned();
        Ok(true)
    }

    pub fn can_select_item(&self, key: &CollectionKey) -> bool {
        self.mode != SelectionMode::None
            && self.known_keys.contains(key)
            && !self.key_has_selection_disabled(key)
    }

    pub fn is_disabled(&self, key: &CollectionKey) -> bool {
        self.disabled_behavior == DisabledBehavior::All
            && (self.item_disabled_keys.contains(key) || self.disabled_keys.contains(key))
    }

    pub fn is_selection_disabled(&self, key: &CollectionKey) -> bool {
        self.key_has_selection_disabled(key)
    }

    pub fn is_selected(&self, key: &CollectionKey) -> bool {
        match &self.selection {
            Selection::All => {
                self.known_keys.contains(key) && !self.key_has_selection_disabled(key)
            }
            Selection::Keys(keys) => keys.contains(key),
        }
    }

    pub fn selected_loaded_keys(&self) -> BTreeSet<CollectionKey> {
        self.ordered_keys
            .iter()
            .filter(|key| self.is_selected(key))
            .cloned()
            .collect()
    }

    pub fn first_selected_key(&self) -> Option<&CollectionKey> {
        self.ordered_keys.iter().find(|key| self.is_selected(key))
    }

    pub fn last_selected_key(&self) -> Option<&CollectionKey> {
        self.ordered_keys
            .iter()
            .rev()
            .find(|key| self.is_selected(key))
    }

    pub fn select(&mut self, key: &CollectionKey) -> bool {
        if self.mode == SelectionMode::Multiple && self.behavior == SelectionBehavior::Toggle {
            self.toggle_selection(key)
        } else {
            self.replace_selection(key)
        }
    }

    pub fn replace_selection(&mut self, key: &CollectionKey) -> bool {
        if !self.can_select_item(key) {
            return false;
        }
        let next = Selection::keys([key.clone()]);
        if next == self.selection {
            return false;
        }
        self.selection = next;
        self.anchor_key = Some(key.clone());
        true
    }

    pub fn toggle_selection(&mut self, key: &CollectionKey) -> bool {
        if !self.can_select_item(key) {
            return false;
        }
        if self.mode == SelectionMode::Single {
            if self.is_selected(key) && !self.disallow_empty_selection {
                self.selection = Selection::empty();
                self.anchor_key = None;
                return true;
            }
            return self.replace_selection(key);
        }

        let mut keys = match &self.selection {
            Selection::All => self.selected_loaded_keys(),
            Selection::Keys(keys) => keys.clone(),
        };
        if keys.remove(key) {
            if self.disallow_empty_selection && keys.is_empty() {
                return false;
            }
        } else {
            keys.insert(key.clone());
        }
        let next = Selection::Keys(keys);
        if next == self.selection {
            return false;
        }
        self.selection = next;
        self.anchor_key = Some(key.clone());
        true
    }

    pub fn extend_selection(&mut self, to_key: &CollectionKey) -> bool {
        if self.mode != SelectionMode::Multiple || !self.can_select_item(to_key) {
            return false;
        }
        let anchor = self
            .anchor_key
            .as_ref()
            .filter(|key| self.known_keys.contains(*key))
            .cloned()
            .unwrap_or_else(|| to_key.clone());
        let Some(anchor_index) = self.ordered_keys.iter().position(|key| key == &anchor) else {
            return false;
        };
        let Some(target_index) = self.ordered_keys.iter().position(|key| key == to_key) else {
            return false;
        };
        let (start, end) = if anchor_index <= target_index {
            (anchor_index, target_index)
        } else {
            (target_index, anchor_index)
        };
        let keys = self.ordered_keys[start..=end]
            .iter()
            .filter(|key| self.can_select_item(key))
            .cloned()
            .collect();
        let next = Selection::Keys(keys);
        if next == self.selection {
            return false;
        }
        self.selection = next;
        self.anchor_key = Some(anchor);
        true
    }

    pub fn select_all(&mut self) -> bool {
        if self.mode != SelectionMode::Multiple || self.selection == Selection::All {
            return false;
        }
        self.selection = Selection::All;
        true
    }

    pub fn clear_selection(&mut self) -> bool {
        if self.selection.is_empty() || self.disallow_empty_selection {
            return false;
        }
        self.selection = Selection::empty();
        self.anchor_key = None;
        true
    }

    pub fn toggle_select_all(&mut self) -> bool {
        let every_loaded_key_selected = self
            .ordered_keys
            .iter()
            .filter(|key| self.can_select_item(key))
            .all(|key| self.is_selected(key));
        if self.selection.is_all() || every_loaded_key_selected {
            self.clear_selection()
        } else {
            self.select_all()
        }
    }

    fn key_has_selection_disabled(&self, key: &CollectionKey) -> bool {
        self.item_disabled_keys.contains(key) || self.disabled_keys.contains(key)
    }

    fn normalize_selection(&self, selection: Selection) -> GuiResult<Selection> {
        match self.mode {
            SelectionMode::None if !selection.is_empty() => Err(GuiError::invalid_tree(
                "selectionMode none cannot contain selected keys",
            )),
            SelectionMode::Single if selection.is_all() => Err(GuiError::invalid_tree(
                "selectionMode single cannot use the all selection",
            )),
            SelectionMode::Single
                if selection.explicit_keys().is_some_and(|keys| keys.len() > 1) =>
            {
                Err(GuiError::invalid_tree(
                    "selectionMode single cannot contain more than one key",
                ))
            }
            _ => Ok(selection),
        }
    }

    fn collapse_to_first_selected(&mut self) {
        let first = self.first_selected_key().cloned().or_else(|| {
            self.selection
                .explicit_keys()
                .and_then(|keys| keys.iter().next().cloned())
        });
        self.selection = first
            .map(|key| Selection::keys([key]))
            .unwrap_or_else(Selection::empty);
        self.anchor_key = self.first_selected_key().cloned();
    }
}

#[cfg(test)]
mod mounted_tests;
#[cfg(test)]
mod tests;
