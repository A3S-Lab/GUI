use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiRouteProps {
    pub class_name: String,
    pub path: String,
    pub label: String,
    pub is_active: bool,
}

pub fn ui_route(cx: &mut ComponentCx<UiRouteProps>) -> RSX {
    cx.use_prop("className", |props: &UiRouteProps| props.class_name.clone());
    cx.use_prop("path", |props: &UiRouteProps| props.path.clone());
    cx.use_prop("label", |props: &UiRouteProps| props.label.clone());
    cx.use_prop("isActive", |props: &UiRouteProps| props.is_active);

    crate::rsx!(
        <Show key="active-route" when={props.isActive}>
            <section
                key="root"
                data-slot="route"
                data-route-path={props.path}
                data-active={props.isActive}
                aria-label={props.label}
                class="grid min-h-0 gap-4"
                className={props.className}
            >
                <Slot key="content" />
            </section>
        </Show>
    )
}
