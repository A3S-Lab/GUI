use std::sync::Mutex;

use crate::error::{GuiError, GuiResult};
use crate::geometry::{Rect, Size};
use crate::native::{NativeElement, NativeProps, NativeRole, ValueSensitivity};
use crate::style::TextDirection;
use crate::web::WebProps;

use super::{
    layout_native_tree, LayoutOptions, ShapedGlyph, ShapedGlyphRun, ShapedText, ShapedTextLine,
    TextContentSource, TextFontFaceId, TextShapeRequest, TextShaper,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ObservedRequest {
    text: String,
    source: TextContentSource,
    sensitivity: ValueSensitivity,
    debug: String,
}

#[derive(Default)]
struct FixtureShaper {
    requests: Mutex<Vec<ObservedRequest>>,
}

impl FixtureShaper {
    fn requests(&self) -> Vec<ObservedRequest> {
        self.requests.lock().unwrap().clone()
    }
}

impl TextShaper for FixtureShaper {
    fn shape(&self, request: &TextShapeRequest<'_>) -> GuiResult<ShapedText> {
        self.requests.lock().unwrap().push(ObservedRequest {
            text: request.text.to_string(),
            source: request.source,
            sensitivity: request.sensitivity,
            debug: format!("{request:?}"),
        });
        fixture_shape(request.text)
    }
}

fn fixture_shape(text: &str) -> GuiResult<ShapedText> {
    let width = (text.chars().count() as f64) * 7.0;
    Ok(ShapedText {
        logical_size: Size::new(width, 9.0),
        ink_bounds: Rect::new(0.0, 0.0, width.max(0.0), 8.0),
        lines: vec![ShapedTextLine {
            byte_start: 0,
            byte_end: text.len() as u32,
            baseline: 7.0,
            ascent: 7.0,
            descent: 2.0,
            advance: width,
            runs: if text.is_empty() {
                Vec::new()
            } else {
                vec![ShapedGlyphRun {
                    font_face: TextFontFaceId::new("fixture/regular")?,
                    font_size: 9.0,
                    direction: TextDirection::Ltr,
                    bidi_level: 0,
                    glyphs: vec![ShapedGlyph {
                        glyph_id: 42,
                        cluster_start: 0,
                        cluster_end: text.len() as u32,
                        x: 0.0,
                        y: 7.0,
                        advance_x: width,
                        advance_y: 0.0,
                    }],
                }]
            },
        }],
    })
}

fn styled(key: &str, role: NativeRole, class_name: &str) -> NativeElement {
    NativeElement::new(key, role)
        .with_props(NativeProps::new().web(WebProps::new().class_name(class_name)))
}

#[test]
fn shaped_text_is_the_single_intrinsic_measurement_and_layout_record() {
    let shaper = FixtureShaper::default();
    let text = styled("copy", NativeRole::Text, "inline-block").with_props(
        NativeProps::new()
            .label("ffi")
            .web(WebProps::new().class_name("inline-block font-mono")),
    );
    let root = styled(
        "root",
        NativeRole::View,
        "flex h-[40px] w-[100px] items-start",
    )
    .child(text);

    let snapshot = layout_native_tree(
        &root,
        LayoutOptions::with_text(Size::new(100.0, 40.0), &shaper),
    )
    .unwrap();

    let node = snapshot.node_by_path("4:root/4:copy").unwrap();
    assert_eq!(node.border_box.width, 21.0);
    assert_eq!(node.border_box.height, 9.0);
    let text = node.text.as_ref().unwrap();
    assert_eq!(text.source, TextContentSource::Label);
    assert_eq!(text.shape.logical_size, Size::new(21.0, 9.0));
    assert_eq!(text.shape.lines[0].runs[0].glyphs[0].glyph_id, 42);
    assert_eq!(shaper.requests().len(), 1);
    assert_eq!(shaper.requests()[0].text, "ffi");
}

#[test]
fn password_values_are_masked_before_shaping_and_never_enter_retained_layout() {
    let shaper = FixtureShaper::default();
    let secret = "secret🚀";
    let field = styled("password", NativeRole::TextField, "inline-block").with_props(
        NativeProps::new()
            .value(secret)
            .input_type("password")
            .web(WebProps::new().class_name("inline-block")),
    );
    let root = styled("root", NativeRole::View, "h-[40px] w-[100px]").child(field);

    let snapshot = layout_native_tree(
        &root,
        LayoutOptions::with_text(Size::new(100.0, 40.0), &shaper),
    )
    .unwrap();

    let observed = &shaper.requests()[0];
    assert_eq!(observed.text, "•••••••");
    assert_eq!(observed.source, TextContentSource::Value);
    assert_eq!(observed.sensitivity, ValueSensitivity::Sensitive);
    assert!(!observed.debug.contains(secret));
    assert!(!serde_json::to_string(&snapshot).unwrap().contains(secret));
    assert!(!format!("{snapshot:?}").contains(secret));
}

#[test]
fn oversized_password_values_are_rejected_before_masking_or_shaping() {
    let shaper = FixtureShaper::default();
    let oversized = "x".repeat(super::MAX_TEXT_SOURCE_BYTES + 1);
    let field = styled("password", NativeRole::TextField, "inline-block").with_props(
        NativeProps::new()
            .value(oversized)
            .input_type("password")
            .web(WebProps::new().class_name("inline-block")),
    );
    let root = styled("root", NativeRole::View, "h-[40px] w-[100px]").child(field);

    let error = layout_native_tree(
        &root,
        LayoutOptions::with_text(Size::new(100.0, 40.0), &shaper),
    )
    .unwrap_err();

    assert!(matches!(error, GuiError::Text { .. }));
    assert!(error.to_string().contains("byte limit"));
    assert!(shaper.requests().is_empty());
}

struct InvalidClusterShaper;

impl TextShaper for InvalidClusterShaper {
    fn shape(&self, request: &TextShapeRequest<'_>) -> GuiResult<ShapedText> {
        let mut shape = fixture_shape(request.text)?;
        shape.lines[0].runs[0].glyphs[0].cluster_end = u32::MAX;
        Ok(shape)
    }
}

#[test]
fn invalid_backend_clusters_are_rejected_before_the_snapshot_is_retained() {
    let secret = "do-not-retain";
    let field = styled("password", NativeRole::TextField, "inline-block").with_props(
        NativeProps::new()
            .value(secret)
            .input_type("password")
            .web(WebProps::new().class_name("inline-block")),
    );
    let root = styled("root", NativeRole::View, "h-[40px] w-[100px]").child(field);

    let error = layout_native_tree(
        &root,
        LayoutOptions::with_text(Size::new(100.0, 40.0), &InvalidClusterShaper),
    )
    .unwrap_err();

    assert!(matches!(error, GuiError::Text { .. }));
    assert!(error.to_string().contains("cluster"));
    assert!(!error.to_string().contains(secret));
}

#[test]
fn shape_validation_rejects_bidi_metric_and_font_identity_corruption() {
    let mut bidi = fixture_shape("a").unwrap();
    bidi.lines[0].runs[0].bidi_level = 1;
    assert!(bidi
        .validate("a")
        .unwrap_err()
        .to_string()
        .contains("bidi level"));

    let mut metric = fixture_shape("a").unwrap();
    metric.lines[0].runs[0].glyphs[0].x = f64::NAN;
    assert!(metric
        .validate("a")
        .unwrap_err()
        .to_string()
        .contains("must be finite"));

    assert!(TextFontFaceId::new("").is_err());
    assert!(TextFontFaceId::new("bad\nface").is_err());
}

#[test]
fn box_only_layout_keeps_text_explicitly_deferred() {
    let text = styled("copy", NativeRole::Text, "inline-block").with_props(
        NativeProps::new()
            .label("not shaped")
            .web(WebProps::new().class_name("inline-block")),
    );
    let root = styled(
        "root",
        NativeRole::View,
        "flex h-[40px] w-[100px] items-start",
    )
    .child(text);

    let snapshot =
        layout_native_tree(&root, LayoutOptions::boxes_only(Size::new(100.0, 40.0))).unwrap();

    let node = snapshot.node_by_path("4:root/4:copy").unwrap();
    assert_eq!((node.border_box.width, node.border_box.height), (0.0, 0.0));
    assert!(node.text.is_none());
}

#[test]
fn layout_diff_damage_includes_visible_text_outside_an_explicit_box() {
    let shaper = FixtureShaper::default();
    let tree = |label: &str| {
        styled("root", NativeRole::View, "relative h-[20px] w-[100px]").child(
            styled(
                "copy",
                NativeRole::Text,
                "absolute left-0 top-0 h-px w-px overflow-visible",
            )
            .with_props(NativeProps::new().label(label).web(
                WebProps::new().class_name("absolute left-0 top-0 h-px w-px overflow-visible"),
            )),
        )
    };
    let first = layout_native_tree(
        &tree("one"),
        LayoutOptions::with_text(Size::new(100.0, 20.0), &shaper),
    )
    .unwrap();
    let second = layout_native_tree(
        &tree("four"),
        LayoutOptions::with_text(Size::new(100.0, 20.0), &shaper),
    )
    .unwrap();

    let diff = first.diff(&second);

    assert_eq!(diff.dirty_bounds, Some(Rect::new(0.0, 0.0, 28.0, 8.0)));
}

#[test]
fn text_shaper_contract_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FixtureShaper>();
}

