use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseFocusRingProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFocusRingProps {
    pub class_name: String,
    pub on_focus: Option<String>,
    pub on_blur: Option<String>,
    pub on_focus_change: Option<String>,
    pub is_disabled: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
    pub is_focus_within: bool,
    pub auto_focus: bool,
    pub tab_index: i32,
}

pub fn ui_focus_ring(cx: &mut ComponentCx<UiFocusRingProps>) -> RSX {
    cx.use_focus_ring(|props: &UiFocusRingProps| {
        UseFocusRingProps::new()
            .on_focus(props.on_focus.clone())
            .on_blur(props.on_blur.clone())
            .on_focus_change(props.on_focus_change.clone())
            .disabled(props.is_disabled)
            .focused(props.is_focused)
            .focus_visible(props.is_focus_visible)
            .focus_within(props.is_focus_within)
            .auto_focus(props.auto_focus)
            .tab_index(props.tab_index)
    });
    cx.use_prop("className", |props: &UiFocusRingProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.focusRingProps}
            data-slot="focus-ring"
            data-focused={props.isFocused}
            data-focus-visible={props.isFocusVisible}
            data-focus-within={props.isFocusWithin}
            class="outline-none transition-shadow data-[focus-visible=true]:ring-[3px] data-[focus-visible=true]:ring-ring/50 data-[focus-visible=true]:ring-offset-2 data-[focus-within=true]:ring-[3px] data-[focus-within=true]:ring-ring/30 disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
