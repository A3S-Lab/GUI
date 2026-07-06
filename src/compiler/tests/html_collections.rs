use super::support::*;

#[test]
fn lowers_html_table_structure_attributes_to_native_state() {
    let bridge = RsxCompilerBridge::new();
    let table = CompiledRsxNode::Element {
        key: "metrics".to_string(),
        tag: "table".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            CompiledRsxNode::Element {
                key: "metric-cols".to_string(),
                tag: "colgroup".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([("span".to_string(), "2".to_string())]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Element {
                    key: "metric-col".to_string(),
                    tag: "col".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([("span".to_string(), "3".to_string())]),
                        ..CompiledProps::default()
                    },
                    children: Vec::new(),
                }],
            },
            CompiledRsxNode::Element {
                key: "metric-row".to_string(),
                tag: "tr".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![
                    CompiledRsxNode::Element {
                        key: "metric-heading".to_string(),
                        tag: "th".to_string(),
                        import_source: None,
                        props: CompiledProps {
                            attributes: BTreeMap::from([
                                ("colSpan".to_string(), "2".to_string()),
                                ("rowspan".to_string(), "3".to_string()),
                                ("headers".to_string(), "quarter revenue".to_string()),
                                ("scope".to_string(), "colgroup".to_string()),
                                ("abbr".to_string(), "Rev".to_string()),
                            ]),
                            ..CompiledProps::default()
                        },
                        children: Vec::new(),
                    },
                    CompiledRsxNode::Element {
                        key: "metric-cell".to_string(),
                        tag: "td".to_string(),
                        import_source: None,
                        props: CompiledProps {
                            attributes: BTreeMap::from([
                                ("colspan".to_string(), "4".to_string()),
                                ("rowSpan".to_string(), "1".to_string()),
                                ("headers".to_string(), "metric-heading".to_string()),
                            ]),
                            ..CompiledProps::default()
                        },
                        children: Vec::new(),
                    },
                ],
            },
        ],
    };

    let native = bridge.lower_to_native(&table).unwrap();
    let native_colgroup = &native.children[0];
    let native_col = &native_colgroup.children[0];
    let native_heading = &native.children[1].children[0];
    let native_cell = &native.children[1].children[1];

    assert_eq!(native_colgroup.role, NativeRole::TableSection);
    assert_eq!(native_colgroup.props.html_collection.column_span, Some(2));
    assert_eq!(native_col.role, NativeRole::TableColumn);
    assert_eq!(native_col.props.html_collection.column_span, Some(3));
    assert_eq!(native_heading.role, NativeRole::TableCell);
    assert_eq!(native_heading.props.html_collection.column_span, Some(2));
    assert_eq!(native_heading.props.html_collection.row_span, Some(3));
    assert_eq!(
        native_heading.props.html_collection.headers.as_deref(),
        Some("quarter revenue")
    );
    assert_eq!(
        native_heading.props.html_collection.scope.as_deref(),
        Some("colgroup")
    );
    assert_eq!(
        native_heading.props.html_collection.cell_abbr.as_deref(),
        Some("Rev")
    );
    assert_eq!(native_cell.role, NativeRole::TableCell);
    assert_eq!(native_cell.props.html_collection.column_span, Some(4));
    assert_eq!(native_cell.props.html_collection.row_span, Some(1));
    assert_eq!(
        native_cell.props.html_collection.headers.as_deref(),
        Some("metric-heading")
    );
}

#[test]
fn lowers_html_list_structure_attributes_to_native_state() {
    let bridge = RsxCompilerBridge::new();
    let list = CompiledRsxNode::Element {
        key: "steps".to_string(),
        tag: "ol".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("start".to_string(), "5".to_string()),
                ("reversed".to_string(), String::new()),
                ("type".to_string(), "A".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: vec![
            CompiledRsxNode::Element {
                key: "step".to_string(),
                tag: "li".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([
                        ("value".to_string(), "7".to_string()),
                        ("type".to_string(), "i".to_string()),
                    ]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
            CompiledRsxNode::Element {
                key: "fallback-step".to_string(),
                tag: "li".to_string(),
                import_source: None,
                props: CompiledProps {
                    value: Some("8".to_string()),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
        ],
    };

    let native = bridge.lower_to_native(&list).unwrap();
    let native_item = &native.children[0];

    assert_eq!(native.role, NativeRole::ListBox);
    assert_eq!(native.props.html_collection.list_start, Some(5));
    assert!(native.props.html_collection.list_reversed);
    assert_eq!(native.props.html_collection.list_type.as_deref(), Some("A"));
    assert_eq!(native_item.role, NativeRole::ListBoxItem);
    assert_eq!(native_item.props.html_collection.list_item_value, Some(7));
    assert_eq!(
        native_item.props.html_collection.list_type.as_deref(),
        Some("i")
    );
    assert_eq!(
        native.children[1].props.html_collection.list_item_value,
        Some(8)
    );
}
