use super::support::*;

#[test]
fn lowers_html_document_metadata_template_and_slot_tags_to_native_roles() {
    let bridge = RsxCompilerBridge::new();
    let document = CompiledRsxNode::Element {
        key: "document".to_string(),
        tag: "html".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            CompiledRsxNode::Element {
                key: "head".to_string(),
                tag: "head".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![
                    CompiledRsxNode::Element {
                        key: "title".to_string(),
                        tag: "title".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledRsxNode::Text {
                            key: "title-text".to_string(),
                            value: "Dashboard".to_string(),
                        }],
                    },
                    CompiledRsxNode::Element {
                        key: "base".to_string(),
                        tag: "base".to_string(),
                        import_source: None,
                        props: CompiledProps {
                            attributes: BTreeMap::from([(
                                "href".to_string(),
                                "https://example.test/".to_string(),
                            )]),
                            ..CompiledProps::default()
                        },
                        children: Vec::new(),
                    },
                    CompiledRsxNode::Element {
                        key: "description".to_string(),
                        tag: "meta".to_string(),
                        import_source: None,
                        props: CompiledProps {
                            attributes: BTreeMap::from([
                                ("name".to_string(), "description".to_string()),
                                ("content".to_string(), "Native dashboard".to_string()),
                            ]),
                            ..CompiledProps::default()
                        },
                        children: Vec::new(),
                    },
                    CompiledRsxNode::Element {
                        key: "stylesheet".to_string(),
                        tag: "link".to_string(),
                        import_source: None,
                        props: CompiledProps {
                            attributes: BTreeMap::from([
                                ("rel".to_string(), "stylesheet".to_string()),
                                ("href".to_string(), "/app.css".to_string()),
                            ]),
                            ..CompiledProps::default()
                        },
                        children: Vec::new(),
                    },
                    CompiledRsxNode::Element {
                        key: "style".to_string(),
                        tag: "style".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledRsxNode::Text {
                            key: "style-text".to_string(),
                            value: ".card{display:grid}".to_string(),
                        }],
                    },
                    CompiledRsxNode::Element {
                        key: "script".to_string(),
                        tag: "script".to_string(),
                        import_source: None,
                        props: CompiledProps {
                            attributes: BTreeMap::from([(
                                "src".to_string(),
                                "/app.js".to_string(),
                            )]),
                            ..CompiledProps::default()
                        },
                        children: Vec::new(),
                    },
                    CompiledRsxNode::Element {
                        key: "noscript".to_string(),
                        tag: "noscript".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledRsxNode::Text {
                            key: "noscript-text".to_string(),
                            value: "JavaScript is disabled".to_string(),
                        }],
                    },
                    CompiledRsxNode::Element {
                        key: "card-template".to_string(),
                        tag: "template".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledRsxNode::Element {
                            key: "template-card".to_string(),
                            tag: "div".to_string(),
                            import_source: None,
                            props: CompiledProps::default(),
                            children: Vec::new(),
                        }],
                    },
                ],
            },
            CompiledRsxNode::Element {
                key: "body".to_string(),
                tag: "body".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![
                    CompiledRsxNode::Element {
                        key: "hero-heading".to_string(),
                        tag: "hgroup".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![
                            CompiledRsxNode::Element {
                                key: "headline".to_string(),
                                tag: "h1".to_string(),
                                import_source: None,
                                props: CompiledProps::default(),
                                children: vec![CompiledRsxNode::Text {
                                    key: "headline-text".to_string(),
                                    value: "Dashboard".to_string(),
                                }],
                            },
                            CompiledRsxNode::Element {
                                key: "tagline".to_string(),
                                tag: "p".to_string(),
                                import_source: None,
                                props: CompiledProps::default(),
                                children: vec![CompiledRsxNode::Text {
                                    key: "tagline-text".to_string(),
                                    value: "Operational summary".to_string(),
                                }],
                            },
                        ],
                    },
                    CompiledRsxNode::Element {
                        key: "actions-slot".to_string(),
                        tag: "slot".to_string(),
                        import_source: None,
                        props: CompiledProps {
                            attributes: BTreeMap::from([(
                                "name".to_string(),
                                "actions".to_string(),
                            )]),
                            ..CompiledProps::default()
                        },
                        children: Vec::new(),
                    },
                ],
            },
        ],
    };

    let native = bridge.lower_to_native(&document).unwrap();
    assert_eq!(native.role, NativeRole::Document);
    assert_eq!(native.children[0].role, NativeRole::DocumentHead);
    assert_eq!(
        native.children[0].children[0].role,
        NativeRole::DocumentTitle
    );
    assert_eq!(
        native.children[0].children[0].props.label.as_deref(),
        Some("Dashboard")
    );
    assert_eq!(native.children[0].children[1].role, NativeRole::Metadata);
    assert_eq!(
        native.children[0].children[2]
            .props
            .web
            .attributes
            .get("content")
            .map(String::as_str),
        Some("Native dashboard")
    );
    assert_eq!(
        native.children[0].children[3].role,
        NativeRole::ResourceLink
    );
    assert_eq!(native.children[0].children[4].role, NativeRole::StyleSheet);
    assert_eq!(native.children[0].children[5].role, NativeRole::Script);
    assert_eq!(native.children[0].children[6].role, NativeRole::Script);
    assert_eq!(native.children[0].children[7].role, NativeRole::Template);
    assert_eq!(native.children[1].role, NativeRole::DocumentBody);
    assert_eq!(
        native.children[1].children[0].role,
        NativeRole::HeadingGroup
    );
    assert_eq!(
        native.children[1].children[0].props.label.as_deref(),
        Some("Dashboard")
    );
    assert_eq!(
        native.children[1].children[0].children[0].role,
        NativeRole::Heading
    );
    assert_eq!(native.children[1].children[1].role, NativeRole::Slot);
    assert_eq!(
        native.children[1].children[1]
            .props
            .web
            .attributes
            .get("name")
            .map(String::as_str),
        Some("actions")
    );
}

