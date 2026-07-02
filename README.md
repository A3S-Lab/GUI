# a3s-gui

Rust-native GUI runtime for structured A3S UI frames.

`a3s-gui` takes semantic UI trees and drives native widget hosts. JSX, React
Aria-style components, HTML/SVG intrinsic tags, protocol JSON, and direct Rust
`NativeElement` trees are accepted as authoring inputs. The runtime lowers them
to a portable native IR, reconciles the tree, emits platform commands, and
routes native events back to stable action ids.

It does not embed a browser, DOM, or WebView.

```text
JSX / protocol JSON / Rust -> NativeElement IR -> keyed diff -> PlatformCommand -> native host
```

## Use It

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
import {Button, createAction, createUiFrame, defineAction} from "@a3s-lab/gui";

const saveDocument = createAction("saveDocument", "Save document");

export const frame = createUiFrame(
  "save-frame",
  <Button className="primary" onPress={saveDocument}>
    Save
  </Button>,
  {actions: [defineAction(saveDocument)]},
);
```

The zero-dependency TypeScript protocol bridge lives in
[sdk/typescript](sdk/typescript).

## Scope

- Native IR, builders, accessibility data, style metadata, and action ids.
- JSX/protocol lowering for semantic components, React Aria names, HTML/SVG
  tags, web attributes, and common event props.
- Keyed rendering, interaction state, action dispatch, and accessibility tree
  projection.
- Headless, recording, platform-planning, and incremental native hosts.

Unsupported web details are preserved as metadata when possible so adapters can
consume them later without changing the protocol.

## Feature Flags

| Feature | Use |
| --- | --- |
| `headless` | Pure Rust host, enabled by default. |
| `appkit` / `appkit-native` | macOS planning types / AppKit surface. |
| `winui` / `winui-native` | Windows planning types / WinUI 3 surface. |
| `gtk4` / `gtk4-native` | Linux planning types / GTK4 surface. |

Native features are platform-specific. `gtk4-native` needs GTK4 development
libraries and `pkg-config`; `winui-native` targets the Windows App SDK;
`appkit-native` targets macOS AppKit.

## Docs

- [docs/architecture.md](docs/architecture.md): renderer, runtime, host, and
  protocol contracts.
- [docs/web-authoring.md](docs/web-authoring.md): accepted JSX, Web props,
  event flow, and lowering rules.
- [sdk/typescript/README.md](sdk/typescript/README.md): JSX runtime and
  protocol helpers.

## Validate

Run checks from this crate directory, not the monorepo root:

```bash
cargo fmt --all
cargo test
npm test --prefix sdk/typescript
git diff --check
```

## Status

The protocol, native IR, renderer, runtime event path, accessibility projection,
and planning adapters are usable for host integration and tests. Native platform
coverage is still incremental.

## License

MIT. See [LICENSE](LICENSE).
