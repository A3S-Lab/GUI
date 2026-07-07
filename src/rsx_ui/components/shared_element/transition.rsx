use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSharedElementTransitionProps {
    pub class_name: String,
    pub id: String,
    pub is_transitioning: bool,
}

pub fn ui_shared_element_transition(cx: &mut ComponentCx<UiSharedElementTransitionProps>) -> RSX {
    cx.use_prop("className", |props: &UiSharedElementTransitionProps| {
        props.class_name.clone()
    });
    cx.use_prop("id", |props: &UiSharedElementTransitionProps| {
        props.id.clone()
    });
    cx.use_prop(
        "isTransitioning",
        |props: &UiSharedElementTransitionProps| props.is_transitioning,
    );

    crate::rsx!(
        <Group
            key="root"
            data-slot="shared-element-transition"
            data-shared-element-id={props.id}
            data-transitioning={props.isTransitioning}
            class="contents"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
