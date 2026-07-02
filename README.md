# a3s-gui

Rust-native GUI runtime for A3S.

`a3s-gui` turns structured UI frames into native widget commands. It accepts
protocol JSON, compiled JSX, React Aria-style component trees, and direct Rust
`NativeElement` trees. Web-shaped input is an authoring format only: the runtime
does not embed a browser, DOM, or WebView.

```text
JSX / protocol JSON / Rust
        |
        v
NativeElement IR
        |
        v
keyed renderer diff
        |
        v
PlatformCommand stream
        |
        v
headless / AppKit / WinUI / GTK4
```

## Quick Start

Rust:

```rust
use a3s_gui::{GuiResult, GuiRuntime, HeadlessHost, NativeElement, NativeProps, NativeRole};

fn main() -> GuiResult<()> {
    let root = NativeElement::new("save", NativeRole::Button)
        .with_props(NativeProps::new().label("Save").action("saveDocument"));

    let mut runtime = GuiRuntime::new(HeadlessHost::default());
    runtime.render_native(&root)?;

    Ok(())
}
```

TypeScript JSX:

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {Button, createAction, createUiFrame, defineAction} from '@a3s-lab/gui';

const saveDocument = createAction('saveDocument', 'Save document');

export const frame = createUiFrame(
  'save-frame',
  <Button className="primary" onPress={saveDocument}>
    Save
  </Button>,
  {actions: [defineAction(saveDocument)]},
);
```

The TypeScript package lives in [sdk/typescript](sdk/typescript).

## What It Owns

- A portable native UI IR: roles, props, accessibility data, style metadata, and
  action ids.
- A compiler bridge for semantic components, React Aria names, HTML intrinsic
  tags, SVG intrinsic tags, `className`, `style`, `data-*`, `aria-*`, and common
  JSX event props.
- A keyed renderer that emits create, update, insert, remove, and root commands.
- A rendered accessibility tree API for hosts that retain native widget state.
- Platform planning adapters for AppKit, WinUI, and GTK4.
- Headless and recording backends for tests and protocol integration.
- Incremental native surfaces behind OS-specific feature flags.

Unsupported web details are preserved as metadata where possible so platform
adapters can learn to consume them without changing the protocol shape.

## Important Paths

- [src/compiler](src/compiler.rs): compiled JSX and intrinsic tag lowering.
- [src/react_aria](src/react_aria.rs): semantic and React Aria tree mapping.
- [src/native.rs](src/native.rs): native roles, elements, props, and builders.
- [src/platform](src/platform/mod.rs): widget blueprints, diffs, and setters.
- [src/backend](src/backend/mod.rs): command executors and driver boundaries.
- [src/style](src/style/mod.rs): CSS and Tailwind projection.
- [docs/web-authoring.md](docs/web-authoring.md): accepted JSX and web props.
- [docs/architecture.md](docs/architecture.md): renderer and backend contract.

## Features

Default builds use the pure Rust `headless` path.

```text
headless       test/planning host, enabled by default
appkit         macOS planning types
appkit-native  real AppKit surface on macOS
winui          Windows planning types
winui-native   real WinUI 3 surface on Windows
gtk4           Linux planning types
gtk4-native    real GTK4 surface on Linux
```

Native features are platform-specific. `gtk4-native` requires GTK4 development
libraries and `pkg-config`; `winui-native` targets the Windows App SDK;
`appkit-native` targets macOS AppKit.

## Validate

Run checks from this crate directory:

```bash
cargo fmt --all
cargo test
npm test --prefix sdk/typescript
git diff --check
```

Platform checks:

```bash
cargo test --features appkit
cargo check --features appkit-native
cargo test --features winui
cargo check --target x86_64-pc-windows-msvc --features winui-native
cargo test --features gtk4
cargo check --features gtk4-native
```

## Status

The protocol, native IR, renderer, and planning path are usable for host
integration and tests. Native platform coverage is still incremental.

## License

MIT. See [LICENSE](LICENSE).
