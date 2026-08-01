use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform_host::PlatformPointerPhase;
use crate::web::WebProps;
use crate::NativeEventKind;

use super::drag_drop_tests::{action_events, input};
use super::tests::{pointer_event, runtime};
use super::{SelfDrawnCollectionDropTarget, SelfDrawnDropPosition, SelfDrawnInputDispatch};
use super::{
    SelfDrawnDropOperation, SelfDrawnDropPolicyDecision, SelfDrawnDropPolicyQuery,
    SelfDrawnDropPolicyRequest, SelfDrawnDropPolicyResolution, SelfDrawnDropPolicyResponse,
};

fn collection(events: &[(&str, &str)], items: Vec<NativeElement>) -> NativeElement {
    let mut web = WebProps::new()
        .class_name("relative h-[150px] w-[150px] bg-white")
        .attribute("data-draggable-collection", "true")
        .attribute("data-droppable-collection", "true")
        .attribute("data-drop-orientation", "vertical")
        .attribute("data-accepted-drag-types", "text/plain")
        .attribute("data-allowed-drop-operations", "move")
        .attribute("data-drop-operation", "move")
        .event("onDragStart", "dragStart")
        .event("onDragMove", "dragMove")
        .event("onDragEnd", "dragEnd")
        .event("onDropEnter", "dropEnter")
        .event("onDropMove", "dropMove")
        .event("onDropExit", "dropExit");
    for (name, action) in events {
        web = web.event(*name, *action);
    }
    NativeElement::new("collection", NativeRole::ListBox)
        .with_props(NativeProps::new().tab_index(Some(-1)).web(web))
        .children(items)
}

fn item(key: &str, top: u32, selected: bool, parent: Option<&str>) -> NativeElement {
    let mut web = WebProps::new()
        .class_name(format!(
            "absolute left-[5px] top-[{top}px] h-[30px] w-[120px] bg-black"
        ))
        .attribute("data-collection-drop-item", "true")
        .attribute("data-collection-key", key)
        .attribute("data-drag-items", format!(r#"[{{"text/plain":"{key}"}}]"#));
    if let Some(parent) = parent {
        web = web.attribute("data-tree-parent-key", parent);
    }
    NativeElement::new(key, NativeRole::ListBoxItem).with_props(
        NativeProps::new()
            .tab_index(Some(if top == 5 { 0 } else { -1 }))
            .selected(selected)
            .web(web),
    )
}

fn flat_items() -> Vec<NativeElement> {
    vec![
        item("a", 5, true, None),
        item("b", 50, false, None),
        item("c", 95, false, None),
    ]
}

fn typed_item(key: &str, top: u32, selected: bool, drag_type: &str) -> NativeElement {
    NativeElement::new(key, NativeRole::ListBoxItem).with_props(
        NativeProps::new()
            .tab_index(Some(if top == 5 { 0 } else { -1 }))
            .selected(selected)
            .web(
                WebProps::new()
                    .class_name(format!(
                        "absolute left-[5px] top-[{top}px] h-[30px] w-[120px] bg-black"
                    ))
                    .attribute("data-collection-drop-item", "true")
                    .attribute("data-collection-key", key)
                    .attribute("data-drag-items", format!(r#"[{{"{drag_type}":"{key}"}}]"#)),
            ),
    )
}

fn dynamic_collection(events: &[(&str, &str)]) -> NativeElement {
    let mut tree = collection(
        events,
        vec![
            typed_item("a", 5, true, "text/plain"),
            typed_item("b", 50, false, "text/plain"),
            typed_item("c", 95, true, "application/json"),
        ],
    );
    tree.props
        .web
        .attributes
        .insert("data-accepted-drag-types".to_string(), "all".to_string());
    tree.props.web.attributes.insert(
        "data-should-accept-item-drop-policy".to_string(),
        "acceptFolder".to_string(),
    );
    tree.props.web.attributes.insert(
        "data-get-drop-operation-policy".to_string(),
        "chooseOperation".to_string(),
    );
    tree
}

fn start_drag(
    runtime: &mut super::SelfDrawnWindowRuntime<
        crate::platform_host::RecordingPlatformHost,
        super::RecordingScenePresenter,
    >,
) -> SelfDrawnInputDispatch {
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 15.0, 15.0, 10))
        .unwrap();
    input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 20.0, 15.0, 20))
            .unwrap(),
    )
}

