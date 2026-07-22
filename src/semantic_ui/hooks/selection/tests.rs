use serde_json::json;

use super::{use_selection, UseSelectionProps};
use crate::selection::{CollectionKey, Selection};

#[test]
fn uncontrolled_selection_does_not_serialize_a_controlled_empty_value() {
    let value = serde_json::to_value(use_selection(UseSelectionProps::new())).unwrap();

    assert_eq!(value["selectedKeys"], json!([]));
    assert!(value["selectionProps"].get("selectedKeys").is_none());
    assert!(value["selectionProps"].get("value").is_none());
    assert!(value["selectionProps"].get("onAction").is_none());
    assert_eq!(
        value["selectionProps"]["escapeKeyBehavior"],
        "clearSelection"
    );
}

#[test]
fn collection_action_and_selection_callbacks_remain_independent() {
    let value = serde_json::to_value(use_selection(
        UseSelectionProps::new()
            .on_action(Some("openItem"))
            .on_selection_change(Some("selectItem")),
    ))
    .unwrap();

    assert_eq!(value["selectionProps"]["onAction"], "openItem");
    assert_eq!(value["selectionProps"]["onSelectionChange"], "selectItem");
}

#[test]
fn default_selection_and_collection_options_are_preserved_without_becoming_controlled() {
    let value = serde_json::to_value(use_selection(
        UseSelectionProps::new()
            .default_selected_keys(Selection::keys([CollectionKey::new("alpha")]))
            .disabled_keys([CollectionKey::new("beta")])
            .selection_mode(Some("multiple"))
            .selection_behavior(Some("replace"))
            .disabled_behavior(Some("selection"))
            .disallow_empty_selection(true)
            .should_focus_wrap(true)
            .escape_key_behavior(Some("none")),
    ))
    .unwrap();

    assert_eq!(value["selectedKeys"], json!(["alpha"]));
    assert_eq!(
        value["selectionProps"]["defaultSelectedKeys"],
        json!(["alpha"])
    );
    assert!(value["selectionProps"].get("selectedKeys").is_none());
    assert_eq!(value["selectionProps"]["disabledKeys"], json!(["beta"]));
    assert_eq!(value["selectionProps"]["selectionBehavior"], "replace");
    assert_eq!(value["selectionProps"]["disabledBehavior"], "selection");
    assert_eq!(value["selectionProps"]["disallowEmptySelection"], true);
    assert_eq!(value["selectionProps"]["shouldFocusWrap"], true);
    assert_eq!(value["selectionProps"]["escapeKeyBehavior"], "none");
}

#[test]
fn explicit_selected_keys_remain_controlled_and_override_the_legacy_value_alias() {
    let result = use_selection(
        UseSelectionProps::new()
            .value(Some("legacy"))
            .selected_keys(Selection::All),
    );
    let value = serde_json::to_value(result).unwrap();

    assert_eq!(value["selectedKeys"], "all");
    assert_eq!(value["selectionProps"]["selectedKeys"], "all");
    assert!(value["selectionProps"].get("value").is_none());
    assert!(value.get("selectedValue").is_none());
}
