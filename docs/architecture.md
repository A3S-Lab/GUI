# a3s-gui Architecture

## Decision

A3S GUI is a fully self-drawn GUI runtime.

Application content is represented as semantic data, laid out by A3S GUI,
lowered into an A3S Graphics scene, and presented through a zero-widget
platform surface. Platform content controls and platform layout engines are
not part of the architecture.

The repository enforces this decision in code:

- the former AppKit, GTK4, and WinUI content backends are deleted;
- their Cargo features and dependencies are deleted;
- platform-specific content-control examples, packaging, and CI lanes are
  deleted;
- dependency-firewall tests reject those packages and modules from both the
  host contract and shared runtime graphs.

A future OS host may call system window, input, IME, accessibility, clipboard,
or dialog APIs. It must never delegate application-content layout, controls,
or painting to a platform widget toolkit.

## Dependency direction

```text
authoring
   |
   v
semantic frame -> semantic runtime -> layout snapshot -> Graphics scene
                       |                    |                 |
                       |                    |                 v
                       |                    |          software / wgpu
                       |                    v                 |
                       +----------> hit regions               v
                       |                              presented pixels
                       v                                     |
                accessibility tree                           |
                       |                                     |
                       +---------------+---------------------+
                                       v
                              zero-widget OS host
                                       |
                                       v
                         normalized events / system results
```

The allowed direction is one way:

1. authoring creates a versioned semantic frame;
2. the semantic runtime reconciles stable keyed identity and behavior;
3. layout produces deterministic geometry and hit regions;
4. scene extraction produces Graphics primitives;
5. the host stages a zero-widget native window and lends an owned lifetime
   target to Graphics;
6. Graphics prepares, renders, and presents pixels around the host commit;
7. the host returns normalized events and system results.

Graphics does not depend on RSX, TSX, components, accessibility, or windows.
The host does not interpret styles or create application-content controls.

## Authoring boundaries

### Rust RSX

Rust authoring uses `ComponentCx`, `RSX`, hooks, reducers, effects, and
registered semantic components. SWC parses RSX source only when the
`authoring` feature is enabled. The resolved output is a versioned `UiFrame`;
the runtime does not retain parser AST nodes.

### TypeScript TSX

The private `@a3s/gui` package uses the standard automatic JSX runtime.
Stock Node or a Nub-style loader owns module loading and TSX transformation.
The JSX runtime normalizes elements, keys, children, props, windows, styles,
and callbacks into the same frame vocabulary used by Rust.

Node owns TypeScript state and callback execution. Rust owns semantic
reconciliation, layout, Graphics resources, native surfaces, and all OS
handles. The process boundary is the strict length-prefixed TSX protocol in
`src/tsx_protocol/`.

## Semantic runtime

`NativeElement` is the resolved semantic IR. It carries:

- stable key and role;
- normalized semantic, HTML, accessibility, and web-compatible props;
- explicit action identifiers and event subscriptions;
- portable style input;
- ordered children.

The mounted runtime owns identity, reconciliation, focus, focus scopes,
interaction state, selection, collection navigation, overlays, drag/drop,
i18n, announcements, and accessibility snapshots.

Behavior is derived from semantic roles and normalized input. It is never
derived from paint output.

## Layout and scene

The layout boundary produces a deterministic `LayoutSnapshot` with:

- schema version;
- 1/64-point quantized geometry;
- stable keyed paths;
- separate hit regions;
- deterministic diagnostics and fingerprints.

Scene extraction converts semantic/layout records into retained A3S Graphics
primitives. Stable layout paths derive stable `DrawId` values. The software
reference path supplies deterministic pixel evidence; the GPU path supplies
the production renderer boundary.

The first M4 text boundary is implemented in this generic path:

- `LayoutOptions::with_text` requires an explicit, bounded `TextShaper`;
- one validated, 1/64-point-quantized `ShapedText` value supplies both
  intrinsic layout measurement and retained glyph/cluster records;
- source strings are not retained, and password values are masked before the
  shaper is called;
- `TextSceneEncoder` consumes that exact record, may maintain a glyph cache,
  and is rejected if its primitives escape the shaped ink bounds;
- box-only M3 layout remains explicit and never estimates character widths.

The concrete font database, fallback/shaping implementation, glyph raster or
atlas encoder, editing model, and complete component visuals remain unfinished.
They must also live in this generic path, not in per-OS renderers.

## Platform host contract (H0)

The `platform-host` feature exposes only zero-widget contracts:

- `PlatformWindowSpec`, commands, and window events;
- atomic `PlatformHostTransaction` and revisioned commit acknowledgements;
- presentation requests, damage rectangles, and presentation acknowledgements;
- normalized pointer, key, and wheel events;
- text-input/IME session commands and events;
- accessibility snapshots, actions, and limits;
- clipboard, file picker, notification, menu, and permission requests;
- bounded recording-host diagnostics.

Host commands are validated before mutation. Transactions and events have
explicit byte/count limits. Sensitive values are redacted from diagnostics.

On Windows, `host-windows` adds only `windows-sys` and `raw-window-handle` and
provides the first real, thread-affine `WindowsPlatformHost`. It stages raw
top-level HWNDs while hidden, owns their bounded message pump and DPI-aware
lifecycle, and lends Graphics an owned surface token. An active token prevents
HWND destruction. The same pump normalizes DPI-aware legacy mouse, keyboard,
wheel, modifier, capture, and focus-loss state. It does not create child
controls or draw application content. `host-macos`, `host-linux-wayland`, and
`host-linux-x11` remain capability markers until their raw hosts land.

## Shared self-drawn runtime (H1)

