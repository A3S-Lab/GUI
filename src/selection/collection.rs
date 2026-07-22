use std::collections::BTreeMap;

use crate::error::{GuiError, GuiResult};

use super::CollectionKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionItem<T> {
    pub key: CollectionKey,
    pub value: T,
    pub disabled: bool,
}

impl<T> CollectionItem<T> {
    pub fn new(key: impl Into<CollectionKey>, value: T) -> Self {
        Self {
            key: key.into(),
            value,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyedCollection<T> {
    items: Vec<CollectionItem<T>>,
    indices: BTreeMap<CollectionKey, usize>,
}

impl<T> Default for KeyedCollection<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            indices: BTreeMap::new(),
        }
    }
}

impl<T> KeyedCollection<T> {
    pub fn new(items: impl IntoIterator<Item = CollectionItem<T>>) -> GuiResult<Self> {
        let mut collection = Self::default();
        collection.replace(items)?;
        Ok(collection)
    }

    pub fn replace(&mut self, items: impl IntoIterator<Item = CollectionItem<T>>) -> GuiResult<()> {
        let items = items.into_iter().collect::<Vec<_>>();
        let mut indices = BTreeMap::new();
        for (index, item) in items.iter().enumerate() {
            if indices.insert(item.key.clone(), index).is_some() {
                return Err(GuiError::invalid_tree(format!(
                    "collection contains duplicate key {:?}",
                    item.key.as_str()
                )));
            }
        }
        self.items = items;
        self.indices = indices;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &[CollectionItem<T>] {
        &self.items
    }

    pub fn keys(&self) -> impl Iterator<Item = &CollectionKey> {
        self.items.iter().map(|item| &item.key)
    }

    pub fn get(&self, key: &CollectionKey) -> Option<&CollectionItem<T>> {
        self.indices
            .get(key)
            .and_then(|index| self.items.get(*index))
    }

    pub fn contains_key(&self, key: &CollectionKey) -> bool {
        self.indices.contains_key(key)
    }
}
