use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiKeyboardProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_keyboard(cx: &mut ComponentCx<UiKeyboardProps>) -> RSX {
    cx.use_keyboard(|props: &UiKeyboardProps| {
        UseTextProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiKeyboardProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <KeyboardInput
            key="root"
            {...props.keyboardProps}
            data-slot="keyboard"
            class="ml-auto rounded border border-hairline bg-surface-strong px-1.5 py-0.5 font-mono text-[11px] text-body"
            className={props.className}
        >
            <Slot key="content" />
        </KeyboardInput>
    )
}
