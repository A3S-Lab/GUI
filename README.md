# A3S GUI

Native GUI runtime for structured A3S UI frames.

A3S GUI renders reducer-driven UI frames into native AppKit, WinUI, GTK4, or a
headless test host without embedding a browser. The primary authoring model is
React-inspired Rust component functions backed by explicit `ComponentCx` hooks.
Optional `.rsx` files are view templates that only consume hook data. A3S lowers
those components into a portable native IR, reconciles keyed updates, and routes
native events back to stable action ids.

It is not a WebView runtime. There is no DOM, CSSOM, browser layout engine, or
JavaScript object graph at the host boundary.

## Status

| Area | Readiness |
| --- | --- |
| Component authoring | Usable with Rust `ComponentCx` functions, `.rsx` view templates, component contracts, and native state bindings. |
| `rsx_ui` design system | React-inspired Rust RSX primitives for forms, dialogs, disclosure, selection, tables, toggles, feedback, tabs, breadcrumbs, file/drop zones, tags, trees, toast, virtualized regions, and layout, backed by the Vercel/Geist tokens in `DESIGN.md`. |
| Headless runtime | Usable for protocol tests, command inspection, reducer loops, and accessibility snapshots. |
| AppKit native surface | Usable for macOS smoke apps with windows, core controls, menus, keyboard events, close actions, and native `autoFocus`. |
| GTK4 native surface | Usable for Linux smoke apps with the same core controls, menus, dialogs, close actions, and scroll containers. |
| WinUI native surface | Usable for Windows smoke apps with core controls, size hints, resize bounds, focus callbacks, keyboard routing, close actions, and root-window exit. Programmatic `autoFocus` is tracked but limited by the current `winio-winui3` safe API. |
| Product app shell | Dogfood-ready. Production distribution still needs signed installers, broader native automation, and longer real-world focus/input hardening. |

## Install

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

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

## RSX Usage

Author stateful UI as Rust function components. `ComponentCx` owns the logic:
hooks register state selectors, semantic props, and reducers; hook handles are
the data layer; the returned `RSX` view only consumes those bindings.

```rust
use a3s_gui::{rsx, ComponentCx, RSX};

#[derive(Default)]
struct CounterState {
    count: u32,
}

fn counter(cx: &mut ComponentCx<CounterState>) -> RSX {
    let count = cx.use_state("count", |state: &CounterState| state.count);
    let increment = cx.use_reducer("increment", |state: &mut CounterState, _action| {
        state.count += 1;
        Ok(())
    });

    rsx!(<Button onPress={increment}>Count {count}</Button>)
}

let counter = ComponentCx::compile("counter", counter)?;
```

For larger views, keep the component logic in Rust and return a `.rsx` view
template from that component function. The `.rsx` file is not the component
logic layer; it only consumes `props.*`, `state.*`, and action bindings:

```rsx
fn CounterView(props: CounterViewProps) -> RSX {
  (
    <UiCard key="root" className="bg-background text-foreground">
      <UiCardHeader key="header">
        <UiCardTitle key="title">A3S Counter</UiCardTitle>
        <UiCardDescription key="description">
          Count {state.count}
        </UiCardDescription>
      </UiCardHeader>
      <UiCardFooter key="footer">
        <UiButton key="increment" onPress={increment}>
          Increment
        </UiButton>
      </UiCardFooter>
    </UiCard>
  )
}
```

`parse_rsx` parses the A3S `.rsx` syntax in-process, with no Node, Bun, React,
DOM, CSSOM, or WebView runtime. Static elements, fragments, string/boolean/
number literal attributes, text children, event action references, `class` /
`className`, Tailwind values, and Rust-style view-template expressions are
supported. The style pipeline recognizes shadcn-compatible semantic color
utilities such as `bg-background`, `text-foreground`, `border-border`, and
`ring-ring`, backed by the Vercel/Geist token values in `DESIGN.md`.

Arbitrary dynamic JS expressions are rejected unless they are represented as
explicit A3S bindings or event action identifiers. See `docs/rsx.md` for the
language contract and token table.

Use `UiFrame::from_rsx_source("frame-id", source)` when the host wants the full
Rust-native path from RSX source to a renderable native frame. The frame infers
registered actions from event props, preserves Tailwind classes for the
Rust-side style resolver, and can be rendered with the existing
`UiFrame::render_into` native runtime API.

```rust
use a3s_gui::{Gtk4Adapter, NativeProtocolSession, UiFrame};

let source = r#"
fn App() -> RSX {
  (
    <Toolbar key="root" orientation="vertical" className="min-w-[920px] bg-background p-3">
      <Button key="save" onPress={saveDocument} className="rounded-md">Save</Button>
    </Toolbar>
  )
}
"#;

let frame = UiFrame::from_rsx_source("desktop", source)?;
let mut session = NativeProtocolSession::new(Gtk4Adapter);
session.render_frame(&frame)?;
```

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
input, initial/rerendered and change-event max-length clamping, focus/blur
routing, toggles, sliders, selects, tabs, keyboard shortcut routing, scroll
containers, window and dialog close actions, reducer-driven rerendering, and
state-driven app loop exit.

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
Those bundle smoke jobs upload compressed `a3s-gui-dogfood-*` artifacts for
manual download and platform QA, plus `.sha256` and `.metadata.txt` files for
archive integrity checks and artifact identification before unpacking. CI
validates that the archive, checksum file, and metadata agree before upload.

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
and target-platform QA. Each staged dogfood bundle includes a `README.txt`
handoff note and a `MANIFEST.txt` with per-file SHA-256 checksums.

## Documentation

- [Architecture](docs/architecture.md)
- [RSX language](docs/rsx.md)
- [RSX framework plan](docs/rsx-framework.md)
- [Native app shell](docs/app-shell.md)
- [Native packaging](docs/packaging.md)

## License

MIT. See [LICENSE](LICENSE).
