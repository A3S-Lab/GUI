#![cfg(all(feature = "authoring", feature = "software-reference"))]

#[path = "../examples/support/calculator/mod.rs"]
mod calculator;

use a3s_gui::accessibility::AccessibilityNode;
use a3s_gui::drawing::{scene_from_layout, LayoutSceneOptions, ReferenceRenderer};
use a3s_gui::layout::{layout_native_tree, LayoutOptions};
use a3s_gui::tsx_protocol::{decode_tsx_json_payload_v1, TsxClientMessageV1, TsxFrameLimitsV1};
use a3s_gui::{CompiledRsxNode, RsxCompilerBridge, Size, UiFrame};

const CALCULATOR_RENDER_FIXTURE: &str =
    include_str!("fixtures/tsx-protocol/render-calculator-v1.json");

#[test]
fn tsx_and_rust_rsx_calculators_share_the_complete_self_drawn_model() {
    let limits = TsxFrameLimitsV1::new(1_048_576).unwrap();
    let message = decode_tsx_json_payload_v1::<TsxClientMessageV1>(
        CALCULATOR_RENDER_FIXTURE.trim().as_bytes(),
        limits,
    )
    .unwrap();
    let TsxClientMessageV1::Render { payload, .. } = message else {
        panic!("expected calculator render fixture")
    };
    let tsx_frame: UiFrame = payload.try_into().unwrap();
    let component =
        calculator::shared_calculator_component("calculator", "A3S Calculator").unwrap();
    let rust_frame =
        calculator::calculator_frame(&component, &calculator::CalculatorState::default()).unwrap();

    let mut tsx_semantic_frame = tsx_frame.clone();
    clear_authoring_provenance(&mut tsx_semantic_frame.root);
    assert_eq!(tsx_semantic_frame, rust_frame);

    let bridge = RsxCompilerBridge::new();
    let tsx_content = bridge.lower_to_native(&tsx_frame.root).unwrap();
    let rust_content = bridge.lower_to_native(&rust_frame.root).unwrap();
    let tsx_native = tsx_frame
        .window
        .as_ref()
        .unwrap()
        .wrap_native_root(&tsx_frame.frame_id, tsx_content);
    let rust_native = rust_frame
        .window
        .as_ref()
        .unwrap()
        .wrap_native_root(&rust_frame.frame_id, rust_content);
    assert_eq!(tsx_native, rust_native);
    assert_eq!(
        AccessibilityNode::from_native(&tsx_native),
        AccessibilityNode::from_native(&rust_native)
    );

    let options = LayoutOptions::boxes_only(Size::new(410.0, 620.0));
    let tsx_layout = layout_native_tree(&tsx_native, options).unwrap();
    let rust_layout = layout_native_tree(&rust_native, options).unwrap();
    assert_eq!(tsx_layout, rust_layout);
    assert_eq!(
        tsx_layout.fingerprint().unwrap(),
        rust_layout.fingerprint().unwrap()
    );

    let tsx_scene = scene_from_layout(&tsx_layout, LayoutSceneOptions::default()).unwrap();
    let rust_scene = scene_from_layout(&rust_layout, LayoutSceneOptions::default()).unwrap();
    assert_eq!(tsx_scene, rust_scene);
    assert_eq!(
        tsx_scene.fingerprint().unwrap(),
        rust_scene.fingerprint().unwrap()
    );

    let mut tsx_renderer = ReferenceRenderer::new();
    let mut rust_renderer = ReferenceRenderer::new();
    let tsx_pixels = tsx_renderer.render(tsx_scene).unwrap();
    let rust_pixels = rust_renderer.render(rust_scene).unwrap();
    assert_eq!(tsx_pixels.rgba8(), rust_pixels.rgba8());
    assert_eq!(tsx_pixels.fingerprint(), rust_pixels.fingerprint());
}

fn clear_authoring_provenance(node: &mut CompiledRsxNode) {
    if let CompiledRsxNode::Element {
        props, children, ..
    } = node
    {
        props.explicit_props.clear();
        for child in children {
            clear_authoring_provenance(child);
        }
    }
}