#[test]
fn lowers_html_ruby_annotation_tags_to_native_roles() {
    let bridge = RsxCompilerBridge::new();
    let ruby = CompiledRsxNode::Element {
        key: "ruby".to_string(),
        tag: "ruby".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            CompiledRsxNode::Element {
                key: "base".to_string(),
                tag: "rb".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledRsxNode::Text {
                    key: "base-text".to_string(),
                    value: "漢".to_string(),
                }],
            },
            CompiledRsxNode::Element {
                key: "open-parenthesis".to_string(),
                tag: "rp".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledRsxNode::Text {
                    key: "open-parenthesis-text".to_string(),
                    value: "(".to_string(),
                }],
            },
            CompiledRsxNode::Element {
                key: "text".to_string(),
                tag: "rt".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledRsxNode::Text {
                    key: "text-value".to_string(),
                    value: "kan".to_string(),
                }],
            },
            CompiledRsxNode::Element {
                key: "close-parenthesis".to_string(),
                tag: "rp".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledRsxNode::Text {
                    key: "close-parenthesis-text".to_string(),
                    value: ")".to_string(),
                }],
            },
            CompiledRsxNode::Element {
                key: "container".to_string(),
                tag: "rtc".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledRsxNode::Element {
                    key: "alternate-text".to_string(),
                    tag: "rt".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledRsxNode::Text {
                        key: "alternate-text-value".to_string(),
                        value: "Han".to_string(),
                    }],
                }],
            },
        ],
    };

    let native = bridge.lower_to_native(&ruby).unwrap();
    assert_eq!(native.role, NativeRole::Ruby);
    assert_eq!(native.props.label.as_deref(), Some("漢"));
    assert_eq!(native.children[0].role, NativeRole::RubyBase);
    assert_eq!(native.children[0].props.label.as_deref(), Some("漢"));
    assert_eq!(native.children[1].role, NativeRole::RubyParenthesis);
    assert_eq!(native.children[1].props.label.as_deref(), Some("("));
    assert_eq!(native.children[2].role, NativeRole::RubyText);
    assert_eq!(native.children[2].props.label.as_deref(), Some("kan"));
    assert_eq!(native.children[3].role, NativeRole::RubyParenthesis);
    assert_eq!(native.children[3].props.label.as_deref(), Some(")"));
    assert_eq!(native.children[4].role, NativeRole::RubyTextContainer);
    assert_eq!(native.children[4].props.label.as_deref(), Some("Han"));
    assert_eq!(native.children[4].children[0].role, NativeRole::RubyText);
}

