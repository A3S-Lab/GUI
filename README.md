# a3s-gui

Rust-native GUI runtime for A3S UI frames.

`a3s-gui` lowers structured Rust or JSX UI trees into a portable native IR,
reconciles keyed updates, and drives AppKit, WinUI, GTK4, or a headless host.

It is not a browser wrapper. There is no DOM, CSSOM, WebView, or browser layout
contract; Web-like input is supported only when it maps to native roles, state,
styles, metadata, accessibility hints, or action ids.

## Scope

- Native UI model: `NativeElement`, `NativeProps`, `NativeRole`.
- Runtime: `GuiRuntime`, keyed reconciliation, event routing, interaction
  state, and accessibility projection.
- Web/JSX bridge: React Aria-style components, HTML/SVG intrinsics, Web props,
  CSS text, Tailwind utilities, and JSX events.
- Hosts: headless testing, platform planning, and native surfaces.
- TypeScript JSX runtime: [sdk/typescript](sdk/typescript).

## Install

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

## Rust

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

## JSX

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

## Feature Flags

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

## Docs

- [Architecture](docs/architecture.md)
- [Web authoring](docs/web-authoring.md)
- [TypeScript SDK](sdk/typescript/README.md)

MIT licensed; see [LICENSE](LICENSE).
