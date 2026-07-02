# a3s-gui

Rust-native GUI runtime for structured A3S UI frames.

`a3s-gui` takes semantic UI data from Rust, protocol JSON, or JSX compiled by
`@a3s-lab/gui`, lowers it into portable native UI IR, reconciles keyed trees,
and emits commands for native hosts. It keeps useful Web, accessibility, style,
and event metadata without embedding a browser, DOM, CSSOM, or WebView.

```text
JSX / JSON / Rust tree
  -> semantic bridge
  -> NativeElement IR
  -> keyed renderer
  -> native host commands
  -> action events
```

## Scope

- Native UI IR for roles, props, style tokens, accessibility hints, action ids,
  and stable keys.
- Lowering for protocol frames, React Aria-style names, semantic names,
  intrinsic HTML/SVG tags, Web props, and event props.
- Keyed reconciliation, interaction state, event routing, action dispatch, and
  accessibility tree projection.
- Headless and planning hosts, plus AppKit, WinUI, and GTK4 surface adapters.

It is not a browser compatibility layer. DOM access, browser layout APIs,
arbitrary CSS selectors, and WebView-hosted apps are outside the contract.

## Rust

```rust
use a3s_gui::{
    GuiResult, GuiRuntime, HeadlessHost, NativeElement, NativeProps, NativeRole, WebProps,
};

fn main() -> GuiResult<()> {
    let root = NativeElement::new("save-button", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .web(WebProps::new().on_press("saveDocument")),
    );

    let mut runtime = GuiRuntime::new(HeadlessHost::default());
    runtime.render_native(&root)?;

    Ok(())
}
```

## JSX Protocol

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {Button, createAction, createUiFrame} from "@a3s-lab/gui";

const saveDocument = createAction("saveDocument", "Save document");

export const frame = createUiFrame(
  "document",
  <Button onPress={saveDocument}>Save</Button>,
  {window: {title: "Document", width: 640, height: 480}},
);
```

The TypeScript package in [sdk/typescript](sdk/typescript) emits serializable
`UiFrame` data for Rust hosts and has no runtime dependencies.

## Feature Flags

`headless` is enabled by default.

| Feature | Purpose |
| --- | --- |
| `headless` | Pure Rust host for tests and protocol validation. |
| `appkit` / `appkit-native` | macOS AppKit planning types and native surface. |
| `winui` / `winui-native` | Windows App SDK planning types and native surface. |
| `gtk4` / `gtk4-native` | GTK4 planning types and native surface. |

Native feature flags are platform-specific. `gtk4-native` requires GTK4
development libraries and `pkg-config`.

## Docs

- [docs/architecture.md](docs/architecture.md): runtime, renderer, host,
  protocol, and platform boundaries.
- [docs/web-authoring.md](docs/web-authoring.md): JSX tags, Web props, event
  flow, and lowering rules.
- [sdk/typescript/README.md](sdk/typescript/README.md): JSX runtime and protocol
  helpers.

## Validate

Run checks from this crate directory:

```bash
cargo fmt --all
cargo test
npm test --prefix sdk/typescript
git diff --check
```

Use native feature checks only on matching systems, for example
`cargo check --features appkit-native` on macOS.

## Status

The protocol, native IR, renderer, event runtime, accessibility projection, and
planning adapters have focused test coverage. Native backend coverage is still
incremental and platform-specific.

MIT licensed; see [LICENSE](LICENSE).
