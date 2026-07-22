use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseFocusScopeProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiFocusScopeProps {
    pub class_name: String,
    pub contain: bool,
    pub restore_focus: bool,
    pub auto_focus: bool,
    pub is_disabled: bool,
    pub tab_index: i32,
}

impl Default for UiFocusScopeProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            contain: false,
            restore_focus: false,
            auto_focus: false,
            is_disabled: false,
            tab_index: -1,
        }
    }
}

pub fn ui_focus_scope(cx: &mut ComponentCx<UiFocusScopeProps>) -> RSX {
    cx.use_focus_scope(|props: &UiFocusScopeProps| {
        UseFocusScopeProps::new()
            .contain(props.contain)
            .restore_focus(props.restore_focus)
            .auto_focus(props.auto_focus)
            .disabled(props.is_disabled)
            .tab_index(props.tab_index)
    });
    cx.use_prop("className", |props: &UiFocusScopeProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.focusScopeProps}
            data-slot="focus-scope"
            data-contain={props.contain}
            data-restore-focus={props.restoreFocus}
            class="outline-none data-[contain=true]:isolate disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
