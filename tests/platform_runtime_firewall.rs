#![cfg(feature = "platform-runtime")]

use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn platform_runtime_feature_owns_only_the_shared_host_and_graphics_edges() {
    let manifest = fs::read_to_string(manifest_path("Cargo.toml")).unwrap();
    let features = manifest
        .split_once("[features]")
        .map(|(_, rest)| rest)
        .and_then(|rest| rest.split_once("\n[").map(|(section, _)| section))
        .unwrap();
    let definition = feature_definition(features, "platform-runtime");

    assert!(definition.contains("platform-host"));
    assert!(definition.contains("graphics"));
    for forbidden in [
        "appkit-native",
        "gtk4-native",
        "winui-native",
        "dep:objc2",
        "dep:gtk4_crate",
        "dep:winui3",
        "dep:windows",
    ] {
        assert!(
            !definition.contains(forbidden),
            "platform-runtime enables legacy or OS dependency {forbidden:?}: {definition}"
        );
    }
}

#[test]
fn raw_surface_ownership_stays_behind_the_pinned_graphics_edge() {
    let manifest = fs::read_to_string(manifest_path("Cargo.toml")).unwrap();
    let dependency = manifest
        .lines()
        .find(|line| line.starts_with("a3s-graphics ="))
        .expect("a3s-graphics dependency must stay explicit");
    assert!(dependency.contains("https://github.com/A3S-Lab/Graphics"));
    assert!(dependency.contains("0afa90bc40ef05f158b1a6ffa4dc7583af3a32a9"));
    assert!(dependency.contains("default-features = false"));
    let features = manifest
        .split_once("[features]")
        .map(|(_, rest)| rest)
        .and_then(|rest| rest.split_once("\n[").map(|(section, _)| section))
        .unwrap();
    let gpu = feature_definition(features, "gpu");
    for edge in [
        "graphics",
        "a3s-graphics/gpu",
        "dep:pollster",
        "dep:raw-window-handle",
    ] {
        assert!(
            gpu.contains(edge),
            "GPU presenter is missing {edge:?}: {gpu}"
        );
    }
    assert!(
        !manifest.lines().any(|line| line.starts_with("wgpu =")),
        "GUI must not duplicate Graphics-owned device or raw-surface state"
    );
}

#[test]
fn platform_runtime_source_has_no_legacy_widget_or_toolkit_path() {
    let source_root = manifest_path("src/platform_runtime");
    let mut files = Vec::new();
    collect_rust_files(&source_root, &mut files);
    assert!(!files.is_empty());

    let forbidden = [
        "NativeWidget",
        "NativeHost",
        "HostNodeId",
        "PortableStyle",
        "GuiRuntime",
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
                "{} crosses the H1 runtime firewall with {token:?}",
                path.display()
            );
        }
    }

    let shared_semantics = manifest_path("src/semantic_event.rs");
    let source = fs::read_to_string(&shared_semantics).unwrap();
    for token in forbidden {
        assert!(
            !source.contains(token),
            "{} crosses the shared semantic-event firewall with {token:?}",
            shared_semantics.display()
        );
    }

    let example = fs::read_to_string(manifest_path("examples/self_drawn_calculator.rs")).unwrap();
    for token in forbidden {
        assert!(
            !example.contains(token),
            "self-drawn calculator crosses the H1 runtime firewall with {token:?}"
        );
    }

    let tsx_host_path = manifest_path("src/bin/tsx_host.rs");
    let tsx_host = fs::read_to_string(&tsx_host_path).unwrap();
    for token in forbidden {
        assert!(
            !tsx_host.contains(token),
            "{} crosses the H1 runtime firewall with {token:?}",
            tsx_host_path.display()
        );
    }
}

#[test]
fn platform_runtime_and_smoke_example_are_feature_gated() {
    let library = fs::read_to_string(manifest_path("src/lib.rs")).unwrap();
    assert!(library.contains("#[cfg(feature = \"platform-runtime\")]\npub mod platform_runtime;"));

    let manifest = fs::read_to_string(manifest_path("Cargo.toml")).unwrap();
    let example = manifest
        .split_once("name = \"self_drawn_calculator\"")
        .map(|(_, rest)| rest.lines().take(2).collect::<Vec<_>>().join("\n"))
        .unwrap();
    for feature in ["authoring", "platform-runtime", "software-reference"] {
        assert!(
            example.contains(feature),
            "self-drawn calculator must require {feature:?}: {example}"
        );
    }

    let tsx_host = manifest
        .split_once("name = \"a3s-gui-tsx-host\"")
        .map(|(_, rest)| rest.lines().take(3).collect::<Vec<_>>().join("\n"))
        .unwrap();
    for feature in ["platform-runtime", "software-reference"] {
        assert!(
            tsx_host.contains(feature),
            "headless TSX host must require {feature:?}: {tsx_host}"
        );
    }
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
