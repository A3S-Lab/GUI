use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCardTitleProps {
    pub class_name: String,
}

pub fn ui_card_title(cx: &mut ComponentCx<UiCardTitleProps>) -> RSX {
    cx.use_prop("className", |props: &UiCardTitleProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <h3
            key="root"
            data-slot="card-title"
            class="text-lg font-semibold leading-7 text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </h3>
    )
}
