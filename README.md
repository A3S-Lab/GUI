# a3s-gui

Rust-native GUI runtime for structured A3S UI frames.

`a3s-gui` turns Rust `NativeElement` trees, protocol JSON, or JSX from the
TypeScript bridge into native widget commands. It owns the portable native IR,
keyed reconciliation, interaction state, action routing, and accessibility
projection. It does not embed a browser, DOM, or WebView.

```text
JSX / JSON / Rust -> NativeElement IR -> keyed diff -> native host -> actions
```

## Use

Rust:

```rust
use a3s_gui::{GuiResult, GuiRuntime, HeadlessHost, NativeElement, NativeProps, NativeRole, WebProps};

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

TypeScript JSX:

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {Button, createAction, createUiFrame} from "@a3s-lab/gui";

const saveDocument = createAction("saveDocument", "Save document");

export const frame = createUiFrame(
  "save-frame",
  <Button onPress={saveDocument}>Save</Button>,
);
```

The TypeScript bridge lives in [sdk/typescript](sdk/typescript).

## Includes

- Native UI IR, props, style tokens, accessibility data, and action ids.
- Lowering for React Aria-style components, HTML/SVG tags, Web attributes, and
  common event props.
- Keyed rendering, interaction state, event routing, action dispatch, and
  accessibility projection.
- Headless, recording, planning, and native-host adapter surfaces.

Web-only details are preserved as metadata when they cannot be mapped to a
portable native field yet.

## Features

`headless` is enabled by default. Platform feature pairs expose planning types
and native surfaces: `appkit` / `appkit-native`, `winui` / `winui-native`, and
`gtk4` / `gtk4-native`.

Native features are platform-specific. `gtk4-native` requires GTK4 development
libraries and `pkg-config`; `winui-native` targets the Windows App SDK.

## Docs

- [docs/architecture.md](docs/architecture.md): renderer, runtime, host, and
  protocol contracts.
- [docs/web-authoring.md](docs/web-authoring.md): JSX tags, Web props, event
  flow, and lowering rules.
- [sdk/typescript/README.md](sdk/typescript/README.md): JSX runtime and
  protocol helpers.

## Validate

Run checks from this crate directory:

```bash
cargo fmt --all
cargo test
npm test --prefix sdk/typescript
git diff --check
```

## Status And License

The protocol, native IR, renderer, event runtime, accessibility projection, and
planning adapters are ready for host integration and tests. Native platform
coverage is still incremental. MIT licensed; see [LICENSE](LICENSE).
