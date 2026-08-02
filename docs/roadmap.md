# A3S GUI Roadmap

_Last updated: 2026-08-03_

## Product direction

A3S GUI is a fully self-drawn cross-platform GUI runtime.

Rust RSX and TypeScript TSX produce one semantic frame model. A3S GUI owns
behavior, layout, hit testing, accessibility, and application-content pixels.
A3S Graphics owns retained scenes, software reference output, GPU surfaces,
rendering, and presentation. Thin OS hosts own only native windows, owned
surface-lifetime targets, commit coordination, normalized input, text/IME,
accessibility providers, clipboard, and explicit system services.

Platform content controls and toolkit layout engines are not an intermediate
or fallback renderer.

## Current status

| Track | Status | Current evidence |
| --- | --- | --- |
| M0 Graphics boundary | Complete | Feature-gated Graphics edge, deterministic reference presenter, stable draw IDs |
| M1 architecture cleanup | Complete | Layer boundaries and graph gates; content-toolkit code/dependencies deleted |
| M2 GPU foundation | In progress | Reviewed calculator GPU slice plus real Graphics-owned DX12 window presentation; Metal/Vulkan and visual parity evidence missing |
| M3 generic layout/scene | In progress | Generic calculator layout/scene/software fixture and all-style ownership matrix |
| M4 text/input/a11y visuals | In progress | Bounded shaper/layout/scene contracts landed; concrete font, glyph, editing, IME, and AT backends missing |
| M5 cutover/deletion | Partial | Legacy deletion complete; real OS-host cutover cannot occur until H2-H4 |
| H0 host contract | Complete | Zero-widget transaction/event/API contract and dependency firewall |
| H1 shared runtime | Complete | Host-first staging, owned presentation targets, typed completion, rollback ordering, input/hit/a11y routing |
| H2 Windows host | In progress | Real HWND lifecycle, owned surface leases, Graphics/DX12 presentation, PMv2 DPI, mouse/keyboard/wheel and WM_POINTER touch/pen input, atomic transactions, Windows CI |
| H3-H4 real hosts | Planned | macOS and Wayland/X11 implementations absent |
| T0 TSX contract | Complete | Process ownership, wire protocol, strict DTOs, generated declarations |
| T1 TSX headless slice | Complete | Automatic JSX, canonical counter, revision-scoped ordered callbacks |
| T2 TSX stateful runtime | Complete | Stateful `createApp`, public identity contract, bounded recovery/replay, and restarted-process keyboard/stale-event gates |
| T3-T5 TSX native product | Planned | Visible OS hosts, watch/reload, platform artifacts, and publication remain |
| M6-M8 React Aria | Planned | 51-family versioned matrix; Button scene smoke only |

No family is yet self-drawn conformant across real macOS, Windows, and Linux
hosts.

## Non-negotiable architecture

1. A3S draws every application-content pixel.
2. Semantic, accessibility, layout, hit, and paint identities share stable
   keys but remain separate products.
3. OS hosts never interpret component styles or create content controls.
4. Graphics never depends on RSX, TSX, semantic roles, accessibility, or
   windows.
5. Semantic-only builds never acquire Graphics, wgpu, SWC, Node, or a platform
   content toolkit.
6. Pre-commit failures preserve the previous complete frame; post-commit
   presentation loss is typed and schedules replay against the matching host
   and logical revision.
7. TypeScript callbacks execute only for a validated committed revision.
8. Cross-platform consistency is proven with a deterministic reference
   environment and a shared story corpus.
9. React Aria status is promoted only through versioned executable evidence.
10. Packaging cannot return before real self-drawn hosts exist.

## Completed legacy removal

The cleanup gate has been executed without a compatibility migration layer:

