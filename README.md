# a3s-gui

Native GUI runtime for structured A3S UI frames.

`a3s-gui` is the Rust side of A3S GUI rendering. It accepts Rust
`NativeElement` trees, protocol JSON, or compiled JSX frames from
`@a3s-lab/gui`; lowers them into portable native UI IR; diffs keyed trees; and
emits native host commands. It preserves useful Web and accessibility metadata,
but it does not embed a browser, DOM, CSSOM, or WebView.

```text
JSX / JSON / Rust
  -> semantic bridge
  -> NativeElement IR
  -> keyed renderer
  -> NativeHost commands
  -> action events
```

## What It Contains

- Native UI IR, props, style tokens, accessibility data, and action ids.
- Lowering for semantic components, React Aria-style names, HTML, SVG, Web
  props, and common event props.
- Keyed reconciliation, interaction state, event routing, action dispatch, and
  accessibility tree projection.
- Headless hosts, planning adapters, and native host surfaces for AppKit, WinUI,
  and GTK4.

## Use From Rust

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

## Use From JSX

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {Button, createAction, createUiFrame} from "@a3s-lab/gui";

const saveDocument = createAction("saveDocument", "Save document");

export const frame = createUiFrame(
  "document",
  <Button onPress={saveDocument}>Save</Button>,
);
```

The TypeScript bridge lives in [sdk/typescript](sdk/typescript).

## Feature Flags

`headless` is enabled by default.

| Feature | Purpose |
| --- | --- |
| `headless` | Pure Rust host for tests and protocol validation. |
| `appkit` / `appkit-native` | macOS AppKit planning types and native surface. |
| `winui` / `winui-native` | Windows App SDK planning types and native surface. |
| `gtk4` / `gtk4-native` | GTK4 planning types and native surface. |

Native features are platform-specific. `gtk4-native` requires GTK4 development
libraries and `pkg-config`.

## Docs

- [docs/architecture.md](docs/architecture.md): runtime, renderer, host, and
  protocol boundaries.
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

## Status

The protocol, native IR, renderer, event runtime, accessibility projection, and
planning adapters are covered by tests. Native backend coverage is still
incremental.

MIT licensed; see [LICENSE](LICENSE).
