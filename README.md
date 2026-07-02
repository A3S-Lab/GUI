# a3s-gui

Native GUI runtime for A3S UI frames.

`a3s-gui` accepts Rust `NativeElement` trees, serialized `UiFrame` JSON, and JSX
frames from `@a3s-lab/gui`. It lowers them into native UI IR, reconciles keyed
updates, drives a `NativeHost`, routes native events, and projects accessibility
trees.

This is not a browser shell: there is no DOM, CSSOM, WebView, or browser layout
contract. Web-shaped tags and props are accepted only when they map to native
roles, state, style tokens, metadata, accessibility hints, or action ids.

## Quick Start

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

Render Rust IR:

```rust
use a3s_gui::{GuiRuntime, HeadlessHost, NativeElement, NativeProps, NativeRole, WebProps};

let root = NativeElement::new("save-button", NativeRole::Button).with_props(
    NativeProps::new()
        .label("Save")
        .web(WebProps::new().on_press("saveDocument")),
);

let mut runtime = GuiRuntime::new(HeadlessHost::default());
runtime.render_native(&root)?;
```

Or emit the same protocol from JSX:

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

## Core Surface

- `NativeElement`, `NativeProps`, and `NativeRole` define the portable UI IR.
- `GuiRuntime` handles rendering, reconciliation, interaction state, events, and
  accessibility projection.
- `ReactCompilerBridge` lowers protocol frames, React Aria-style components,
  semantic tags, HTML/SVG intrinsics, Web props, CSS text, Tailwind utilities,
  and event props.
- `HeadlessHost` and planning hosts support tests without native SDKs.
- [sdk/typescript](sdk/typescript) provides the dependency-free JSX runtime and
  protocol helpers.

## Features

`headless` is enabled by default.

| Feature | Purpose |
| --- | --- |
| `headless` | Pure Rust host for tests and protocol validation. |
| `appkit`, `winui`, `gtk4` | Platform planning and handle adapter types. |
| `appkit-native`, `winui-native`, `gtk4-native` | Real native surfaces for the matching OS. |

Native surface flags are platform-specific. `gtk4-native` also requires GTK4
development libraries and `pkg-config`.

## Validate

Run checks from this crate directory:

```bash
cargo fmt --all
cargo test
cargo test --features appkit,winui,gtk4
npm test --prefix sdk/typescript
git diff --check
```

Run native-surface checks only on matching systems:

```bash
cargo check --features appkit-native
```

## Docs

- [docs/architecture.md](docs/architecture.md): runtime and host boundaries.
- [docs/web-authoring.md](docs/web-authoring.md): JSX, Web props, lowering,
  events, and accessibility behavior.
- [sdk/typescript/README.md](sdk/typescript/README.md): JSX runtime and protocol
  helper API.

MIT licensed; see [LICENSE](LICENSE).
