# A3S GUI

**Native GUI runtime for A3S UI frames**

A3S GUI renders structured UI protocol frames without embedding a browser. It
accepts Rust-native trees or serialized JSX frames, lowers them into a portable
native UI IR, reconciles keyed updates, routes native events to stable action
ids, and targets AppKit, WinUI, GTK4, or a headless test host.

---

## Overview

The crate owns the native UI boundary for A3S applications:

- **Protocol input**: `UiFrame` records from the TypeScript JSX SDK or any
  producer that emits the same serializable shape.
- **Native IR**: typed roles, props, metadata, style tokens, event bindings, and
  accessibility hints shared by all platform backends.
- **Incremental renderer**: keyed create, update, insert-child, remove, and
  set-root commands for native hosts.
- **Platform backends**: AppKit, WinUI, GTK4 planning adapters plus feature-gated
  native surfaces for the matching operating systems.
- **State loops**: reusable protocol and embedded-runtime loops for reducer
  driven rerendering after native actions.

A3S GUI is not a WebView runtime. It does not provide a DOM, CSSOM, browser
layout engine, or JavaScript object graph at the host boundary.

## Current Status

| Area | Status |
|------|--------|
| Headless runtime | Usable for protocol tests, command inspection, and accessibility snapshots. |
| TypeScript JSX protocol SDK | Usable for emitting `UiFrame` JSON with semantic components, HTML/SVG tags, style, metadata, and event props. |
| AppKit native surface | Usable for small macOS smoke apps with windows, text and number input, buttons, toggles, sliders, selects, tabs, menus, keyboard events, `window.onClose` actions, and native `autoFocus`. |
| GTK4 native surface | Usable for small Linux smoke apps with the same core controls plus window/dialog `window.onClose` actions; requires GTK4 development libraries and `pkg-config`. |
| WinUI native surface | Usable for Windows smoke apps with core controls, HWND initial size, min/max resize bounds, resizable state, focus callbacks, keyboard message routing, `window.onClose` actions, and root-window close exit. Programmatic `autoFocus` is tracked but limited by `winio-winui3` 0.4.2 not exposing a safe focus method. |
| Production app shell | In progress. Crate-local `just` recipes cover local verification, native dogfood entrypoints, host-native release builds, and staged dogfood bundles. Overflow-aware container scroll shells are available for longer forms. Signed installers, layout polish, broader native automation, and platform-specific edge cases still need hardening. |

Known boundaries:

- Web-like input is accepted only when it can be lowered to native roles,
  control state, accessibility hints, metadata, events, or portable style
  tokens.
- Arbitrary CSS selectors, browser layout behavior, Web APIs, and treating
  `HTMLElement` objects as application state are out of scope.
- Media, resource, table, and rich document roles are represented in the native
  IR; native platform controls for every browser feature are not implied.

## Quick Start

Add the Rust crate:

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

Render a native tree into the headless host:

```rust
use a3s_gui::{
    GuiResult, GuiRuntime, HeadlessHost, NativeElement, NativeProps, NativeRole, WebProps,
};

fn main() -> GuiResult<()> {
    let root = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .web(WebProps::new().on_press("saveDocument")),
    );

    let mut runtime = GuiRuntime::new(HeadlessHost::default());
    runtime.render_native(&root)?;
    Ok(())
}
```

Generate the same protocol shape from JSX:

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {Button, createAction, createUiFrame, defineAction} from '@a3s-lab/gui';

const saveDocument = createAction('saveDocument', 'Save document');

export const frame = createUiFrame(
  'document',
  <Button onPress={saveDocument}>Save</Button>,
  {
    window: {title: 'Document', width: 640, height: 480},
    actions: [defineAction(saveDocument)],
  },
);
```

The TypeScript package lives in [`sdk/typescript`](sdk/typescript/) and exports
`@a3s-lab/gui`.

## Examples

Run the protocol examples from this crate directory:

```bash
cargo run --example protocol_session
cargo run --example state_loop
cargo run --example native_runtime_app
cargo run --example dogfood_session
```

Run native smoke examples on the matching operating system:

```bash
# macOS
cargo run --example appkit_counter --features appkit-native
cargo run --example appkit_controls --features appkit-native
cargo run --example appkit_dogfood --features appkit-native

# Windows
cargo run --example winui_counter --features winui-native
cargo run --example winui_controls --features winui-native
cargo run --example winui_dogfood --features winui-native

# Linux
cargo run --example gtk4_counter --features gtk4-native
cargo run --example gtk4_controls --features gtk4-native
cargo run --example gtk4_dogfood --features gtk4-native
```

The `*_controls` examples exercise text input, toggles, sliders, selects, tabs,
actions, rerendering, and root-window close handling through the same reducer
loop. Native window close requests route through `window.onClose` action ids.
The `*_dogfood` examples run one shared task editor and review workflow
across the native surfaces, including menus, a dialog gate, assignment,
checklists, keyboard shortcuts, window close lifecycle actions, and
state-driven app loop exit. The dogfood shell uses an overflow-aware root
container so longer native forms get a platform scroll viewport instead of
growing past the window. `NativeProtocolApp` is the reusable host-side
protocol loop; `NativeRuntimeApp` is the embedded loop for Rust-owned native
hosts. Platform `run_*_while` loops stop draining queued native events as soon
as their state predicate returns false.

## Feature Flags

The default feature is `headless`.

| Feature | Description |
|---------|-------------|
| `headless` | Pure Rust host for tests and protocol validation |
| `appkit` | AppKit planning adapter and handle types |
| `winui` | WinUI planning adapter and handle types |
| `gtk4` | GTK4 planning adapter and handle types |
| `appkit-native` | Native AppKit surface on macOS |
| `winui-native` | Native WinUI surface on Windows |
| `gtk4-native` | Native GTK4 surface on Linux |

`gtk4-native` requires GTK4 development libraries and `pkg-config`.

## Development

Run the common checks from this crate directory:

```bash
just verify
```

The `just verify` recipe runs formatting, Rust tests, example tests, platform
planning tests, TypeScript SDK tests, and whitespace checks. The underlying
commands are:

```bash
cargo fmt --all
cargo test
cargo test --features appkit,winui,gtk4
cargo test --examples
npm test --prefix sdk/typescript
git diff --check
```

Dogfood and native surface checks are available as focused recipes:

```bash
just dogfood-regression
just check-native
just dogfood-native
just check-winui
```

Build and stage native dogfood release artifacts on the matching operating
system:

```bash
just release-native
just bundle-native
```

## Documentation

- [Architecture](docs/architecture.md)
- [Native app shell](docs/app-shell.md)
- [Native packaging](docs/packaging.md)
- [Web authoring](docs/web-authoring.md)
- [TypeScript SDK](sdk/typescript/README.md)

## License

MIT. See [LICENSE](LICENSE).
