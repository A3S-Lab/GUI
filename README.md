# a3s-gui

Rust-native GUI runtime for A3S UI frames.

`a3s-gui` turns structured UI input into native widget operations. It accepts
Rust `NativeElement` trees, protocol JSON, or JSX emitted by `@a3s-lab/gui`,
then lowers the tree into portable native UI IR, reconciles keyed updates, and
drives a platform host.

It is not a browser wrapper. There is no DOM, CSSOM, WebView, or browser layout
contract. Web-style tags and props are accepted only when they can be projected
into native roles, state, styles, accessibility hints, metadata, and action ids.

```text
JSX / JSON / Rust
        |
        v
NativeElement IR
        |
        v
keyed renderer
        |
        v
NativeHost commands
        |
        v
AppKit / WinUI / GTK4 / headless
```

## What It Provides

- A typed native UI IR for roles, props, style tokens, accessibility data,
  stable keys, and event actions.
- Lowering from protocol frames, React Aria-style component names, semantic
  names, HTML/SVG intrinsic tags, Web props, and event props.
- A keyed renderer with incremental updates, event routing, interaction state,
  and accessibility tree projection.
- Headless and planning hosts for tests, plus AppKit, WinUI, and GTK4 adapters.

## Rust Example

```rust
use a3s_gui::{
    GuiResult, GuiRuntime, HeadlessHost, NativeElement, NativeProps, NativeRole, WebProps,
};

fn main() -> GuiResult<()> {
    let button = NativeElement::new("save-button", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .web(WebProps::new().on_press("saveDocument")),
    );

    let mut runtime = GuiRuntime::new(HeadlessHost::default());
    runtime.render_native(&button)?;

    Ok(())
}
```

## JSX Protocol Example

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
| `appkit` | macOS AppKit planning types. |
| `appkit-native` | macOS AppKit native surface. |
| `winui` | Windows App SDK planning types. |
| `winui-native` | Windows App SDK native surface. |
| `gtk4` | GTK4 planning types. |
| `gtk4-native` | GTK4 native surface. |

Native feature flags are platform-specific. `gtk4-native` requires GTK4
development libraries and `pkg-config`.

## Repository Map

- [docs/architecture.md](docs/architecture.md): runtime and platform boundary.
- [docs/web-authoring.md](docs/web-authoring.md): JSX tags, Web props, and
  lowering rules.
- [sdk/typescript](sdk/typescript): JSX runtime and protocol types.
- [src/platform](src/platform): platform planning adapters.
- [src/backend](src/backend): command and handle driver abstractions.

## Validate

Run checks from this crate directory:

```bash
cargo fmt --all
cargo test
npm test --prefix sdk/typescript
git diff --check
```

Use native checks only on matching systems, for example:

```bash
cargo check --features appkit-native
```

## Status

The protocol, native IR, renderer, event runtime, accessibility projection, and
planning adapters have focused test coverage. Native backend coverage is still
incremental and platform-specific.

MIT licensed; see [LICENSE](LICENSE).