#[test]
fn lowers_html_text_annotation_tags_to_native_roles() {
    fn text_annotation(key: &str, tag: &str, text: &str) -> CompiledRsxNode {
        CompiledRsxNode::Element {
            key: key.to_string(),
            tag: tag.to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![CompiledRsxNode::Text {
                key: format!("{key}-text"),
                value: text.to_string(),
            }],
        }
    }

    let bridge = RsxCompilerBridge::new();
    let root = CompiledRsxNode::Element {
        key: "annotations".to_string(),
        tag: "div".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            CompiledRsxNode::Element {
                key: "abbr".to_string(),
                tag: "abbr".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([(
                        "title".to_string(),
                        "HyperText Markup Language".to_string(),
                    )]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Text {
                    key: "abbr-text".to_string(),
                    value: "HTML".to_string(),
                }],
            },
            text_annotation("cite", "cite", "Spec"),
            text_annotation("dfn", "dfn", "Term"),
            CompiledRsxNode::Element {
                key: "data".to_string(),
                tag: "data".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([("value".to_string(), "42".to_string())]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Text {
                    key: "data-text".to_string(),
                    value: "Answer".to_string(),
                }],
            },
            CompiledRsxNode::Element {
                key: "ins".to_string(),
                tag: "ins".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([
                        (
                            "cite".to_string(),
                            "https://example.test/changes#added".to_string(),
                        ),
                        ("dateTime".to_string(), "2026-07-02T09:00:00Z".to_string()),
                    ]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Text {
                    key: "ins-text".to_string(),
                    value: "added".to_string(),
                }],
            },
            CompiledRsxNode::Element {
                key: "del".to_string(),
                tag: "del".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([
                        (
                            "cite".to_string(),
                            "https://example.test/changes#removed".to_string(),
                        ),
                        ("datetime".to_string(), "2026-07-01T18:00:00Z".to_string()),
                    ]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Text {
                    key: "del-text".to_string(),
                    value: "removed".to_string(),
                }],
            },
            text_annotation("mark", "mark", "highlight"),
            CompiledRsxNode::Element {
                key: "time".to_string(),
                tag: "time".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([(
                        "datetime".to_string(),
                        "2026-07-02".to_string(),
                    )]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Text {
                    key: "time-text".to_string(),
                    value: "Today".to_string(),
                }],
            },
        ],
    };

    let native = bridge.lower_to_native(&root).unwrap();
    assert_eq!(native.role, NativeRole::View);
    let expected = [
        (NativeRole::Abbreviation, "HTML"),
        (NativeRole::Citation, "Spec"),
        (NativeRole::Definition, "Term"),
        (NativeRole::DataValue, "Answer"),
        (NativeRole::InsertedText, "added"),
        (NativeRole::DeletedText, "removed"),
        (NativeRole::MarkedText, "highlight"),
        (NativeRole::Time, "Today"),
    ];
    for (index, (role, label)) in expected.iter().enumerate() {
        assert_eq!(native.children[index].role, *role);
        assert_eq!(native.children[index].props.label.as_deref(), Some(*label));
    }
    assert_eq!(native.children[3].props.value.as_deref(), Some("42"));
    assert_eq!(
        native.children[0]
            .props
            .web
            .attributes
            .get("title")
            .map(String::as_str),
        Some("HyperText Markup Language")
    );
    assert_eq!(
        native.children[3]
            .props
            .web
            .attributes
            .get("value")
            .map(String::as_str),
        Some("42")
    );
    assert_eq!(
        native.children[7]
            .props
            .html_text_annotation
            .date_time
            .as_deref(),
        Some("2026-07-02")
    );
    assert_eq!(
        native.children[4]
            .props
            .html_text_annotation
            .cite
            .as_deref(),
        Some("https://example.test/changes#added")
    );
    assert_eq!(
        native.children[4]
            .props
            .html_text_annotation
            .date_time
            .as_deref(),
        Some("2026-07-02T09:00:00Z")
    );
    assert_eq!(
        native.children[5]
            .props
            .html_text_annotation
            .cite
            .as_deref(),
        Some("https://example.test/changes#removed")
    );
    assert_eq!(
        native.children[5]
            .props
            .html_text_annotation
            .date_time
            .as_deref(),
        Some("2026-07-01T18:00:00Z")
    );
    assert_eq!(
        native.children[7]
            .props
            .web
            .attributes
            .get("datetime")
            .map(String::as_str),
        Some("2026-07-02")
    );
}

