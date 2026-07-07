use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSeparatorProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSeparatorProps {
    pub class_name: String,
    pub orientation: String,
}

pub fn ui_separator(cx: &mut ComponentCx<UiSeparatorProps>) -> RSX {
    cx.use_separator(|props: &UiSeparatorProps| {
        UseSeparatorProps::new().orientation(Some(props.orientation.clone()))
    });
    cx.use_prop("className", |props: &UiSeparatorProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Separator
            key="root"
            {...props.separatorProps}
            data-slot="separator"
            class="bg-hairline shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px"
            className={props.className}
        />
    )
}
