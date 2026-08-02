# A3S GUI Roadmap

Updated: 2026-08-02

## Product Direction

A3S GUI is a Rust-native, cross-platform implementation of the interaction,
accessibility, layout, and component model represented by
[React Aria](https://react-aria.adobe.com/getting-started). It does not embed a
browser and does not expose a DOM or CSSOM at runtime.

Application content is moving from three operating-system widget renderers to
one A3S-owned drawing pipeline. The shared
[`a3s-graphics`](https://github.com/A3S-Lab/Graphics) crate owns the versioned
graphics scene, retained frame damage, render preparation, deterministic
software reference renderer, and GPU rendering. Its production GPU backend is
built directly on `wgpu` for Metal, Direct3D 12, and Vulkan. No UI framework
owns the scene, layout, or content renderer.

A3S GUI continues to own the UI-specific layers: semantic components, portable
style, layout, text editing, hit testing, interaction, focus, selection, IME
coordination, and accessibility. Rust RSX remains the in-process authoring path.
An optional TypeScript TSX frontend is planned as an external Node application
runtime that emits the same resolved, versioned UI records; it does not add a
browser or a second renderer. Platform code becomes a thin host for windows,
input, IME, accessibility bridges, menus, dialogs, clipboard, and presentation.
It does not choose component geometry or draw content.

## Planning Rules

- Milestones are dependency ordered. A later prototype cannot compensate for a
  failed earlier contract or parity gate.
- Status is based on executable evidence, not the presence of a type or stub.
- Changes land as small, reviewable commits and are pushed as soon as their
  tests pass. There is no milestone-sized integration branch.
- New work follows the self-drawn path. Legacy control backends are frozen
  except for fixes needed to preserve the migration baseline.
- Dead code is removed with its last consumer. A compatibility module is not
  deleted before an equivalent tested path exists, and it does not remain after
  the cutover gate passes.
- Every visual fixture must lower the shared `NativeElement` tree. A bespoke
  calculator or component-only scene does not count as renderer progress.
- The complete official React Aria component catalog is version-pinned in
  `docs/react-aria-component-matrix.json`. Every upstream release requires an
  audited matrix delta; a registered component name alone is not parity.
- Unsupported style, text, graphics, input, or accessibility fields are
  explicit diagnostics. No layer silently discards a built-in requirement.
- Files over 1,000 lines are split when their area is changed; new files target
  one concern and remain well below that threshold.

| Priority | Scope |
| --- | --- |
| P0 | Architecture cleanup, Graphics GPU foundation, generic layout/scene path, calculator slice, input/IME/accessibility, and legacy backend removal |
| P0-H | Zero-widget platform-host contract, shared presentation runtime, macOS/Windows/Linux shells, and dependency-audited cutover |
| P0-T | Optional TSX-to-native authoring track: automatic JSX runtime, versioned local session, Node-owned component state, and self-drawn host integration |
| P1 | All 51 React Aria 1.19.0 component families and public semantic parts, including full self-drawn behavior, accessibility, overlays, collections, date/color controls, tables, virtualization, themes, assets, and localization |
| P2 | Developer tooling, animation, advanced content surfaces, performance work, and shared Graphics capabilities needed by future game runtimes |

## Non-Negotiable Architecture

```text
Rust ComponentCx + optional RSX       TSX components in Node / Nub
                    |                         |
                    |                         v
                    |               @a3s/gui JSX runtime
                    |                         |
                    +------------+------------+
                                 |
                                 v
                 resolved, versioned UI frame
                         |
                         v
              semantic NativeElement tree
                 /         |          \
                /          |           \
               v           v            v
       UI layout tree   semantic tree   interaction tree
               |       + accessibility  + hit regions
               v
       a3s-graphics Scene
               |
               v
          FramePlanner
               |
               v
       render preparation
          /           \
         v             v
software reference   wgpu renderer
                         |
            +------------+------------+
            v            v            v
          Metal         DX12        Vulkan
            |            |            |
            +------------+------------+
                         |
                         v
             thin platform window host
                         |
                         v
        normalized input / IME / accessibility
                         |
                         v
              InteractionState -> actions
```

The UI layout tree, paint scene, semantic tree, and interaction tree share
stable element identity but remain separate data products. Paint commands do
not become the accessibility tree, and the graphics engine never infers an
action from a colored rectangle.

### Ownership Boundaries

| Layer | Owns | Must not own |
| --- | --- | --- |
| Authoring and design system | Rust components and RSX; optional Node-owned TSX components and hooks; typed props, contracts, variants, tokens, and Stories | GPU resources, native handles, layout truth, product workflows inside the GUI core |
| GUI semantic runtime | `NativeElement`, reducer flow, actions, interaction, focus, selection, overlays, i18n, capabilities, and accessibility | Graphics device, window toolkit state, product I/O |
| GUI layout and scene adapter | portable style resolution, intrinsic measurement requests, layout boxes, paint extraction, hit regions, scene diagnostics | product state, backend-specific GPU calls, OS widget geometry |
| A3S Graphics | scene schema, geometry, draw identity, damage, preparation, resources, shaders, software reference, and GPU rendering | RSX, CSS, widgets, accessibility, IME, game world/ECS, windows |
| Platform host | windows, event loop, raw input, IME, accessibility bridge, clipboard, menus, dialogs, surface attachment, and frame presentation | component layout, style interpretation, application-content drawing |
| Product application | model/messages, effects, data sources, storage, capability broker, ACL loading, theme and asset provisioning | renderer handles, scene mutation after submission, platform branching in components |

Graphics types may flow into the GUI scene adapter and platform presentation
edge. `wgpu` types remain inside `a3s-graphics` and its surface integration.
Thread-affine platform and GPU handles never enter protocol, semantic, or
authoring APIs.

The TSX process/session boundary and its dependency-ordered delivery gates are
defined in the [TSX native runtime architecture](tsx-native-runtime.md). The
TSX SDK reuses the resolved frame input vocabulary but does not expose legacy
planned-widget commands as its public host protocol.

The operating-system boundary and its H0-H5 delivery gates are defined in the
[self-drawn platform host architecture](platform-hosts.md). The target macOS
binary uses AppKit only as the system application/window shell around one
custom Metal-backed view. Linux uses Wayland/X11 without GTK4, and Windows uses
Win32 without WinUI/XAML. None of these shells owns application-content
layout, controls, or pixels.

## Cross-Platform Consistency Contract

One scene and one renderer eliminate toolkit layout divergence, but GPU drivers
and host text services can still differ. Consistency is therefore proven at
several layers rather than inferred from one screenshot hash.

### Deterministic Reference Environment

Reference fixtures fix:

- logical viewport, scale, sRGB target, and transparent-background policy
- light/dark theme, density, locale, direction, time zone, and reduced motion
- animation time, focus visibility, input modality, clock, and random seed
- embedded font bytes, fallback order, icons, and raster assets
- Graphics scene schema, shader revision, and snapshot schema

System fonts are forbidden in reference fixtures. Every checked-in asset must
carry its source, license, and checksum.

### Required Evidence

| Layer | Gate |
| --- | --- |
| Semantic | Canonical Native IR and accessibility fingerprints match for the same input |
| Layout | Quantized boxes, baselines, clipping, scroll extents, and z-order match |
| Scene | Ordered primitives, transforms, clips, resources, opacity, and hit identities have the same fingerprint |
| Software image | Repeated clean and incremental renders are byte-identical |
| GPU image | Metal, DX12, and Vulkan output stays within reviewed non-text and text thresholds against the software reference |
| Interaction | The same scenario produces the same actions, focus path, model state, and final scene fingerprint |
| Accessibility | The same supported semantic nodes and actions reach each OS bridge; platform smoke evidence verifies exposure |

Layout drift, different line breaks, missing glyphs, missing assets, or changed
component geometry always fail even when an aggregate image metric passes.

The first required visual fixture remains the calculator at 410 by 620 logical
pixels. Component Stories add fixed viewports after that slice is stable.

## Current Baseline

The repository already provides:

- Rust `ComponentCx` functions, `.rsx` templates, reducers, hooks, routing,
  contracts, and a broad semantic component registry
- compiled RSX, shared Native IR, stable element keys, protocol frames, ordered
  transactions, rollback, ACK validation, and recovery replay
- portable style parsing with Tailwind-compatible utilities and interaction
  variants
- press, hover, focus, selection, collection navigation, overlays, i18n,
  NumberField behavior, and live-region semantics
- accessibility names, descriptions, relationships, states, structure,
  conformance checks, and capability reports
- a shared calculator state model, reducer, RSX component tree, and three
  platform entrypoints
- AppKit, GTK4, and WinUI control backends used as migration baselines
- an H0 zero-widget platform-host contract with stable accessibility identity,
  bounded revision transactions, a recording host, target feature markers, and
  executable dependency/source firewalls

The independent Graphics repository has a versioned scene, stable draw IDs,
canonical fingerprints, retained damage, affine transforms, clipping, opacity,
solid and rounded rectangles, borders, a deterministic software renderer, and
an owned `wgpu` rectangle backend as of commit `8748fab`. The GPU backend has
local Direct3D 12 evidence; Metal and Vulkan CI evidence remains an M2 gate.

## Cleanup Inventory

The existing control renderer remains compatibility code during the cutover.
Its removal gates are explicit so “temporary” code cannot become permanent.

| Current area | Migration use | Removal gate |
| --- | --- | --- |
| `renderer.rs`, `host.rs` | Stable-tree and rollback baseline | New layout/scene renderer preserves keyed state, transaction behavior, and runtime queries |
| `platform/`, `backend/` | Portable command and recovery baseline | Scene frames, resource commits, presentation ACKs, and recovery have equivalent tests |
| `appkit.rs`, `gtk4.rs`, `winui.rs` | Headless widget-planning evidence | Generic scene and capability audits replace class/setter assertions |
| `appkit_native/`, `gtk4_native/`, `winui_native/` | Current real input, IME, accessibility, menu, dialog, and window evidence | H2-H4 hosts cover those services and all three self-drawn calculator lanes pass |
| platform-specific examples | Migration comparison and OS smoke | One shared self-drawn example plus platform-host smoke runners covers the same scenarios |
| legacy Cargo features and dependencies | Build compatibility | Their final source and CI consumer is deleted in the same commit |
| native-input conformance artifacts | Behavioral evidence | Generalized host manifests preserve or strengthen every claimed scenario |

Immediate cleanup includes obsolete renderer plans, unused dependencies,
unreferenced exports, duplicate wrappers, completed TODOs, and docs for paths
that no longer exist. Large active modules are split by concern as their
milestones touch them; tests move beside the extracted concern rather than into
another catch-all file.

## P0 Renderer Program

### M0 - Graphics boundary and deterministic core

Status: complete at Graphics commit `2cad948` and the pinned GUI boundary.

Deliverables:

- standalone `a3s-graphics` repository with no GUI or window dependency
- accepted Graphics architecture and roadmap
- versioned scene, validation, stable IDs, fingerprints, retained damage, and
  deterministic reference rasterization
- GUI decision record selecting Graphics and rejecting framework-owned drawing

Acceptance gates:

- Graphics default and no-default-feature checks pass on Rust 1.95
- software incremental output equals a clean full repaint
- scene validation rejects duplicate identity, non-finite geometry, invalid
  transforms, invalid scale, and invalid opacity before mutation
- GUI contains no obsolete framework-renderer dependency, module, feature, or
  active architecture plan

### M1 - GUI architecture cleanup and dependency integration

Status: complete.

Landed evidence:

- architecture and public docs now select A3S Graphics with no framework-owned
  renderer residue
- the engine dependency is pinned to full commit
  `8748fab595f8dd7f7ca28767f1c58bd7f3f34ee0`
- `graphics`, `software-reference`, and `gpu` separate scene consumers from
  reference and accelerated rendering while no-default remains semantic-only
- `ReferenceRenderer` preserves frame fingerprints and retained damage behind a
  GUI-owned error boundary
- `GpuSceneRenderer` preserves the same scene/planner boundary and maps GPU and
  readback failures into the GUI error contract
- the first compatibility cleanup removed class-name widget mapping shims and
  replaced broad dead-code allowances with target-accurate compilation
- the versioned renderer inventory accounts for all 504 `PortableStyle`
  fields, every `NativeRole`, all normalized input events, and the focus,
  overlay, text, and accessibility records required by cutover
- the first generic `NativeElement -> LayoutSnapshot -> Graphics Scene`
  adapter landed with stable keyed identity and explicit projection diagnostics

Deliverables:

- replace obsolete architecture, README, and roadmap claims with this boundary
- pin GUI to one reviewed Graphics commit; no floating branch dependency
- define feature boundaries for semantic-only, software-reference, GPU, and
  platform-host builds
- add a compile-time dependency-direction gate
- inventory every `PortableStyle`, `NativeRole`, input, focus, overlay, text,
  and accessibility field needed by the cutover
- remove only code proven to have no current consumer or superseded contract
- split touched 1,000-line modules before adding new responsibilities

Acceptance gates:

- source and dependency searches find no obsolete framework-renderer residue
- `cargo check --no-default-features --lib` remains green
- default tests remain green before and after cleanup
- dependency tooling reports no unused direct dependencies
- deletion commits name the replacement evidence for every removed path

### M2 - Graphics GPU backend

Status: implementation landed at Graphics commit `8748fab`; cross-platform CI
evidence pending.

Deliverables:

- owned `wgpu` device selection and capability report
- surface-independent sRGB render target and asynchronous readback
- WGSL pipelines for fills, rounded rectangles, borders, affine transforms,
  clipping, opacity, and ordered source-over blending
- bounded grow-only instance buffers and frame diagnostics
- typed adapter absence, device loss, validation, internal, out-of-memory,
  capacity, and readback errors

Acceptance gates:

- shader and pipeline validation passes for Metal, DX12, and Vulkan CI targets
- GPU output matches the software fixtures within reviewed edge-AA thresholds
- transparent overlap preserves command order
- GPU-disabled Graphics builds contain no `wgpu` dependency

### M3 - Generic layout and scene vertical slice

Status: current.

Landed evidence:

- layout schema version 1 records 1/64-point quantized boxes, stable
  length-prefixed element paths, paint, clips, z/order, and separate hit regions
- the generic engine covers the calculator's row/column flow, box model,
  explicit/min/max size, alignment, absolute positioning, overflow clipping,
  opacity, and solid rectangle paint without a calculator renderer model
- unsupported M3 fields are error diagnostics rejected by scene extraction;
  later role/style work remains visible as warnings
- stable layout diffs feed the Graphics scene/damage diff, and stable layout
  paths derive retained `DrawId` values
- the existing shared 410 by 620 calculator Native IR pins layout fingerprint
  `16529597026056060935` and scene fingerprint `2100550662756266801`
- repeated software output is byte-identical with no retained damage; the local
  Direct3D 12 readback passed the reviewed 0.5%/96 non-text threshold with
  exact solid-color checkpoints

Remaining work includes full flex growth/shrink/wrap, complete stacking
contexts, redraw scheduling, Metal/Vulkan evidence, and real thin-host window
presentation.

Deliverables:

- versioned GUI layout records with stable element identity
- deterministic block, row/column flex, padding, margin, gap, explicit/min/max
  size, alignment, absolute positioning, overflow clipping, and z-order for the
  calculator subset
- `NativeElement -> layout -> Graphics Scene` lowering with diagnostics
- separate hit regions and semantic references keyed to the same elements
- retained layout/scene diff and redraw scheduling
- software and GPU scene snapshots for the existing shared calculator tree

Acceptance gates:

- no calculator-specific visual tree or platform-specific component layout
- the same Native IR produces the same layout and scene fingerprints on all
  three desktop systems
- unsupported required style fails the fixture instead of being omitted
- incremental layout and scene output equals a clean rebuild
- the rectangle-only path presents through the H1 contract inside real H2-H4
  macOS, Linux, and Windows windows

### M4 - Text, input, IME, accessibility, and overlays

Status: in progress alongside M3; shared zero-widget pointer, keyboard,
long-press, move, and drag/drop foundations landed, while text, editing/IME,
accessibility bridges, overlays, and real thin-host input remain.

Deliverables:

- embedded reference fonts, shaping, fallback, line layout, glyph atlas, and
  text selection geometry through Graphics
- platform input normalization and GUI-owned pointer capture, hover, focus,
  keyboard navigation, scrolling, drag, and gesture policy
- text editing with composition ranges, candidate positioning, clipboard,
  caret, selection, password handling, and platform IME bridges
- parallel accessibility tree bridges for macOS, Linux, and Windows
- overlay layout, clipping, stacking, focus containment, dismissal, and system
  menu/dialog exceptions

Acceptance gates:

- the 410x620 calculator passes reference/GPU image, keyboard, pointer, focus,
  and action scenarios on all desktop hosts
- composition tests cover CJK, dead keys, emoji, RTL, selection replacement,
  and cancellation
- screen-reader/UI Automation smoke verifies names, roles, values, states,
  actions, focus, and live regions
- password values never enter scene snapshots, diagnostics, or automation logs

### M5 - Default cutover and legacy deletion

Status: planned after M4.

Deliverables:

- self-drawn renderer becomes the sole application-content path
- one shared calculator, controls, playground, and dogfood entrypoint
- AppKit/GTK4/WinUI widget creation, styling, layout, and update code removed
- thin platform hosts retain only windows, input, IME, accessibility, menus,
  dialogs, clipboard, surface presentation, and automation hooks
- obsolete features, dependencies, examples, packaging entries, CI lanes,
  command schemas, exports, tests, and documentation removed with their last
  consumers

Acceptance gates:

- source search finds no platform widget creation for application content
- default macOS, Linux, and Windows builds open the self-drawn calculator
- the full semantic, interaction, accessibility, reference-image, GPU-image,
  recovery, packaging, and platform smoke matrix passes
- target Linux and Windows dependency trees and artifacts contain no
  GTK4/GDK/GSK or WinUI/XAML runtime; macOS retains only an audited AppKit
  system-shell feature allowlist with no content-control path
- fresh builds prove removed legacy dependencies and features are absent from
  `Cargo.lock`, `cargo tree -e features`, and distribution imports
- repository file-size and dead-code audits pass

## P0-H Self-Drawn Platform Host Track

Status: H0 complete; H1 atomic frame/lifecycle work is in progress.

This track turns the existing offscreen layout/scene/GPU boundary into real
windows without moving component rendering back into an OS toolkit. All
application content remains A3S-owned. A top-level `NSWindow`, `wl_surface`,
X11 window, or `HWND` is an operating-system presentation target, not a native
component tree.

The complete ownership model, target platform API matrix, contract surface,
migration rules, and verification matrix are recorded in
[`platform-hosts.md`](platform-hosts.md).

### H0 - Host contract and dependency firewall

Status: complete.

- add typed window, presentation, input, text-input, accessibility, and system
  service records under a new `platform_host/` boundary
- add fake-host conformance and complete frame/service transaction tests
- plan `host-macos`, `host-windows`, `host-linux-wayland`, and
  `host-linux-x11` features without enabling legacy backends
- add dependency and source audits that forbid application-content widgets

Gates:

- the contract exposes no widget create/update/remove operation
- host records contain no component style, Node.js value, toolkit object, or
  `wgpu` handle
- semantic-only builds stay free of Graphics and platform dependencies
- target features do not import or enable legacy renderer modules

Evidence:

- `platform_host/` owns bounded records for windows, presentation, raw input,
  text input, stable-id accessibility, system services, and ordered events
- `PlatformHostTransaction` validates one monotonic revision before mutation;
  `RecordingPlatformHost` proves prepare/commit/rollback, failed-commit
  recovery, bounded queues/history, redaction, and explicit shutdown
- `platform-host`, `host-macos`, `host-windows`, `host-linux-wayland`,
  `host-linux-x11`, and `host-linux` compile without enabling a legacy backend
- 13 focused contract tests and three recursive source/feature firewall tests
  cover the H0 gates; `just verify` includes their build, graph, and test lanes

### H1 - Shared self-drawn window runtime

Status: in progress; atomic frame orchestration, presentation lifecycle, and
portable input/reducer routing landed, while a real raw-surface presenter
remains.

- transact scene, hit-region, accessibility, and window state as one committed
  host frame
- attach Graphics to a raw platform surface and handle resize, scale, damage,
  occlusion, redraw, surface loss, and presentation acknowledgements
- route host events through portable interaction, focus, and reducer state
- add one shared self-drawn calculator and fake-host smoke entrypoint

Landed evidence:

- `platform-runtime` connects one candidate Native IR, layout, hit-region,
  Graphics Scene, and accessibility snapshot to a monotonic host transaction
- scene candidates use prepare/publish/discard semantics so rejected host
  commits retain the complete previous frame and pixels
- unchanged frames perform no layout, scene, host, or present work;
  semantic-only changes avoid redundant presentation
- resize, fractional scale, damage, occlusion, redraw, delayed acknowledgement,
  dropped-frame, and surface-loss tests preserve stable semantic ids
- raw pointer, keyboard, Tab-focus, hover, press, cancellation, and wheel events
  resolve through committed layout hit regions and `PlatformElementId` paths;
  action selection shares the semantic callback rules used during migration
  without importing a widget blueprint or legacy runtime
- ordered action batches carry the hit-tested frame revision and monotonic
  event sequence, preserve bubbling current targets and static payloads, and
  restore staged interaction state if an application reducer fails
- long press exposes a monotonic event-loop deadline, tracks style-only and
  callback-driven targets, resets across pointer leave/re-entry, recognizes at
  the deadline or on release as a scheduling fallback, and atomically emits
  `LongPressEnd`, `PressCancel`, then terminal `LongPress`
- move starts only after the first non-zero pointer delta, stays captured
  outside the original hit region, preserves initiating pointer identity and
  incremental deltas across keyed frames, reference-counts concurrent moves,
  and shares reducer rollback; arrow keys emit a handled one-unit lifecycle
- drag starts on the first non-zero primary-pointer delta or keyboard Enter,
  cancels the competing press/long-press path, remains captured by stable id,
  and exposes typed source data plus allowed and negotiated
  copy/move/link/cancel operations; pointer hit testing reports target-local
  coordinates, while keyboard Tab visits only compatible targets and Enter or
  Escape commits or cancels the session
- drop targets match multiple source types, MIME wildcards such as `image/*`,
  custom exact types, and `all`; `DropEnter`/`DropMove`/`DropExit` ordering,
  `data-[dragging]`/`data-[drop-target]` state, keyed-frame reconciliation, and
  reducer failure rollback share the same transactional interaction session
- valid ordinary targets and collection item targets schedule React Aria's
  800ms `DropActivate` lifecycle through the same monotonic host deadline;
  movement inside an equivalent target updates context without postponing the
  timer, while target changes, exit, drop, cancellation, and invalid
  reconciliation reset or clear it. Keyboard and pointer paths are identical,
  and collection root targets intentionally do not activate
- drag sources retain multiple text items and every per-item MIME/custom
  representation; target callbacks receive only matching items without losing
  the other representations on those items, while legacy `dragType` plus
  `dragValue` normalizes to one compatible text item
- collection drag sources aggregate the stable keys, payloads, and visual
  dragging state of selected draggable items. The layout-backed delegate emits
  React Aria-shaped root and keyed before/on/after targets, routes
  external `onRootDrop`/`onInsert`, item `onItemDrop`, internal `onMove`, and
  same-parent `onReorder` with ordered multi-callback dispatch and low-level
  `onDrop` precedence. It filters selected descendants, rejects internal
  self/descendant targets, treats adjacent insertion descriptors as one target,
  and exposes each collection as one Tab stop with arrow/Home/End navigation.
  ListBox, GridList, Tree, Table, and explicit DropIndicator authoring all lower
  to this shared self-drawn path
- generic targets and collection targets expose a synchronous
  `getDropOperation` policy, and collection item-on targets expose
  `shouldAcceptItemDrop`. Queries carry the committed frame revision, event
  sequence, query sequence, stable policy id, typed target, drag types, and
  allowed operations. High-level collection drops filter each item again at
  drop time, low-level `onDrop` bypasses that high-level filter, and missing,
  stale, timed-out, malformed, or disallowed responses resolve to `cancel`.
  Protocol v1 includes strict query/response DTOs and an exchange adapter whose
  transport must own the bounded wait; the future Node runtime still owns
  callback execution
- the shared 410x620 calculator preserves its reviewed layout and scene
  fingerprints, routes eight fake-host events through four reducer actions,
  commits the resulting frames, and reaches display value `10`
- 67 focused runtime/software tests plus four recursive H1 firewall tests are
  included in `just verify`

Remaining H1 work:

- a Graphics raw-surface presenter implementation for the H2-H4 OS shells;
  pinned Graphics commit `8748fab` owns only a surface-independent texture and
  readback today, so its safe host-owned surface attachment/recovery contract
  must land before GUI can implement this edge without duplicating `wgpu`
- native file/directory and cross-application transfer, drag previews, text
  editing, IME, overlay gestures, the Node-side policy callback transport, and
  component-specific pixel/accessibility/real-host conformance remain explicit
  M4 and M6-M8 work

Gates:

- unchanged frames produce no layout, scene, or present work
- rejected frames retain the last committed visual, interaction, and
  accessibility revisions
- resize and scale changes preserve stable semantic identity
- the fake host proves zero application-content widgets by construction

### H2 - Windows Win32 host

Status: planned after H1; its rectangle slice can land during M3 and its full
input/IME/accessibility gate depends on M4.

- own `HWND` lifecycle, message pumping, and DX12-backed Graphics presentation
- translate pointer, keyboard, wheel, focus, DPI, clipboard, and window events
- bridge Text Services Framework and UI Automation directly to portable state
- package and automate without initializing XAML

Gate: the shared calculator passes rendering, input, composition, UI
Automation, resize/DPI, recovery, and close scenarios with no WinUI/XAML
content object or dependency.

### H3 - macOS system-shell host

Status: planned after H1; its full gate depends on M4.

- own `NSApplication`, `NSWindow`, and one custom root `NSView`
- attach a `CAMetalLayer`, translate `NSEvent`, and handle scale/occlusion
- implement `NSTextInputClient` over GUI-owned text state
- project the portable accessibility tree through method-based APIs

Gate: the shared calculator passes rendering, input, composition,
VoiceOver-facing semantics, resize/scale, recovery, and close scenarios without
creating `NSButton`, `NSTextField`, `NSStackView`, or any application-content
AppKit control.

### H4 - Linux Wayland/X11 host

Status: planned after H1; its full gate depends on M4.

- use Wayland plus `xdg-shell` as the primary window path, with X11 behind a
  separate fallback feature
- attach Vulkan-backed Graphics surfaces and translate compositor/input state
- integrate compositor text input, AT-SPI2, clipboard protocols, and portals
- report unsupported compositor/IME capabilities instead of dropping them

Gate: the shared calculator passes the declared Wayland/X11 compositor, input,
IME, AT-SPI, scale/configure, portal, recovery, and disconnect matrix with no
GTK4, GDK, or GSK dependency.

### H5 - Host cutover and legacy deletion

Status: planned after H2-H4 and M4; completes M5.

- switch desktop defaults, shared examples, packaging, and CI to the new hosts
- pass one cross-platform calculator/control conformance matrix
- delete legacy renderer features, controls, adapters, examples, dependencies,
  packaging, tests, and documentation in platform-scoped commits
- retain only the audited macOS system-shell AppKit binding surface

Gates:

- source and runtime audits report zero application-content platform widgets
- target dependency trees and packaged imports pass the platform allowlists
- all semantic, layout, scene, software/GPU, input, IME, accessibility,
  recovery, packaging, and teardown evidence is green
- every superseded legacy consumer is removed in the same platform cutover

## P0-T TSX Native Authoring Track

Status: architecture accepted; the Rust-side strict handshake/framing,
transactional render/commit/event session, self-drawn snapshot/event adapters,
counter golden/parity fixtures, and revision-scoped drop-policy
protocol/resolver adapter have landed. Rust-generated TypeScript declarations,
a fixed schema fingerprint, a private `@a3s/gui` package skeleton, and shared
Rust/Node fixture gates have also landed. The automatic JSX entry points,
synchronous function-component expansion, strict frame normalization,
callback-to-action lowering, and real TSX type gate are now included. Process
I/O, committed callback scopes, state/hooks, the native host, and a visible TSX
application remain.

This track is dependency-coupled to the renderer and H0-H5 host programs
without blocking Rust RSX work. Headless protocol and JSX-runtime work can
begin during M3. A supported visible TSX application cannot ship until H1, at
least one H2-H4 platform slice, and the minimum M4 text, input, focus, and
accessibility slice are complete.

The target command is `nub app.tsx`. Nub or another standard TSX tool emits
automatic JSX-runtime calls into `@a3s/gui`; the TypeScript runtime resolves
components and callbacks into versioned frame records; an independent Rust
host owns native windows and the existing Native IR/layout/Graphics pipeline.
No DOM, WebView, React renderer, embedded JavaScript engine, or default N-API
GUI host enters the core.

The full process, protocol, identity, package, failure, security, and testing
decisions are recorded in
[`tsx-native-runtime.md`](tsx-native-runtime.md).

### T0 - Architecture and cross-language contract

- accept the process boundary, ownership model, full-frame transport, action
  identity, and no-install-script packaging decisions
- reuse the resolved `ProtocolUiFrameV1` input vocabulary behind a new TSX
  session envelope
- extend the landed Rust RSX/static-TSX counter parity fixture to the
  calculator; generated TypeScript and Node fixture CI now cover the counter

Gate: TSX is a peer authoring frontend and cannot bypass Native IR, layout,
Graphics, interaction, accessibility, or capability checks.

### T1 - Headless JSX and protocol slice

Status: Rust transport foundation in progress. Strict `hello`/`welcome` plus
`render`/`committed`/`event` DTOs, atomic negotiation and commit ordering, the
fixed protocol/session/message/revision envelope, independent TSX and
self-drawn host revisions, 16 MiB-capped little-endian framing, incremental
decoding, four canonical JSON fixtures, and static counter Native
IR/accessibility parity have landed. The core remains free of Node and Graphics
dependencies; self-drawn conversions compile only with `platform-runtime`.
All numeric `u64` fields are bounded to JavaScript's safe integer range, full
64-bit scene/layout fingerprints use fixed hexadecimal strings, and optional
`typescript-schema` generation now produces the checked-in declarations and
fingerprint. Standard automatic JSX entry points, immutable elements,
synchronous function-component expansion, strict child/key/prop/style/window
normalization, deterministic callback-to-action ids, and read-only per-frame
callback snapshots now lower into those declarations. Node 24 canonicalizes
the same four fixtures, while pinned TypeScript 5.9 type-checks a real
`react-jsx`/`jsxImportSource` counter; the package has no production dependency
or install script.

- committed/rollback callback scopes and ordered event-vector dispatch
- add command messages, local process I/O, and generated structured diagnostic
  declarations to the landed application session
- extend the landed static counter semantic parity to the calculator fixture

Gates:

- cross-language golden frames canonicalize identically
- malformed and stale input fails before committed-state mutation
- Rust RSX and static TSX counter Native IR/accessibility fingerprints match
- Rust-only and semantic-only builds contain no Node, Nub, N-API, or npm
  dependency

### T2 - Stateful TypeScript runtime

- function components, state/reducer/memo/ref/context hooks, and post-commit
  effects
- revision-scoped callback registry and ordered multi-invocation event batches
- rerender coalescing, rollback, cleanup, graceful shutdown, and development
  host replay

Gates:

- counter interaction, keyed rerender, stale event, effect, cleanup, host loss,
  and replay tests pass
- one event batch schedules at most one next frame
- a rejected frame preserves the last committed UI and callback scope

### T3 - Self-drawn native window

Dependencies: H1, one supported H2-H4 host, and minimum M4 text/input work.

- launch the real platform host from `nub app.tsx`
- present the TSX counter and shared calculator through A3S Graphics
- expose typed focus, clipboard, window, inspector, and shutdown commands

Gates:

- TSX creates no legacy application-content widget
- Rust RSX and TSX calculator scenarios produce the same model, Native IR,
  layout, scene, interaction, and accessibility evidence
- software, GPU, input, focus, and OS accessibility gates pass

### T4 - Watch mode and tri-platform packaging

- transactional last-good-frame reload under `nub watch`
- TSX source mapping for native diagnostics
- prebuilt optional platform packages with no install-time downloader
- macOS, Linux, and Windows launch, signing, packaging, and recovery lanes

Gate: editing keeps the last good frame on errors, clean installs resolve the
correct signed host without runtime downloads, and all three platform packages
pass the counter and calculator smoke matrix.

### T5 - Production SDK

- stable public TypeScript API and semantic component declarations
- cross-version SDK/host compatibility policy and release automation
- inspector, replay, accessibility audit, and performance telemetry
- production examples and browser React/TSX migration guidance

Gate: TSX is documented as supported only after macOS, Linux, and Windows pass
the renderer, interaction, accessibility, recovery, packaging, and protocol
compatibility matrix.

## P1 Component Projection

Catalog accounting starts before the default cutover so no semantic family is
lost during the renderer migration. Component completion starts after the
shared self-drawn runtime gate. The executable matrix pins all 51 official
React Aria Components 1.19.0 families and currently records eight public-part
gaps: the Field/Button splits for Checkbox, Radio, and Switch, plus ToastList
and ToastContent.

Every family advances independently from `planned` to `scene-smoke` and then
to `conformant`. A family lands through its semantic behavior, layout, scene,
hit testing, accessibility, visual Story, software oracle, and real
macOS/Windows/Linux host evidence together. Existing AppKit, GTK4, or WinUI
content-control execution is migration comparison evidence, never final
self-drawn conformance.

### M6 - Foundations and forms

- Breadcrumbs, Button, Checkbox, CheckboxGroup, FileTrigger, Form, Group, Link,
  Meter, NumberField, ProgressBar, RadioGroup, SearchField, Separator, Slider,
  Switch, TextField, ToggleButton, ToggleButtonGroup, and Toolbar
- tokens, themes, density, disabled/read-only/invalid states, focus rings, and
  reduced motion
- intrinsic sizing and baseline alignment
- implement CheckboxField/CheckboxButton, RadioField/RadioButton, and
  SwitchField/SwitchButton composition without duplicating shared state

### M7 - Overlays, selection, and collections

- Autocomplete, ComboBox, Disclosure, DisclosureGroup, DropZone, GridList,
  ListBox, Menu, Modal, Popover, Select, Tabs, TagGroup, Toast, Tooltip, Tree,
  and Virtualizer
- portals/layers, anchored placement, scroll containers, virtualization,
  typeahead, range selection, and collection mutation
- implement ToastList and ToastContent and preserve drag/drop, focus-scope,
  selection-indicator, collection-section, and load-more semantic parts
- close the executable 1.19.0 behavior deltas: embedded-control keyboard
  navigation for GridList/Tree, Menu action key plus value, arbitrary Popover
  target rectangles, and multi-MIME/wildcard drag type negotiation
- build on the landed shared collection root/item/insertion delegate,
  reorder/move policies, and dynamic item/operation acceptance with OS
  transfer, drag previews, the Node policy transport, and
  software/accessibility/three-host conformance stories

### M8 - Date, color, tables, and advanced data

- Calendar, ColorArea, ColorField, ColorPicker, ColorSlider, ColorSwatch,
  ColorSwatchPicker, ColorWheel, DateField, DatePicker, DateRangePicker,
  RangeCalendar, Table, and TimeField
- date/time/range state, color models, data grids, column resizing, sorting,
  large data sets, and localized formatting
- virtualization budgets and stable accessibility semantics for recycled views

## P2 Runtime, Tooling, and Shared Graphics

- typed application message/effect profile and deterministic session replay
- hot reload with transactional scene/resource replacement
- component Stories, inspector, layout/paint diagnostics, and accessibility audit
- animation timelines resolved before scene submission
- image, path, gradient, filter, shadow, and custom canvas primitives
- offscreen targets and custom GPU surfaces for visualization and future games
- Graphics cameras, sprites, meshes, materials, frame graph, particles, and
  compute work remain in the Graphics roadmap and never pull game-world state
  into GUI

## Continuous Integration Matrix

| Lane | Required evidence |
| --- | --- |
| Semantic-only | no-default-feature check, protocol/IR snapshots, reducers, interaction, focus, selection, i18n, and accessibility conformance |
| Graphics software | scene validation, fingerprints, full/incremental parity, reference images, serialization, and fuzz/property checks |
| Graphics GPU | shader validation, headless rendering, readback comparison, cache/resource recovery, and adapter report |
| Linux host | Vulkan window, Wayland/X11 input and IME, accessibility bridge, packaging, and controlled screenshot artifacts |
| macOS host | Metal window, input/IME/accessibility smoke, reference Stories, bundle validation, and lifecycle recovery |
| Windows host | DX12 window, real input injection, IME/UI Automation bridge, reference Stories, packaging, and lifecycle recovery |
| Dependency | pinned Graphics revision, licenses, advisories, unused dependency audit, and semantic-only boundary proof |

## Performance and Reliability Budgets

Budgets are recorded only after representative fixtures have stable
measurements. Required fixtures include the calculator, component playground,
large form, overlay stack, 1,000-row collection, 100,000-row virtualized list,
mixed-script text document, and image-heavy grid.

Metrics include:

- event, reducer, semantic projection, layout, scene extraction, preparation,
  GPU submission, presentation, and accessibility timings
- visited, relaid-out, repainted, and redrawn node counts
- draw commands, batches, vertices, glyphs, uploads, evictions, and dirty area
- CPU allocations, retained memory, GPU memory, queue depth, and high-water marks
- dropped frames, missed frame deadlines, device/surface recovery, and replay
- cold build, incremental build, binary size, and package size

## Risk Controls

| Risk | Control |
| --- | --- |
| Graphics API churn | Exact commit pin in GUI, dedicated upgrade commits, schema compatibility tests |
| GPU driver variance | Software oracle, backend image thresholds, adapter metadata, tri-platform fixtures |
| Framework creep | Window hosts cannot own scene, layout, or application state |
| Game requirements distort GUI | Graphics owns general rendering only; GUI and future game runtime keep separate extraction/state layers |
| Legacy code survives indefinitely | Named removal gate and last-consumer deletion for every compatibility area |
| Premature deletion breaks dogfood | Preserve legacy path until equivalent real-host evidence passes |
| Silent style loss | Required/deferred/unsupported inventory plus fixture-failing diagnostics |
| Text divergence | Embedded fonts, deterministic shaping records, fixed fallback, baseline and line-break gates |
| Accessibility regression | Parallel semantic truth, conformance tests, and real OS bridge smoke |
| Large-module growth | Split-on-touch rule, one concern per module, file-size CI audit |

## Definition of Done

A component or subsystem is complete only when:

- it uses the shared Native IR, layout, scene, Graphics, and platform-host path
- behavior and state transitions are deterministic and tested
- supported style and accessibility fields are projected explicitly
- software reference and required GPU/OS evidence pass
- failure, cancellation, resize, recovery, and teardown are covered
- diagnostics redact sensitive values and explain unsupported capabilities
- documentation and examples match the implemented path
- superseded code, exports, dependencies, tests, fixtures, and docs are deleted
- formatting, clippy, default tests, semantic-only checks, and relevant native
  lanes pass

## Explicit Non-Goals

- No browser, DOM, CSSOM, or WebView application-content renderer.
- No framework-owned content renderer or second renderer state model.
- No platform widget tree for application content after cutover.
- No calculator-specific scene that bypasses shared Native IR.
- No second public reactive store in the renderer.
- No silent fallback from GPU or required styles to a different visual result.
- No game world, ECS, physics, audio, or scripting inside `a3s-graphics`.
- No deletion justified only by a name such as “legacy”; removal requires
  replacement evidence or proof that no consumer remains.

## Immediate Commit Sequence

1. Finish the H1 Graphics raw-surface presenter edge now that portable
   event/reducer routing is executable.
2. Present the generic rectangle slice through the H2 Windows Win32 host.
3. Present the same slice through the H3 macOS system-shell host.
4. Present it through H4 Wayland, then the separately gated X11 fallback.
5. Add text shaping/rasterization, hit testing, input, IME, and accessibility
   against the shared host contract.
6. Pass the shared calculator cutover matrix on all three platforms.
7. Delete WinUI/XAML, GTK4, and AppKit content-control code and all final
   consumers in reviewable, platform-scoped commits while preserving the thin
   OS shells.
8. Execute M6, M7, and M8 against the versioned React Aria matrix until all 51
   families and their public semantic parts have self-drawn software and real
   macOS/Windows/Linux conformance evidence.
