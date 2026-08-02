# Renderer Field Inventory

Updated: 2026-08-01

This inventory is the cutover contract between the existing semantic runtime
and the self-drawn A3S Graphics path. A field being parsed into `PortableStyle`
or `NativeProps` is not evidence that layout or paint implements it.

`src/render_contract.rs` is the executable source of truth. It enumerates all
504 current top-level `PortableStyle` fields from their serialized shape and
assigns each one to its first complete delivery milestone. Its tests pin the
field count, so adding or removing a field requires an intentional inventory
update. `NativeRole` and `NativeEventKind` use exhaustive matches, so new enum
variants cannot compile without an assignment.

## Milestone assignments

| Assignment | Owned projection |
| --- | --- |
| Semantic | Raw declarations, custom properties, interaction-variant declarations, cascade bookkeeping, and the parser's explicit unsupported map |
| M3 layout/scene | Box sizing, block and row/column flex flow, explicit and constrained size, padding, margin, gap, inset, solid background and border rectangles, radius, opacity, overflow clipping, z-order, visibility, and pointer hit eligibility |
| M4 interaction/text | Font and text fields, writing direction, editing, caret and selection, focus visuals, normalized input, IME, clipboard, overlays, speech semantics, and accessibility bridges |
| P1 components | Grid, tables, columns, lists, containment, assets, images, advanced backgrounds, SVG component paint, scrolling policy, themes, and complete collection/component projection |
| P2 advanced Graphics | Transforms, masks, filters, blend/isolation, paths and shapes, advanced SVG paint, animation, transitions, and view transitions |

M3 may carry any `NativeRole` through generic box layout. The role assignment
records when its visible content and behavior become complete: structural
containers are M3; text and basic controls are M4; assets, collections,
overlays, and tables are P1; Canvas is P2. Metadata-only roles remain semantic.

## Native tree fields

The `NativeElement` identity, role, props, and ordered children remain the
shared input. The renderer uses a path-qualified identity derived from stable
sibling keys; it must not use a platform handle or a tree index as identity.

`NativeProps` fields are owned as follows:

| Field group | Owner/gate |
| --- | --- |
| `web.class_name`, `web.style`, visibility, orientation | M3 style resolution and layout |
| label, value, placeholder, range values, input hints, editing constraints | M4 text/input/IME; sensitive values are never copied into paint diagnostics |
| action, disabled, required, invalid, read-only, selected, checked, expanded, focus flags | Existing semantic runtime plus M4 hit/focus/action projection |
| accessibility name, description, relationships, structure, and state | Existing semantic truth plus parallel M4 OS bridges; never inferred from paint |
| popover/anchor and overlay metadata | Existing overlay policy plus M4 layout/layer projection |
| resource URLs, intrinsic media size, loading/decoding policy, HTML resource groups | P1 asset and component projection; product I/O remains outside GUI |
| HTML activation, form, collection, dialog, shadow, microdata, and metadata groups | Semantic-only unless a listed M4/P1 projection consumes the field |

## Input, focus, overlay, text, and accessibility

- Input inventory covers every `NativeEventKind`, modality, modifier, position,
  delta, repeat, click count, handled-activation marker, related target, and
  optional value. All belong to M4; coordinates feed GUI-owned hit testing.
- Focus inventory covers node identity, containment, restoration, autofocus,
  disabled state, tab order, focusability, input modality, focus-visible, and
  focus-within. Semantic state exists today; geometry integration is M4.
- Overlay inventory covers modal/underlay/dismiss policies, blur and Escape
  behavior, autofocus, anchor and boundary rectangles, placement, direction,
  offsets, flip policy, container padding, arrow geometry, and maximum height.
  M4 owns the self-drawn layer and focus containment.
- Text inventory includes label/value/placeholder content, font selection and
  fallback, size/weight/style/features, shaping direction and writing mode,
  line layout, wrapping, alignment, decoration, selection, caret, composition,
  and password redaction. Embedded reference fonts are mandatory for fixtures.
  The bounded `TextShaper`/`ShapedText`/`TextSceneEncoder` interfaces and
  pre-shaping password mask are implemented; the concrete font, paragraph,
  glyph, editing, and decoration backends remain M4 work.
- Accessibility inventory includes role, name, value sensitivity, description,
  relationships, structural indices/spans, live state, control state, focus,
  selection, checked/expanded state, children, actions, and announcements. It
  is a parallel product of semantic state and must not be reconstructed from
  Graphics commands.

## Diagnostic rule

The layout/scene adapter reports every requested field whose assigned
milestone has not landed. A field assigned to a later milestone is deferred
with its owner; a field present in `PortableStyle::unsupported` is rejected.
No adapter may treat a parsed field as implemented merely by ignoring it.