- deleted AppKit, GTK4, and WinUI planning/content-host modules;
- deleted all associated Cargo features and target dependencies;
- deleted toolkit-specific input conformance and smoke executables;
- deleted platform content-control examples and support code;
- deleted old bundle manifests, scripts, validators, and unsigned artifacts;
- deleted platform toolkit and bundle CI matrices;
- collapsed generic planner tests to one nonvisual `HeadlessAdapter`;
- collapsed backend capability reporting to portable self-drawn semantics;
- regenerated Cargo and TypeScript protocol graphs;
- retained negative dependency-firewall assertions.

A future OS host is a new implementation of `PlatformHost`; it may not reuse
or restore deleted content backends.

## P0 renderer program

### M0 — Graphics boundary and deterministic core

Status: complete.

Delivered:

- optional `a3s-graphics` integration;
- independent `graphics`, `software-reference`, and `gpu` features;
- stable semantic/layout-to-scene boundary;
- deterministic reference rendering and retained damage;
- validation and redaction at the GUI/Graphics edge;
- graph tests keeping semantic-only builds renderer-free.

### M1 — architecture cleanup

Status: complete.

Delivered:

- authoring, semantic runtime, layout/scene, Graphics, host, and TSX process
  ownership boundaries;
- semantic behavior independent of platform controls;
- zero-widget host/runtime firewalls;
- removal of content-toolkit code, features, dependencies, examples, packaging,
  and CI;
- portable headless planner retained only for transaction/protocol tests.

### M2 — GPU foundation

Status: in progress.

Delivered:

- retained Graphics scene preparation;
- wgpu path;
- deterministic non-text calculator scene;
- local reviewed DX12 readback evidence;
- safe Graphics-owned native surfaces with prepared-frame commit tokens;
- real Win32/DX12 submission and presentation completion evidence.

Remaining:

- real Metal and Vulkan host surfaces and presentation;
- Metal/DX12/Vulkan parity stories;
- device-loss fault injection plus minimize/restore and capture evidence;
- production text and clipping coverage.

### M3 — generic layout and scene vertical slice

Status: in progress.

Delivered:

- schema-versioned deterministic layout records;
- 1/64-point quantization and stable paths;
- separate hit regions;
- all `NativeRole`, style-field, and event-kind ownership assignments;
- generic calculator frame through semantic compilation, layout, scene,
  software output, and GPU boundary;
- retained-damage checks.

Remaining:

- production font/shaping implementation and paragraph algorithms behind the
  landed measurement contract;
- complete scroll/clipping/transform behavior;
- reusable visual primitives for M6 components;
- larger story corpus and performance budgets.

### M4 — text, editing, IME, accessibility, overlays

Status: in progress; the measurement/shaping and scene-encoding contracts are
complete, while production backends and the remaining M4 capabilities are not.

Delivered:

- explicit `LayoutOptions` modes with no character-width fallback;
- bounded, handle-free `TextShaper` requests and owned glyph/run/line output;
- one quantized shape record shared by intrinsic layout and scene extraction;
- UTF-8 cluster, bidi direction, finite geometry, resource-count, and font-face
  identity validation;
- source-free retained text records and pre-shaping password masking;
- a stateful `TextSceneEncoder` edge with stable draw slots, clipping, opacity,
  primitive limits, and shaped ink-bound enforcement;
- layout-diff damage that includes visible text ink outside explicit boxes.

Remaining:

- production font discovery, shaping, fallback, bidi, line breaking, selection,
  caret, and text decoration;
- editable text model, clipboard editing, undo/redo, password handling;
- platform text/IME sessions over H0;
- accessibility geometry/action bridges over H0;
- overlay compositing, clipping, shadows, placement, and focus restoration;
- deterministic text and accessibility story evidence.

M4 must be generic. Per-platform text widgets are forbidden.

### M5 — default self-drawn product cutover

Status: partial.

Already complete:

- legacy content backend deletion;
- dependency and CI cleanup;
- documentation and feature graph cutover to self-drawn-only architecture.

Still required:

- H2 Windows completion plus H3-H4 real hosts;
- default visible example uses `SelfDrawnWindowRuntime`;
- all required system-service bridges;
- packaging/signing for self-drawn artifacts;
- removal or renaming of any remaining test-only historic “widget” vocabulary
  that causes architectural ambiguity;
