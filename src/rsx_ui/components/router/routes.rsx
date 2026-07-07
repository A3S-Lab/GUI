use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiRoutesProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_routes(cx: &mut ComponentCx<UiRoutesProps>) -> RSX {
    cx.use_prop("className", |props: &UiRoutesProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiRoutesProps| props.label.clone());

    crate::rsx!(
        <main
            key="root"
            data-slot="routes"
            aria-label={props.label}
            class="grid min-h-0 gap-4"
            className={props.className}
        >
            <Slot key="content" />
        </main>
    )
}
