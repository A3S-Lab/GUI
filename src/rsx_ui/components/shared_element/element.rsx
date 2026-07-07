use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSharedElementProps {
    pub class_name: String,
    pub id: String,
}

pub fn ui_shared_element(cx: &mut ComponentCx<UiSharedElementProps>) -> RSX {
    cx.use_prop("className", |props: &UiSharedElementProps| {
        props.class_name.clone()
    });
    cx.use_prop("id", |props: &UiSharedElementProps| props.id.clone());

    crate::rsx!(
        <Group
            key="root"
            data-slot="shared-element"
            data-shared-element-id={props.id}
            class="contents"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
