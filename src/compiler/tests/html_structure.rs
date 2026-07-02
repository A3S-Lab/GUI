use super::support::*;

#[test]
fn lowers_html_embedded_media_and_table_tags_to_native_roles() {
    let bridge = ReactCompilerBridge::new();
    let img = CompiledJsxNode::Element {
        key: "hero".to_string(),
        tag: "img".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("alt".to_string(), "Product screenshot".to_string()),
                ("src".to_string(), "/hero.png".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };

    let native_img = bridge.lower_to_native(&img).unwrap();
    assert_eq!(native_img.role, NativeRole::Image);
    assert_eq!(
        native_img.props.label.as_deref(),
        Some("Product screenshot")
    );
    assert_eq!(
        native_img
            .props
            .metadata
            .get(HTML_TAG_METADATA_KEY)
            .map(String::as_str),
        Some("img")
    );

    let video = CompiledJsxNode::Element {
        key: "demo-video".to_string(),
        tag: "video".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![CompiledJsxNode::Element {
            key: "demo-source".to_string(),
            tag: "source".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([("src".to_string(), "/demo.mp4".to_string())]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        }],
    };

    let native_video = bridge.lower_to_native(&video).unwrap();
    assert_eq!(native_video.role, NativeRole::Media);
    assert_eq!(native_video.children.len(), 1);
    assert_eq!(native_video.children[0].role, NativeRole::EmbeddedContent);

    let table = CompiledJsxNode::Element {
        key: "metrics".to_string(),
        tag: "table".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            CompiledJsxNode::Element {
                key: "metrics-caption".to_string(),
                tag: "caption".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: "metrics-caption-text".to_string(),
                    value: "Metrics".to_string(),
                }],
            },
            CompiledJsxNode::Element {
                key: "metrics-body".to_string(),
                tag: "tbody".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Element {
                    key: "metrics-row".to_string(),
                    tag: "tr".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Element {
                        key: "metrics-cell".to_string(),
                        tag: "td".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledJsxNode::Text {
                            key: "metrics-cell-text".to_string(),
                            value: "42".to_string(),
                        }],
                    }],
                }],
            },
        ],
    };

    let native_table = bridge.lower_to_native(&table).unwrap();
    assert_eq!(native_table.role, NativeRole::Table);
    assert_eq!(native_table.children[0].role, NativeRole::TableCaption);
    assert_eq!(
        native_table.children[0].props.label.as_deref(),
        Some("Metrics")
    );
    assert_eq!(native_table.children[1].role, NativeRole::TableSection);
    assert_eq!(
        native_table.children[1].children[0].role,
        NativeRole::TableRow
    );
    assert_eq!(
        native_table.children[1].children[0].children[0].role,
        NativeRole::TableCell
    );
    assert_eq!(
        native_table.children[1].children[0].children[0]
            .props
            .label
            .as_deref(),
        Some("42")
    );
}

#[test]
fn lowers_html_sectioning_landmark_and_heading_tags_to_native_roles() {
    let bridge = ReactCompilerBridge::new();
    let tree = CompiledJsxNode::Element {
        key: "main".to_string(),
        tag: "main".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            CompiledJsxNode::Element {
                key: "top-nav".to_string(),
                tag: "nav".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([(
                        "aria-label".to_string(),
                        "Primary navigation".to_string(),
                    )]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
            CompiledJsxNode::Element {
                key: "article".to_string(),
                tag: "article".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![
                    CompiledJsxNode::Element {
                        key: "headline".to_string(),
                        tag: "h1".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledJsxNode::Text {
                            key: "headline-text".to_string(),
                            value: "Release notes".to_string(),
                        }],
                    },
                    CompiledJsxNode::Element {
                        key: "summary".to_string(),
                        tag: "section".to_string(),
                        import_source: None,
                        props: CompiledProps {
                            attributes: BTreeMap::from([(
                                "aria-label".to_string(),
                                "Summary".to_string(),
                            )]),
                            ..CompiledProps::default()
                        },
                        children: Vec::new(),
                    },
                ],
            },
            CompiledJsxNode::Element {
                key: "related".to_string(),
                tag: "aside".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: Vec::new(),
            },
            CompiledJsxNode::Element {
                key: "search".to_string(),
                tag: "search".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: Vec::new(),
            },
        ],
    };

    let native = bridge.lower_to_native(&tree).unwrap();
    assert_eq!(native.role, NativeRole::Main);
    assert_eq!(
        native
            .props
            .metadata
            .get(HTML_TAG_METADATA_KEY)
            .map(String::as_str),
        Some("main")
    );
    assert_eq!(native.children[0].role, NativeRole::Navigation);
    assert_eq!(
        native.children[0].props.label.as_deref(),
        Some("Primary navigation")
    );
    assert_eq!(native.children[1].role, NativeRole::Article);
    assert_eq!(native.children[1].children[0].role, NativeRole::Heading);
    assert_eq!(
        native.children[1].children[0].props.label.as_deref(),
        Some("Release notes")
    );
    assert_eq!(native.children[1].children[1].role, NativeRole::Section);
    assert_eq!(
        native.children[1].children[1].props.label.as_deref(),
        Some("Summary")
    );
    assert_eq!(native.children[2].role, NativeRole::Aside);
    assert_eq!(native.children[3].role, NativeRole::Search);
}

