use std::collections::BTreeMap;

use crate::native::NativeProps;

use super::drag_drop::*;

#[test]
fn type_matching_supports_multiple_queries_and_mime_wildcards() {
    assert!(accepts_types(
        &["image/*".to_string(), "application/json".to_string()],
        &["text/plain".to_string(), "image/png".to_string()]
    ));
    assert!(!accepts_types(
        &["image/*".to_string(), "application/json".to_string()],
        &["text/plain".to_string(), "application/pdf".to_string()]
    ));
    assert!(accepts_types(
        &["all".to_string()],
        &["custom-type".to_string()]
    ));
}

#[test]
fn invalid_requested_operation_rejects_instead_of_falling_back() {
    let props = NativeProps::new().web(
        crate::web::WebProps::new()
            .attribute("data-accepted-drag-types", "text/plain")
            .attribute("data-drop-operation", "archive"),
    );
    let target = SelfDrawnDropTarget::from_props(&props, false).unwrap();
    assert_eq!(
        target.operation_for(
            &["text/plain".to_string()],
            &[SelfDrawnDropOperation::Copy, SelfDrawnDropOperation::Move],
        ),
        SelfDrawnDropOperation::Cancel
    );
}

#[test]
fn drag_item_wire_shape_is_typed_and_old_contexts_default_items() {
    let item = SelfDrawnDropItem::text(BTreeMap::from([
        ("text/plain".to_string(), "alpha".to_string()),
        ("text/html".to_string(), "<b>alpha</b>".to_string()),
    ]));
    let wire = serde_json::to_value(&item).unwrap();
    assert_eq!(wire["kind"], "text");
    assert_eq!(
        wire["types"],
        serde_json::json!(["text/html", "text/plain"])
    );
    assert_eq!(wire["formats"]["text/plain"], "alpha");

    let old = serde_json::from_value::<SelfDrawnDragContext>(serde_json::json!({
        "types": ["text/plain"],
        "value": "alpha",
        "allowedOperations": ["copy"],
        "dropOperation": "copy"
    }))
    .unwrap();
    assert!(old.items.is_empty());
    assert!(old.dragging_keys.is_empty());
    assert!(old.target.is_none());
    assert!(!old.is_internal);
}

#[test]
fn encoded_drag_items_reject_non_text_or_empty_representations() {
    assert!(parse_drag_items(r#"[{"text/plain":1}]"#).is_none());
    assert!(parse_drag_items(r#"[{}]"#).is_none());
    assert!(parse_drag_items("[]").is_none());
}
