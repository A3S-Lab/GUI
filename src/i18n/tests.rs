use super::*;
use crate::native::{ElementKey, NativeProps, NativeRole};
use crate::renderer::MountedNodeSnapshot;
use crate::web::WebProps;

fn snapshot(node: u64, parent: Option<u64>, props: NativeProps) -> MountedNodeSnapshot {
    MountedNodeSnapshot {
        node: HostNodeId::new(node),
        parent: parent.map(HostNodeId::new),
        key: ElementKey::new(format!("node-{node}")),
        role: NativeRole::View,
        props,
    }
}

#[test]
fn locale_direction_recognizes_language_and_script_subtags() {
    assert_eq!(direction_for_locale("en-US"), TextDirection::Ltr);
    assert_eq!(direction_for_locale("ar-EG"), TextDirection::Rtl);
    assert_eq!(direction_for_locale("az-Arab"), TextDirection::Rtl);
    assert_eq!(direction_for_locale("az-Latn"), TextDirection::Ltr);
}

#[test]
fn mounted_context_inherits_and_allows_nested_overrides() {
    let records = vec![
        snapshot(1, None, NativeProps::new().lang("ar-EG")),
        snapshot(2, Some(1), NativeProps::new()),
        snapshot(3, Some(1), NativeProps::new().lang("en-GB").dir("rtl")),
    ];
    let mut manager = I18nManager::new();
    manager.sync(&records);

    assert_eq!(manager.locale(HostNodeId::new(2)), Some("ar-EG"));
    assert_eq!(manager.direction(HostNodeId::new(2)), TextDirection::Rtl);
    assert_eq!(manager.locale(HostNodeId::new(3)), Some("en-GB"));
    assert_eq!(manager.direction(HostNodeId::new(3)), TextDirection::Rtl);
}

#[test]
fn projection_applies_effective_context_to_native_descendants() {
    let mut root = NativeElement::new("root", NativeRole::View)
        .with_props(NativeProps::new().lang("ar-EG"))
        .child(NativeElement::new("child", NativeRole::Button).with_props(
            NativeProps::new().web(WebProps::new().attribute("lang", "").attribute("dir", "")),
        ));
    I18nManager::new().project_native_tree(&mut root);

    let child = &root.children[0].props;
    assert_eq!(child.lang.as_deref(), Some("ar-EG"));
    assert_eq!(child.dir.as_deref(), Some("rtl"));
    assert_eq!(
        child.web.attributes.get("dir").map(String::as_str),
        Some("rtl")
    );
}

#[test]
fn default_locale_can_seed_a_tree_without_an_explicit_provider() {
    let mut manager = I18nManager::new();
    manager.set_default_locale(Some("he-IL"));
    let mut root = NativeElement::new("root", NativeRole::View)
        .child(NativeElement::new("child", NativeRole::Text));

    manager.project_native_tree(&mut root);

    assert_eq!(root.children[0].props.lang.as_deref(), Some("he-IL"));
    assert_eq!(root.children[0].props.dir.as_deref(), Some("rtl"));
}
