use super::*;

fn collection() -> KeyedCollection<&'static str> {
    KeyedCollection::new([
        CollectionItem::new("a", "Alpha"),
        CollectionItem::new("b", "Beta").disabled(true),
        CollectionItem::new("c", "Gamma"),
        CollectionItem::new("d", "Delta"),
    ])
    .unwrap()
}

#[test]
fn selection_serializes_as_key_set_or_all() {
    let keys = Selection::keys([CollectionKey::from("a"), CollectionKey::from("c")]);
    assert_eq!(
        serde_json::to_value(keys).unwrap(),
        serde_json::json!(["a", "c"])
    );
    assert_eq!(
        serde_json::to_value(Selection::All).unwrap(),
        serde_json::json!("all")
    );
    assert_eq!(
        serde_json::from_value::<Selection>(serde_json::json!("all")).unwrap(),
        Selection::All
    );
    assert_eq!(
        serde_json::from_value::<Selection>(serde_json::json!("legacy-key")).unwrap(),
        Selection::keys([CollectionKey::from("legacy-key")])
    );
}

#[test]
fn keyed_collection_rejects_duplicate_identity_atomically() {
    let mut collection = collection();
    let original = collection.clone();

    let result = collection.replace([
        CollectionItem::new("a", "Changed"),
        CollectionItem::new("a", "Duplicate"),
    ]);

    assert!(result.is_err());
    assert_eq!(collection, original);
}

#[test]
fn multiple_toggle_selection_respects_disabled_keys() {
    let collection = collection();
    let mut manager = SelectionManager::new(SelectionMode::Multiple);
    manager.sync_collection(&collection);
    manager.set_selection_behavior(SelectionBehavior::Toggle);

    assert!(manager.select(&CollectionKey::from("a")));
    assert!(!manager.select(&CollectionKey::from("b")));
    assert!(manager.select(&CollectionKey::from("c")));
    assert_eq!(
        manager.selected_loaded_keys(),
        [CollectionKey::from("a"), CollectionKey::from("c")]
            .into_iter()
            .collect()
    );
    assert!(manager.select(&CollectionKey::from("a")));
    assert!(!manager.is_selected(&CollectionKey::from("a")));
}

#[test]
fn replace_and_range_selection_follow_collection_order() {
    let collection = collection();
    let mut manager = SelectionManager::new(SelectionMode::Multiple);
    manager.sync_collection(&collection);

    assert!(manager.replace_selection(&CollectionKey::from("a")));
    assert!(manager.extend_selection(&CollectionKey::from("d")));
    assert_eq!(
        manager.selected_loaded_keys(),
        [
            CollectionKey::from("a"),
            CollectionKey::from("c"),
            CollectionKey::from("d")
        ]
        .into_iter()
        .collect()
    );
    assert_eq!(manager.first_selected_key().unwrap().as_str(), "a");
    assert_eq!(manager.last_selected_key().unwrap().as_str(), "d");
}

#[test]
fn all_survives_async_collection_replacement() {
    let mut collection = collection();
    let mut manager = SelectionManager::new(SelectionMode::Multiple);
    manager.sync_collection(&collection);
    assert!(manager.select_all());

    collection
        .replace([
            CollectionItem::new("c", "Gamma"),
            CollectionItem::new("e", "Epsilon"),
        ])
        .unwrap();
    manager.sync_collection(&collection);

    assert_eq!(manager.selection(), &Selection::All);
    assert!(manager.is_selected(&CollectionKey::from("e")));
}

#[test]
fn explicit_unloaded_keys_survive_collection_updates() {
    let collection = collection();
    let mut manager = SelectionManager::new(SelectionMode::Multiple);
    manager.sync_collection(&collection);
    manager
        .set_selection(Selection::keys([CollectionKey::from("future")]))
        .unwrap();

    assert_eq!(
        manager.selection(),
        &Selection::keys([CollectionKey::from("future")])
    );
    assert!(manager.selected_loaded_keys().is_empty());

    manager.set_selection_behavior(SelectionBehavior::Toggle);
    assert!(manager.select(&CollectionKey::from("a")));
    assert_eq!(
        manager.selection(),
        &Selection::keys([CollectionKey::from("a"), CollectionKey::from("future")])
    );
}

