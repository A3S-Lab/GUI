use std::collections::BTreeSet;

use crate::rsx_app::{ComponentCx, RSX};
use crate::selection::{CollectionKey, Selection};
use crate::semantic_ui::UseTreeProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTreeProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub selected_keys: Option<Selection>,
    pub default_selected_keys: Option<Selection>,
    pub disabled_keys: BTreeSet<CollectionKey>,
    pub on_action: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
    pub selection_behavior: String,
    pub disabled_behavior: String,
    pub disallow_empty_selection: bool,
    pub should_focus_wrap: bool,
    pub escape_key_behavior: String,
    pub expanded_keys: Option<Selection>,
    pub default_expanded_keys: Option<Selection>,
    pub on_expanded_change: String,
    pub on_drag_start: String,
    pub on_drag_move: String,
    pub on_drag_end: String,
    pub on_drop: String,
    pub on_drop_enter: String,
    pub on_drop_move: String,
    pub on_drop_activate: String,
    pub on_drop_exit: String,
    pub on_root_drop: String,
    pub on_item_drop: String,
    pub on_insert: String,
    pub on_reorder: String,
    pub on_move: String,
    pub allowed_drop_operations: String,
    pub accepted_drag_types: String,
    pub drop_operation: String,
    pub get_drop_operation: String,
    pub should_accept_item_drop: String,
    pub drop_orientation: String,
}

pub fn ui_tree(cx: &mut ComponentCx<UiTreeProps>) -> RSX {
    cx.use_tree(|props: &UiTreeProps| {
        UseTreeProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .selected_keys(props.selected_keys.clone())
            .default_selected_keys(props.default_selected_keys.clone())
            .disabled_keys(props.disabled_keys.clone())
            .on_action(Some(props.on_action.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
            .selection_behavior(Some(props.selection_behavior.clone()))
            .disabled_behavior(Some(props.disabled_behavior.clone()))
            .disallow_empty_selection(props.disallow_empty_selection)
            .should_focus_wrap(props.should_focus_wrap)
            .escape_key_behavior(Some(props.escape_key_behavior.clone()))
            .expanded_keys(
                props
                    .expanded_keys
                    .as_ref()
                    .and_then(|keys| keys.explicit_keys().cloned()),
            )
            .default_expanded_keys(
                props
                    .default_expanded_keys
                    .as_ref()
                    .and_then(|keys| keys.explicit_keys().cloned()),
            )
            .on_expanded_change(Some(props.on_expanded_change.clone()))
    });
    cx.use_prop("className", |props: &UiTreeProps| props.class_name.clone());

    crate::rsx!(
        <Tree
            key="root"
            {...props.treeProps}
            data-slot="tree"
            data-draggable-collection="true"
            data-droppable-collection="true"
            data-drop-orientation={props.dropOrientation}
            allowedDropOperations={props.allowedDropOperations}
            acceptedDragTypes={props.acceptedDragTypes}
            dropOperation={props.dropOperation}
            data-get-drop-operation-policy={props.getDropOperation}
            data-should-accept-item-drop-policy={props.shouldAcceptItemDrop}
            onDragStart={props.onDragStart}
            onDragMove={props.onDragMove}
            onDragEnd={props.onDragEnd}
            onDrop={props.onDrop}
            onDropEnter={props.onDropEnter}
            onDropMove={props.onDropMove}
            onDropActivate={props.onDropActivate}
            onDropExit={props.onDropExit}
            onRootDrop={props.onRootDrop}
            onItemDrop={props.onItemDrop}
            onInsert={props.onInsert}
            onReorder={props.onReorder}
            onCollectionMove={props.onMove}
            class="grid gap-1"
            className={props.className}
        >
            <Slot key="content" />
        </Tree>
    )
}