- release evidence on all targets.

## P0-H platform host track

### H0 — host contract and firewall

Status: complete.

Delivered:

- versioned window, presentation, input, text/IME, accessibility, and system
  service contracts;
- bounded atomic transactions and acknowledgements;
- event limits, diagnostics, redaction, and recording host;
- dependency-free target capability markers;
- graph and source firewalls.

### H1 — shared self-drawn runtime

Status: complete at the portable contract level.

Delivered:

- `SelfDrawnWindowRuntime`;
- generic scene presenter abstraction plus recording/reference/GPU presenters;
- host-first target staging and prepare/commit/publish ordering;
- owned-target rollback ordering that discards GPU work before native cleanup;
- queued host acknowledgements plus `Presented`, `Dropped`, and `SurfaceLost`
  completion outcomes;
- hit testing, pointer capture, keyboard routing, press/long-press/move;
- focus, collection navigation, drag/drop policy;
- accessibility snapshot/action routing;
- bounded stats and failure-path tests.

### H2 — Windows host

Status: in progress.

Delivered:

- target-gated, thread-affine `WindowsPlatformHost` backed directly by Win32;
- top-level `HWND` class/lifecycle with hidden or visible committed state;
- per-monitor-v2 DPI context, logical client sizing, constraints, resize,
  scale, occlusion, focus, redraw, and explicit close handling;
- bounded `PeekMessageW` pump and owned `raw-window-handle` surface leases that
  reject HWND destruction while Graphics retains a target;
- pure planning plus hidden-HWND staging, stale-revision rejection, atomic
  native reconciliation, rollback, and queued host acknowledgements;
- Graphics-owned surface-compatible adapter selection, swapchain resize,
  prepared-frame discard, presentation, and surface recreation;
- real Windows lifecycle, lease rollback, H1 first-frame, and DX12 presentation
  tests in target CI;
- DPI-correct legacy mouse buttons/motion, vertical and horizontal wheel,
  physical/logical keyboard translation, repeat/modifier state, mouse capture,
  and focus-loss cancellation in the bounded message pump;
- bounded `WM_POINTER` touch/pen translation with DPI-correct coordinates,
  normalized pressure/buttons, stable namespaced identities, concurrent
  contacts, compatibility-mouse suppression, and capture/focus cancellation;
- source/dependency firewalls confining unsafe Win32 ABI calls and rejecting
  content toolkits.

Remaining:

- device-loss fault injection, minimize/restore recovery, and reviewed GPU
  capture evidence;
- hardware-injected touch/pen message-path and extended input conformance;
- TSF, UI Automation, clipboard, and explicit system services;
- visible RSX/TSX story plus deterministic/GPU capture evidence.

No WinUI/XAML dependency or application-content control is allowed.

### H3 — macOS host

Status: planned.

Deliver system application/window lifecycle, one custom drawable surface,
Metal presentation, scale/resize/input/focus/close, text input/IME,
accessibility provider, pasteboard, and explicit system panels. No platform
content control or toolkit layout is allowed.

### H4 — Linux host

Status: planned.

Deliver Wayland + xdg-shell, gated X11 fallback, Vulkan presentation, XKB and
pointer/wheel input, compositor text input, AT-SPI, clipboard protocols, and
portals. No GTK4/GDK/GSK dependency is allowed.

### H5 — tri-platform host gate

Status: planned.

Complete when the same versioned stories pass lifecycle, input, text,
accessibility, system-service, deterministic layout/pixel, GPU capture,
recovery, and dependency gates on all three targets.

## P0-T TypeScript track

### T0 — architecture and cross-language contract

Status: complete.

Delivered:

- supervised-process ownership model;
- strict hello/welcome/render/commit/event DTOs;
- length-prefixed codecs and payload limits;
- JavaScript-safe integer and fingerprint rules;
- Rust-generated TypeScript declarations;
- canonical Rust/Node fixtures.

### T1 — automatic JSX and callback scope

