use super::support::*;

#[test]
fn lowers_html_label_and_output_associations_to_native_state() {
    let bridge = RsxCompilerBridge::new();
    let root = CompiledRsxNode::Element {
        key: "form-associations".to_string(),
        tag: "form".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            element(
                "email-label",
                "label",
                BTreeMap::from([("htmlFor".to_string(), "email".to_string())]),
            ),
            element(
                "phone-label",
                "label",
                BTreeMap::from([("for".to_string(), "phone".to_string())]),
            ),
            element(
                "price-output",
                "output",
                BTreeMap::from([("for".to_string(), "price quantity".to_string())]),
            ),
        ],
    };

    let native = bridge.lower_to_native(&root).unwrap();
    let email_label = &native.children[0];
    let phone_label = &native.children[1];
    let price_output = &native.children[2];

    assert_eq!(email_label.role, NativeRole::Text);
    assert_eq!(
        email_label.props.html_form_association.label_for.as_deref(),
        Some("email")
    );
    assert_eq!(phone_label.role, NativeRole::Text);
    assert_eq!(
        phone_label.props.html_form_association.label_for.as_deref(),
        Some("phone")
    );
    assert_eq!(price_output.role, NativeRole::Output);
    assert_eq!(
        price_output
            .props
            .html_form_association
            .output_for
            .as_deref(),
        Some("price quantity")
    );
}

#[test]
fn lowers_html_meter_range_metadata_to_native_state() {
    let bridge = RsxCompilerBridge::new();
    let meter = element(
        "quota",
        "meter",
        BTreeMap::from([
            ("min".to_string(), "0".to_string()),
            ("max".to_string(), "100".to_string()),
            ("value".to_string(), "73".to_string()),
            ("low".to_string(), "25".to_string()),
            ("high".to_string(), "90".to_string()),
            ("optimum".to_string(), "75".to_string()),
        ]),
    );

    let native = bridge.lower_to_native(&meter).unwrap();

    assert_eq!(native.role, NativeRole::Meter);
    assert_eq!(native.props.min, Some(0.0));
    assert_eq!(native.props.max, Some(100.0));
    assert_eq!(native.props.current, Some(73.0));
    assert_eq!(native.props.html_form_association.meter_low, Some(25.0));
    assert_eq!(native.props.html_form_association.meter_high, Some(90.0));
    assert_eq!(native.props.html_form_association.meter_optimum, Some(75.0));
}

fn element(key: &str, tag: &str, attributes: BTreeMap<String, String>) -> CompiledRsxNode {
    CompiledRsxNode::Element {
        key: key.to_string(),
        tag: tag.to_string(),
        import_source: None,
        props: CompiledProps {
            attributes,
            ..CompiledProps::default()
        },
        children: Vec::new(),
    }
}
