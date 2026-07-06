use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSwitchProps {
    pub class_name: String,
    pub on_change: String,
    pub is_checked: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
}

pub fn ui_switch(cx: &mut ComponentCx<UiSwitchProps>) -> RSX {
    cx.use_prop("className", |props: &UiSwitchProps| {
        props.class_name.clone()
    });
    cx.use_prop("onChange", |props: &UiSwitchProps| props.on_change.clone());
    cx.use_prop("isChecked", |props: &UiSwitchProps| props.is_checked);
    cx.use_prop("isDisabled", |props: &UiSwitchProps| props.is_disabled);
    cx.use_prop("isRequired", |props: &UiSwitchProps| props.is_required);
    cx.use_prop("isInvalid", |props: &UiSwitchProps| props.is_invalid);

    crate::rsx!(
        <Switch
            key="root"
            data-slot="switch"
            data-checked={props.isChecked}
            class="peer inline-flex h-5 w-9 shrink-0 items-center rounded-full border border-transparent bg-input shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[checked=true]:bg-primary aria-invalid:border-destructive aria-invalid:ring-destructive/20"
            className={props.className}
            checked={props.isChecked}
            disabled={props.isDisabled}
            required={props.isRequired}
            aria-invalid={props.isInvalid}
            onChange={props.onChange}
        >
            <Slot key="content" />
        </Switch>
    )
}
