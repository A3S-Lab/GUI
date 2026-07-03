# a3s-gui

Native UI runtime for A3S frames.

`a3s-gui` compiles Rust, JSX, or React Aria-style UI trees into a portable
native IR, diffs keyed updates, routes host events, and drives AppKit, WinUI,
GTK4, or a headless test host.

It is not a browser wrapper. There is no DOM, CSSOM, WebView, or browser layout
contract. HTML-like input is accepted only when it can lower to native roles,
state, accessibility data, action ids, metadata, or portable style tokens.

## What It Covers

- Native planning for AppKit, WinUI, GTK4, and headless tests.
- A keyed runtime that turns render diffs into host commands.
- React Aria and JSX bridges for serializable `UiFrame` data.
- Native control state for text fields, buttons, menus, dialogs, tabs, ranges,
  tables, media, and common HTML form controls.

## Install

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

## Rust Quick Start

```rust
use a3s_gui::{
    GuiRuntime, GuiResult, HeadlessHost, NativeElement, NativeProps, NativeRole, WebProps,
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

## JSX Frames

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

The TypeScript SDK emits JSON-safe frame data. Rust consumes it through
`ReactCompilerBridge` and `GuiRuntime`.

## Features

The default feature is `headless`.

| Feature | Enables |
| --- | --- |
| `headless` | Pure Rust host for tests and protocol validation. |
| `appkit`, `winui`, `gtk4` | Platform planning adapters and handle types. |
| `appkit-native`, `winui-native`, `gtk4-native` | Real native surfaces for each OS. |

`gtk4-native` requires GTK4 development libraries and `pkg-config`.

## Checks

```bash
cargo fmt --all
cargo test
cargo test --features appkit,winui,gtk4
npm test --prefix sdk/typescript
git diff --check
```

Platform-native checks depend on the target OS:

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