fn move_to(
    runtime: &mut super::SelfDrawnWindowRuntime<
        crate::platform_host::RecordingPlatformHost,
        super::RecordingScenePresenter,
    >,
    y: f64,
    timestamp: u64,
) -> SelfDrawnInputDispatch {
    input(
        runtime
            .handle_event(pointer_event(
                PlatformPointerPhase::Moved,
                15.0,
                y,
                timestamp,
            ))
            .unwrap(),
    )
}

fn drop_at(
    runtime: &mut super::SelfDrawnWindowRuntime<
        crate::platform_host::RecordingPlatformHost,
        super::RecordingScenePresenter,
    >,
    y: f64,
    timestamp: u64,
) -> SelfDrawnInputDispatch {
    input(
        runtime
            .handle_event(pointer_event(
                PlatformPointerPhase::Released,
                15.0,
                y,
                timestamp,
            ))
            .unwrap(),
    )
}

fn drop_actions(dispatch: &SelfDrawnInputDispatch) -> Vec<&str> {
    dispatch
        .invocations
        .iter()
        .filter(|invocation| invocation.event == NativeEventKind::Drop)
        .map(|invocation| invocation.action.as_str())
        .collect()
}

#[test]
fn internal_item_drop_dispatches_item_then_move_callbacks() {
    let mut runtime = runtime();
    runtime
        .render(collection(
            &[("onItemDrop", "itemDrop"), ("onCollectionMove", "move")],
            flat_items(),
        ))
        .unwrap();
    start_drag(&mut runtime);
    move_to(&mut runtime, 65.0, 30);

    let dropped = drop_at(&mut runtime, 65.0, 40);
    assert_eq!(drop_actions(&dropped), ["itemDrop", "move"]);
    let drag = dropped.invocations[0].context.drag.as_ref().unwrap();
    assert!(drag.is_internal);
    assert_eq!(
        drag.target,
        Some(SelfDrawnCollectionDropTarget::Item {
            key: "b".to_string(),
            drop_position: SelfDrawnDropPosition::On,
        })
    );
}

#[test]
fn internal_between_drop_dispatches_move_then_same_parent_reorder() {
    let mut runtime = runtime();
    runtime
        .render(collection(
            &[("onCollectionMove", "move"), ("onReorder", "reorder")],
            flat_items(),
        ))
        .unwrap();
    start_drag(&mut runtime);
    move_to(&mut runtime, 52.0, 30);

    let dropped = drop_at(&mut runtime, 52.0, 40);
    assert_eq!(drop_actions(&dropped), ["move", "reorder"]);
}

#[test]
fn reorder_rejects_cross_parent_drop_but_move_accepts_it() {
    let items = || {
        vec![
            item("a", 5, true, Some("parent-a")),
            item("b", 50, false, Some("parent-b")),
            item("c", 95, false, Some("parent-b")),
        ]
    };
    let mut reorder_only = runtime();
    reorder_only
        .render(collection(&[("onReorder", "reorder")], items()))
        .unwrap();
    start_drag(&mut reorder_only);
    let rejected = move_to(&mut reorder_only, 52.0, 30);
    assert_eq!(
        action_events(&rejected),
        [
            ("dragMove", NativeEventKind::DragMove),
            ("dropExit", NativeEventKind::DropExit),
        ]
    );
    assert!(drop_actions(&drop_at(&mut reorder_only, 52.0, 40)).is_empty());

    let mut movable = runtime();
    movable
        .render(collection(
            &[("onReorder", "reorder"), ("onCollectionMove", "move")],
            items(),
        ))
        .unwrap();
    start_drag(&mut movable);
    move_to(&mut movable, 52.0, 30);
    assert_eq!(drop_actions(&drop_at(&mut movable, 52.0, 40)), ["move"]);
}

