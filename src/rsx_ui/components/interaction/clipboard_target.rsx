use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseClipboardProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiClipboardTargetProps {
    pub class_name: String,
    pub label: String,
    pub on_copy: Option<String>,
    pub on_cut: Option<String>,
    pub on_paste: Option<String>,
    pub copy_value: String,
    pub copy_mime_type: String,
    pub accepted_mime_types: String,
    pub is_disabled: bool,
}

pub fn ui_clipboard_target(cx: &mut ComponentCx<UiClipboardTargetProps>) -> RSX {
    cx.use_clipboard(|props: &UiClipboardTargetProps| {
        UseClipboardProps::new()
            .on_copy(props.on_copy.clone())
            .on_cut(props.on_cut.clone())
            .on_paste(props.on_paste.clone())
            .copy_value(Some(props.copy_value.clone()))
            .copy_mime_type(Some(props.copy_mime_type.clone()))
            .accepted_mime_types(Some(props.accepted_mime_types.clone()))
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiClipboardTargetProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiClipboardTargetProps| {
        props.label.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.clipboardProps}
            data-slot="clipboard-target"
            aria-label={props.label}
            data-clipboard-disabled={props.isClipboardDisabled}
            class="outline-none transition-colors focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:text-muted-soft data-[clipboard-disabled=true]:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
