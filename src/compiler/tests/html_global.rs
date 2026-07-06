use super::support::*;

#[test]
fn lowers_html_global_attributes_to_native_state() {
    let bridge = RsxCompilerBridge::new();
    let node = CompiledRsxNode::Element {
        key: "panel".to_string(),
        tag: "section".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("title".to_string(), "Profile summary".to_string()),
                ("hidden".to_string(), String::new()),
                ("lang".to_string(), "en-US".to_string()),
                ("dir".to_string(), "rtl".to_string()),
                ("tabIndex".to_string(), "-1".to_string()),
                ("role".to_string(), "region".to_string()),
                ("accessKey".to_string(), "p".to_string()),
                ("contentEditable".to_string(), "plaintext-only".to_string()),
                ("draggable".to_string(), "true".to_string()),
                ("spellCheck".to_string(), "false".to_string()),
                ("translate".to_string(), "no".to_string()),
                ("inert".to_string(), String::new()),
                ("popover".to_string(), String::new()),
                ("anchor".to_string(), "profile-card-anchor".to_string()),
                ("is".to_string(), "profile-card".to_string()),
                ("nonce".to_string(), "nonce-1".to_string()),
                ("slot".to_string(), "summary".to_string()),
                ("part".to_string(), "panel header".to_string()),
                (
                    "exportParts".to_string(),
                    "header: panel-header".to_string(),
                ),
                ("itemScope".to_string(), String::new()),
                ("itemProp".to_string(), "profile".to_string()),
                (
                    "itemType".to_string(),
                    "https://schema.org/ProfilePage".to_string(),
                ),
                (
                    "itemID".to_string(),
                    "https://example.test/profiles/1".to_string(),
                ),
                (
                    "itemRef".to_string(),
                    "profile-name profile-email".to_string(),
                ),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };

    let native = bridge.lower_to_native(&node).unwrap();

    assert_eq!(native.role, NativeRole::Section);
    assert_eq!(native.props.title.as_deref(), Some("Profile summary"));
    assert!(native.props.hidden);
    assert_eq!(native.props.lang.as_deref(), Some("en-US"));
    assert_eq!(native.props.dir.as_deref(), Some("rtl"));
    assert_eq!(native.props.tab_index, Some(-1));
    assert_eq!(native.props.explicit_role.as_deref(), Some("region"));
    assert_eq!(native.props.access_key.as_deref(), Some("p"));
    assert_eq!(
        native.props.content_editable.as_deref(),
        Some("plaintext-only")
    );
    assert_eq!(native.props.draggable.as_deref(), Some("true"));
    assert_eq!(native.props.spell_check, Some(false));
    assert_eq!(native.props.translate, Some(false));
    assert!(native.props.inert);
    assert_eq!(native.props.popover.as_deref(), Some("auto"));
    assert_eq!(native.props.anchor.as_deref(), Some("profile-card-anchor"));
    assert_eq!(
        native.props.custom_element_is.as_deref(),
        Some("profile-card")
    );
    assert_eq!(native.props.nonce.as_deref(), Some("nonce-1"));
    assert_eq!(
        native.props.html_shadow.slot_name.as_deref(),
        Some("summary")
    );
    assert_eq!(
        native.props.html_shadow.part.as_deref(),
        Some("panel header")
    );
    assert_eq!(
        native.props.html_shadow.export_parts.as_deref(),
        Some("header: panel-header")
    );
    assert!(native.props.html_microdata.item_scope);
    assert_eq!(
        native.props.html_microdata.item_prop.as_deref(),
        Some("profile")
    );
    assert_eq!(
        native.props.html_microdata.item_type.as_deref(),
        Some("https://schema.org/ProfilePage")
    );
    assert_eq!(
        native.props.html_microdata.item_id.as_deref(),
        Some("https://example.test/profiles/1")
    );
    assert_eq!(
        native.props.html_microdata.item_ref.as_deref(),
        Some("profile-name profile-email")
    );
    assert_eq!(
        native
            .props
            .metadata
            .get(HTML_TAG_METADATA_KEY)
            .map(String::as_str),
        Some("section")
    );
}
