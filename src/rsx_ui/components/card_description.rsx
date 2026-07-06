use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCardDescriptionProps {
    pub class_name: String,
}

pub fn ui_card_description(cx: &mut ComponentCx<UiCardDescriptionProps>) -> RSX {
    cx.use_prop("className", |props: &UiCardDescriptionProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <p
            key="root"
            data-slot="card-description"
            class="text-muted-foreground text-sm"
            className={props.className}
        >
            <Slot key="content" />
        </p>
    )
}
