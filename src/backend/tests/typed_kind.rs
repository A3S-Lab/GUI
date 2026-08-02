use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{HeadlessAdapter, NativeTextInputKind, NativeWidgetKind, PlatformAdapter};

#[test]
fn typed_widget_kind_maps_dynamic_flavors_before_backend_execution() {
    let textarea = NativeElement::new("body", NativeRole::TextField)
        .with_props(NativeProps::new().metadata("data-a3s-html-tag", "textarea"));
    let number = NativeElement::new("amount", NativeRole::TextField)
        .with_props(NativeProps::new().input_type("number"));
    let scroll = NativeElement::new("feed", NativeRole::View)
        .with_props(NativeProps::new().web(crate::web::WebProps::new().style("overflowY", "auto")));

    assert_eq!(
        HeadlessAdapter.blueprint(&textarea).widget_kind,
        NativeWidgetKind::TextInput(NativeTextInputKind::Multiline)
    );
    assert_eq!(
        HeadlessAdapter.blueprint(&number).widget_kind,
        NativeWidgetKind::TextInput(NativeTextInputKind::Number)
    );
    assert_eq!(
        HeadlessAdapter.blueprint(&scroll).widget_kind,
        NativeWidgetKind::ScrollContainer
    );
}
