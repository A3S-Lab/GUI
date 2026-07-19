use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseKeyboardInteractionProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiKeyboardTargetProps {
    pub class_name: String,
    pub on_key_down: Option<String>,
    pub on_key_up: Option<String>,
    pub is_disabled: bool,
    pub is_keyboard_active: bool,
    pub tab_index: i32,
}

pub fn ui_keyboard_target(cx: &mut ComponentCx<UiKeyboardTargetProps>) -> RSX {
    cx.use_keyboard_interaction(|props: &UiKeyboardTargetProps| {
        UseKeyboardInteractionProps::new()
            .on_key_down(props.on_key_down.clone())
            .on_key_up(props.on_key_up.clone())
            .disabled(props.is_disabled)
            .keyboard_active(props.is_keyboard_active)
            .tab_index(props.tab_index)
    });
    cx.use_prop("className", |props: &UiKeyboardTargetProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.keyboardInteractionProps}
            data-slot="keyboard-target"
            data-keyboard-active={props.isKeyboardActive}
            class="outline-none data-[keyboard-active=true]:ring-[2px] data-[keyboard-active=true]:ring-ink/40 disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
