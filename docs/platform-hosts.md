# Self-Drawn Platform Host Architecture

Status: H0 complete; H1 atomic frames, presentation lifecycle, and portable
input/reducer routing implemented. The real Graphics raw-surface presenter and
desktop OS shells are not implemented yet.

Updated: 2026-08-02

## Decision

A3S GUI owns every application-content pixel. Rust RSX and the planned TSX
frontend both lower into the same `NativeElement`, layout, interaction,
accessibility, and A3S Graphics pipeline. A platform host may expose an
operating-system window and system services, but it must never create one
native control per A3S element.

This distinction is the architecture boundary:

- **self-drawn application content** includes text, buttons, fields, lists,
  menus, popovers, dialogs owned by the application, focus rings, selection,
  scrolling, and every other component visual
- **operating-system shell and services** include application lifecycle,
  top-level windows, compositor surfaces, raw input delivery, IME integration,
  accessibility transport, clipboard, file pickers, permission prompts, and
  native window chrome

An `NSWindow`, `wl_surface`, X11 window, or `HWND` is a presentation target. It
is not a component renderer. An OS file picker is an explicit system-service
request. Neither is permission to replace an A3S `Button`, `TextField`, menu,
or layout container with a toolkit widget.

## Current State and Gap

The repository currently contains three distinct layers:

1. `appkit_native/`, `gtk4_native/`, and `winui_native/` create toolkit
   controls. They provide migration evidence for behavior, input,
   accessibility, packaging, and real OS execution.
2. `layout/` and `drawing/layout_scene.rs` form the new generic self-drawn
   path. The calculator already lowers into stable layout and scene records,
   and the software and offscreen GPU renderers have executable evidence.
3. `platform_host/` is the H0 zero-widget boundary. It provides bounded window,
   presentation, raw-input, text-input, stable-id accessibility, and explicit
   system-service records; atomic revision transactions; a recording host; and
   executable dependency/source firewalls.

The shared H1 layer connects committed Native IR, layout, scene, hit-region,
interaction, and accessibility state to the new host contract. It owns
transactional scene preparation/publication; resize, scale, damage, occlusion,
redraw, delayed acknowledgement, and surface-loss replay; and stable-id raw
pointer, keyboard, focus, wheel, action, and reducer routing. Production H2-H4
hosts must next attach Graphics to real top-level surfaces and return OS
services without constructing content controls. The old control backends are
frozen until this replacement passes its gates. They are migration inputs, not
the target architecture and not a base for new TSX work.

## Target Pipeline

```text
Rust RSX                         TSX in Node / Nub
   |                                   |
   +---------------+-------------------+
                   |
                   v
       resolved, versioned UI frame
                   |
                   v
              NativeElement
          /          |          \
         v           v           v
      layout     semantics    interaction
         |       + a11y       + hit regions
         v
       A3S Graphics Scene
         |
         v
   FramePlanner + wgpu
         |
         +---------------+---------------+
         |               |               |
         v               v               v
      Metal             DX12           Vulkan
         |               |               |
         v               v               v
  macOS shell      Windows shell      Linux shell
         |               |               |
         +---------------+---------------+
                         |
                         v
       raw input / IME / accessibility / system services
                         |
                         v
            portable interaction and actions
```

The host receives render products and service state, not component style or
widget instructions. Stable element identity links layout, hit testing, and
accessibility, while each remains a separate data product.

## Final Platform Boundary

| Platform | Allowed target integration | Explicitly excluded from application content |
| --- | --- | --- |
| macOS | AppKit application lifecycle, `NSWindow`, one custom root `NSView`, `CAMetalLayer`, `NSEvent`, `NSTextInputClient`, method-based accessibility, pasteboard, and explicit system panels | `NSButton`, `NSTextField`, `NSStackView`, toolkit layout, and other AppKit content controls |
| Linux | Wayland plus `xdg-shell` as the primary window path, an X11 fallback, raw window/display handles for the Vulkan-backed Graphics surface, XKB/input protocols, compositor text-input integration, AT-SPI2, clipboard protocols, and portals | GTK4, GDK, GSK, GTK layout, and GTK content controls |
| Windows | Win32 application lifecycle, an `HWND`, the message loop, a DX12/DXGI-backed Graphics surface, pointer/keyboard messages, Text Services Framework, UI Automation providers, clipboard, and explicit system dialogs | WinUI 3, XAML, `Button`, `TextBox`, `StackPanel`, and other WinUI content controls |

