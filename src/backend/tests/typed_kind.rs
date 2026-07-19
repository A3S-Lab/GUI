use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{
    AppKitAdapter, Gtk4Adapter, NativeTextInputKind, NativeWidgetKind, PlatformAdapter,
    WinUiAdapter,
};

#[test]
fn typed_widget_kind_maps_dynamic_flavors_before_backend_execution() {
    let textarea = NativeElement::new("body", NativeRole::TextField)
        .with_props(NativeProps::new().metadata("data-a3s-html-tag", "textarea"));
    let number = NativeElement::new("amount", NativeRole::TextField)
        .with_props(NativeProps::new().input_type("number"));
    let scroll = NativeElement::new("feed", NativeRole::View)
        .with_props(NativeProps::new().web(crate::web::WebProps::new().style("overflowY", "auto")));

    for adapter in [
        &AppKitAdapter as &dyn PlatformAdapter,
        &Gtk4Adapter as &dyn PlatformAdapter,
        &WinUiAdapter as &dyn PlatformAdapter,
    ] {
        assert_eq!(
            adapter.blueprint(&textarea).widget_kind,
            NativeWidgetKind::TextInput(NativeTextInputKind::Multiline)
        );
        assert_eq!(
            adapter.blueprint(&number).widget_kind,
            NativeWidgetKind::TextInput(NativeTextInputKind::Number)
        );
        assert_eq!(
            adapter.blueprint(&scroll).widget_kind,
            NativeWidgetKind::ScrollContainer
        );
    }
}

#[cfg(feature = "gtk4")]
#[test]
fn gtk_driver_ignores_legacy_class_string_when_typed_kind_is_present() {
    use crate::backend::NativeWidgetDriver;
    use crate::gtk4::{Gtk4WidgetDriver, Gtk4WidgetKind};
    use crate::host::HostNodeId;

    let element = NativeElement::new("save", NativeRole::Button);
    let id = HostNodeId::new(1);

    let mut gtk_blueprint = Gtk4Adapter.blueprint(&element);
    gtk_blueprint.widget_class = "invalid diagnostic class".to_string();
    let mut gtk = Gtk4WidgetDriver::default();
    gtk.create_widget(id, &gtk_blueprint).unwrap();
    assert_eq!(gtk.object(id).unwrap().kind, Gtk4WidgetKind::Button);
}

#[cfg(all(feature = "appkit", target_os = "macos"))]
#[test]
fn appkit_driver_ignores_legacy_class_string_when_typed_kind_is_present() {
    use crate::appkit::{AppKitWidgetDriver, AppKitWidgetKind};
    use crate::backend::NativeWidgetDriver;
    use crate::host::HostNodeId;

    let element = NativeElement::new("save", NativeRole::Button);
    let id = HostNodeId::new(1);
    let mut blueprint = AppKitAdapter.blueprint(&element);
    blueprint.widget_class = "invalid diagnostic class".to_string();
    let mut driver = AppKitWidgetDriver::default();
    driver.create_widget(id, &blueprint).unwrap();
    assert_eq!(driver.object(id).unwrap().kind, AppKitWidgetKind::Button);
}

#[cfg(feature = "winui")]
#[test]
fn winui_driver_ignores_legacy_class_string_when_typed_kind_is_present() {
    use crate::backend::NativeWidgetDriver;
    use crate::host::HostNodeId;
    use crate::winui::{WinUiWidgetDriver, WinUiWidgetKind};

    let element = NativeElement::new("save", NativeRole::Button);
    let id = HostNodeId::new(1);
    let mut blueprint = WinUiAdapter.blueprint(&element);
    blueprint.widget_class = "invalid diagnostic class".to_string();
    let mut driver = WinUiWidgetDriver::default();
    driver.create_widget(id, &blueprint).unwrap();
    assert_eq!(driver.object(id).unwrap().kind, WinUiWidgetKind::Button);
}
