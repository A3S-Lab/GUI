use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSelectValueProps {
    pub class_name: String,
    pub value: String,
    pub placeholder: String,
}

pub fn ui_select_value(cx: &mut ComponentCx<UiSelectValueProps>) -> RSX {
    cx.use_prop("className", |props: &UiSelectValueProps| {
        props.class_name.clone()
    });
    cx.use_prop("value", |props: &UiSelectValueProps| props.value.clone());
    cx.use_prop("placeholder", |props: &UiSelectValueProps| {
        props.placeholder.clone()
    });

    crate::rsx!(
        <SelectValue
            key="root"
            data-slot="select-value"
            class="line-clamp-1 flex-1 text-left text-sm"
            className={props.className}
            value={props.value}
            placeholder={props.placeholder}
        >
            <Slot key="content" />
        </SelectValue>
    )
}
