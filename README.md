# a3s-gui

Native GUI runtime for A3S UI frames.

`a3s-gui` turns structured UI data into native widget commands. Input can come
from Rust `NativeElement` trees, protocol JSON, or JSX compiled by
`@a3s-lab/gui`.

It is not a browser shell: there is no DOM, CSSOM, WebView, or browser layout
contract. Web-shaped tags and props are accepted only when they can be lowered
to native roles, control state, style tokens, accessibility hints, metadata, and
action ids.

```text
JSX / JSON / Rust -> NativeElement IR -> Renderer -> NativeHost
                                                   -> headless / AppKit / WinUI / GTK4
```

## Scope

- Portable native UI IR with roles, props, style tokens, accessibility data,
  stable keys, and action bindings.
- Lowering for protocol frames, React Aria-style component names, semantic
  names, HTML/SVG intrinsic tags, Web props, and event props.
- Keyed reconciliation, incremental updates, interaction state, event routing,
  and accessibility tree projection.
- Headless and planning hosts for tests, plus AppKit, WinUI, and GTK4 adapter
  layers.
- A zero-dependency TypeScript JSX runtime in [sdk/typescript](sdk/typescript).

## Rust

```rust
use a3s_gui::{
    GuiResult, GuiRuntime, HeadlessHost, NativeElement, NativeProps, NativeRole, WebProps,
};

fn main() -> GuiResult<()> {
    let root = NativeElement::new("save-button", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .web(WebProps::new().on_press("saveDocument")),
    );

    GuiRuntime::new(HeadlessHost::default()).render_native(&root)?;
    Ok(())
}
```

## JSX

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {Button, createAction, createUiFrame, defineAction} from "@a3s-lab/gui";

const saveDocument = createAction("saveDocument", "Save document");

export const frame = createUiFrame(
  "document",
  <Button onPress={saveDocument}>Save</Button>,
  {
    window: {title: "Document", width: 640, height: 480},
    actions: [defineAction(saveDocument)],
  },
);
```

The TypeScript SDK emits plain `UiFrame` JSON for Rust hosts and process
boundaries.

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

## More Detail

- [docs/architecture.md](docs/architecture.md): runtime and host boundaries.
- [docs/web-authoring.md](docs/web-authoring.md): JSX tags, Web props, and
  lowering rules.
- [sdk/typescript](sdk/typescript): JSX runtime and protocol helpers.
- [src/platform](src/platform): platform planning adapters and widget mapping.
- [src/backend](src/backend): command execution and handle drivers.

## Validate

Run checks from this crate directory:

```bash
cargo fmt --all
cargo test
npm test --prefix sdk/typescript
git diff --check
```

Run native-surface checks only on matching systems, for example:

```bash
cargo check --features appkit-native
```

The protocol, renderer, event runtime, accessibility projection, and planning
adapters have focused test coverage. Native backend coverage is incremental and
platform-specific.

MIT licensed; see [LICENSE](LICENSE).