Status: complete.

Delivered:

- standard automatic `jsx`, `jsxs`, and fragment entry points;
- synchronous function-component expansion;
- strict element/key/children/prop/style/window normalization;
- callback extraction into revision-scoped action IDs;
- candidate/active/rollback callback scopes;
- atomic commit/reject behavior;
- stale revision rejection;
- ordered awaited multi-callback dispatch;
- real TSX counter parity with Rust RSX through Native IR and accessibility.

### T2 — stateful TypeScript runtime

Status: complete.

Delivered:

- transport-neutral `createApp` start/render/dispatch/rerender/shutdown
  lifecycle over a typed host object;
- stable keyed function-component identities and deterministic hook slots;
- `useState`, `useReducer`, `useMemo`, `useRef`, and post-commit `useEffect`;
- whole-event batching with at most one immediate follow-up render;
- transactional candidate hook trees tied to candidate/active callback scopes;
- rejected-frame preservation and same-revision retry;
- deterministic effect cleanup on dependency change, committed unmount, and
  shutdown;
- typed nested context providers and `useContext`, kept outside the wire tree;
- transactional render error boundaries with candidate rollback, committed
  fallback cleanup, and last-frame preservation when fallback rendering fails;
- validated host `welcome`, complete client `render` envelopes, independent
  per-sender message sequencing, negotiated byte limits, and session identity
  before callback-registry preflight;
- dependency-free client `hello` construction plus an accessor-safe,
  incremental little-endian JSON frame codec matching the Rust boundary;
- single-reader framed connection plus explicit no-shell Node child-process
  transport with serialized writes, bounded stderr, shutdown timeout, and real
  success/crash process fixtures;
- strict `a3s-gui-tsx-host` framed process with hello/render/ping/pong/close
  sequencing, idle host probes with fixed response deadlines, and real
  `SelfDrawnWindowRuntime` software-reference commits;
- `A3sFramedApplicationHostV1` with one shared client session, single-reader
  commit/event pumping, bounded event tasks, timeout-backed client ping/pong,
  host-ping replies across pending commit/event application, sequenced close
  acknowledgement, fatal propagation, and real Node-to-Rust `createApp`
  render/bidirectional-liveness/shutdown coverage;
- bounded ordered wire receipt separated from the contiguous semantic-apply
  high-water mark, so control traffic cannot corrupt asynchronous UI ordering;
- hostless `createApp(App).run()` with strict platform-package manifest and
  executable checksum validation, automatic UUID session identity, event
  binding before the first render, startup rollback, and real Node-to-Rust
  process coverage;
- observable Host termination plus opt-in globally bounded restart attempts,
  fresh session identity enforcement, transactional committed-frame/callback
  replay, event gating until replay commit, deterministic exhaustion cleanup,
  and a real post-commit child-process crash/restart fixture;
- Rust-owned, TypeScript-generated public `a3s:c1:` component and `a3s:a1:`
  automatic-action identity shapes, canonical UTF-8 length validation at both
  process ends, strict reservation of the generated `a3s:` namespace, and
  action identity independent of erased function-component wrappers;
- a real three-generation child-process gate covering initial Host crash,
  replay, keyboard activation, rejection of an older render revision before
  callback execution, and a second fresh-session replay;
- serialized event/commit consumption when an active-revision callback overlaps
  an in-flight render acknowledgement;
- source-located hook-order failures and Node interaction tests.

### T3 — executable self-drawn window

Status: blocked on H2-H4/M4.

Deliver the first real TSX application using the same H1 runtime, with no
alternate renderer path.

### T4 — development loop and packaging

Status: planned.

Deliver watch/reload, diagnostic source mapping, host restart/recovery, target
bundles, signing metadata, and tri-platform smoke after concrete hosts exist.

### T5 — production SDK

Status: planned.

Deliver publishable packages, API stability policy, compatibility matrix,
examples, security/resource limits, release automation, and migration guidance
from browser React/TSX applications.

## P1 React Aria component projection

