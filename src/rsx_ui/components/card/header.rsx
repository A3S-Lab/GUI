use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCardHeaderProps {
    pub class_name: String,
}

pub fn ui_card_header(cx: &mut ComponentCx<UiCardHeaderProps>) -> RSX {
    cx.use_prop("className", |props: &UiCardHeaderProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <div
            key="root"
            data-slot="card-header"
            class="@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-4"
            className={props.className}
        >
            <Slot key="content" />
        </div>
    )
}
