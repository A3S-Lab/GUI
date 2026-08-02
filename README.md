<p align="center">
  <img src="./assets/readme/hero.svg" width="100%" alt="A3S GUI turns Rust RSX and TypeScript TSX into one self-drawn semantic, layout, interaction, accessibility, and Graphics pipeline">
</p>

<p align="center">
  <a href="https://github.com/A3S-Lab/GUI/actions/workflows/ci.yml"><img alt="CI status" src="https://github.com/A3S-Lab/GUI/actions/workflows/ci.yml/badge.svg"></a>
  <img alt="Rust 1.95.0" src="https://img.shields.io/badge/Rust-1.95.0-2F3945?style=flat-square&logo=rust&logoColor=white">
  <img alt="Self-drawn only" src="https://img.shields.io/badge/renderer-self--drawn%20only-0067C0?style=flat-square">
  <img alt="TSX T2 complete" src="https://img.shields.io/badge/TSX-T2%20complete-1687D9?style=flat-square">
  <a href="LICENSE"><img alt="MIT license" src="https://img.shields.io/badge/license-MIT-2F3945?style=flat-square"></a>
</p>

**A3S GUI** is a Rust-native semantic UI and self-drawn rendering runtime.
Rust RSX works today, and the private `@a3s/gui` automatic JSX runtime lowers
standard TSX into the same versioned frame protocol. Neither authoring path
uses a DOM, CSSOM, WebView, or platform content-widget toolkit.

> [!IMPORTANT]
> A3S owns every application-content pixel. The former AppKit, GTK4, and WinUI
> content backends, features, dependencies, examples, packaging scripts, and CI
> lanes have been deleted. The repository currently contains no executable OS
> window host; `platform-host` defines the zero-widget boundary and
> `platform-runtime` implements the shared atomic self-drawn frame runtime.
> Raw macOS, Windows, and Linux hosts remain roadmap work.

## One semantic tree, one owned-pixel pipeline

```text
Rust ComponentCx / .rsx           TypeScript / TSX
            |                           |
            |                  @a3s/gui/jsx-runtime
            +-------------+-------------+
                          |
                          v
                 versioned UI frame
                          |
                          v
                   NativeElement tree
                 /          |           \
                v           v            v
       layout snapshot  accessibility  interaction + hit regions
                |                         |
                +------------+------------+
                             v
                    A3S Graphics Scene
                             |
                 +-----------+-----------+
                 v                       v
       deterministic software          wgpu
                                         |
                              Metal / DX12 / Vulkan
                                         |
                              zero-widget OS surface
```

Semantic identity is shared across layout, hit testing, accessibility, focus,
selection, and rendering. Paint commands never become the accessibility tree,
and Graphics never infers behavior from pixels.

