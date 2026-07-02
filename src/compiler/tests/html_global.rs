use super::support::*;

#[test]
fn lowers_html_global_attributes_to_native_state() {
    let bridge = ReactCompilerBridge::new();
    let node = CompiledJsxNode::Element {
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
    assert_eq!(
        native
            .props
            .metadata
            .get(HTML_TAG_METADATA_KEY)
            .map(String::as_str),
        Some("section")
    );
}