#[test]
fn lowers_html_phrasing_text_tags_to_native_roles() {
    fn phrasing(key: &str, tag: &str, text: &str) -> CompiledRsxNode {
        CompiledRsxNode::Element {
            key: key.to_string(),
            tag: tag.to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![CompiledRsxNode::Text {
                key: format!("{key}-text"),
                value: text.to_string(),
            }],
        }
    }

    let bridge = RsxCompilerBridge::new();
    let root = CompiledRsxNode::Element {
        key: "phrasing".to_string(),
        tag: "p".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            phrasing("em", "em", "emphasized"),
            phrasing("strong", "strong", "important"),
            phrasing("code", "code", "let value = 1;"),
            phrasing("kbd", "kbd", "Command K"),
            phrasing("samp", "samp", "OK"),
            phrasing("var", "var", "x"),
            CompiledRsxNode::Element {
                key: "quote".to_string(),
                tag: "q".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([(
                        "cite".to_string(),
                        "https://example.test/spec".to_string(),
                    )]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Text {
                    key: "quote-text".to_string(),
                    value: "quoted".to_string(),
                }],
            },
            phrasing("sub", "sub", "2"),
            phrasing("sup", "sup", "3"),
            phrasing("small", "small", "fine print"),
            phrasing("b", "b", "attention"),
            phrasing("i", "i", "idiomatic"),
            phrasing("s", "s", "obsolete"),
            phrasing("u", "u", "annotation"),
            phrasing("bdi", "bdi", "مرحبا"),
            CompiledRsxNode::Element {
                key: "bdo".to_string(),
                tag: "bdo".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([("dir".to_string(), "rtl".to_string())]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Text {
                    key: "bdo-text".to_string(),
                    value: "abc".to_string(),
                }],
            },
        ],
    };

    let native = bridge.lower_to_native(&root).unwrap();
    assert_eq!(native.role, NativeRole::Paragraph);
    let expected = [
        (NativeRole::Emphasis, "emphasized"),
        (NativeRole::StrongText, "important"),
        (NativeRole::Code, "let value = 1;"),
        (NativeRole::KeyboardInput, "Command K"),
        (NativeRole::SampleOutput, "OK"),
        (NativeRole::Variable, "x"),
        (NativeRole::InlineQuote, "quoted"),
        (NativeRole::Subscript, "2"),
        (NativeRole::Superscript, "3"),
        (NativeRole::SmallText, "fine print"),
        (NativeRole::BoldText, "attention"),
        (NativeRole::ItalicText, "idiomatic"),
        (NativeRole::StruckText, "obsolete"),
        (NativeRole::UnderlinedText, "annotation"),
        (NativeRole::BidirectionalIsolate, "مرحبا"),
        (NativeRole::BidirectionalOverride, "abc"),
    ];
    for (index, (role, label)) in expected.iter().enumerate() {
        assert_eq!(native.children[index].role, *role);
        assert_eq!(native.children[index].props.label.as_deref(), Some(*label));
    }
    assert_eq!(
        native.children[6]
            .props
            .html_text_annotation
            .cite
            .as_deref(),
        Some("https://example.test/spec")
    );
    assert_eq!(
        native.children[6]
            .props
            .web
            .attributes
            .get("cite")
            .map(String::as_str),
        Some("https://example.test/spec")
    );
    assert_eq!(
        native.children[15]
            .props
            .web
            .attributes
            .get("dir")
            .map(String::as_str),
        Some("rtl")
    );
}

