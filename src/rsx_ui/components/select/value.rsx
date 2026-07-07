use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSelectDisplayProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSelectValueProps {
    pub class_name: String,
    pub value: String,
    pub placeholder: String,
}

pub fn ui_select_value(cx: &mut ComponentCx<UiSelectValueProps>) -> RSX {
    cx.use_select_display(|props: &UiSelectValueProps| {
        UseSelectDisplayProps::new()
            .value(Some(props.value.clone()))
            .placeholder(Some(props.placeholder.clone()))
    });
    cx.use_prop("className", |props: &UiSelectValueProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <SelectValue
            key="root"
            {...props.selectValueProps}
            data-slot="select-value"
            class="line-clamp-1 flex-1 text-left text-sm"
            className={props.className}
        >
            <Slot key="content" />
        </SelectValue>
    )
}