The `platform-runtime` feature provides `SelfDrawnWindowRuntime<H, P>`,
where `H` implements `PlatformHost` and `P` implements
`PlatformScenePresenter`.

One frame is atomic across:

1. semantic/layout snapshot validation;
2. host transaction planning and hidden-window staging;
3. surface-target lease and Graphics frame preparation;
4. host commit;
5. Graphics publication/presentation;
6. runtime snapshot promotion and typed presentation status.

Pre-commit preparation and host failures reject the candidate without
advancing the active snapshot. A post-commit `Dropped` or `SurfaceLost` result
advances the logical snapshot to match the committed host state and schedules
a replay; it is never reported as successfully presented.

The shared runtime also owns hit testing, pointer capture, keyboard routing,
press/long-press/move lifecycles, focus, collection navigation, drag/drop
policy, accessibility action routing, and normalized action dispatch. OS hosts
only translate platform events into the contract.

## Accessibility

Accessibility is a semantic projection, never a paint reconstruction.

The runtime computes a versioned accessibility snapshot from the mounted tree,
focus, selection, state, relationships, structure metadata, and live-region
policy. A platform host exposes that snapshot through the operating system's
assistive-technology API and routes accessibility actions back through
`PlatformAccessibilityAction`.

Current capability audits mark semantic features as `Portable`. No native
assistive-technology bridge is claimed until a real zero-widget host supplies
and tests it.

## Nonvisual planning infrastructure

`HeadlessAdapter`, `PlatformPlanningHost`, `CommandExecutingHost`, and
the blueprint/command types under `src/platform/` and `src/backend/` remain
for protocol, reconciliation, transaction, redaction, and failure-injection
tests. They create no visible controls and link no platform toolkit.

`HeadlessAdapter` always reports `NativeBackendKind::Headless` and emits the
diagnostic class `a3s_gui::HeadlessNode`. Production visible applications use
`platform_host` plus `platform_runtime`, not this planner.

The historic “widget” terms in this test IR describe planned semantic nodes;
they do not authorize or imply a widget renderer.

## TSX transaction model

The TSX protocol separates four identities:

- message sequence;
- render revision;
- committed host-frame revision;
- semantic element key.

A candidate callback scope is staged with a render, promoted only by the
matching commit, and discarded on rejection. The active revision and one
rollback revision are retained. Events validate message, render, host-frame,
and element identity before any callback executes. Multi-callback vectors run
sequentially and are awaited in wire order.

This prevents stale UI events from mutating current TypeScript state and
prevents rejected frames from exposing candidate callbacks.

Node-local function-component instances use canonical `a3s:c1:` identities
derived from keyed component addresses. Function components remain erased from
the native tree. Automatic action ids therefore use a separate canonical
`a3s:a1:` identity derived only from the native key path and event prop; adding
or removing a transparent function-component wrapper does not change native or
action identity. The `a3s:` namespace is reserved for generated identities.
Rust owns these constants, emits them in the generated TypeScript protocol
module, and rejects malformed or reserved action identities before advancing a
Host session.

## Threading and ownership

- Component state and semantic runtime state are ordinary Rust-owned data.
- TypeScript state remains in the supervised Node process.
- Graphics devices, surfaces, and prepared frames remain in Rust.
- OS window/input/IME/accessibility objects remain inside the matching host.
- Native surface identity enters Graphics only through an owned lifetime token;
  the host refuses native destruction until the token is released. It is never
  serialized or moved into TSX.
- Callbacks cross process boundaries as versioned action identifiers and
  validated payloads.

## Feature graph

Semantic-only builds do not pull A3S Graphics, wgpu, SWC, Node, or platform
content toolkits. Graphics, software reference rendering, GPU rendering,
authoring, platform contracts, platform runtime, and TypeScript schema
generation are independently gated.

The dependency firewall rejects:

- GTK4/GDK/GSK;
- WinUI/XAML helper crates;
- AppKit content bindings;
- embedded JavaScript runtimes;
- renderer dependencies in the H0 graph.

## Verification

The portable `just verify` gate covers:

- formatting and whitespace;
- feature-graph and dependency-firewall checks;
- semantic-only and renderer feature builds;
- H0 and H1 contract/runtime tests;
- Rust protocol generation and drift;
- TypeScript type checking and Node fixtures;
- Clippy and rustdoc warnings;
- all maintained Rust tests and examples;
- React Aria catalog schema and coverage;
- software and GPU boundary tests.

Target-native CI additionally runs Win32 lifecycle, normalized legacy input and
cancellation, H1 first-frame transaction, surface-lease rollback, and real
Graphics/DX12 presentation tests. Equivalent macOS/Linux lanes arrive with
their hosts. Deleted toolkit and bundle lanes are not retained as migration
evidence.

## Current gaps

The architecture is implemented through the generic scene slice, H0 host
contract, H1 shared runtime, the first raw H2 Win32 lifecycle/DX12 surface slice,
and the complete T2 stateful TSX runtime, including the strict software
self-drawn process host, ordered Node/Rust application pump, finalized
component/action identity contract, bounded restart/replay, and
restarted-process keyboard/stale-event gates. The remaining critical work is:

1. production font discovery/shaping and glyph encoding behind the landed
   generic layout/scene contracts, followed by editing, IME, and accessibility
   semantics;
2. Windows touch/pen, TSF/UIA, device-loss evidence, and visible TSX completion
   plus raw macOS and Wayland/X11 hosts;
3. component-by-component React Aria conformance;
4. packaging, signing, installers, and real tri-platform evidence.

See [Roadmap](roadmap.md), [Platform hosts](platform-hosts.md), and
[TSX native runtime](tsx-native-runtime.md).
