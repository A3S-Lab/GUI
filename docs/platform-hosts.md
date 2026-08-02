# Self-Drawn Platform Host Architecture

## Decision

A3S GUI uses one self-drawn content engine on every operating system.

A platform host may own a top-level window, surface-lifetime target, event
loop, raw input translation, text/IME bridge, accessibility provider,
clipboard, and explicit system services. Graphics owns the GPU surface and
presentation. Neither layer may create application-content controls, delegate
content layout, or interpret A3S styles in the host.

The old AppKit, GTK4, and WinUI content hosts are deleted. No current Cargo
feature links those toolkits.

## Current status

| Track | Status | Evidence |
| --- | --- | --- |
| H0 contract | Implemented | Versioned transactions/events, validation limits, recording host, dependency firewall |
| H1 shared runtime | Implemented | Host-first target staging, typed presenter completion, rollback ordering, input/hit/accessibility routing |
| H2 Windows host | In progress | Real Win32 lifecycle, owned surface leases, Graphics/DX12 presentation, visible TSX process, DPI/size/focus/close, mouse/keyboard/wheel plus WM_POINTER touch/pen input, target CI |
| H3 macOS host | Planned | No concrete macOS window/Metal host in the repository |
| H4 Linux host | Planned | No concrete Wayland/X11/Vulkan host in the repository |
| H5 product cutover | Planned | Requires all three hosts, packaging, and conformance evidence |

`host-windows` exposes a real target-gated `WindowsPlatformHost`. It stages a
new raw HWND while hidden, leases its lifetime to the Graphics-owned surface,
commits visibility, and then publishes the prepared frame through DX12. The
same bounded pump translates DPI-correct legacy mouse, keyboard, wheel, and
concurrent `WM_POINTER` touch/pen input, including pressure, modifier, capture,
and focus-loss state. Target-native tests prove the normalized state machine,
compile the raw ABI path, and inject complete touch and synthetic-pen sequences
through a real visible HWND. The pen gate verifies pressure, motion, stable
identity, and barrel-button state without enabling Win32 content-control
bindings. The native TSX process lane also verifies a process-owned visible
HWND, GPU commit, normalized mouse action, semantic window-close action, and
framed shutdown. Hardware-device pen capture, reviewed screenshots, and full
Windows conformance remain incomplete.
`host-macos` and `host-linux` remain capability markers.

## Target pipeline

```text
NativeElement + portable style + interaction state
                         |
                         v
                  LayoutSnapshot
                 /      |       \
                v       v        v
          hit regions  a11y   Graphics Scene
                |       |        |
                |       |        v
                |       |   prepared GPU frame
                |       |        ^
                |       |        |
                +-------+--- staged target --- PlatformHost
                        |              |          window lifecycle
                        |              v
                        +------ host commit ------+
                                       |
                                       v
                              Graphics presentation
```

Every platform consumes the same layout and scene. A host-specific difference
may exist only at the OS service or presentation boundary.

## Contract ownership

### Window and presentation

The host owns:

- top-level window creation, visibility, title, size, minimum/maximum size;
- logical/physical scale reporting and resize events;
- owned native target lifetime and destruction ordering;
- presentation transaction scheduling and host acknowledgements;
- close requests and lifecycle events.

Graphics owns GPU-surface creation, swapchain acquisition, rendering,
presentation, and recoverable surface status. The shared runtime owns the frame
revision, damage list, prepared scene, commit decision, and retry policy.

### Input and focus

The host translates operating-system input into:

- pointer identifiers, phases, buttons, coordinates, pressure, and modifiers;
- key state, logical key, physical code, text, repeat, and modifiers;
- wheel deltas and units;
- window focus changes.

The shared runtime owns hit testing, pointer capture, press/long-press/move
lifecycles, keyboard activation, focus scopes, collection navigation,
drag/drop policy, and action routing.

### Text and IME

The host owns the system text-service connection. It receives a versioned text
input state and returns composition, commit, selection, and deletion events.

The semantic runtime owns the canonical value, selection, validation,
formatting, undo policy, and rerender. A host must not insert an invisible
platform text field as the application editor.

### Accessibility

The runtime supplies a complete semantic accessibility snapshot. The host
publishes it through the operating system's accessibility provider API and
returns typed actions. Geometry is derived from the same layout snapshot used
for painting and hit testing.

### System services

Clipboard, file pickers, notifications, permissions, menus, and similar
services use explicit request/outcome messages. They are never hidden behind a
content-control abstraction.

## H0 transaction contract

`PlatformHostTransaction` is revisioned, bounded, and validated before
mutation. A transaction may contain window, text-input, accessibility, system,
and presentation commands.

A host implementation must provide atomic semantics:

1. prepare and validate all commands;
2. reject without visible partial state if preparation fails;
3. commit the matching revision once;
4. return a typed acknowledgement;
5. never acknowledge an unknown or stale revision.

Events are independently bounded and versioned. Diagnostic histories are
bounded and redact sensitive values.

## H1 shared runtime

`SelfDrawnWindowRuntime<H, P>` coordinates a `PlatformHost` and
`PlatformScenePresenter`.