#[test]
fn selected_descendants_are_filtered_and_cannot_receive_their_parent() {
    let mut runtime = runtime();
    runtime
        .render(collection(
            &[("onItemDrop", "itemDrop"), ("onCollectionMove", "move")],
            vec![
                item("parent", 5, true, None),
                item("child", 50, true, Some("parent")),
                item("sibling", 95, false, None),
            ],
        ))
        .unwrap();
    let started = start_drag(&mut runtime);
    assert_eq!(
        started.invocations[0]
            .context
            .drag
            .as_ref()
            .unwrap()
            .dragging_keys,
        ["parent"]
    );

    let rejected = move_to(&mut runtime, 65.0, 30);
    assert_eq!(
        action_events(&rejected),
        [("dragMove", NativeEventKind::DragMove)]
    );
    assert!(drop_actions(&drop_at(&mut runtime, 65.0, 40)).is_empty());
}

#[test]
fn internal_insert_and_root_callbacks_do_not_accept_the_collection() {
    let mut runtime = runtime();
    runtime
        .render(collection(
            &[("onInsert", "insert"), ("onRootDrop", "rootDrop")],
            flat_items(),
        ))
        .unwrap();
    start_drag(&mut runtime);
    let between = move_to(&mut runtime, 52.0, 30);
    assert_eq!(
        action_events(&between),
        [("dragMove", NativeEventKind::DragMove)]
    );
    let root = move_to(&mut runtime, 140.0, 40);
    assert_eq!(
        action_events(&root),
        [("dragMove", NativeEventKind::DragMove)]
    );
    assert!(drop_actions(&drop_at(&mut runtime, 140.0, 50)).is_empty());
}

#[test]
fn dynamic_collection_policy_filters_each_high_level_drop_item() {
    let mut runtime = runtime();
    runtime
        .render(dynamic_collection(&[
            ("onItemDrop", "itemDrop"),
            ("onCollectionMove", "move"),
        ]))
        .unwrap();
    start_drag(&mut runtime);

    let mut queries = Vec::<SelfDrawnDropPolicyQuery>::new();
    let mut resolver = |query: &SelfDrawnDropPolicyQuery| {
        queries.push(query.clone());
        let response = match &query.request {
            SelfDrawnDropPolicyRequest::ShouldAcceptItemDrop { types, .. } => {
                SelfDrawnDropPolicyResponse::accept_item_drop(
                    query,
                    types
                        .iter()
                        .any(|drag_type| drag_type == "application/json"),
                )
            }
            SelfDrawnDropPolicyRequest::GetDropOperation { .. } => {
                SelfDrawnDropPolicyResponse::drop_operation(query, SelfDrawnDropOperation::Move)
            }
        };
        SelfDrawnDropPolicyResolution::Resolved(response)
    };

    runtime
        .handle_event_with_drop_policy(
            pointer_event(PlatformPointerPhase::Moved, 15.0, 65.0, 30),
            &mut resolver,
        )
        .unwrap();
    let dropped = input(
        runtime
            .handle_event_with_drop_policy(
                pointer_event(PlatformPointerPhase::Released, 15.0, 65.0, 40),
                &mut resolver,
            )
            .unwrap(),
    );

    assert_eq!(drop_actions(&dropped), ["itemDrop", "move"]);
    let drop = dropped
        .invocations
        .iter()
        .find(|invocation| {
            invocation.event == NativeEventKind::Drop && invocation.action == "itemDrop"
        })
        .unwrap();
    let drag = drop.context.drag.as_ref().unwrap();
    assert_eq!(drag.items.len(), 1);
    assert_eq!(drag.items[0].get_text("application/json"), Some("c"));
    assert!(queries.iter().any(|query| matches!(
        &query.request,
        SelfDrawnDropPolicyRequest::ShouldAcceptItemDrop { types, .. }
            if types == &["text/plain".to_string()]
    )));
    assert!(queries.iter().any(|query| matches!(
        &query.request,
        SelfDrawnDropPolicyRequest::ShouldAcceptItemDrop { types, .. }
            if types == &["application/json".to_string()]
    )));
    assert!(queries
        .iter()
        .all(|query| query.policy_id == "acceptFolder" || query.policy_id == "chooseOperation"));
    assert_eq!(runtime.stats().drop_policy_failures, 0);
}

