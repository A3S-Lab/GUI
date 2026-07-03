# a3s-gui

Native GUI runtime for structured A3S UI frames.

`a3s-gui` turns Rust-native trees or serialized JSX frames into a portable
native UI IR, reconciles keyed updates, routes host events back to stable action
ids, and plans or drives platform widgets for AppKit, WinUI, GTK4, and headless
tests.

It is not a WebView runtime. There is no DOM, CSSOM, browser layout engine, or
JavaScript object graph at the host boundary. Web-like input is accepted when it
can be lowered to native roles, control state, accessibility hints, metadata,
events, or portable style tokens.

## Install

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

The TypeScript protocol package lives in `sdk/typescript` and exports
`@a3s-lab/gui` for JSX frame generation.

## Rust

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

The SDK emits JSON-safe `UiFrame` data. Rust consumes that data through
`ReactCompilerBridge`, `UiFrame`, `NativeProtocolSession`, or `GuiRuntime`.

## What Is Supported

- Semantic component names, intrinsic HTML/SVG tags, React Aria-style props, and
  stable keyed children.
- Native roles for text, forms, buttons, links, menus, tabs, dialogs, tables,
  media, ranges, and common HTML form controls.
- Portable control state, accessibility relationships and state, `aria-*`,
  `data-*`, selected HTML attributes, inline style objects, CSS text, and
  Tailwind-like utility classes.
- Event routing for press, change, focus, blur, toggle, selection, keyboard, and
  platform host events.
- Headless validation plus platform planning adapters for macOS AppKit,
  Windows WinUI, and Linux GTK4.

## Feature Flags

The default feature is `headless`.

| Feature | Purpose |
| --- | --- |
| `headless` | Pure Rust host for tests and protocol validation. |
| `appkit`, `winui`, `gtk4` | Platform planning adapters and handle types. |
| `appkit-native`, `winui-native`, `gtk4-native` | Real native surfaces on each target OS. |

`gtk4-native` requires GTK4 development libraries and `pkg-config`.

## Checks

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

MIT licensed. See [LICENSE](LICENSE).
