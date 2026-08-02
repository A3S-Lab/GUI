#![cfg(feature = "platform-host")]

use std::fs;
use std::path::{Path, PathBuf};

const TARGET_FEATURES: &[&str] = &[
    "platform-host",
    "host-macos",
    "host-windows",
    "host-linux-wayland",
    "host-linux-x11",
    "host-linux",
];

#[test]
fn target_host_features_do_not_enable_legacy_renderers() {
    let manifest = fs::read_to_string(manifest_path("Cargo.toml")).unwrap();
    assert!(
        !manifest.contains("\"Win32_UI_Controls\""),
        "the raw Windows host must not enable Win32 content-control bindings"
    );
    let features = manifest
        .split_once("[features]")
        .map(|(_, rest)| rest)
        .and_then(|rest| rest.split_once("\n[").map(|(section, _)| section))
        .unwrap();
    let forbidden = [
        "appkit-native",
        "gtk4-native",
        "winui-native",
        "dep:gtk4_crate",
        "dep:winui3",
        "dep:windows-collections",
    ];

    for feature in TARGET_FEATURES {
        let definition = feature_definition(features, feature);
        for token in forbidden {
            assert!(
                !definition.contains(token),
                "target feature {feature:?} enables forbidden legacy token {token:?}: {definition}"
            );
        }
    }

    assert_eq!(
        feature_definition(features, "platform-host"),
        "platform-host = []"
    );
    for feature in [
        "host-macos",
        "host-windows",
        "host-linux-wayland",
        "host-linux-x11",
    ] {
        assert!(
            feature_definition(features, feature).contains("platform-host"),
            "target feature {feature:?} must include the shared platform-host contract"
        );
    }

    let windows = feature_definition(features, "host-windows");
    assert!(windows.contains("dep:raw-window-handle"));
    assert!(windows.contains("dep:windows-sys"));
}

#[test]
fn raw_platform_unsafe_is_confined_to_reviewed_windows_abi_files() {
    let source_root = manifest_path("src/platform_host");
    let mut files = Vec::new();
    collect_rust_files(&source_root, &mut files);
    let allowed = [
        source_root.join("windows/input.rs"),
        source_root.join("windows/input/mouse.rs"),
        source_root.join("windows/input/pointer.rs"),
        source_root.join("windows/input/pointer/native.rs"),
        source_root.join("windows/keyboard.rs"),
        source_root.join("windows/native.rs"),
        source_root.join("windows/surface.rs"),
    ];

    for path in files {
        let source = fs::read_to_string(&path).unwrap();
        if source.contains("allow(unsafe_code)") || source.contains("unsafe {") {
            assert!(
                allowed.contains(&path),
                "unsafe platform-host code escaped the reviewed Win32 ABI boundary: {}",
                path.display()
            );
        }
    }
    for path in allowed {
        let source = fs::read_to_string(&path).unwrap();
        assert!(source.starts_with("#![allow(unsafe_code)]"));
    }
}

#[test]
fn platform_host_source_has_no_widget_or_toolkit_boundary() {
    let source_root = manifest_path("src/platform_host");
    let mut files = Vec::new();
    collect_rust_files(&source_root, &mut files);
    assert!(!files.is_empty());

    let forbidden = [
        "NativeElement",
        "crate::accessibility::AccessibilityNode",
        "crate::host::HostNodeId",
        "PortableStyle",
        "NativeWidget",
        "appkit_native",
        "gtk4_native",
        "winui_native",
        "objc2",
        "gtk4_crate",
        "winui3",
        "wgpu::",
    ];
    for path in files {
        if path.file_name().and_then(|name| name.to_str()) == Some("tests.rs") {
            continue;
        }
        let source = fs::read_to_string(&path).unwrap();
        for token in forbidden {
            assert!(
                !source.contains(token),
                "{} crosses the platform-host firewall with {token:?}",
                path.display()
            );
        }
    }

    let contract = fs::read_to_string(source_root.join("contract.rs")).unwrap();
    for widget_operation in [
        "fn create(",
        "fn update(",
        "fn insert_child(",
        "fn remove(",
        "fn set_root(",
    ] {
        assert!(
            !contract.contains(widget_operation),
            "platform host contract exposes forbidden widget operation {widget_operation:?}"
        );
    }
}

#[test]
fn public_module_is_feature_gated() {
    let library = fs::read_to_string(manifest_path("src/lib.rs")).unwrap();
    let lines = library.lines().collect::<Vec<_>>();
    assert!(lines.windows(2).any(|pair| {
        pair == [
            "#[cfg(feature = \"platform-host\")]",
            "pub mod platform_host;",
        ]
    }));
}

fn feature_definition(features: &str, name: &str) -> String {
    let prefix = format!("{name} =");
    let mut lines = features.lines();
    while let Some(line) = lines.next() {
        if !line.starts_with(&prefix) {
            continue;
        }
        let mut definition = line.to_string();
        let mut depth = bracket_delta(line);
        while depth > 0 {
            let continuation = lines
                .next()
                .unwrap_or_else(|| panic!("unterminated feature definition for {name}"));
            definition.push('\n');
            definition.push_str(continuation);
            depth += bracket_delta(continuation);
        }
        return definition;
    }
    panic!("missing Cargo feature {name}");
}

fn bracket_delta(line: &str) -> i32 {
    line.chars().fold(0, |depth, character| match character {
        '[' => depth + 1,
        ']' => depth - 1,
        _ => depth,
    })
}

fn collect_rust_files(directory: &Path, output: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_rust_files(&path, output);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            output.push(path);
        }
    }
}

fn manifest_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}
