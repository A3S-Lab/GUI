# A3S GUI

Native GUI runtime for structured A3S UI frames.

A3S GUI renders reducer-driven UI frames into native AppKit, WinUI, GTK4, or a
headless test host without embedding a browser. It accepts Rust-native trees or
serialized JSX frames, lowers them into a portable native IR, reconciles keyed
updates, and routes native events back to stable action ids.

It is not a WebView runtime. There is no DOM, CSSOM, browser layout engine, or
JavaScript object graph at the host boundary.

## Status

| Area | Readiness |
| --- | --- |
| Headless runtime | Usable for protocol tests, command inspection, reducer loops, and accessibility snapshots. |
| TypeScript JSX SDK | Usable for emitting `UiFrame` JSON with semantic components, intrinsic HTML/SVG tags, style, metadata, and event props. |
| AppKit native surface | Usable for macOS smoke apps with windows, core controls, menus, keyboard events, close actions, and native `autoFocus`. |
| GTK4 native surface | Usable for Linux smoke apps with the same core controls, menus, dialogs, close actions, and scroll containers. |
| WinUI native surface | Usable for Windows smoke apps with core controls, size hints, resize bounds, focus callbacks, keyboard routing, close actions, and root-window exit. Programmatic `autoFocus` is tracked but limited by the current `winio-winui3` safe API. |
| Product app shell | Dogfood-ready. Production distribution still needs signed installers, broader native automation, and longer real-world focus/input hardening. |

## Install

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

The TypeScript protocol package lives in [`sdk/typescript`](sdk/typescript/) and
exports `@a3s-lab/gui`.

## Rust Usage

Render a native tree into the headless host:

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

## JSX Usage

Generate the same protocol shape from JSX:

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

JSX frames preserve semantic component names, intrinsic HTML/SVG tags, event
props, ARIA metadata, portable style tokens, and window metadata. Native hosts
lower only the parts that make sense for native UI; browser-only behavior is out
of scope.

## Examples

Run protocol and reducer examples from this crate directory:

```bash
cargo run --example protocol_session
cargo run --example state_loop
cargo run --example native_runtime_app
cargo run --example dogfood_session
```

Run native smoke apps on the matching operating system:

```bash
# macOS
cargo run --features appkit-native --example appkit_controls
cargo run --features appkit-native --example appkit_dogfood

# Linux
cargo run --features gtk4-native --example gtk4_controls
cargo run --features gtk4-native --example gtk4_dogfood

# Windows
cargo run --features winui-native --example winui_controls
cargo run --features winui-native --example winui_dogfood
```

The shared dogfood app exercises windows, menus, dialogs, text input, number
input, toggles, sliders, selects, tabs, keyboard routing, scroll containers,
close actions, reducer-driven rerendering, and state-driven app loop exit.

## Features

| Feature | Description |
| --- | --- |
| `headless` | Pure Rust host for tests and protocol validation. Enabled by default. |
| `appkit` | AppKit planning adapter and handle types. |
| `winui` | WinUI planning adapter and handle types. |
| `gtk4` | GTK4 planning adapter and handle types. |
| `appkit-native` | Native AppKit surface on macOS. |
| `winui-native` | Native WinUI surface on Windows. |
| `gtk4-native` | Native GTK4 surface on Linux. Requires GTK4 development libraries and `pkg-config`. |

## Development

Run the full local verification suite from `crates/gui`:

```bash
just verify
```

CI runs the same verification gate on Linux and also runs host-native AppKit,
GTK4, and WinUI compile/dogfood checks on their matching operating systems.
Pushes to `main` additionally stage and validate the unsigned dogfood bundles.

Focused native and dogfood checks:

```bash
just dogfood-regression
just check-native
just dogfood-native
```

Build and stage host-native dogfood release artifacts:

```bash
just release-native
just bundle-native
just check-bundle-native
```

The staged bundles are unsigned smoke artifacts. Product repositories still own
bundle identifiers, icons, signing, notarization, installers, update metadata,
and target-platform QA.

## Documentation

- [Architecture](docs/architecture.md)
- [Native app shell](docs/app-shell.md)
- [Native packaging](docs/packaging.md)
- [Web authoring](docs/web-authoring.md)
- [TypeScript SDK](sdk/typescript/README.md)

## License

MIT. See [LICENSE](LICENSE).
