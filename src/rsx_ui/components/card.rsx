use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCardProps {
    pub class_name: String,
}

pub fn ui_card(cx: &mut ComponentCx<UiCardProps>) -> RSX {
    cx.use_prop("className", |props: &UiCardProps| props.class_name.clone());

    crate::rsx!(
        <div
            key="root"
            data-slot="card"
            class="bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm"
            className={props.className}
        >
            <Slot key="content" />
        </div>
    )
}
