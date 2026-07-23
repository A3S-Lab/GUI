# A3S GUI

<p align="center">
  <strong>Rust-Native Cross-Platform GUI Runtime for A3S</strong>
</p>

<p align="center">
  <em>Reducer-driven Rust components, structured RSX, and direct AppKit, GTK4, and WinUI rendering without a browser runtime.</em>
</p>

---

## Overview

**A3S GUI** is a native GUI runtime for structured A3S UI frames. Applications
describe semantic UI in Rust function components or `.rsx` modules, update
state through reducers, and render the same portable native IR through one of
four hosts:

- AppKit on macOS
- GTK4 on Linux
- WinUI on Windows
- a deterministic headless host for tests and protocol integration

The crate provides:

- `ComponentCx` function components with state, props, context, derived values,
  effects, resources, reducers, and interaction hooks
- static RSX lowering for semantic components, intrinsic elements, bindings,
  actions, fragments, slots, and spreads
- keyed reconciliation and ordered native command batches
- a strict versioned protocol with prepare, commit, ACK, recovery, replay, and
  sensitive-value redaction
- portable interaction behavior for press, hover, focus, selection, overlays,
  collections, keyboard navigation, and localized number fields
- accessibility tree generation, conformance checks, live regions, native
  announcements, and field-level backend capability reporting
- Tailwind-compatible utility parsing into a platform-neutral native style
  model
- a built-in `rsx_ui` component registry plus calculator, dogfood, and
  component-playground applications

A3S GUI does not embed a DOM, CSSOM, WebView, or JavaScript object graph. RSX
uses familiar component syntax, but it compiles into typed native data and
platform widget operations.