The authoritative scope is
[`react-aria-component-matrix.json`](react-aria-component-matrix.json).

### M6 — foundations and forms

Deliver production scene/text/input/accessibility evidence for foundational
content, buttons/toggles, fields/forms, number/search/text input, sliders,
progress/meter, toolbar, links/groups/separators, and file trigger.

Close the explicit Checkbox, Radio, and Switch Field/Button authoring gaps.

### M7 — overlays and collections

Deliver disclosure, modal/dialog, popover/tooltip/toast, menus, select,
combobox/autocomplete, list/grid/tree/table, tabs/tags, virtualization, and
drag/drop with complete overlay/scroll/collection/accessibility evidence.

Close the explicit ToastList/ToastContent gaps and upstream 1.19 contract
deltas.

### M8 — date, time, color, and advanced data

Deliver calendar/date/time segment editing and pickers, color controls, complex
geometry, locale coverage, and advanced data interaction evidence.

## Continuous verification

The portable CI gate must always cover:

- format and whitespace;
- locked dependency graph;
- semantic-only feature builds;
- H0/H1 feature builds and firewall tests;
- Rust/TypeScript protocol drift;
- Clippy and rustdoc warnings;
- all maintained Rust tests/examples;
- TypeScript type checking and Node fixtures;
- React Aria matrix validation;
- software and GPU boundaries.

The first target-native job now runs real Win32 lifecycle, legacy input and
cancellation, H1 integration, DX12 presentation, and firewall evidence.
Equivalent macOS/Linux jobs are added with their raw hosts; packaging jobs are
added only with real self-drawn artifacts.

## Reliability budgets

Before production cutover, define and enforce budgets for:

- startup and first frame;
- input-to-present latency;
- layout and scene rebuild cost;
- retained damage and GPU memory;
- text shaping caches;
- accessibility snapshot/update size;
- host/event/action queue depth;
- TSX IPC payload, backpressure, and callback latency;
- surface-loss and process-restart recovery;
- long-running diagnostic history.

All queues and histories must remain bounded.

## Definition of done

A release candidate requires:

- same semantic story corpus on Rust RSX and TypeScript TSX;
- real self-drawn macOS, Windows, and Linux hosts;
- production text/editing/IME/accessibility;
- deterministic software baselines and reviewed GPU captures;
- no content-widget toolkit dependency;
- no alternate platform renderer;
- all claimed React Aria statuses backed by matrix evidence;
- packaging/signing/installer gates;
- security, redaction, resource-bound, stale-event, and recovery tests;
- documentation that describes actual code rather than future claims.

## Immediate implementation sequence

Completed on 2026-08-02: production text measurement/shaping interfaces in
layout/scene.

Completed on 2026-08-03: the first H2 Win32 lifecycle/message-pump/raw-surface
skeleton and its target-native CI lane.

Completed on 2026-08-03: owned HWND surface leases, host-first hidden staging,
Graphics swapchain preparation, real DX12 presentation, typed completion, and
destruction-order tests.

Completed on 2026-08-03: DPI-correct Win32 legacy mouse, keyboard, and wheel
translation with capture, modifier/repeat tracking, system-key default handling,
and focus-loss cancellation.

Completed on 2026-08-03: bounded `WM_POINTER` touch/pen translation with
pressure, stable device and pointer identities, concurrent contacts,
full-sequence consumption, and capture/focus-loss cancellation. Hardware
injection, text/IME, accessibility, system-service, and reviewed visual-parity
evidence remain incomplete.

1. Connect completed T2 frame commits to a visible host executable and add
   hardware-injected touch/pen message-path evidence.
2. Add minimize/restore and device-loss fault injection, then capture the same
   deterministic story through DX12.
3. Implement production font/shaping and glyph encoder backends, then TSF and
   UI Automation bridges.
4. Add Windows clipboard/system-service smokes and port the same contract to
   H3 and H4.
5. Expand M6 deterministic stories and promote evidence through the matrix.
6. Restore packaging only after H2-H4 artifacts exist.