#[cfg(feature = "graphics")]
#[derive(Default)]
struct FixtureTextEncoder {
    calls: usize,
}

#[cfg(feature = "graphics")]
impl crate::drawing::TextSceneEncoder for FixtureTextEncoder {
    fn encode(
        &mut self,
        request: crate::drawing::TextSceneRequest<'_>,
    ) -> GuiResult<Vec<crate::drawing::Primitive>> {
        use crate::drawing::{Color, FillRect, Primitive, Rect as GraphicsRect};

        self.calls += 1;
        assert_eq!(request.scale_factor, 1.0);
        let text = request.text;
        let ink = text.ink_bounds().unwrap();
        Ok(vec![Primitive::FillRect(FillRect {
            rect: GraphicsRect::new(
                ink.x as f32,
                ink.y as f32,
                ink.width as f32,
                ink.height as f32,
            ),
            color: Color::rgba(
                text.color.red,
                text.color.green,
                text.color.blue,
                text.color.alpha,
            ),
        })])
    }
}

#[cfg(feature = "graphics")]
struct EscapingTextEncoder;

#[cfg(feature = "graphics")]
impl crate::drawing::TextSceneEncoder for EscapingTextEncoder {
    fn encode(
        &mut self,
        _request: crate::drawing::TextSceneRequest<'_>,
    ) -> GuiResult<Vec<crate::drawing::Primitive>> {
        use crate::drawing::{Color, FillRect, Primitive, Rect as GraphicsRect};

        Ok(vec![Primitive::FillRect(FillRect {
            rect: GraphicsRect::new(500.0, 500.0, 1.0, 1.0),
            color: Color::BLACK,
        })])
    }
}

