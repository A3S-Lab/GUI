use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseFocusWithinProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFocusWithinProps {
    pub class_name: String,
    pub on_focus_within: Option<String>,
    pub on_blur_within: Option<String>,
    pub on_focus_within_change: Option<String>,
    pub is_disabled: bool,
    pub is_focus_within: bool,
}

pub fn ui_focus_within(cx: &mut ComponentCx<UiFocusWithinProps>) -> RSX {
    cx.use_focus_within(|props: &UiFocusWithinProps| {
        UseFocusWithinProps::new()
            .on_focus_within(props.on_focus_within.clone())
            .on_blur_within(props.on_blur_within.clone())
            .on_focus_within_change(props.on_focus_within_change.clone())
            .disabled(props.is_disabled)
            .focus_within(props.is_focus_within)
    });
    cx.use_prop("className", |props: &UiFocusWithinProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.focusWithinProps}
            data-slot="focus-within"
            data-focus-within={props.isFocusWithin}
            class="outline-none data-[focus-within=true]:ring-[2px] data-[focus-within=true]:ring-ink/30 disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