#[test]
fn lowers_html_flow_and_legacy_text_tags_to_native_roles() {
    fn flow(key: &str, tag: &str, text: &str) -> CompiledRsxNode {
        CompiledRsxNode::Element {
            key: key.to_string(),
            tag: tag.to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![CompiledRsxNode::Text {
                key: format!("{key}-text"),
                value: text.to_string(),
            }],
        }
    }

    let bridge = RsxCompilerBridge::new();
    let root = CompiledRsxNode::Element {
        key: "flow".to_string(),
        tag: "div".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            flow("paragraph", "p", "Paragraph"),
            flow("pre", "pre", "line 1\nline 2"),
            CompiledRsxNode::Element {
                key: "blockquote".to_string(),
                tag: "blockquote".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([(
                        "cite".to_string(),
                        "https://example.test/quote".to_string(),
                    )]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Element {
                    key: "quote-p".to_string(),
                    tag: "p".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledRsxNode::Text {
                        key: "quote-p-text".to_string(),
                        value: "Quoted paragraph".to_string(),
                    }],
                }],
            },
            flow("address", "address", "help@example.test"),
            CompiledRsxNode::Element {
                key: "break".to_string(),
                tag: "br".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: Vec::new(),
            },
            CompiledRsxNode::Element {
                key: "word-break".to_string(),
                tag: "wbr".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: Vec::new(),
            },
            flow("nobr", "nobr", "No break"),
            flow("center", "center", "Centered"),
            CompiledRsxNode::Element {
                key: "font".to_string(),
                tag: "font".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([("color".to_string(), "red".to_string())]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Text {
                    key: "font-text".to_string(),
                    value: "Font text".to_string(),
                }],
            },
            flow("big", "big", "Big"),
            flow("tt", "tt", "Teletype"),
            flow("listing", "listing", "Legacy listing"),
            flow("plaintext", "plaintext", "Plain text"),
            flow("xmp", "xmp", "Example"),
            flow("basefont", "basefont", "Base font"),
            CompiledRsxNode::Element {
                key: "directory".to_string(),
                tag: "dir".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledRsxNode::Element {
                    key: "directory-item".to_string(),
                    tag: "li".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledRsxNode::Text {
                        key: "directory-item-text".to_string(),
                        value: "Item".to_string(),
                    }],
                }],
            },
        ],
    };

    let native = bridge.lower_to_native(&root).unwrap();
    assert_eq!(native.role, NativeRole::View);
    let expected = [
        (NativeRole::Paragraph, "Paragraph"),
        (NativeRole::PreformattedText, "line 1\nline 2"),
        (NativeRole::BlockQuote, "Quoted paragraph"),
        (NativeRole::ContactAddress, "help@example.test"),
        (NativeRole::LineBreak, ""),
        (NativeRole::WordBreakOpportunity, ""),
        (NativeRole::NoBreakText, "No break"),
        (NativeRole::CenteredText, "Centered"),
        (NativeRole::FontText, "Font text"),
        (NativeRole::BigText, "Big"),
        (NativeRole::TeletypeText, "Teletype"),
        (NativeRole::PreformattedText, "Legacy listing"),
        (NativeRole::PreformattedText, "Plain text"),
        (NativeRole::PreformattedText, "Example"),
        (NativeRole::FontText, "Base font"),
        (NativeRole::ListBox, ""),
    ];
    for (index, (role, label)) in expected.iter().enumerate() {
        assert_eq!(native.children[index].role, *role);
        if label.is_empty() {
            assert_eq!(native.children[index].props.label, None);
        } else {
            assert_eq!(native.children[index].props.label.as_deref(), Some(*label));
        }
    }
    assert_eq!(
        native.children[15].children[0].props.label.as_deref(),
        Some("Item")
    );
    assert_eq!(
        native.children[2]
            .props
            .html_text_annotation
            .cite
            .as_deref(),
        Some("https://example.test/quote")
    );
    assert_eq!(
        native.children[2]
            .props
            .web
            .attributes
            .get("cite")
            .map(String::as_str),
        Some("https://example.test/quote")
    );
    assert_eq!(native.children[2].children[0].role, NativeRole::Paragraph);
    assert_eq!(
        native.children[8]
            .props
            .web
            .attributes
            .get("color")
            .map(String::as_str),
        Some("red")
    );
    assert_eq!(
        native.children[15].children[0].role,
        NativeRole::ListBoxItem
    );
}

