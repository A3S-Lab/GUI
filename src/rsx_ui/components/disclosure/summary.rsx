use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDisclosureProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDisclosureSummaryProps {
    pub class_name: String,
    pub on_press: Option<String>,
    pub is_disabled: bool,
    pub is_expanded: bool,
}

pub fn ui_disclosure_summary(cx: &mut ComponentCx<UiDisclosureSummaryProps>) -> RSX {
    cx.use_disclosure(|props: &UiDisclosureSummaryProps| {
        UseDisclosureProps::new()
            .expanded(props.is_expanded)
            .on_expanded_change(props.on_press.clone())
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiDisclosureSummaryProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <DisclosureSummary
            key="root"
            {...props.disclosureTriggerProps}
            data-slot="disclosure-summary"
            class="flex min-h-10 cursor-default items-center justify-between gap-3 rounded-md border border-hairline bg-canvas px-4 py-2 text-sm font-medium text-ink outline-none transition-colors active:bg-surface-strong focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </DisclosureSummary>
    )
}
