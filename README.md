# A3S GUI

**Native GUI runtime for structured UI protocol frames**

A3S GUI is a Rust library for rendering A3S UI frames without embedding a
browser. It lowers Rust-native trees or serialized JSX frames into a portable
native UI IR, reconciles keyed updates, routes host events to stable action
ids, and targets AppKit, WinUI, GTK4, or a headless test host.

---

## Overview

A3S GUI owns the native UI boundary for A3S applications:

- **Portable native IR**: Typed roles, props, style tokens, metadata, and
  accessibility hints that are independent of any one platform.
- **Keyed renderer**: Incremental create, update, insert, remove, and set-root
  commands for native hosts.
- **Protocol frames**: `UiFrame` input from the TypeScript JSX SDK or any
  producer that emits the same serializable shape.
- **Native adapters**: Planning adapters for AppKit, WinUI, and GTK4, plus
  native surface features for target operating systems.
- **Headless host**: Pure Rust validation for tests, protocol fixtures, and
  integration harnesses.

## Quick Start

Add the Rust crate:

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

Render a native tree:

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

Generate a JSX frame:

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

The TypeScript protocol package lives in `sdk/typescript` and exports
`@a3s-lab/gui`.

Run the protocol session example to see frame rendering and event dispatch
through the GTK4 planning adapter:

```bash
cargo run --example protocol_session
```

Run the state loop example to see action dispatch, application state updates,
and frame rerendering in one host-side cycle:

```bash
cargo run --example state_loop
```

Use `NativeProtocolApp` when the host wants a reusable state loop: it owns a
`NativeProtocolSession`, calls a frame builder from the current state, applies
action invocations through a reducer, and returns the follow-up native render
commands.

Run the native runtime app example to see the same reducer loop attached to an
embedded native host event queue:

```bash
cargo run --example native_runtime_app
```

Use `NativeRuntimeApp` when Rust owns the native host directly: it owns a
`GuiRuntime`, drains pending native events from `NativeEventSource`, applies
action invocations through a reducer, and rerenders the next frame into the
same host.

## Features

- **Semantic input**: React Aria-style component names, intrinsic HTML/SVG
  tags, stable keys, text children, and DOM-style event props.
- **Native controls**: Text fields, buttons, links, forms, menus, tabs,
  dialogs, tables, media, ranges, and common HTML form controls.
- **Accessibility**: Labels, relationships, descriptions, structure, state,
  live-region hints, and accessibility tree projection.
- **Portable styling**: Inline style objects, CSS text, Tailwind-like utility
  classes, and native visibility/interactivity state.
- **Event routing**: Press, change, focus, blur, toggle, selection, keyboard,
  and host-native events resolved to registered action ids.
- **App state loops**: Reusable protocol and embedded-runtime loops for
  reducer-driven rerendering after native actions.

## Boundaries

A3S GUI is not a WebView runtime. It does not provide a DOM, CSSOM, browser
layout engine, or JavaScript object graph at the host boundary.

| Concern | Status |
|---------|--------|
| Native widget rendering | In scope |
| Serializable UI protocol frames | In scope |
| AppKit, WinUI, GTK4 planning | In scope |
| Browser DOM APIs | Out of scope |
| Arbitrary CSS selector/layout behavior | Out of scope |
| Treating `HTMLElement` objects as app state | Out of scope |

Web-like input is accepted when it can be lowered to native roles, control
state, accessibility hints, metadata, events, or portable style tokens.

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

Run checks from this crate directory:

```bash
cargo fmt --all
cargo test
cargo test --features appkit,winui,gtk4
npm test --prefix sdk/typescript
git diff --check
```

Native surface checks are OS-specific:

```bash
cargo check --features appkit-native
cargo check --target x86_64-pc-windows-msvc --features winui-native
cargo check --features gtk4-native
```

## Documentation

- [Architecture](docs/architecture.md)
- [Web authoring](docs/web-authoring.md)
- [TypeScript SDK](sdk/typescript/README.md)

## License

MIT. See [LICENSE](LICENSE).
