use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiRouterProps {
    pub class_name: String,
    pub current_path: String,
}

pub fn ui_router(cx: &mut ComponentCx<UiRouterProps>) -> RSX {
    cx.use_prop("className", |props: &UiRouterProps| {
        props.class_name.clone()
    });
    cx.use_prop("currentPath", |props: &UiRouterProps| {
        props.current_path.clone()
    });

    crate::rsx!(
        <div
            key="root"
            data-slot="router"
            data-current-path={props.currentPath}
            class="grid min-h-0 gap-4"
            className={props.className}
        >
            <Slot key="content" />
        </div>
    )
}
