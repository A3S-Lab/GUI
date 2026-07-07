use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCardFooterProps {
    pub class_name: String,
}

pub fn ui_card_footer(cx: &mut ComponentCx<UiCardFooterProps>) -> RSX {
    cx.use_prop("className", |props: &UiCardFooterProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <div
            key="root"
            data-slot="card-footer"
            class="flex items-center [.border-t]:pt-4"
            className={props.className}
        >
            <Slot key="content" />
        </div>
    )
}
