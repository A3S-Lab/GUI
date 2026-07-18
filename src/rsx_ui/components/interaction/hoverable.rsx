use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseHoverProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiHoverableProps {
    pub class_name: String,
    pub on_hover_start: Option<String>,
    pub on_hover_end: Option<String>,
    pub on_hover_change: Option<String>,
    pub is_disabled: bool,
    pub is_hovered: bool,
}

pub fn ui_hoverable(cx: &mut ComponentCx<UiHoverableProps>) -> RSX {
    cx.use_hover(|props: &UiHoverableProps| {
        UseHoverProps::new()
            .on_hover_start(props.on_hover_start.clone())
            .on_hover_end(props.on_hover_end.clone())
            .on_hover_change(props.on_hover_change.clone())
            .disabled(props.is_disabled)
            .hovered(props.is_hovered)
    });
    cx.use_prop("className", |props: &UiHoverableProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.hoverProps}
            data-slot="hoverable"
            data-hovered={props.isHovered}
            class="outline-none data-[hovered=true]:bg-canvas-soft disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