A successful frame stages the host, prepares Graphics work against the leased
target, commits the host, presents, and advances the matching logical snapshot.
Any pre-commit failure discards Graphics work and releases its lease before
host rollback. A post-commit drop or surface loss is typed, retains logical
state matching the host commit, and schedules replay.

The recording and reference implementations are executable specifications for
future OS hosts.

## Platform boundaries

| Platform | Allowed host responsibilities | Forbidden content responsibilities |
| --- | --- | --- |
| Windows | Win32 lifecycle/message loop, HWND, DXGI/DX12 surface, pointer/keyboard messages, TSF, UI Automation, clipboard and system dialogs | WinUI/XAML controls, platform layout, content painting |
| macOS | System application/window lifecycle, one custom drawable surface, Metal presentation, native input/text services, accessibility provider, pasteboard and system panels | AppKit content controls, stacks, fields, buttons, toolkit layout |
| Linux | Wayland + xdg-shell, X11 fallback, Vulkan surface handles, XKB/input protocols, compositor text input, AT-SPI, clipboard protocols and portals | GTK4/GDK/GSK controls, GTK layout, toolkit painting |

The macOS row names forbidden content APIs to make the boundary auditable; the
current dependency graph contains none of those bindings.

## Delivery plan

### H2 — Windows

Delivered:

- hidden and visible window lifecycle;
- per-monitor-v2 DPI-aware client sizing, constraints, resize, scale,
  occlusion, focus, redraw, and close messages;
- bounded raw Win32 message pump;
- owned HWND/HINSTANCE surface leases that block premature destruction;
- hidden first-frame staging plus atomic host reconciliation and rollback;
- Graphics-owned swapchain preparation, DX12 presentation, and typed
  completion status;
- DPI-correct legacy mouse motion/buttons, vertical/horizontal wheel, physical
  and logical keyboard values, modifier/repeat state, capture, and focus-loss
  cancellation;
- bounded `WM_POINTER` touch/pen translation with pressure, stable namespaced
  pointer identities, concurrent contacts, full-sequence consumption, and
  capture/focus-loss cancellation;
- real-HWND User32 touch and synthetic-pen injection, including pen pressure,
  movement, stable identity, and barrel-button normalization;
- real Windows lifecycle/H1/DX12 tests and unsafe/dependency firewalls.

Remaining:

- device-loss fault injection, minimize/restore recovery, and reviewed GPU
  capture evidence;
- hardware-device pen capture plus tilt, rotation, and eraser conformance;
- TSF text input and UI Automation snapshot/action bridge;
- clipboard and explicit file-picker smoke;
- deterministic reference story plus GPU screenshot evidence.

Acceptance requires no WinUI/XAML dependency or runtime component.

### H3 — macOS

Deliver:

- application/window lifecycle with one custom drawable surface;
- Metal-backed Graphics presentation;
- scale, resize, occlusion, and surface recovery;
- pointer, keyboard, wheel, focus, and close events;
- text input/IME and accessibility snapshot/action bridge;
- pasteboard and explicit system-panel smoke;
- deterministic reference story plus GPU screenshot evidence.

Acceptance requires no platform content controls or toolkit layout.

### H4 — Linux

Deliver:

- Wayland + xdg-shell primary host;
- explicitly gated X11 fallback;
- Vulkan-backed Graphics presentation;
- XKB keyboard and pointer/wheel input;
- text-input protocol integration and AT-SPI bridge;
- clipboard/portal services;
- deterministic reference story plus GPU screenshot evidence.

Acceptance requires no GTK4, GDK, or GSK dependency.

### H5 — product host gate

H5 completes only when:

- all three hosts run the same versioned story corpus;
- layout, hit regions, accessibility geometry, and event traces match the
  deterministic reference model;
- lifecycle and surface-loss recovery pass;
- dependency firewalls remain clean;
- packaging/signing gates operate on self-drawn artifacts;
- no legacy content backend, feature, example, CI job, or dependency returns.

## Verification matrix

Every host must eventually provide:

| Dimension | Required evidence |
| --- | --- |
| Build | Host feature compiles on the target with no forbidden dependency |
| Lifecycle | create/show/resize/focus/close plus recovery |
| Presentation | frame revision, damage, present acknowledgement, surface loss |
| Input | mouse/pen/touch where available, keyboard, wheel, cancellation |
| Text | composition, commit, selection, deletion, focus transitions |
| Accessibility | tree snapshot, geometry, focus, value/action, announcements |
| System | clipboard and explicitly supported dialogs/services |
| Visual | deterministic software baseline and reviewed GPU capture |
| Reliability | bounded queues/history, stale revision rejection, recovery |

Portable H0/H1 tests remain the cross-platform contract authority. The
Windows-native lane additionally proves raw lifecycle, lease rollback, legacy
mouse/keyboard/wheel translation and cancellation, target-compiled
`WM_POINTER` touch/pen normalization, real-HWND touch injection without
compatibility-mouse events, User32 synthetic-pen pressure/motion/barrel state,
and Graphics/DX12 presentation. It is not hardware-device pen, text,
accessibility-provider, system-service, or reviewed visual-conformance evidence
yet.
