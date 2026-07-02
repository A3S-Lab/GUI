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
fn lowers_html_media_and_resource_attributes_to_native_state() {
    let bridge = ReactCompilerBridge::new();
    let picture = CompiledJsxNode::Element {
        key: "picture".to_string(),
        tag: "picture".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("width".to_string(), "640".to_string()),
                ("height".to_string(), "360".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: vec![
            CompiledJsxNode::Element {
                key: "hero-source".to_string(),
                tag: "source".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([
                        (
                            "srcSet".to_string(),
                            "/hero.avif 1x, /hero@2x.avif 2x".to_string(),
                        ),
                        (
                            "sizes".to_string(),
                            "(min-width: 60rem) 50vw, 100vw".to_string(),
                        ),
                        ("media".to_string(), "(min-width: 48rem)".to_string()),
                        ("type".to_string(), "image/avif".to_string()),
                        ("width".to_string(), "640".to_string()),
                        ("height".to_string(), "360".to_string()),
                    ]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
            CompiledJsxNode::Element {
                key: "hero-img".to_string(),
                tag: "img".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([
                        ("alt".to_string(), "Hero".to_string()),
                        ("src".to_string(), "/hero.png".to_string()),
                        (
                            "srcset".to_string(),
                            "/hero.png 1x, /hero@2x.png 2x".to_string(),
                        ),
                        ("sizes".to_string(), "100vw".to_string()),
                        ("width".to_string(), "640".to_string()),
                        ("height".to_string(), "360".to_string()),
                        ("loading".to_string(), "lazy".to_string()),
                        ("decoding".to_string(), "async".to_string()),
                        ("fetchPriority".to_string(), "high".to_string()),
                        ("crossOrigin".to_string(), "anonymous".to_string()),
                        ("referrerPolicy".to_string(), "no-referrer".to_string()),
                    ]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
        ],
    };
    let video = CompiledJsxNode::Element {
        key: "demo-video".to_string(),
        tag: "video".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("src".to_string(), "/demo.mp4".to_string()),
                ("poster".to_string(), "/poster.png".to_string()),
                ("width".to_string(), "1280".to_string()),
                ("height".to_string(), "720".to_string()),
                ("controls".to_string(), String::new()),
                ("autoplay".to_string(), String::new()),
                ("loop".to_string(), String::new()),
                ("muted".to_string(), String::new()),
                ("playsInline".to_string(), String::new()),
                ("preload".to_string(), "metadata".to_string()),
                ("crossorigin".to_string(), "use-credentials".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: vec![CompiledJsxNode::Element {
            key: "captions".to_string(),
            tag: "track".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([
                    ("src".to_string(), "/captions.vtt".to_string()),
                    ("kind".to_string(), "captions".to_string()),
                    ("srcLang".to_string(), "en".to_string()),
                    ("label".to_string(), "English".to_string()),
                    ("default".to_string(), String::new()),
                ]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        }],
    };
    let link = CompiledJsxNode::Element {
        key: "stylesheet".to_string(),
        tag: "link".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("href".to_string(), "/app.css".to_string()),
                ("media".to_string(), "screen".to_string()),
                ("type".to_string(), "text/css".to_string()),
                ("fetchpriority".to_string(), "low".to_string()),
                ("crossorigin".to_string(), String::new()),
                ("referrerpolicy".to_string(), "origin".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let script = CompiledJsxNode::Element {
        key: "analytics".to_string(),
        tag: "script".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("src".to_string(), "/analytics.js".to_string()),
                ("type".to_string(), "module".to_string()),
                ("fetchPriority".to_string(), "low".to_string()),
                ("crossOrigin".to_string(), "anonymous".to_string()),
                ("referrerPolicy".to_string(), "same-origin".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let object = CompiledJsxNode::Element {
        key: "pdf".to_string(),
        tag: "object".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("data".to_string(), "/spec.pdf".to_string()),
                ("type".to_string(), "application/pdf".to_string()),
                ("width".to_string(), "800".to_string()),
                ("height".to_string(), "600".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };

    let native_picture = bridge.lower_to_native(&picture).unwrap();
    let native_video = bridge.lower_to_native(&video).unwrap();
    let native_link = bridge.lower_to_native(&link).unwrap();
    let native_script = bridge.lower_to_native(&script).unwrap();
    let native_object = bridge.lower_to_native(&object).unwrap();

    assert_eq!(native_picture.role, NativeRole::Image);
    assert_eq!(native_picture.props.intrinsic_width, Some(640));
    assert_eq!(native_picture.props.intrinsic_height, Some(360));

    let native_source = &native_picture.children[0];
    assert_eq!(native_source.role, NativeRole::EmbeddedContent);
    assert_eq!(
        native_source.props.srcset.as_deref(),
        Some("/hero.avif 1x, /hero@2x.avif 2x")
    );
    assert_eq!(
        native_source.props.sizes.as_deref(),
        Some("(min-width: 60rem) 50vw, 100vw")
    );
    assert_eq!(
        native_source.props.media.as_deref(),
        Some("(min-width: 48rem)")
    );
    assert_eq!(
        native_source.props.resource_type.as_deref(),
        Some("image/avif")
    );
    assert_eq!(native_source.props.intrinsic_width, Some(640));
    assert_eq!(native_source.props.intrinsic_height, Some(360));

    let native_img = &native_picture.children[1];
    assert_eq!(native_img.role, NativeRole::Image);
    assert_eq!(native_img.props.label.as_deref(), Some("Hero"));
    assert_eq!(native_img.props.alt.as_deref(), Some("Hero"));
    assert_eq!(native_img.props.src.as_deref(), Some("/hero.png"));
    assert_eq!(
        native_img.props.srcset.as_deref(),
        Some("/hero.png 1x, /hero@2x.png 2x")
    );
    assert_eq!(native_img.props.sizes.as_deref(), Some("100vw"));
    assert_eq!(native_img.props.intrinsic_width, Some(640));
    assert_eq!(native_img.props.intrinsic_height, Some(360));
    assert_eq!(native_img.props.loading.as_deref(), Some("lazy"));
    assert_eq!(native_img.props.decoding.as_deref(), Some("async"));
    assert_eq!(native_img.props.fetch_priority.as_deref(), Some("high"));
    assert_eq!(native_img.props.cross_origin.as_deref(), Some("anonymous"));
    assert_eq!(
        native_img.props.referrer_policy.as_deref(),
        Some("no-referrer")
    );

    assert_eq!(native_video.role, NativeRole::Media);
    assert_eq!(native_video.props.src.as_deref(), Some("/demo.mp4"));
    assert_eq!(native_video.props.poster.as_deref(), Some("/poster.png"));
    assert_eq!(native_video.props.intrinsic_width, Some(1280));
    assert_eq!(native_video.props.intrinsic_height, Some(720));
    assert!(native_video.props.controls);
    assert!(native_video.props.autoplay);
    assert!(native_video.props.loop_playback);
    assert!(native_video.props.muted);
    assert!(native_video.props.plays_inline);
    assert_eq!(native_video.props.preload.as_deref(), Some("metadata"));
    assert_eq!(
        native_video.props.cross_origin.as_deref(),
        Some("use-credentials")
    );

    let native_track = &native_video.children[0];
    assert_eq!(native_track.role, NativeRole::EmbeddedContent);
    assert_eq!(native_track.props.src.as_deref(), Some("/captions.vtt"));
    assert_eq!(native_track.props.track_kind.as_deref(), Some("captions"));
    assert_eq!(native_track.props.srclang.as_deref(), Some("en"));
    assert_eq!(native_track.props.track_label.as_deref(), Some("English"));
    assert!(native_track.props.default_track);

    assert_eq!(native_link.role, NativeRole::ResourceLink);
    assert_eq!(native_link.props.href.as_deref(), Some("/app.css"));
    assert_eq!(native_link.props.media.as_deref(), Some("screen"));
    assert_eq!(native_link.props.resource_type.as_deref(), Some("text/css"));
    assert_eq!(native_link.props.fetch_priority.as_deref(), Some("low"));
    assert_eq!(native_link.props.cross_origin.as_deref(), Some(""));
    assert_eq!(native_link.props.referrer_policy.as_deref(), Some("origin"));

    assert_eq!(native_script.role, NativeRole::Script);
    assert_eq!(native_script.props.src.as_deref(), Some("/analytics.js"));
    assert_eq!(native_script.props.resource_type.as_deref(), Some("module"));
    assert_eq!(native_script.props.fetch_priority.as_deref(), Some("low"));
    assert_eq!(
        native_script.props.cross_origin.as_deref(),
        Some("anonymous")
    );
    assert_eq!(
        native_script.props.referrer_policy.as_deref(),
        Some("same-origin")
    );

    assert_eq!(native_object.role, NativeRole::EmbeddedContent);
    assert_eq!(native_object.props.src.as_deref(), Some("/spec.pdf"));
    assert_eq!(
        native_object.props.resource_type.as_deref(),
        Some("application/pdf")
    );
    assert_eq!(native_object.props.intrinsic_width, Some(800));
    assert_eq!(native_object.props.intrinsic_height, Some(600));
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
