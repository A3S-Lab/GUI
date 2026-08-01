# Layout and Scene Contract

Updated: 2026-08-01

The first self-drawn renderer slice consumes the existing `NativeElement`
tree. It does not define a calculator-specific visual tree or copy component
state into a renderer model.

```rust
use a3s_gui::drawing::{scene_from_layout, LayoutSceneOptions};
use a3s_gui::layout::layout_native_tree;
use a3s_gui::Size;

let layout = layout_native_tree(&native, Size::new(410.0, 620.0))?;
layout.require_supported()?;
let scene = scene_from_layout(&layout, LayoutSceneOptions::default())?;
# Ok::<(), a3s_gui::GuiError>(())
```

Layout is available in semantic-only builds. Graphics scene extraction is
available only with the `graphics` feature, while rendering remains behind
`software-reference` or `gpu`.

## Versioned records

`LayoutSnapshot` schema version 1 contains:

- logical surface size and boxes quantized to 1/64 logical point
- flat `LayoutNodeRecord` values with role, parent identity, border/content
  boxes, inherited clip, z-index, paint order, hit eligibility, and box paint
- separate `LayoutHitRegion` values keyed to the same semantic elements
- structured warnings and errors with element and field attribution
- deterministic serialization, fingerprinting, and node-level layout diffs

`LayoutElementId` is a path of byte-length-prefixed sibling keys. Reordering a
sibling does not change its identity, and `/` or other punctuation in a key
cannot make two paths collide. The Graphics adapter derives each `DrawId` from
that path plus a stable paint slot.

The snapshot contains no labels, values, passwords, native handles, GPU
objects, or product state.

## Implemented M3 slice

The current deterministic box path implements the subset needed by the shared
calculator:

- vertical block flow and no-wrap horizontal/vertical flex flow
- explicit, percentage, minimum, and maximum size with border/content sizing
- physical and horizontal-writing logical padding, margin, inset, and border
- row/column gap, source order, integer order, main/cross-axis alignment, and
  stretch for auto cross sizes
- relative, absolute, and fixed positioning
- overflow rectangle clipping, visibility, opacity, pointer eligibility,
  simple sibling z-order, and cumulative descendant opacity
- solid color backgrounds, per-edge solid borders, circular corner radii, and
  opaque rounded background/border composition

The portable Tailwind contract historically treats a positive border width as
solid when no explicit border style is present. Explicit `none` and `hidden`
still suppress the border. The layout path preserves that behavior so it
matches the existing native backends without depending on browser Preflight.

Full flex growth and shrinkage, wrapping, grid/table layout, vertical writing,
baseline alignment, elliptical radii, CSS expression evaluation, inherited or
functional colors, and complete stacking contexts are not claimed by this
slice.

## Projection diagnostics

Every effective `PortableStyle` field is checked against the executable
[renderer field inventory](renderer-field-inventory.md).

- A later-milestone field produces a warning. Text and control roles therefore
  retain their M3 boxes while their visible content remains an explicit M4
  item.
- An M3 field or value that this slice cannot project produces an error.
- A property retained in `PortableStyle::unsupported` produces an error.
- `scene_from_layout` rejects any snapshot containing an error before it emits
  a draw command.

This lets inspection tools display partial boxes without allowing a required
calculator style to disappear silently from an accepted scene fixture.

## Retention and paint ordering

Repeated snapshots have identical fingerprints. `LayoutSnapshot::diff`
reports stable-key additions, removals, changed records, dirty bounds, and
surface rebuilds. Graphics performs the subsequent primitive diff and retained
damage calculation.

A parent background is emitted before its descendants. Siblings are placed by
z-index, flex order, and source order, and each subtree remains contiguous.
This is sufficient for the calculator and simple overlays; complete CSS
stacking-context behavior remains M3 work.

Overflow clips apply to descendants and are intersected through the tree.
Fixed-position descendants use the viewport rather than inheriting an ordinary
ancestor clip. Hit regions use the same quantized identity and visible bounds
but remain separate from paint commands.

## Calculator evidence

`tests/calculator_layout_scene.rs` compiles the existing shared calculator RSX,
lowers it through `RsxCompilerBridge`, wraps the real 410 by 620 window native
tree, and then uses this generic path. The fixture pins:

- layout fingerprint `16529597026056060935`
- Graphics scene fingerprint `2100550662756266801`
- exact repeated software output and retained no-damage behavior
- background, transparent exterior, ordinary-key, and equals-key pixels

On the 2026-08-01 local Direct3D 12 run, GPU readback differed from the software
reference at 940 of 254,200 pixels (0.370%), all around rasterized edges, with a
maximum channel delta of 91. The reviewed non-text gate allows at most 0.5% of
pixels and a maximum channel delta of 96 while requiring the listed solid
pixels to match exactly.

An unavailable adapter skips the local GPU test and is not cross-platform
evidence. Metal and Vulkan runs, real window presentation, text, input, IME,
and accessibility remain separate roadmap gates.
