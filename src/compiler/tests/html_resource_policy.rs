use super::support::*;

#[test]
fn lowers_html_navigation_policy_attributes_to_native_state() {
    let bridge = ReactCompilerBridge::new();
    let root = CompiledJsxNode::Element {
        key: "navigation-policy".to_string(),
        tag: "div".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            element(
                "docs",
                "a",
                BTreeMap::from([
                    ("href".to_string(), "/docs".to_string()),
                    ("target".to_string(), "_blank".to_string()),
                    ("download".to_string(), "guide.pdf".to_string()),
                    ("ping".to_string(), "/analytics".to_string()),
                    ("rel".to_string(), "noopener noreferrer".to_string()),
                    ("hreflang".to_string(), "en".to_string()),
                    ("type".to_string(), "application/pdf".to_string()),
                    ("referrerPolicy".to_string(), "no-referrer".to_string()),
                ]),
            ),
            element(
                "shape",
                "area",
                BTreeMap::from([
                    ("href".to_string(), "/map".to_string()),
                    ("target".to_string(), "map-frame".to_string()),
                    ("download".to_string(), String::new()),
                    ("hrefLang".to_string(), "fr".to_string()),
                    ("referrerpolicy".to_string(), "origin".to_string()),
                ]),
            ),
            element(
                "base",
                "base",
                BTreeMap::from([
                    ("href".to_string(), "https://example.test/".to_string()),
                    ("target".to_string(), "_self".to_string()),
                ]),
            ),
        ],
    };

    let native = bridge.lower_to_native(&root).unwrap();
    let anchor = &native.children[0].props;
    let area = &native.children[1].props;
    let base = &native.children[2].props;

    assert_eq!(anchor.href.as_deref(), Some("/docs"));
    assert_eq!(anchor.resource_type.as_deref(), Some("application/pdf"));
    assert_eq!(anchor.referrer_policy.as_deref(), Some("no-referrer"));
    assert_eq!(
        anchor.html_resource_policy.target.as_deref(),
        Some("_blank")
    );
    assert_eq!(
        anchor.html_resource_policy.download.as_deref(),
        Some("guide.pdf")
    );
    assert_eq!(
        anchor.html_resource_policy.ping.as_deref(),
        Some("/analytics")
    );
    assert_eq!(
        anchor.html_resource_policy.rel.as_deref(),
        Some("noopener noreferrer")
    );
    assert_eq!(anchor.html_resource_policy.href_lang.as_deref(), Some("en"));

    assert_eq!(
        area.html_resource_policy.target.as_deref(),
        Some("map-frame")
    );
    assert_eq!(area.html_resource_policy.download.as_deref(), Some(""));
    assert_eq!(area.html_resource_policy.href_lang.as_deref(), Some("fr"));
    assert_eq!(area.referrer_policy.as_deref(), Some("origin"));

    assert_eq!(base.href.as_deref(), Some("https://example.test/"));
    assert_eq!(base.html_resource_policy.target.as_deref(), Some("_self"));
}

#[test]
fn lowers_html_loading_and_embedding_policy_attributes_to_native_state() {
    let bridge = ReactCompilerBridge::new();
    let root = CompiledJsxNode::Element {
        key: "loading-policy".to_string(),
        tag: "div".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: vec![
            element(
                "preload",
                "link",
                BTreeMap::from([
                    ("href".to_string(), "/hero.avif".to_string()),
                    ("rel".to_string(), "preload".to_string()),
                    ("as".to_string(), "image".to_string()),
                    ("integrity".to_string(), "sha384-link".to_string()),
                    ("blocking".to_string(), "render".to_string()),
                    ("imagesrcset".to_string(), "/hero.avif 1x".to_string()),
                    ("imageSizes".to_string(), "100vw".to_string()),
                    ("disabled".to_string(), String::new()),
                ]),
            ),
            element(
                "script",
                "script",
                BTreeMap::from([
                    ("src".to_string(), "/app.js".to_string()),
                    ("integrity".to_string(), "sha384-script".to_string()),
                    ("blocking".to_string(), "render".to_string()),
                    ("nonce".to_string(), "nonce-1".to_string()),
                    ("async".to_string(), String::new()),
                    ("defer".to_string(), String::new()),
                    ("noModule".to_string(), String::new()),
                ]),
            ),
            element(
                "style",
                "style",
                BTreeMap::from([
                    ("blocking".to_string(), "render".to_string()),
                    ("nonce".to_string(), "nonce-2".to_string()),
                ]),
            ),
            element(
                "frame",
                "iframe",
                BTreeMap::from([
                    ("src".to_string(), "https://example.test/embed".to_string()),
                    ("name".to_string(), "preview".to_string()),
                    (
                        "allow".to_string(),
                        "fullscreen; clipboard-write".to_string(),
                    ),
                    ("allowFullScreen".to_string(), String::new()),
                    ("sandbox".to_string(), "allow-scripts".to_string()),
                    ("srcDoc".to_string(), "<p>Preview</p>".to_string()),
                ]),
            ),
        ],
    };

    let native = bridge.lower_to_native(&root).unwrap();
    let preload = &native.children[0].props.html_resource_policy;
    let script = &native.children[1].props.html_resource_policy;
    let style = &native.children[2].props.html_resource_policy;
    let frame = &native.children[3].props.html_resource_policy;

    assert_eq!(preload.rel.as_deref(), Some("preload"));
    assert_eq!(preload.link_as.as_deref(), Some("image"));
    assert_eq!(preload.integrity.as_deref(), Some("sha384-link"));
    assert_eq!(preload.blocking.as_deref(), Some("render"));
    assert_eq!(preload.image_srcset.as_deref(), Some("/hero.avif 1x"));
    assert_eq!(preload.image_sizes.as_deref(), Some("100vw"));
    assert!(preload.resource_disabled);

    assert_eq!(script.integrity.as_deref(), Some("sha384-script"));
    assert_eq!(script.blocking.as_deref(), Some("render"));
    assert_eq!(script.nonce.as_deref(), Some("nonce-1"));
    assert!(script.async_script);
    assert!(script.defer_script);
    assert!(script.no_module);

    assert_eq!(style.blocking.as_deref(), Some("render"));
    assert_eq!(style.nonce.as_deref(), Some("nonce-2"));

    assert_eq!(frame.frame_name.as_deref(), Some("preview"));
    assert_eq!(
        frame.frame_allow.as_deref(),
        Some("fullscreen; clipboard-write")
    );
    assert!(frame.frame_allow_fullscreen);
    assert_eq!(frame.frame_sandbox.as_deref(), Some("allow-scripts"));
    assert_eq!(frame.frame_srcdoc.as_deref(), Some("<p>Preview</p>"));
}

fn element(key: &str, tag: &str, attributes: BTreeMap<String, String>) -> CompiledJsxNode {
    CompiledJsxNode::Element {
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