#[cfg(feature = "graphics")]
#[test]
fn scene_encoding_reuses_the_retained_shape_and_requires_an_explicit_encoder() {
    use crate::drawing::{scene_from_layout, LayoutSceneOptions, Primitive};

    let shaper = FixtureShaper::default();
    let text = styled("copy", NativeRole::Text, "inline-block")
        .with_props(NativeProps::new().label("ffi"));
    let root = styled(
        "root",
        NativeRole::View,
        "flex h-[40px] w-[100px] items-start",
    )
    .child(text);
    let layout = layout_native_tree(
        &root,
        LayoutOptions::with_text(Size::new(100.0, 40.0), &shaper),
    )
    .unwrap();

    let error = scene_from_layout(&layout, LayoutSceneOptions::default()).unwrap_err();
    assert!(error.to_string().contains("no text scene encoder"));

    let mut encoder = FixtureTextEncoder::default();
    let first = scene_from_layout(
        &layout,
        LayoutSceneOptions::default().with_text_encoder(&mut encoder),
    )
    .unwrap();
    let second = scene_from_layout(
        &layout,
        LayoutSceneOptions::default().with_text_encoder(&mut encoder),
    )
    .unwrap();

    assert_eq!(shaper.requests().len(), 1);
    assert_eq!(encoder.calls, 2);
    assert_eq!(first.fingerprint().unwrap(), second.fingerprint().unwrap());
    assert_eq!(first.commands.len(), 1);
    assert!(matches!(
        first.commands[0].primitive,
        Primitive::FillRect(_)
    ));

    let mut escaping = EscapingTextEncoder;
    let error = scene_from_layout(
        &layout,
        LayoutSceneOptions::default().with_text_encoder(&mut escaping),
    )
    .unwrap_err();
    assert!(error.to_string().contains("outside its shaped ink bounds"));
}
