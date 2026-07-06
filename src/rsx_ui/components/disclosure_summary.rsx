use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UsePressProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDisclosureSummaryProps {
    pub class_name: String,
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
}

pub fn ui_disclosure_summary(cx: &mut ComponentCx<UiDisclosureSummaryProps>) -> RSX {
    cx.use_press(|props: &UiDisclosureSummaryProps| {
        UsePressProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiDisclosureSummaryProps| {
        props.class_name.clone()
    });
    cx.use_prop("isDisabled", |props: &UiDisclosureSummaryProps| {
        props.is_disabled
    });

    crate::rsx!(
        <DisclosureSummary
            key="root"
            {...props.pressProps}
            data-slot="disclosure-summary"
            data-pressed={props.isPressed}
            class="flex min-h-10 cursor-default items-center justify-between gap-3 rounded-md px-4 py-2 text-sm font-medium outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </DisclosureSummary>
    )
}
