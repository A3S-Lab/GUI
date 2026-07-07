use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseVirtualizerProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiVirtualizerProps {
    pub class_name: String,
    pub label: String,
    pub layout: String,
    pub orientation: String,
    pub item_count: usize,
    pub estimated_item_size: u32,
    pub visible_start: usize,
    pub visible_end: usize,
    pub overscan: usize,
    pub gap: u32,
    pub padding: u32,
    pub is_scrolling: bool,
    pub is_disabled: bool,
    pub tab_index: i32,
}

pub fn ui_virtualizer(cx: &mut ComponentCx<UiVirtualizerProps>) -> RSX {
    cx.use_virtualizer(|props: &UiVirtualizerProps| {
        UseVirtualizerProps::new()
            .label(Some(props.label.clone()))
            .layout(Some(props.layout.clone()))
            .orientation(Some(props.orientation.clone()))
            .item_count(props.item_count)
            .estimated_item_size(props.estimated_item_size)
            .visible_start(props.visible_start)
            .visible_end(props.visible_end)
            .overscan(props.overscan)
            .gap(props.gap)
            .padding(props.padding)
            .scrolling(props.is_scrolling)
            .disabled(props.is_disabled)
            .tab_index(props.tab_index)
    });
    cx.use_prop("className", |props: &UiVirtualizerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.virtualizerProps}
            data-slot="virtualizer"
            data-scrolling={props.isScrolling}
            data-disabled={props.isDisabled}
            class="grid min-h-0 overflow-auto rounded-md border border-hairline-strong bg-canvas text-ink outline-none transition-colors data-[scrolling=true]:border-ring data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
