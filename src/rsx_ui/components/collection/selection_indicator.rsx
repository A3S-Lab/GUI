use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSelectionIndicatorProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSelectionIndicatorProps {
    pub class_name: String,
    pub label: String,
    pub is_selected: bool,
}

pub fn ui_selection_indicator(cx: &mut ComponentCx<UiSelectionIndicatorProps>) -> RSX {
    cx.use_selection_indicator(|props: &UiSelectionIndicatorProps| {
        UseSelectionIndicatorProps::new()
            .label(Some(props.label.clone()))
            .selected(props.is_selected)
    });
    cx.use_prop("className", |props: &UiSelectionIndicatorProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Text
            key="root"
            {...props.selectionIndicatorProps}
            data-slot="selection-indicator"
            class="flex size-4 shrink-0 items-center justify-center text-current opacity-0 data-[selected=true]:opacity-100"
            className={props.className}
        >
            <Slot key="content" />
        </Text>
    )
}
