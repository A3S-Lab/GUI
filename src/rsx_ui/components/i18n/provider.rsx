use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseI18nProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiI18nProviderProps {
    pub class_name: String,
    pub locale: String,
    pub direction: String,
}

pub fn ui_i18n_provider(cx: &mut ComponentCx<UiI18nProviderProps>) -> RSX {
    cx.use_i18n(|props: &UiI18nProviderProps| {
        UseI18nProps::new()
            .locale(Some(props.locale.clone()))
            .direction(Some(props.direction.clone()))
    });
    cx.use_prop("className", |props: &UiI18nProviderProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.i18nProps}
            data-slot="i18n-provider"
            data-rtl={props.isRtl}
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
