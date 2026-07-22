use super::*;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{BlueprintHost, Gtk4Adapter, PlatformPlanningHost};
use crate::style::TextDirection;

#[test]
fn runtime_projects_inherited_locale_and_direction_to_native_widgets() {
    let tree = NativeElement::new("root", NativeRole::View)
        .with_props(NativeProps::new().lang("ar-EG"))
        .child(NativeElement::new("save", NativeRole::Button))
        .child(
            NativeElement::new("english", NativeRole::View)
                .with_props(NativeProps::new().lang("en-GB"))
                .child(NativeElement::new("description", NativeRole::Text)),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    let root = runtime.render_native(&tree).unwrap();
    let children = runtime.renderer.child_ids(root);
    let save = children[0];
    let description = runtime.renderer.child_ids(children[1])[0];

    assert_eq!(runtime.i18n().locale(save), Some("ar-EG"));
    assert_eq!(runtime.i18n().direction(save), TextDirection::Rtl);
    assert_eq!(runtime.i18n().locale(description), Some("en-GB"));
    assert_eq!(runtime.i18n().direction(description), TextDirection::Ltr);
    let save_state = &runtime.host().blueprint(save).unwrap().control_state;
    assert_eq!(save_state.lang.as_deref(), Some("ar-EG"));
    assert_eq!(save_state.dir.as_deref(), Some("rtl"));
    let description_state = &runtime.host().blueprint(description).unwrap().control_state;
    assert_eq!(description_state.lang.as_deref(), Some("en-GB"));
    assert_eq!(description_state.dir.as_deref(), Some("ltr"));
}

#[test]
fn runtime_default_locale_seeds_unscoped_trees() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.i18n_mut().set_default_locale(Some("fa-IR"));
    let root = runtime
        .render_native(&NativeElement::new("root", NativeRole::Button))
        .unwrap();

    assert_eq!(runtime.i18n().locale(root), Some("fa-IR"));
    assert_eq!(runtime.i18n().direction(root), TextDirection::Rtl);
    let state = &runtime.host().blueprint(root).unwrap().control_state;
    assert_eq!(state.lang.as_deref(), Some("fa-IR"));
    assert_eq!(state.dir.as_deref(), Some("rtl"));
}