#[test]
fn disallow_empty_selection_prevents_last_toggle_and_clear() {
    let collection = collection();
    let mut manager = SelectionManager::new(SelectionMode::Single);
    manager.sync_collection(&collection);
    manager.set_disallow_empty_selection(true);
    assert!(manager.replace_selection(&CollectionKey::from("a")));

    assert!(!manager.toggle_selection(&CollectionKey::from("a")));
    assert!(!manager.clear_selection());
    assert!(manager.is_selected(&CollectionKey::from("a")));
}

#[test]
fn disabled_behavior_distinguishes_selection_from_all_interactions() {
    let collection = collection();
    let key = CollectionKey::from("b");
    let mut manager = SelectionManager::new(SelectionMode::Multiple);
    manager.sync_collection(&collection);

    manager.set_disabled_behavior(DisabledBehavior::Selection);
    assert!(!manager.can_select_item(&key));
    assert!(!manager.is_disabled(&key));

    manager.set_disabled_behavior(DisabledBehavior::All);
    assert!(manager.is_disabled(&key));
}

#[test]
fn explicitly_selected_disabled_key_remains_selected_but_cannot_be_changed() {
    let collection = collection();
    let key = CollectionKey::from("b");
    let mut manager = SelectionManager::new(SelectionMode::Multiple);
    manager.sync_collection(&collection);
    manager
        .set_selection(Selection::keys([key.clone()]))
        .unwrap();

    assert!(manager.is_selected(&key));
    assert!(!manager.toggle_selection(&key));
    assert!(manager.is_selected(&key));
}

#[test]
fn focused_key_tracks_loaded_focusable_items() {
    let mut manager = SelectionManager::new(SelectionMode::Multiple);
    manager.sync_collection(&collection());

    assert!(manager.set_focused_key(Some(CollectionKey::from("a"))));
    assert!(manager.set_focused(true));
    assert_eq!(manager.focused_key().unwrap().as_str(), "a");
    assert!(manager.is_focused());
    assert!(!manager.set_focused_key(Some(CollectionKey::from("b"))));

    let replacement = KeyedCollection::new([CollectionItem::new("c", "Gamma")]).unwrap();
    manager.sync_collection(&replacement);
    assert!(manager.focused_key().is_none());
}

#[test]
fn focus_navigation_skips_fully_disabled_items_and_can_wrap() {
    let collection = collection();
    let mut manager = SelectionManager::new(SelectionMode::Multiple);
    manager.sync_collection(&collection);
    let first = CollectionKey::from("a");
    let middle = CollectionKey::from("c");
    let last = CollectionKey::from("d");

    assert_eq!(manager.first_focusable_key(), Some(&first));
    assert_eq!(manager.last_focusable_key(), Some(&last));
    assert_eq!(
        manager.next_focusable_key(Some(&first), false),
        Some(&middle)
    );
    assert_eq!(manager.next_focusable_key(Some(&last), false), None);
    assert_eq!(manager.next_focusable_key(Some(&last), true), Some(&first));
    assert_eq!(
        manager.previous_focusable_key(Some(&last), false),
        Some(&middle)
    );
    assert_eq!(
        manager.previous_focusable_key(Some(&first), true),
        Some(&last)
    );
}

#[test]
fn selection_only_disabled_items_remain_keyboard_focusable() {
    let collection = collection();
    let mut manager = SelectionManager::new(SelectionMode::Multiple);
    manager.sync_collection(&collection);
    manager.set_disabled_behavior(DisabledBehavior::Selection);
    let first = CollectionKey::from("a");
    let disabled = CollectionKey::from("b");

    assert_eq!(
        manager.next_focusable_key(Some(&first), false),
        Some(&disabled)
    );
}