The long-term behavioral direction is a native, cross-platform counterpart to
[React Aria](https://react-aria.adobe.com/getting-started). This is not a DOM
compatibility target, and the project does not claim full React Aria parity
yet. See [React Aria Native Direction](docs/react-aria-native.md) for the
implemented contracts and remaining gaps.

## Quick Start

The crate is currently consumed from its Git repository:

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

The default feature set includes the headless runtime, RSX authoring, and the
built-in design system. Add the matching native feature to open real platform
widgets:

```toml
# macOS
a3s-gui = { git = "https://github.com/A3S-Lab/GUI", features = ["appkit-native"] }

# Linux
a3s-gui = { git = "https://github.com/A3S-Lab/GUI", features = ["gtk4-native"] }

# Windows
a3s-gui = { git = "https://github.com/A3S-Lab/GUI", features = ["winui-native"] }
```

Define state, register behavior, and return an RSX view:

```rust
use a3s_gui::{rsx, ComponentCx, GuiResult, RSX};

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

    rsx!(
        <Button key="increment" onPress={increment}>
          Count {count}
        </Button>
    )
}

fn main() -> GuiResult<()> {
    let component = ComponentCx::compile("counter", counter)?;
    let frame = component.render(&CounterState::default())?;
    println!("rendered {} with {} action(s)", frame.frame_id, frame.actions.len());
    Ok(())
}
```

Run the headless component atlas from this repository:

```sh
cargo run --locked --example component_playground
```

Open the native atlas on the current operating system:

```sh
just playground
```

## Cargo Features

| Feature | Purpose |
| --- | --- |
| `headless` | Deterministic runtime and host behavior without an OS GUI |
| `authoring` | SWC-backed RSX parsing, `ComponentCx`, and explicit component registries |
| `design-system` | Built-in `rsx_ui` registry; implies `authoring` |
| `appkit`, `gtk4`, `winui` | Pure planning adapters that do not link an OS widget toolkit |
| `appkit-native` | Real AppKit surface on macOS |
| `gtk4-native` | Real GTK4 surface on Linux |
| `winui-native` | Real WinUI 3 surface on Windows |

Runtime and protocol consumers can exclude the authoring stack:

```sh
cargo check --locked --no-default-features --lib
cargo check --locked --no-default-features --features authoring --lib
```

## Runtime Architecture

```text
Rust ComponentCx function or .rsx module
                    |
                    v
          CompiledRsxNode tree
                    |
                    v
       versioned UiFrame protocol
                    |
                    v
        semantic native UI IR
                    |
                    v
 keyed reconciliation and command batches
                    |
                    v
 AppKit / GTK4 / WinUI / headless host
                    |
                    v
 normalized native events and action reducers
```

The main ownership boundaries are explicit:

| Layer | Responsibility |
| --- | --- |
| Authoring | Rust components, hooks, RSX parsing, component contracts, and bindings |
| Protocol | Serializable frames, actions, revisions, events, ACKs, and recovery |
| Runtime | Reducer flow, interaction state, effects, focus, selection, and overlays |
| Renderer | Stable-key reconciliation and ordered host operations |
| Platform planning | Portable widget blueprints, setters, capabilities, and command batches |
| Native surface | OS widget lifetime, thread affinity, raw input, focus, accessibility, and event delivery |

Application state and product I/O remain outside the renderer. Native backends
never execute component functions, and thread-affine native handles never cross
the serializable host boundary.

## Component And Hook Model

`ComponentCx` keeps React-aligned names where the same mental model is useful
and exposes explicit native extensions where browser behavior does not map
cleanly.

| Concern | A3S GUI API |
| --- | --- |
| State projection | `use_selector`, legacy `use_state`, and object-valued `use_reactive` |
| State transition | `use_reducer`, `use_value_reducer`, and `use_payload_reducer` |
| Props and context | `use_prop`, `use_context`, and `use_id` |
| Derived values | `use_memo` and `use_derived` |
| Lifecycle and work | effect variants, transition effects, mount/unmount hooks, and `use_resource` |
| References | `use_ref` and `use_imperative_handle` |
| Interaction | press, hover, focus, keyboard, move, long-press, selection, and overlay hooks |
| Semantic controls | button, form, text field, table, tree, menu, slider, date, color, and related hooks |

RSX is structural. Template braces resolve registered state, prop, derived,
context, resource, local-item, or action bindings. Arbitrary computation,
mutation, and asynchronous work belong in Rust selectors, reducers, effects,
and resources before the final `rsx!(...)` expression.

## Interaction And Accessibility

The portable behavior layer currently includes:

- normalized keyboard, mouse, touch, pen, virtual, and unknown input modality
- press start, release, end, cancellation, long press, hover, move, wheel, and
  direct focus/focus-within lifecycles
- modality-aware focus visibility and keyed interaction-state preservation
- single, multiple, range, toggle, replace, select-all, and clear selection
  over stable collection keys
- list, grid-list, table, menu, tabs, and tree keyboard behavior, including tree
  expansion and layout-aware page navigation
- activation-ordered overlay dismissal, modal background inertness, focus
  containment/restoration, and logical LTR/RTL placement
- inherited locale and direction, ICU4X collation, decimal and percent parsing,
  localized stepping, and a 34-locale NumberField accessibility catalog
- independent visible labels and accessible names
- native accessibility descriptions, supported ID-reference relationships,
  ARIA state, live-region, and structural metadata projection
- field-level `Native`, `Portable`, or `Unsupported` capability audits

Native accessibility support is intentionally reported per field. GTK4 has the
broadest generic accessibility property and relation surface. AppKit and WinUI
project exact native equivalents where they exist and retain the remaining
semantic data in the portable accessibility tree instead of claiming false
native parity.

## Styling

`class` and `className` are aliases. Supported Tailwind-compatible utilities,
interaction variants, React Aria `data-*` variants, inline style objects, and
CSS declaration strings lower into `PortableStyle`; there is no runtime CSS
engine.

Prefer semantic components and tokenized utilities:

```rust
rsx!(
    <UiButton
      key="run"
      variant="outline"
      size="lg"
      className="w-full justify-start border-hairline-strong bg-surface-card text-ink active:bg-surface-strong"
      onPress={run}
    >
      Run command
    </UiButton>
)
```

Arbitrary native sizing values such as `w-[396px]`, `rounded-[12px]`, and
platform font stacks are available when a fixed design requires them.

## Platform Support

| Host | Cargo feature | Current role |
| --- | --- | --- |
| Headless | default `headless` | Protocol tests, deterministic reducer flow, command inspection, capability audits, and accessibility snapshots |
| macOS | `appkit-native` | AppKit dogfood and smoke applications with native controls, input, focus, menus, overlays, and accessibility |
| Linux | `gtk4-native` | GTK4 4.14+ dogfood and smoke applications with native controls, input, focus, menus, dialogs, scrolling, and accessibility |
| Windows | `winui-native` | WinUI 3 dogfood and smoke applications with native controls, input, focus, dialogs, overlays, and accessibility |

All three native surfaces are suitable for project dogfood and focused smoke
testing. They are not yet presented as a stable production application
framework.

## Current Status

| Area | Readiness |
| --- | --- |
| RSX and `ComponentCx` authoring | Usable; component contracts, bindings, routing, effects, resources, and the built-in registry are implemented |
| Headless runtime | Usable for deterministic tests, protocol sessions, rendering, accessibility, and capability inspection |
| Protocol and native execution | Versioned protocol v1 with ordered revisions, retained resend, exact ACK validation, degraded state, and fresh-executor replay |
| Built-in design system | Broad dogfood component set with calculator and component-atlas coverage |
| AppKit, GTK4, and WinUI | Real native surfaces with host-native CI; continuing platform-edge hardening |
| Native input evidence | Canonical manifests and a strict verifier are implemented; the WinUI harness covers its current 98-case matrix, while AppKit and GTK4 OS-automation evidence is incomplete |
| Packaging | Reproducible unsigned smoke bundles; signing, notarization, installers, and product update metadata remain application responsibilities |
| React Aria direction | Substantial shared behavior foundation; full component, platform, and assistive-technology parity is still in progress |

The dependency-ordered delivery plan and acceptance gates live in
[docs/roadmap.md](docs/roadmap.md).

## Examples

Headless examples:

```sh
cargo run --locked --example protocol_session
cargo run --locked --example state_loop
cargo run --locked --example native_runtime_app
cargo run --locked --example dogfood_session
cargo run --locked --example component_playground
```

Host-selecting recipes:

```sh
just controls-native
just calculator
just playground
just dogfood-native
```

The AppKit, GTK4, and WinUI calculator entrypoints use the same
`shared_calculator_component`, state model, reducer, RSX component tree, window
constraints, and explicit keypad sizing. Their semantics and layout intent are
therefore shared. Exact pixels still follow each native toolkit's control
metrics, font fallback, DPI scaling, and rendering behavior.

Direct native examples follow the same naming pattern:

```sh
# Replace <backend> with appkit, gtk4, or winui on the matching host.
cargo run --locked --features <backend>-native --example <backend>_controls
cargo run --locked --features <backend>-native --example <backend>_calculator
cargo run --locked --features <backend>-native --example <backend>_component_playground
cargo run --locked --features <backend>-native --example <backend>_dogfood
```

## Design Direction

| Concept | Direction |
| --- | --- |
| Authoring | Rust-first function components and static RSX, not JavaScript execution |
| State | Explicit reducers and one-way state-to-frame-to-action flow |
| Rendering | Semantic native IR and stable-key reconciliation |
| Widgets | Real AppKit, GTK4, and WinUI controls by default |
| Behavior | Shared headless contracts with field- and role-level native capability reporting |
| Accessibility | Portable semantic truth plus exact native projection where supported |
| Styling | Portable native style tokens and selected utility syntax, not a browser CSS engine |
| Configuration | Product-owned typed grants; new product configuration uses ACL outside GUI core |
| Testing | Semantic trees, events, capabilities, and OS evidence over platform class-name assertions |
| Packaging | Runtime-owned smoke bundles; product repositories own signing and distribution |

## Source Layout

```text
src/
├── accessibility/    # Semantic tree, conformance, relationships, and native-ready values
├── app/              # Reducer-driven native application loop
├── backend/          # Command execution, recording, and recovery
├── platform/         # Portable widget planning and setter batches
├── protocol.rs       # Versioned frame, action, event, and ACK boundary
├── native.rs         # Portable native UI IR
├── renderer.rs       # Stable-key reconciliation
├── runtime/          # Interaction, focus, overlays, effects, and rerender flow
├── rsx_app/          # ComponentCx, hooks, components, and binding scope
├── rsx_ui/           # Built-in semantic design-system registry
├── appkit_native/    # Real macOS surface
├── gtk4_native/      # Real Linux surface
└── winui_native/     # Real Windows surface

examples/             # Headless, calculator, dogfood, controls, and playground apps
docs/                 # Architecture, language, platform, packaging, and roadmap contracts
packaging/            # Unsigned native smoke-bundle assets and validators
```

## Development

The repository pins Rust 1.95.0 in `rust-toolchain.toml` and commits
`Cargo.lock`. Run commands from this crate directory.

Run the complete portable verification gate:

```sh
just verify
```

This checks formatting, runtime-only build boundaries, high-confidence Clippy
groups, rustdoc warnings, library tests, example tests, all planning adapters,
and diff whitespace.

Run the matching host-native library tests and all-target compile check:

```sh
just native-ci
```

Generate or verify native input conformance artifacts:

```sh
just native-input-manifest winui
just native-input-conformance path/to/native-evidence.json
just winui-input-smoke path/to/winui-smoke.json
```

The WinUI smoke runner requires an interactive Windows desktop and the Windows
App Runtime 1.7 framework package. AppKit and GTK4 native checks require their
matching host toolchains; GTK4 requires version 4.14 or newer development
libraries.

## Documentation

- [Runtime and protocol architecture](docs/architecture.md)
- [Native app shell](docs/app-shell.md)
- [Native packaging](docs/packaging.md)
- [React Aria native direction](docs/react-aria-native.md)
- [RSX language and hooks](docs/rsx.md)
- [RSX framework plan](docs/rsx-framework.md)
- [Native style contract](docs/style-contract.md)
- [Delivery roadmap](docs/roadmap.md)

## License

MIT
