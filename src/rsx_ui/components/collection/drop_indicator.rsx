use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDropIndicatorProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDropIndicatorProps {
    pub class_name: String,
    pub orientation: String,
    pub is_target: bool,
}

pub fn ui_drop_indicator(cx: &mut ComponentCx<UiDropIndicatorProps>) -> RSX {
    cx.use_drop_indicator(|props: &UiDropIndicatorProps| {
        UseDropIndicatorProps::new()
            .orientation(Some(props.orientation.clone()))
            .target(props.is_target)
    });
    cx.use_prop("className", |props: &UiDropIndicatorProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.dropIndicatorProps}
            data-slot="drop-indicator"
            class="bg-ink opacity-0 transition-opacity data-[target=true]:opacity-100 data-[orientation=horizontal]:h-0.5 data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-0.5"
            className={props.className}
        />
    )
}
