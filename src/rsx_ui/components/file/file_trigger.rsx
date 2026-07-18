use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseFileTriggerProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFileTriggerProps {
    pub class_name: String,
    pub on_press: Option<String>,
    pub on_select: String,
    pub accepted_file_types: String,
    pub allows_multiple: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
}

pub fn ui_file_trigger(cx: &mut ComponentCx<UiFileTriggerProps>) -> RSX {
    cx.use_file_trigger(|props: &UiFileTriggerProps| {
        UseFileTriggerProps::new()
            .on_press(props.on_press.clone())
            .on_select(Some(props.on_select.clone()))
            .accepted_file_types(Some(props.accepted_file_types.clone()))
            .allows_multiple(props.allows_multiple)
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiFileTriggerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <button
            key="root"
            {...props.fileTriggerProps}
            data-slot="file-trigger"
            class="inline-flex h-9 items-center justify-center gap-2 whitespace-nowrap rounded-md border border-hairline-strong bg-surface-card px-3 py-1.5 text-sm font-medium leading-none text-ink disabled:pointer-events-none disabled:text-muted-soft [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none active:bg-surface-strong focus-visible:ring-[2px] focus-visible:ring-ink/40 aria-invalid:border-semantic-error has-[>svg]:px-3"
            className={props.className}
        >
            <Slot key="content" />
        </button>
    )
}
