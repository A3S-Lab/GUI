use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiPopoverProps {
    pub class_name: String,
}

pub fn ui_popover(cx: &mut ComponentCx<UiPopoverProps>) -> RSX {
    cx.use_prop("className", |props: &UiPopoverProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Popover
            key="root"
            data-slot="popover"
            class="z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md"
            className={props.className}
        >
            <Slot key="content" />
        </Popover>
    )
}
