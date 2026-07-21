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
            class="flex min-h-9 cursor-default items-center justify-between gap-3 rounded-md border border-hairline bg-surface-card px-3 py-1.5 text-sm font-medium text-ink outline-none active:bg-surface-strong focus-visible:ring-[2px] focus-visible:ring-ink/40 disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </DisclosureSummary>
    )
}