#[test]
fn lowers_html_disclosure_figure_and_description_list_tags_to_native_roles() {
    let bridge = ReactCompilerBridge::new();
    let disclosure = CompiledJsxNode::Element {
        key: "release-notes".to_string(),
        tag: "details".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([("open".to_string(), String::new())]),
            ..CompiledProps::default()
        },
        children: vec![
            CompiledJsxNode::Element {
                key: "release-summary".to_string(),
                tag: "summary".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: "release-summary-text".to_string(),
                    value: "Release details".to_string(),
                }],
            },
            CompiledJsxNode::Element {
                key: "release-body".to_string(),
                tag: "p".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: "release-body-text".to_string(),
                    value: "Native semantic roles are preserved.".to_string(),
                }],
            },
        ],
    };

    let native_disclosure = bridge.lower_to_native(&disclosure).unwrap();
    assert_eq!(native_disclosure.role, NativeRole::Disclosure);
    assert_eq!(native_disclosure.props.expanded, Some(true));
    assert_eq!(
        native_disclosure
            .props
            .metadata
            .get(HTML_TAG_METADATA_KEY)
            .map(String::as_str),
        Some("details")
    );
    assert_eq!(
        native_disclosure.children[0].role,
        NativeRole::DisclosureSummary
    );
    assert_eq!(
        native_disclosure.children[0].props.label.as_deref(),
        Some("Release details")
    );

    let figure = CompiledJsxNode::Element {
        key: "chart".to_string(),
        tag: "figure".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            CompiledJsxNode::Element {
                key: "chart-image".to_string(),
                tag: "img".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([("alt".to_string(), "Revenue chart".to_string())]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
            CompiledJsxNode::Element {
                key: "chart-caption".to_string(),
                tag: "figcaption".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: "chart-caption-text".to_string(),
                    value: "Revenue by quarter".to_string(),
                }],
            },
        ],
    };

    let native_figure = bridge.lower_to_native(&figure).unwrap();
    assert_eq!(native_figure.role, NativeRole::Figure);
    assert_eq!(native_figure.children[0].role, NativeRole::Image);
    assert_eq!(
        native_figure.children[0].props.label.as_deref(),
        Some("Revenue chart")
    );
    assert_eq!(native_figure.children[1].role, NativeRole::FigureCaption);
    assert_eq!(
        native_figure.children[1].props.label.as_deref(),
        Some("Revenue by quarter")
    );

    let description_list = CompiledJsxNode::Element {
        key: "terms".to_string(),
        tag: "dl".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            CompiledJsxNode::Element {
                key: "term".to_string(),
                tag: "dt".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: "term-text".to_string(),
                    value: "IR".to_string(),
                }],
            },
            CompiledJsxNode::Element {
                key: "details".to_string(),
                tag: "dd".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: "details-text".to_string(),
                    value: "Intermediate representation".to_string(),
                }],
            },
        ],
    };

    let native_description_list = bridge.lower_to_native(&description_list).unwrap();
    assert_eq!(native_description_list.role, NativeRole::DescriptionList);
    assert_eq!(
        native_description_list.children[0].role,
        NativeRole::DescriptionTerm
    );
    assert_eq!(
        native_description_list.children[0].props.label.as_deref(),
        Some("IR")
    );
    assert_eq!(
        native_description_list.children[1].role,
        NativeRole::DescriptionDetails
    );
    assert_eq!(
        native_description_list.children[1].props.label.as_deref(),
        Some("Intermediate representation")
    );
}
