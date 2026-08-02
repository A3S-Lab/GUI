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
5. Graphics prepares and renders pixels;
6. the host presents pixels and returns normalized events.

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

Text shaping, editing, and complete component visuals are intentionally
unfinished. They must land in this generic path, not in per-OS renderers.

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

The `host-macos`, `host-windows`, `host-linux-wayland`, and
`host-linux-x11` features are dependency-free capability markers today.
They do not create real windows yet.

## Shared self-drawn runtime (H1)

The `platform-runtime` feature provides `SelfDrawnWindowRuntime<H, P>`,
where `H` implements `PlatformHost` and `P` implements
`PlatformScenePresenter`.

One frame is atomic across:

1. semantic/layout snapshot validation;
2. scene preparation;
3. host transaction preparation;
4. presentation;
5. host commit;
6. runtime snapshot promotion.

Prepare, commit, reject, presentation failure, and recovery paths are
explicit. A failed frame cannot partially advance the active semantic,
interaction, accessibility, scene, or host revision.

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

## Threading and ownership

- Component state and semantic runtime state are ordinary Rust-owned data.
- TypeScript state remains in the supervised Node process.
- Graphics devices, surfaces, and prepared frames remain in Rust.
- OS window/input/IME/accessibility objects remain inside the matching host.
- Thread-affine resources must not cross the host boundary as raw handles.
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

Real host lanes will be added only when the corresponding zero-widget host
exists. Deleted toolkit and bundle lanes are not retained as migration
evidence.

## Current gaps

The architecture is implemented through the generic scene slice, H0 host
contract, H1 shared runtime, and TSX T1 callback boundary. The remaining
critical work is:

1. production text shaping, editing, IME, and accessibility semantics on the
   generic layout/scene path;
2. concrete Windows, macOS, and Wayland/X11 hosts;
3. the Rust TSX host executable, application message pump, crash recovery, and
   replay over the landed Node process transport;
4. component-by-component React Aria conformance;
5. packaging, signing, installers, and real tri-platform evidence.

See [Roadmap](roadmap.md), [Platform hosts](platform-hosts.md), and
[TSX native runtime](tsx-native-runtime.md).
