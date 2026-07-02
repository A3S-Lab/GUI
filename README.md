# a3s-gui

Native GUI runtime for A3S protocol frames.

`a3s-gui` turns structured Rust or JSX UI trees into a portable native UI IR,
diffs keyed updates, routes events, and drives AppKit, WinUI, GTK4, or a
headless test host.

This is not a browser shell. There is no DOM, CSSOM, WebView, or browser layout
contract. Web-style input is accepted only when it can lower into native roles,
state, style tokens, metadata, accessibility hints, or action ids.

## Use It For

- Rendering `UiFrame` protocol data into native widgets.
- Testing UI protocol and reconciliation logic without a platform GUI.
- Bridging JSX and React Aria-style component trees into A3S native UI.
- Building or validating AppKit, WinUI, and GTK4 native surface adapters.

## Install

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

## Rust

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
    let _rendered = runtime.render_native(&root)?;

    Ok(())
}
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

The TypeScript SDK emits plain serializable frame data. The Rust side consumes
that data with `ReactCompilerBridge`, renders it with `GuiRuntime`, and returns
native commands, routed events, and accessibility trees.

## Features

The default feature is `headless`.

| Feature | Enables |
| --- | --- |
| `headless` | Pure Rust host for tests and protocol validation. |
| `appkit`, `winui`, `gtk4` | Platform planning adapters and handle types. |
| `appkit-native`, `winui-native`, `gtk4-native` | Real native surfaces for the matching OS. |

`gtk4-native` needs GTK4 development libraries and `pkg-config`.

## Validate

Run the common checks from this directory:

```bash
cargo fmt --all
cargo test
cargo test --features appkit,winui,gtk4
npm test --prefix sdk/typescript
git diff --check
```

Native-surface checks are platform-specific:

```bash
cargo check --features appkit-native
cargo check --target x86_64-pc-windows-msvc --features winui-native
cargo check --features gtk4-native
```

## More

- [Architecture](docs/architecture.md)
- [Web authoring](docs/web-authoring.md)
- [TypeScript SDK](sdk/typescript/README.md)

MIT licensed; see [LICENSE](LICENSE).