AppKit remains in the macOS binary only because it is the operating-system
application and window API. Its target feature set is reduced to shell, input,
IME, accessibility, and system-service types. GTK4 and WinUI/XAML have no role
in the final target dependency graph.

System-owned UI is always explicit. File and directory pickers, permission
prompts, the macOS application menu, and native window chrome may be supplied
by the operating system. A normal A3S dialog, menu, tooltip, popover, or form
control remains self-drawn.

The platform choices follow the operating-system contracts rather than a
content toolkit:

- Apple documents a custom macOS Metal surface as an `NSView` backed by
  [`CAMetalLayer`](https://developer.apple.com/documentation/metal/creating-a-custom-metal-view)
  and custom text entry through
  [`NSTextInputClient`](https://developer.apple.com/documentation/appkit/nstextinputclient).
- Windows exposes composition through
  [Text Services Framework](https://learn.microsoft.com/en-us/windows/win32/api/_tsf/)
  and custom semantic trees through
  [UI Automation providers](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-providersoverview).
- Linux window and text integration follows the upstream
  [`xdg-shell`](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/stable/xdg-shell/xdg-shell.xml)
  and
  [`text-input-v3`](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/text-input/text-input-unstable-v3.xml)
  protocols, with accessibility projected through
  [AT-SPI2](https://docs.gtk.org/atspi2/). Using the AT-SPI protocol does not
  require using GTK as the content renderer.

## Host Contract

The new host boundary must not reuse the legacy widget-oriented `NativeHost`
contract. It needs a small set of responsibilities that can be implemented by
a fake host and by all three desktop shells.

### Window and presentation

- create, show, resize, scale, occlude, minimize, restore, and close top-level
  windows from typed `WindowSpec` values
- expose thread-affine raw window/display handles only to the Graphics surface
  attachment edge
- schedule redraws from scene damage, resize, scale, exposure, and animation
- acknowledge presented or dropped render revisions without changing model
  state
- preserve the last committed frame during recoverable surface loss

### Input and focus

- translate OS pointer, keyboard, wheel, touch, pen, focus, scale, and window
  events into the existing normalized event vocabulary
- keep hit testing, pointer capture policy, focus navigation, gestures, and
  action selection in the portable runtime
- report timestamps, device identity, modifiers, coordinates, and cancellation
  without applying toolkit default control behavior

### Text and IME

- activate or deactivate a text-input session from GUI-owned edit state
- exchange surrounding text, selection, composition ranges, committed text,
  commands, and candidate-window geometry
- keep the authoritative text buffer, undo policy, caret, selection, password
  redaction, and text layout in A3S GUI

### Accessibility

- project the portable accessibility snapshot and ordered diffs into the OS
  accessibility transport
- answer hit-test, focus, bounds, value, text-range, and action requests from
  stable semantic ids
- send accessibility actions back through the same portable action router
- never infer semantics from pixels or create hidden toolkit controls as an
  accessibility substitute

### System services

- execute typed, capability-checked clipboard, file-picker, URL, notification,
  permission, and application-menu commands
- return structured results or cancellation through the runtime transaction
  boundary
- avoid exposing native handles or arbitrary platform calls to components or
  the TSX process

## Repository Shape

```text
src/platform_host/
|- mod.rs               public H0 boundary
|- contract.rs          revision transaction, presentation, events, host trait
|- window.rs            top-level window records only
|- input.rs             un-targeted pointer, key, and wheel records
|- text_input.rs        bounded IME/edit-session state and events
|- accessibility.rs     stable PlatformElementId tree, bounds, and actions
|- system.rs            typed clipboard, picker, URL, permission, menu services
|- recording.rs         bounded deterministic fake host
|- validation.rs        shared record validation
`- tests.rs             contract, recovery, redaction, and wire-shape tests

tests/platform_host_firewall.rs

src/platform_runtime/
|- frame.rs             committed Native IR/layout/scene/a11y snapshot
|- runtime.rs           atomic candidate preparation and host commit
|- events.rs            lifecycle plus ordered input/reducer dispatch
|- interaction_tree.rs  stable path index, hit testing, and focus order
|- interaction.rs       portable state, actions, changes, and event context
|- input.rs             pointer capture, hover, press, and cancellation
|- keyboard_input.rs    keyboard activation, Tab focus, and wheel routing
|- long_press_input.rs  event-loop deadlines and terminal hold recognition
|- move_input.rs        captured incremental pointer and keyboard movement
|- drag_drop.rs         typed transfer data and source/target negotiation
|- drag_drop_activation_input.rs
|                       800ms target hold deadline and activation routing
|- drag_drop_input.rs   captured pointer drag lifecycle and target routing
|- drag_drop_keyboard_input.rs
|                       Enter/Tab/Escape accessible drag lifecycle
|- long_press_tests.rs  scheduled hold ordering and recovery gates
|- move_tests.rs        move capture, identity, rollback, and reconcile gates
|- drag_drop_tests.rs   pointer negotiation, rollback, and reconcile gates
|- drag_drop_keyboard_tests.rs
|                       keyboard and style-only drag/drop gates
|- drag_drop_items_tests.rs
|                       multi-item/per-format transfer and filtering gates
|- drag_drop_activation_tests.rs
|                       pointer/keyboard/item timing and rollback gates
|- presenter.rs         raw-surface prepare/publish contract and recorder
|- reference_presenter.rs
|                       transactional software Graphics evidence
|- accessibility.rs     stable layout-path semantic projection
`- tests.rs             atomicity, input, reducer, recovery, and identity gates

tests/platform_runtime_firewall.rs
examples/self_drawn_calculator.rs

Planned next:
src/platform_host/macos/
src/platform_host/windows/
src/platform_host/linux/

src/bin/a3s_gui_host.rs
```

The `platform-host`, `host-macos`, `host-windows`, `host-linux-wayland`,
`host-linux-x11`, and `host-linux` features landed in H0. H1 adds the separate
`platform-runtime = platform-host + graphics` feature so the H0 wire boundary
stays Graphics-free. The target features currently select the common contract
only; OS dependencies arrive with their H2-H4 implementation. Neither H0 nor
H1 enables or imports `appkit-native`, `gtk4-native`, `winui-native`, or the
legacy widget-planning modules.

The new modules are created beside the legacy directories rather than by
renaming a control backend. This makes accidental content-widget reuse visible
and lets each old backend be deleted after its replacement evidence exists.

### Graphics surface capability gate

The pinned Graphics commit `8748fab` currently renders to a
surface-independent texture and supports deterministic readback; it does not
yet expose its planned safe window-surface attachment API. GUI therefore does
not import `wgpu`, create a second device/queue owner, or place raw handles in
`PlatformHost` records. H1's remaining presenter can land only after Graphics
provides host-owned attachment, configure/acquire/present, resize/suspend, and
surface/device recovery while retaining Graphics resource identity. The H1
firewall pins that dependency and rejects a direct GUI `wgpu` dependency.

## H0 Transaction and Thread Contract

`PlatformHost` is intentionally thread-affine. The trait does not require
`Send` or `Sync`, so an H2-H4 implementation can stay on its owning OS event
loop. Every public record crossing the boundary is `Send + Sync` and contains
no native or GPU handle.

The runtime prepares one complete `PlatformHostTransaction` at a monotonically
increasing `PlatformHostRevision`. Validation runs before host mutation and
enforces bounded command counts, finite geometry, stable accessibility ids,
UTF-8 text ranges, sensitive-value rules, and unique per-revision presentation,
accessibility, and system-request identities.

Commit atomically applies the pending revision. A failed commit leaves that
revision pending so the owner must explicitly retry or roll it back; rollback
does not advance the last committed revision. Shutdown rejects a pending
transaction, releases queued events, and makes later operations fail. The
recording host keeps bounded diagnostic history with text-input and sensitive
clipboard payloads redacted while preserving byte lengths and command shape.

## Delivery Plan

The host track is named H0-H5. It is dependency-coupled to renderer milestones
M3-M5 and to the visible-window portion of the TSX track.

```text
H0 host contract --------+
                         v
M3 layout/scene -------> H1 shared host runtime
                         |\
                         | +------------+-------------+
                         v              v             v
                    H2 Windows      H3 macOS      H4 Linux
                         ^              ^             ^
                         +--------------+-------------+
                                        |
                              M4 text/input/a11y

H2 + H3 + H4 + M4 ----------> H5 / M5 default cutover
                                        |
                                        v
                             legacy widget deletion

T1-T2 headless TSX ----------> T3 first supported host
H2 + H3 + H4 ----------------> T4 tri-platform packages
```

H2, H3, and H4 share H1 and may progress independently. Their rectangle-only
presentation slices can land during M3; their complete text, IME, and
accessibility gates require M4.

### H0 - Contract and dependency firewall

Status: complete.

Deliverables:

- add window, presentation, input, text-input, accessibility, and
  system-service records under `platform_host/`
- add a fake host that records complete render and service transactions
- define target features without enabling a legacy renderer
- add source and dependency audits for forbidden toolkit content paths
- document thread ownership, revision ordering, error recovery, and teardown

Gates:

- the contract contains no widget create/update/remove operation
- platform-host records contain no `NativeElement`, portable style, Node.js
  runtime value, toolkit object, or `wgpu` handle
- semantic-only builds remain free of Graphics and platform dependencies
- every host-facing record is bounded, deterministic, and `Send + Sync` where
  thread affinity does not require an owning executor

Evidence:

- all six H0/target marker features compile without a legacy renderer
- the common `platform-host` dependency graph contains no Graphics or OS
  toolkit package
- 13 unit tests cover validation, stable ids, wire shape, atomic revisions,
  commit failure, rollback, teardown, bounded storage, and redaction
- three integration tests recursively audit source imports, Cargo feature
  edges, the public feature gate, and absence of widget CRUD
- `just verify` runs the H0 compile, dependency-graph, contract, and firewall
  gates on every normal CI change

### H1 - Shared self-drawn window runtime

Status: in progress; atomic frame, presentation lifecycle, and portable
input/reducer slices landed. The raw-surface presenter remains.

Deliverables:

- connect committed Native IR, layout, scene, hit regions, and accessibility
  snapshots to one host-frame transaction
- attach Graphics to a raw surface and drive resize, scale, damage, occlusion,
  redraw, surface-loss recovery, and presentation acknowledgements
- route normalized host events through the existing interaction and reducer
  pipeline
- add the shared `self_drawn_calculator` entrypoint and fake-host smoke runner

Landed evidence:

- `SelfDrawnWindowRuntime` builds one immutable Native IR, layout, hit-region,
  scene, and stable-id accessibility snapshot before host mutation
- `PlatformScenePresenter` prepares candidate pixels, publishes only after a
  matching host commit, and discards rejected candidates
- identical frames skip layout, scene, host, and presenter work; semantic-only
  changes commit accessibility without presenting identical pixels
- resize, fractional scale, damage, occlusion, redraw, delayed acknowledgements,
  dropped frames, and surface loss replay the retained scene deterministically
- raw pointer, keyboard, Tab-focus, hover, press, cancellation, and wheel input
  route through committed hit regions and stable `PlatformElementId` paths;
  ordered action batches retain frame revision, event sequence, bubbling
  target, input context, and static action payload
- callback-driven and style-only long press exposes the next monotonic host
  deadline, cancels and restarts across pointer boundaries, falls back to
  release-time recognition, and routes the terminal action through the same
  rollback-aware reducer path
- callback-driven and style-only move starts on the first non-zero pointer
  delta, remains captured outside its hit region, retains incremental delta and
  pointer identity, and ends on release, cancellation, or terminal long press;
  arrow keys route a handled one-unit lifecycle through the same action batch
- callback-driven and style-only drag/drop retains multiple text items and all
  per-item MIME/custom representations, filters target items by exact or
  wildcard types, and negotiates copy/move/link/cancel operations; pointer
  drags report target-local coordinates, keyboard Enter/Tab/Escape provides the
  same accessible source/compatible-target lifecycle, and keyed frames plus
  reducer errors preserve or roll back the entire session atomically. Collection
  targets additionally distinguish external insertion/root drops from internal
  move and same-parent reorder, reject self/descendant drops, and coalesce
  adjacent insertion boundaries. Ordinary targets and collection items emit
  `DropActivate` after an exact 800ms hold through the same host deadline;
  collection roots do not activate, and target exit/change/cancel/drop clears
  or restarts the timer
- reducer errors restore the staged interaction state and sequence before the
  event is exposed as successful; successful frame reconciliation preserves
  focused stable ids, while rejected frames do not touch them
- 63 focused runtime/software tests and four recursive feature/source
  firewall tests pass without any legacy renderer or OS toolkit dependency
- `self_drawn_calculator` reproduces layout fingerprint
  `16529597026056060935`, scene fingerprint `2100550662756266801`, and
  deterministic RGBA pixels, then routes eight fake-host events through four
  reducers and reaches display value `10`

Remaining before H1 is complete:

- implement the Graphics raw-surface presenter used by H2-H4; the landed
  presenter contract and software implementation deliberately expose no raw
  handle to components or common host records, and pinned Graphics commit
  `8748fab` must first supply its planned safe surface attachment API

Gates:

- an unchanged frame creates no new layout, scene, or present work
- rejected frames keep the last committed scene, interaction, and
  accessibility revisions
- resize and scale changes rebuild deterministically without changing semantic
  identity
- the fake host reports zero application-content widgets by construction

### H2 - Windows thin host

Status: planned after H1. It is the first platform slice because local DX12
reference evidence already exists.

Deliverables:

- Win32 `HWND` lifecycle and message loop
- Graphics surface attachment and presentation through the DX12 backend
- pointer, keyboard, wheel, focus, scale, clipboard, and window events
- Text Services Framework bridge and candidate geometry
- UI Automation fragment provider over the portable accessibility tree
- packaging and automation that do not initialize XAML

Gates:

- the shared calculator presents and interacts in an `HWND` with zero WinUI or
  XAML content objects
- target dependency inspection contains no `winio-winui3`, XAML, or
  `windows-collections` dependency
- keyboard, pointer, composition, focus, UI Automation, resize, DPI, suspend,
  surface-loss, and close scenarios pass

### H3 - macOS thin host

Status: planned after H1.

Deliverables:

- `NSApplication`, `NSWindow`, and one custom root `NSView`
- `CAMetalLayer` surface attachment and Metal presentation
- `NSEvent` translation, responder focus, clipboard, window, and scale events
- `NSTextInputClient` implementation over GUI-owned text state
- method-based accessibility objects over the portable semantic tree
- bundle, signing, application-menu, and automation support

Gates:

- the shared calculator presents and interacts without `NSButton`,
  `NSTextField`, `NSStackView`, or other application-content controls
- the AppKit dependency feature allowlist contains only shell, event, IME,
  accessibility, pasteboard, menu, panel, and root-view APIs
- text composition, VoiceOver-facing semantics, resize, backing-scale,
  occlusion, surface-loss, and close scenarios pass

### H4 - Linux thin host

Status: planned after H1.

Deliverables:

- Wayland connection, registry, seat, `xdg-shell` toplevel, scale, configure,
  frame callbacks, clipboard, and surface lifecycle
- X11 fallback behind a separate feature and the same host contract
- Graphics surface attachment and Vulkan presentation
- keyboard/pointer translation and compositor text-input integration, with
  capability diagnostics where a protocol is unavailable
- AT-SPI2 projection and portal-backed system-service requests
- desktop entry, packaging, and compositor test coverage

Gates:

- the shared calculator presents and interacts on Wayland and the supported X11
  fallback with no GTK4, GDK, or GSK dependency
- text input is tested on the declared compositor/IME matrix; unsupported
  protocol combinations fail capability checks instead of dropping composition
- AT-SPI, scale, configure, clipboard, portal cancellation, surface-loss, and
  compositor disconnect scenarios pass

### H5 - Default cutover and deletion

Status: planned after H2-H4 and M4; this is the platform side of M5.

Deliverables:

- make the shared self-drawn entrypoints and target host features the defaults
  for supported desktop packages
- pass the same calculator and control scenarios across all three hosts
- delete legacy widget creation, styling, layout, update, and event adapters in
  platform-scoped commits
- delete legacy examples, features, dependencies, CI jobs, packaging assets,
  exports, command shapes, fixtures, and documentation with their last consumer
- trim macOS AppKit bindings to the audited system-shell allowlist

Gates:

- source and runtime audits find zero platform content-widget creation
- target Linux artifacts contain no GTK4/GDK/GSK libraries
- target Windows artifacts contain no WinUI/XAML runtime dependency
- target macOS builds contain no linked or enabled AppKit content-control path
- semantic, layout, scene, software/GPU image, input, IME, accessibility,
  recovery, packaging, and teardown matrices pass on all supported hosts
- `Cargo.lock`, `cargo tree -e features`, distribution imports, examples, and
  documentation contain no orphaned legacy dependency or claim

## Migration Rules

| Legacy responsibility | Target treatment |
| --- | --- |
| Native control creation and hierarchy | Delete; A3S layout and Graphics replace it |
| Native control styling and property setters | Delete; portable style and scene extraction replace it |
| Window creation and event loop | Reimplement behind the new host contract without importing legacy content modules |
| Input translation | Preserve tested behavior, move OS translation to the host, and keep policy in the portable interaction runtime |
| Text input and IME | Implement against GUI-owned edit state; do not hide a native text field |
| Accessibility | Project the virtual portable tree directly; do not use invisible native controls |
| Menus, dialogs, and clipboard | Separate A3S-drawn components from explicit OS system-service requests |
| Platform examples | Replace with one shared self-drawn app plus minimal per-host smoke launchers |
| Conformance fixtures | Keep or strengthen behavioral evidence before deleting the old producer |

Code may copy a narrowly scoped OS-service technique only after it is detached
from widget identity, toolkit layout, and control callbacks. Target modules must
not import a legacy backend as a convenience layer.

## Verification Matrix

Every host commit must report evidence in these categories:

| Category | Required evidence |
| --- | --- |
| Dependency firewall | Feature graph, target dependency tree, forbidden-symbol/source audit, and package import audit |
| Presentation | First frame, unchanged frame, damage, resize, scale, occlusion, minimize/restore, and surface-loss recovery |
| Determinism | Native IR, layout, scene, model, and action fingerprints against the shared fixtures |
| Rendering | Software byte identity plus reviewed Metal, DX12, and Vulkan thresholds |
| Input | Mouse, touch where available, pen where available, keyboard, wheel, focus, capture, cancellation, and close |
| Text and IME | CJK, dead keys, emoji, RTL, selection replacement, candidate geometry, commit, and cancellation |
| Accessibility | Tree, names, roles, values, states, bounds, focus, actions, live regions, and text ranges |
| Recovery | Rejected frame, device/surface loss, host shutdown, app shutdown, and last-good-frame replay |
| Packaging | Clean install, signing/metadata, launch, dependency scan, and uninstallation residue |

Image parity alone cannot satisfy a host gate. A host also fails if it draws the
right pixels while losing interaction, text input, accessibility, transaction,
or dependency-boundary evidence.

## First Reviewable Commit Sequence

1. Finish the H1 Graphics raw-surface presenter edge using the landed portable
   hit/focus/reducer routing.
2. Present the rectangle-only shared calculator through the Windows host.
3. Present the same rectangle-only calculator through the macOS host.
4. Present it through Wayland, then add the separately gated X11 fallback.
5. Land text shaping, editing, input/IME, and accessibility slices against the
   shared host contract, one subsystem at a time.
6. Pass the full calculator cutover matrix on all three platforms.
7. Delete WinUI/XAML, GTK4, and AppKit content-control code in independent
   platform commits while preserving the new shells.
8. Switch defaults, examples, packaging, and documentation, then run the final
   repository-wide dead-code and dependency audit.

Each commit must leave the tree buildable for its declared feature set. No
milestone-sized integration branch is required, and no target host waits for
TSX: Rust RSX remains the first executable consumer of the shared path.