#[test]
fn lowers_html_remaining_legacy_and_foreign_tags_to_native_roles() {
    fn container(key: &str, tag: &str, text: &str) -> CompiledRsxNode {
        CompiledRsxNode::Element {
            key: key.to_string(),
            tag: tag.to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![CompiledRsxNode::Text {
                key: format!("{key}-text"),
                value: text.to_string(),
            }],
        }
    }

    let bridge = RsxCompilerBridge::new();
    let root = CompiledRsxNode::Element {
        key: "legacy".to_string(),
        tag: "div".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            CompiledRsxNode::Element {
                key: "applet".to_string(),
                tag: "applet".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([("code".to_string(), "Demo.class".to_string())]),
                    ..CompiledProps::default()
                },
                children: vec![CompiledRsxNode::Text {
                    key: "applet-text".to_string(),
                    value: "Applet fallback".to_string(),
                }],
            },
            CompiledRsxNode::Element {
                key: "bgsound".to_string(),
                tag: "bgsound".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([("src".to_string(), "/tone.wav".to_string())]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
            CompiledRsxNode::Element {
                key: "frameset".to_string(),
                tag: "frameset".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledRsxNode::Element {
                    key: "frame".to_string(),
                    tag: "frame".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([(
                            "src".to_string(),
                            "/legacy-frame.html".to_string(),
                        )]),
                        ..CompiledProps::default()
                    },
                    children: Vec::new(),
                }],
            },
            container("noembed", "noembed", "No embed fallback"),
            container("noframes", "noframes", "No frames fallback"),
            container("marquee", "marquee", "Moving text"),
            container("math", "math", "x+y"),
            CompiledRsxNode::Element {
                key: "nextid".to_string(),
                tag: "nextid".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([("n".to_string(), "z42".to_string())]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
            container("selected-content", "selectedcontent", "Selected option"),
        ],
    };

    let native = bridge.lower_to_native(&root).unwrap();
    assert_eq!(native.role, NativeRole::View);
    let expected = [
        (NativeRole::Applet, Some("Applet fallback")),
        (NativeRole::BackgroundSound, None),
        (NativeRole::FrameSet, None),
        (NativeRole::NoEmbedFallback, Some("No embed fallback")),
        (NativeRole::NoFramesFallback, Some("No frames fallback")),
        (NativeRole::Marquee, Some("Moving text")),
        (NativeRole::Math, Some("x+y")),
        (NativeRole::NextId, None),
        (NativeRole::SelectedContent, Some("Selected option")),
    ];
    for (index, (role, label)) in expected.iter().enumerate() {
        assert_eq!(native.children[index].role, *role);
        assert_eq!(native.children[index].props.label.as_deref(), *label);
    }
    assert_eq!(
        native.children[0]
            .props
            .web
            .attributes
            .get("code")
            .map(String::as_str),
        Some("Demo.class")
    );
    assert_eq!(
        native.children[1]
            .props
            .web
            .attributes
            .get("src")
            .map(String::as_str),
        Some("/tone.wav")
    );
    assert_eq!(native.children[2].children[0].role, NativeRole::Frame);
    assert_eq!(
        native.children[2].children[0]
            .props
            .web
            .attributes
            .get("src")
            .map(String::as_str),
        Some("/legacy-frame.html")
    );
    assert_eq!(
        native.children[7]
            .props
            .web
            .attributes
            .get("n")
            .map(String::as_str),
        Some("z42")
    );
}
