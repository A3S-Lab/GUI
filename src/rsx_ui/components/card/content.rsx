use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCardContentProps {
    pub class_name: String,
}

pub fn ui_card_content(cx: &mut ComponentCx<UiCardContentProps>) -> RSX {
    cx.use_prop("className", |props: &UiCardContentProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <div
            key="root"
            data-slot="card-content"
            class=""
            className={props.className}
        >
            <Slot key="content" />
        </div>
    )
}
