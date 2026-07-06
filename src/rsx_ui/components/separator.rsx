use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSeparatorProps {
    pub class_name: String,
    pub orientation: String,
}

pub fn ui_separator(cx: &mut ComponentCx<UiSeparatorProps>) -> RSX {
    cx.use_prop("className", |props: &UiSeparatorProps| {
        props.class_name.clone()
    });
    cx.use_prop("orientation", |props: &UiSeparatorProps| {
        props.orientation.clone()
    });

    crate::rsx!(
        <div
            key="root"
            data-slot="separator"
            data-orientation={props.orientation}
            class="bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px"
            className={props.className}
        />
    )
}
