use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiOverlayArrowProps {
    pub class_name: String,
    pub placement: String,
}

pub fn ui_overlay_arrow(cx: &mut ComponentCx<UiOverlayArrowProps>) -> RSX {
    cx.use_prop("className", |props: &UiOverlayArrowProps| {
        props.class_name.clone()
    });
    cx.use_prop("placement", |props: &UiOverlayArrowProps| {
        props.placement.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            data-slot="overlay-arrow"
            data-placement={props.placement}
            class="size-2 rotate-45 border-l border-t border-hairline-strong bg-surface-card data-[placement=bottom]:border-b data-[placement=bottom]:border-r data-[placement=bottom]:border-l-0 data-[placement=bottom]:border-t-0"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
