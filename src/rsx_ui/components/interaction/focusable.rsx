use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseFocusableProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFocusableProps {
    pub class_name: String,
    pub on_focus: Option<String>,
    pub on_blur: Option<String>,
    pub on_focus_change: Option<String>,
    pub is_disabled: bool,
    pub is_focused: bool,
    pub auto_focus: bool,
    pub tab_index: i32,
}

pub fn ui_focusable(cx: &mut ComponentCx<UiFocusableProps>) -> RSX {
    cx.use_focusable(|props: &UiFocusableProps| {
        UseFocusableProps::new()
            .on_focus(props.on_focus.clone())
            .on_blur(props.on_blur.clone())
            .on_focus_change(props.on_focus_change.clone())
            .disabled(props.is_disabled)
            .focused(props.is_focused)
            .auto_focus(props.auto_focus)
            .tab_index(props.tab_index)
    });
    cx.use_prop("className", |props: &UiFocusableProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.focusProps}
            data-slot="focusable"
            data-focused={props.isFocused}
            class="outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:text-muted-soft data-[focused=true]:ring-[3px] data-[focused=true]:ring-ring/50"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
