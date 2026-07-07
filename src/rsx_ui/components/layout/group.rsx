use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseGroupProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiGroupProps {
    pub class_name: String,
    pub label: String,
    pub on_hover_start: Option<String>,
    pub on_hover_end: Option<String>,
    pub on_hover_change: Option<String>,
    pub on_focus: Option<String>,
    pub on_blur: Option<String>,
    pub on_focus_change: Option<String>,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
    pub is_focus_within: bool,
    pub auto_focus: bool,
    pub tab_index: i32,
}

pub fn ui_group(cx: &mut ComponentCx<UiGroupProps>) -> RSX {
    cx.use_group(|props: &UiGroupProps| {
        UseGroupProps::new()
            .label(Some(props.label.clone()))
            .on_hover_start(props.on_hover_start.clone())
            .on_hover_end(props.on_hover_end.clone())
            .on_hover_change(props.on_hover_change.clone())
            .on_focus(props.on_focus.clone())
            .on_blur(props.on_blur.clone())
            .on_focus_change(props.on_focus_change.clone())
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
            .hovered(props.is_hovered)
            .focused(props.is_focused)
            .focus_visible(props.is_focus_visible)
            .focus_within(props.is_focus_within)
            .auto_focus(props.auto_focus)
            .tab_index(props.tab_index)
    });
    cx.use_prop("className", |props: &UiGroupProps| props.class_name.clone());

    crate::rsx!(
        <Group
            key="root"
            {...props.groupProps}
            data-slot="group"
            data-disabled={props.isDisabled}
            data-invalid={props.isInvalid}
            data-readonly={props.isReadOnly}
            data-hovered={props.isHovered}
            data-focused={props.isFocused}
            data-focus-visible={props.isFocusVisible}
            data-focus-within={props.isFocusWithin}
            class="grid gap-2 outline-none transition-colors data-[hovered=true]:bg-canvas-soft data-[focus-within=true]:ring-[3px] data-[focus-within=true]:ring-ring/30 data-[focus-visible=true]:ring-[3px] data-[focus-visible=true]:ring-ring/50 data-[disabled=true]:pointer-events-none data-[disabled=true]:text-muted-soft data-[invalid=true]:text-semantic-error data-[readonly=true]:opacity-80"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