#[test]
fn low_level_collection_drop_bypasses_item_acceptance_and_receives_all_items() {
    let mut runtime = runtime();
    runtime
        .render(dynamic_collection(&[("onDrop", "lowDrop")]))
        .unwrap();

    let mut should_accept_types = Vec::<Vec<String>>::new();
    let mut resolver = |query: &SelfDrawnDropPolicyQuery| {
        let response = match &query.request {
            SelfDrawnDropPolicyRequest::ShouldAcceptItemDrop { types, .. } => {
                should_accept_types.push(types.clone());
                SelfDrawnDropPolicyResponse::accept_item_drop(query, false)
            }
            SelfDrawnDropPolicyRequest::GetDropOperation { .. } => {
                SelfDrawnDropPolicyResponse::drop_operation(query, SelfDrawnDropOperation::Move)
            }
        };
        SelfDrawnDropPolicyResolution::Resolved(response)
    };
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 15.0, 15.0, 10))
        .unwrap();
    runtime
        .handle_event_with_drop_policy(
            pointer_event(PlatformPointerPhase::Moved, 20.0, 15.0, 20),
            &mut resolver,
        )
        .unwrap();
    runtime
        .handle_event_with_drop_policy(
            pointer_event(PlatformPointerPhase::Moved, 15.0, 65.0, 30),
            &mut resolver,
        )
        .unwrap();
    let dropped = input(
        runtime
            .handle_event_with_drop_policy(
                pointer_event(PlatformPointerPhase::Released, 15.0, 65.0, 40),
                &mut resolver,
            )
            .unwrap(),
    );

    assert_eq!(drop_actions(&dropped), ["lowDrop"]);
    let drag = dropped
        .invocations
        .iter()
        .find(|invocation| invocation.action == "lowDrop")
        .unwrap()
        .context
        .drag
        .as_ref()
        .unwrap();
    assert_eq!(drag.items.len(), 2);
    assert!(should_accept_types.is_empty());
    assert_eq!(runtime.stats().drop_policy_failures, 0);
}

#[test]
fn missing_stale_and_disallowed_policy_answers_fail_closed() {
    let mut runtime = runtime();
    runtime
        .render(dynamic_collection(&[("onItemDrop", "itemDrop")]))
        .unwrap();
    start_drag(&mut runtime);

    let missing = move_to(&mut runtime, 65.0, 30);
    assert_eq!(
        action_events(&missing),
        [("dragMove", NativeEventKind::DragMove)]
    );
    assert_eq!(runtime.stats().drop_policy_queries, 1);
    assert_eq!(runtime.stats().drop_policy_failures, 1);

    let mut stale = |query: &SelfDrawnDropPolicyQuery| {
        let mut response = SelfDrawnDropPolicyResponse::accept_item_drop(query, true);
        response.event_sequence = response.event_sequence.saturating_sub(1);
        SelfDrawnDropPolicyResolution::Resolved(response)
    };
    runtime
        .handle_event_with_drop_policy(
            pointer_event(PlatformPointerPhase::Moved, 16.0, 65.0, 40),
            &mut stale,
        )
        .unwrap();
    assert_eq!(runtime.stats().drop_policy_failures, 2);

    let mut disallowed = |query: &SelfDrawnDropPolicyQuery| {
        let decision = match &query.request {
            SelfDrawnDropPolicyRequest::ShouldAcceptItemDrop { .. } => {
                SelfDrawnDropPolicyDecision::AcceptItemDrop { accepted: true }
            }
            SelfDrawnDropPolicyRequest::GetDropOperation { .. } => {
                SelfDrawnDropPolicyDecision::DropOperation {
                    operation: SelfDrawnDropOperation::Copy,
                }
            }
        };
        SelfDrawnDropPolicyResolution::Resolved(SelfDrawnDropPolicyResponse::for_query(
            query, decision,
        ))
    };
    let rejected = input(
        runtime
            .handle_event_with_drop_policy(
                pointer_event(PlatformPointerPhase::Moved, 17.0, 65.0, 50),
                &mut disallowed,
            )
            .unwrap(),
    );
    assert_eq!(
        action_events(&rejected),
        [("dragMove", NativeEventKind::DragMove)]
    );
    assert_eq!(runtime.stats().drop_policy_failures, 3);
}
