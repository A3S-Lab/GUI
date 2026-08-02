# Self-Drawn Platform Host Architecture

## Decision

A3S GUI uses one self-drawn content engine on every operating system.

A platform host may own a top-level window, graphics surface, event loop, raw
input translation, text/IME bridge, accessibility provider, clipboard, and
explicit system services. It must not create application-content controls,
delegate content layout, or interpret A3S styles.

The old AppKit, GTK4, and WinUI content hosts are deleted. No current Cargo
feature links those toolkits.

## Current status

| Track | Status | Evidence |
| --- | --- | --- |
| H0 contract | Implemented | Versioned transactions/events, validation limits, recording host, dependency firewall |
| H1 shared runtime | Implemented | Atomic self-drawn frames, scene presenters, input/hit/accessibility routing, recovery tests |
| H2 Windows host | Planned | No concrete Win32/DXGI host in the repository |
| H3 macOS host | Planned | No concrete macOS window/Metal host in the repository |
| H4 Linux host | Planned | No concrete Wayland/X11/Vulkan host in the repository |
| H5 product cutover | Planned | Requires all three hosts, packaging, and conformance evidence |

Host feature names are capability markers only. Compiling `host-windows`,
`host-macos`, or `host-linux` does not currently create a visible window.

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
                |       |        |
                +-------+--------+
                        v
                 PlatformHost
          window / surface / presentation
                        |
                        v
       normalized input, IME, a11y and system events
```

Every platform consumes the same layout and scene. A host-specific difference
may exist only at the OS service or presentation boundary.

## Contract ownership

### Window and presentation

The host owns:

- top-level window creation, visibility, title, size, minimum/maximum size;
- logical/physical scale reporting and resize events;
- graphics-surface creation and loss/recovery;
- presentation scheduling and acknowledgements;
- close requests and lifecycle events.

The shared runtime owns the frame revision, damage list, prepared scene, and
commit decision.

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

A successful frame advances semantic, interaction, accessibility, scene,
presentation, and host revisions together. If preparation, presentation, or
host commit fails, the candidate frame is rejected and the active snapshot
remains unchanged. Recovery can rebuild a fresh presenter or host from the
active snapshot.

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

Deliver:

- hidden and visible window lifecycle;
- DXGI/DX12-backed Graphics presentation;
- resize, DPI, occlusion, and surface-loss recovery;
- pointer, keyboard, wheel, focus, and close events;
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

Until those lanes exist, the portable H0/H1 tests are the authoritative host
contract evidence.
