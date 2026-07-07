use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSwitchProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSwitchProps {
    pub class_name: String,
    pub on_change: String,
    pub is_checked: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_switch(cx: &mut ComponentCx<UiSwitchProps>) -> RSX {
    cx.use_switch(|props: &UiSwitchProps| {
        UseSwitchProps::new()
            .on_change(Some(props.on_change.clone()))
            .checked(props.is_checked)
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiSwitchProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Switch
            key="root"
            {...props.switchProps}
            data-slot="switch"
            class="peer inline-flex h-5 w-9 shrink-0 items-center rounded-full border border-hairline-strong bg-surface-strong transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[checked=true]:border-primary data-[checked=true]:bg-primary aria-invalid:border-semantic-error aria-invalid:ring-semantic-error/20"
            className={props.className}
        >
            <Slot key="content" />
        </Switch>
    )
}
