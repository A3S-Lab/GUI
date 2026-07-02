# a3s-gui

Native GUI runtime for A3S protocol frames.

`a3s-gui` turns structured Rust, JSON, or JSX UI trees into keyed native command
streams for AppKit, WinUI, GTK4, and headless hosts. It owns the portable UI IR,
render reconciliation, event routing, interaction state, and accessibility tree
projection.

It is not a browser runtime. There is no DOM, CSSOM, WebView, or browser layout
contract; Web-shaped input is accepted only when it can lower to native roles,
state, style tokens, metadata, accessibility hints, or action ids.

## Install

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

## Rust Usage

```rust
use a3s_gui::{GuiRuntime, HeadlessHost, NativeElement, NativeProps, NativeRole, WebProps};

let root = NativeElement::new("save", NativeRole::Button).with_props(
    NativeProps::new()
        .label("Save")
        .web(WebProps::new().on_press("saveDocument")),
);

let mut runtime = GuiRuntime::new(HeadlessHost::default());
let rendered = runtime.render_native(&root)?;
```

## JSX Usage

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

## What Is Included

- Portable native IR: `NativeElement`, `NativeProps`, `NativeRole`.
- Runtime orchestration: `GuiRuntime`, keyed reconciliation, interaction state,
  event routing, and accessibility projection.
- Web/JSX lowering through `ReactCompilerBridge`, including React Aria-style
  components, HTML/SVG intrinsics, Web props, CSS text, Tailwind utilities, and
  event props.
- Headless and platform-planning hosts for tests and process-boundary protocols.
- A dependency-free TypeScript JSX runtime in [sdk/typescript](sdk/typescript).

## Features

`headless` is enabled by default.

| Feature | Purpose |
| --- | --- |
| `headless` | Pure Rust host for tests and protocol validation. |
| `appkit`, `winui`, `gtk4` | Platform planning and handle adapter types. |
| `appkit-native`, `winui-native`, `gtk4-native` | Real native surfaces for the matching OS. |

Native surface flags are platform-specific. `gtk4-native` requires GTK4
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

Native-surface checks depend on the target platform:

```bash
cargo check --features appkit-native
cargo check --target x86_64-pc-windows-msvc --features winui-native
cargo check --features gtk4-native
```

## More

- [docs/architecture.md](docs/architecture.md)
- [docs/web-authoring.md](docs/web-authoring.md)
- [sdk/typescript/README.md](sdk/typescript/README.md)

MIT licensed; see [LICENSE](LICENSE).
