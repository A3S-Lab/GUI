use super::support::*;

#[test]
fn lowers_all_conforming_html_elements_without_rejecting_intrinsic_tags() {
    let bridge = RsxCompilerBridge::new();

    for tag in HTML_CONFORMING_ELEMENTS {
        let props = intrinsic_props_for_tag(tag);
        let compiled = CompiledRsxNode::Element {
            key: format!("{tag}-key"),
            tag: tag.to_string(),
            import_source: None,
            props,
            children: Vec::new(),
        };

        let native = bridge
            .lower_to_native(&compiled)
            .unwrap_or_else(|error| panic!("{tag} should lower to native IR: {error}"));

        assert_eq!(
            native
                .props
                .metadata
                .get(HTML_TAG_METADATA_KEY)
                .map(String::as_str),
            Some(*tag)
        );
    }
}

#[test]
fn lowers_all_known_html_elements_without_rejecting_intrinsic_tags() {
    let bridge = RsxCompilerBridge::new();

    for tag in HTML_ELEMENTS {
        let props = intrinsic_props_for_tag(tag);
        let compiled = CompiledRsxNode::Element {
            key: format!("{tag}-key"),
            tag: tag.to_string(),
            import_source: None,
            props,
            children: Vec::new(),
        };

        let native = bridge
            .lower_to_native(&compiled)
            .unwrap_or_else(|error| panic!("{tag} should lower to native IR: {error}"));

        assert_eq!(
            native
                .props
                .metadata
                .get(HTML_TAG_METADATA_KEY)
                .map(String::as_str),
            Some(*tag)
        );
    }
}

#[test]
fn lowers_all_known_svg_elements_without_rejecting_intrinsic_tags() {
    let bridge = RsxCompilerBridge::new();

    for tag in SVG_ELEMENTS {
        let compiled = CompiledRsxNode::Element {
            key: format!("{tag}-key"),
            tag: tag.to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: Vec::new(),
        };

        let native = bridge
            .lower_to_native(&compiled)
            .unwrap_or_else(|error| panic!("{tag} should lower to native IR: {error}"));

        assert_eq!(
            native
                .props
                .metadata
                .get(SVG_TAG_METADATA_KEY)
                .map(String::as_str),
            Some(*tag)
        );
    }
}

#[test]
fn lowers_html_link_and_image_map_tags_to_native_roles() {
    let bridge = RsxCompilerBridge::new();
    let link = CompiledRsxNode::Element {
        key: "docs-link".to_string(),
        tag: "a".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([("href".to_string(), "/docs".to_string())]),
            ..CompiledProps::default()
        },
        children: vec![CompiledRsxNode::Text {
            key: "docs-link-text".to_string(),
            value: "Docs".to_string(),
        }],
    };

    let native_link = bridge.lower_to_native(&link).unwrap();
    assert_eq!(native_link.role, NativeRole::Link);
    assert_eq!(native_link.props.label.as_deref(), Some("Docs"));
    assert_eq!(
        native_link
            .props
            .metadata
            .get(HTML_TAG_METADATA_KEY)
            .map(String::as_str),
        Some("a")
    );
    assert_eq!(
        native_link
            .props
            .web
            .attributes
            .get("href")
            .map(String::as_str),
        Some("/docs")
    );

    let protocol_link = CompiledRsxNode::Element {
        key: "protocol-docs-link".to_string(),
        tag: "a".to_string(),
        import_source: None,
        props: CompiledProps {
            href: Some("/protocol-docs".to_string()),
            ..CompiledProps::default()
        },
        children: vec![CompiledRsxNode::Text {
            key: "protocol-docs-link-text".to_string(),
            value: "Protocol docs".to_string(),
        }],
    };

    let native_protocol_link = bridge.lower_to_native(&protocol_link).unwrap();
    assert_eq!(native_protocol_link.role, NativeRole::Link);
    assert_eq!(
        native_protocol_link.props.href.as_deref(),
        Some("/protocol-docs")
    );

    let clickable_anchor = CompiledRsxNode::Element {
        key: "archive-anchor".to_string(),
        tag: "a".to_string(),
        import_source: None,
        props: CompiledProps {
            events: BTreeMap::from([("onClick".to_string(), "archive".to_string())]),
            ..CompiledProps::default()
        },
        children: vec![CompiledRsxNode::Text {
            key: "archive-anchor-text".to_string(),
            value: "Archive".to_string(),
        }],
    };

    let native_clickable_anchor = bridge.lower_to_native(&clickable_anchor).unwrap();
    assert_eq!(native_clickable_anchor.role, NativeRole::Button);
    assert_eq!(
        native_clickable_anchor.props.label.as_deref(),
        Some("Archive")
    );
    assert_eq!(
        native_clickable_anchor.props.action.as_deref(),
        Some("archive")
    );

    let image_map = CompiledRsxNode::Element {
        key: "hero-map".to_string(),
        tag: "map".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([("name".to_string(), "hero-map".to_string())]),
            ..CompiledProps::default()
        },
        children: vec![CompiledRsxNode::Element {
            key: "cta-area".to_string(),
            tag: "area".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([
                    ("href".to_string(), "/signup".to_string()),
                    ("alt".to_string(), "Sign up".to_string()),
                    ("shape".to_string(), "rect".to_string()),
                    ("coords".to_string(), "0,0,120,48".to_string()),
                ]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        }],
    };

    let native_image_map = bridge.lower_to_native(&image_map).unwrap();
    assert_eq!(native_image_map.role, NativeRole::ImageMap);
    assert_eq!(native_image_map.children.len(), 1);
    assert_eq!(native_image_map.children[0].role, NativeRole::ImageMapArea);
    assert_eq!(
        native_image_map.children[0].props.label.as_deref(),
        Some("Sign up")
    );
    assert_eq!(
        native_image_map.children[0]
            .props
            .web
            .attributes
            .get("href")
            .map(String::as_str),
        Some("/signup")
    );
}

fn intrinsic_props_for_tag(tag: &str) -> CompiledProps {
    if tag == "input" {
        CompiledProps {
            attributes: BTreeMap::from([("type".to_string(), "checkbox".to_string())]),
            ..CompiledProps::default()
        }
    } else {
        CompiledProps::default()
    }
}