| Layer | Owns | Explicitly does not own |
| --- | --- | --- |
| Authoring | Rust components/RSX; automatic TSX elements and callbacks | OS handles, GPU resources, layout truth |
| Semantic runtime | Roles, props, actions, focus, selection, overlays, i18n, drag/drop, accessibility | Platform widgets, toolkit layout, product I/O |
| Layout and scene | Portable style, quantized boxes, stable paths, hit regions, scene extraction | Product state or OS geometry |
| [A3S Graphics](https://github.com/A3S-Lab/Graphics) | Retained scenes, damage, software reference output, GPU preparation and rendering | Components, windows, IME, accessibility |
| Platform boundary | Windows/surfaces, presentation, normalized input, text/IME, accessibility bridge, clipboard and system services | Application-content controls, styling, layout, or drawing |

Read [Architecture](docs/architecture.md) and
[Self-drawn platform hosts](docs/platform-hosts.md) for the contracts.

## Current implementation

The repository already provides:

- a stable `NativeElement` semantic IR and strict versioned frame protocols;
- Rust RSX parsing, components, hooks, reducers, effects, and a built-in
  semantic component catalog;
- focus, interaction, overlays, selection, collection navigation, i18n,
  drag/drop policy, live regions, and accessibility snapshots;
- portable style resolution across all 504 top-level `PortableStyle` fields;
- deterministic layout snapshots, stable scene identity, software reference
  output, retained damage, and a reviewed GPU calculator slice;
- explicit production text boundaries: a bounded `TextShaper` produces the
  only intrinsic measurement and retained glyph record, password values are
  masked before shaping, and a stateful `TextSceneEncoder` consumes that same
  record without receiving source text or escaping its ink bounds;
- a bounded `PlatformHost` transaction/event contract with dependency
  firewalls;
- `SelfDrawnWindowRuntime` with atomic prepare/commit/reject, recovery,
  presentation acknowledgements, normalized input, hit testing, drag/drop,
  accessibility actions, and reference/recording presenters;
- Rust-generated TypeScript protocol declarations, canonical cross-language
  fixtures, automatic JSX lowering, strict frame normalization, and
  revision-scoped ordered callbacks;
- a transport-neutral TypeScript `createApp` lifecycle with keyed component
  instances, typed context, render error boundaries, state/reducer/memo/ref/
  effect hooks, batched rerenders, post-commit cleanup, and strict post-handshake
  session/message identity;
- a finalized TSX identity contract: Node-local component instances use
  canonical `a3s:c1:` identities, automatic actions use canonical `a3s:a1:`
  ids derived only from native key paths plus event names, and the generated
  `a3s:` namespace cannot be claimed by explicit actions; Rust owns and
  validates the constants at the Host boundary and generates their TypeScript
  declarations;
- a dependency-free TypeScript client handshake and incremental little-endian
  JSON frame codec aligned with the Rust protocol boundary;
- an ordered framed connection and explicit no-shell Node child-process byte
  transport with bounded stderr, shutdown timeout, and real process fixtures.
- `a3s-gui-tsx-host`, a strict framed stdin/stdout process that negotiates one
  TSX session, lowers full frames into `NativeElement`, produces real software
  reference pixels through `SelfDrawnWindowRuntime`, emits idle host liveness
  probes with fixed response deadlines, and sequences commit, pong, and close
  replies without a content-widget backend.
- `A3sFramedApplicationHostV1`, an ordered single-reader application pump that
  shares the negotiated client session with `createApp`, bounds outstanding
  event work and host-message reordering, answers host pings across pending
  commit/event application, performs timeout-bounded client ping/pong and
  close/ack control, propagates fatal/stream failures, and is exercised from
  real Node through the Rust software host.
- the hostless `createApp(App).run()` path, which selects a validated
  platform host artifact, creates a unique protocol session, binds native events
  before the first render, and closes a partially started host on failure.
- opt-in bounded Host supervision, which observes abnormal termination,
  negotiates a fresh session, transactionally replays the last committed full
  frame and callback scope, gates events until replay commits, and closes the
  application after the configured restart budget is exhausted;
- a real three-generation child-process gate that crashes the first Host,
  accepts keyboard activation after replay, rejects an older render revision
  without invoking its callback, and replays again into a fresh session.

Still required before a production native application:

- real zero-widget macOS, Windows, and Wayland/X11 hosts;
- a production font database/shaper and glyph raster/atlas encoder, followed by
  text editing, IME, and assistive-technology bridges;
- packaging, signing, installer work, and tri-platform visual evidence;
- full self-drawn conformance evidence for every React Aria family.

The nonvisual `HeadlessAdapter` remains only as protocol and transaction test
infrastructure. It is not a renderer and always emits the diagnostic class
`a3s_gui::HeadlessNode`.

## Complete React Aria scope

The target is every semantic family in the official
[React Aria](https://react-aria.adobe.com/) catalog. The checked-in
[component matrix](docs/react-aria-component-matrix.json) pins
`react-aria-components` 1.19.0 and is schema-tested in CI.

- All 51 official top-level families have an explicit A3S component mapping.
- `Button` has the first scene/software-pixel smoke evidence.
- No family is marked self-drawn conformant yet.
- A family becomes conformant only after authoring, behavior, layout/hit,
  scene, deterministic pixels, accessibility, and real macOS/Windows/Linux
  host evidence all pass.

See [React Aria self-drawn direction](docs/react-aria-native.md) for the
acceptance contract and known API gaps.

## TSX without a browser

The TypeScript design adopts
[Nub](https://github.com/nubjs/nub)'s useful boundary: stock Node loads normal
TSX, while a narrow Rust process owns semantic reconciliation, layout,
rendering, and OS resources.

```tsx
import { Button, Text, View, Window, createApp, useState } from "@a3s/gui";

function Counter() {
  const [count, setCount] = useState(0);
  return (
    <Window title="Counter" width={360} height={220}>
      <View className="flex-col gap-4 p-6">
        <Text>Count: {count}</Text>
        <Button onPress={() => setCount((value) => value + 1)}>
          Increment
        </Button>
      </View>
    </Window>
  );
}

await createApp(Counter).run();
```

This zero-argument startup API is implemented and exercised from Node against
the real Rust software/self-drawn process. It resolves the exact platform
artifact package and validates its manifest, size, target, protocol range, and
SHA-256 checksum before spawn; the repository test path uses an explicit
absolute development artifact override plus an unverified-Host opt-in.
Published native platform packages and visible OS hosts do not exist yet, so
this is still not an end-user native package. The underlying scheduler also
remains usable with a typed host object. It owns keyed function-component
instances, typed nested context, render error boundaries, state/reducer/memo/
ref/effect hooks, one-microtask rerender batching, revision-scoped callbacks,
effect promotion only after `committed`, full render envelopes, independent
client/host message ordering, client `hello`/`welcome` negotiation, bounded
incremental framing, an explicit Node child-process transport, the strict
`a3s-gui-tsx-host` software/self-drawn process, and an ordered framed
application host shared with `createApp`, including bidirectional ping/pong,
fixed host liveness deadlines, protocol-level graceful close, and opt-in
bounded restart/replay supervision over fresh protocol sessions. Native host
presentation, platform artifact publication, and npm publication remain T3-T5
work.

Read [TSX native runtime](docs/tsx-native-runtime.md) for protocol and delivery
gates.

## Rust RSX quick start

Consume the crate from Git:

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

Define state, register a reducer, and return semantic RSX:

```rust
use a3s_gui::{rsx, ComponentCx, GuiResult, RSX};

#[derive(Default)]
struct CounterState {
    count: u32,
}

fn counter(cx: &mut ComponentCx<CounterState>) -> RSX {
    let count = cx.use_state("count", |state: &CounterState| state.count);
    let increment = cx.use_reducer("increment", |state: &mut CounterState, _| {
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
    println!("{} action(s)", frame.actions.len());
    Ok(())
}
```

Run maintained examples:

```sh
cargo run --locked --example component_playground
cargo run --locked --example dogfood_session
cargo run --locked --no-default-features \
  --features authoring,platform-runtime,software-reference \
  --example self_drawn_calculator
```

## Cargo features

| Feature | Purpose |
| --- | --- |
| `default` | `headless + authoring + design-system + software-reference` |
| `headless` | Semantic/protocol test host; no visible renderer |
| `authoring` | SWC-backed Rust RSX parsing and compilation |
| `design-system` | Built-in semantic component registrations |
| `graphics` | GUI-to-A3S-Graphics scene boundary |
| `software-reference` | Deterministic software presenter |
| `gpu` | wgpu-backed Graphics path |
| `platform-host` | Zero-widget OS boundary contracts and recording host |
| `platform-runtime` | Shared self-drawn frame/input/accessibility runtime |
| `host-macos`, `host-windows`, `host-linux-*` | Dependency-free host capability markers; concrete hosts are not implemented yet |
| `typescript-schema` | Rust-to-TypeScript protocol declaration generation |

There are deliberately no AppKit, GTK4, WinUI, or corresponding native
feature flags.

## Development

The full CI-equivalent gate is:

```sh
just verify
```

Useful focused gates:

```sh
just check-core
just check-platform-host
just check-platform-runtime
just test-platform-host
just test-platform-runtime
just test-graphics
just check-tsx-protocol
just test-typescript
just test-tsx-host
```

`just verify` also checks dependency firewalls, formatting, Clippy, rustdoc,
all Rust tests and examples, the React Aria catalog, TypeScript fixtures, and
whitespace. CI has one portable verification job; toolkit-specific and legacy
bundle lanes no longer exist.

## Repository map

```text
src/
|- compiler/              RSX/intrinsic lowering into semantic IR
|- runtime/               reconciliation, focus, interaction, selection
|- layout/                deterministic layout snapshots and hit regions
|- drawing/               semantic/layout to A3S Graphics scenes
|- platform_host/         zero-widget OS contracts and recording host
|- platform_runtime/      shared atomic self-drawn window runtime
|- tsx_protocol/          strict Node/Rust wire protocol
|- bin/tsx_host.rs        software self-drawn TSX process host
|- semantic_ui/           React Aria-aligned semantic components and hooks
`- platform/              nonvisual planning/transaction test IR
packages/typescript/      private automatic JSX runtime and protocol SDK
tests/                    dependency firewalls and catalog gates
docs/                     architecture, roadmap, protocol, and conformance
```

## Roadmap

The next critical path is:

1. implement the first raw Windows host, then macOS and Wayland/X11 hosts,
   without adding content-widget dependencies;
2. implement the font discovery/shaping and glyph raster/atlas backends on the
   landed generic text contracts, then add editing and IME;
3. connect the completed T2 TSX runtime to those visible self-drawn hosts;
4. close React Aria families milestone by milestone with tri-platform evidence;
5. restore packaging only after the self-drawn host artifacts exist.

The versioned plan lives in [ROADMAP](docs/roadmap.md).

## Documentation

- [Architecture](docs/architecture.md)
- [Roadmap](docs/roadmap.md)
- [Self-drawn platform hosts](docs/platform-hosts.md)
- [TSX native runtime](docs/tsx-native-runtime.md)
- [React Aria self-drawn direction](docs/react-aria-native.md)
- [React Aria component matrix](docs/react-aria-component-matrix.json)
- [RSX guide](docs/rsx.md)
- [RSX framework](docs/rsx-framework.md)
- [Layout and scene](docs/layout-scene.md)
- [App shell](docs/app-shell.md)
- [Style contract](docs/style-contract.md)
- [Renderer field inventory](docs/renderer-field-inventory.md)
- [Packaging gate](docs/packaging.md)

## License

[MIT](LICENSE)
