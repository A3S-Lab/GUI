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
            class="flex flex-col gap-4 rounded-lg border border-hairline-strong bg-canvas p-6 text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </div>
    )
}
