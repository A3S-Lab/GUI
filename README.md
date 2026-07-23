# A3S GUI

Native GUI runtime for structured A3S UI frames.

A3S GUI renders reducer-driven UI into AppKit, GTK4, WinUI, or a headless test
host without embedding a browser. The authoring model is Rust function
components backed by `ComponentCx` hooks and `.rsx` views. Hooks register state
selectors, props, derived data, effects, resources, and reducers; RSX views
consume those bindings and compile into a portable native IR.

RSX is intentionally native and static. It uses familiar component tags and
Tailwind-compatible classes, but there is no DOM, CSSOM, WebView, or JavaScript
object graph at the host boundary.

## Highlights

- Rust `ComponentCx` function components with React-aligned hook names where the
  native runtime can preserve the same mental model.
- A React Aria-inspired native interaction kernel with typed input modality,
  press lifecycle, long press, pointer/keyboard move gestures, hover state,
  modality-aware focus visibility, direct-target focus callbacks,
  subtree-aware focus-within boundaries, and mounted
  stable-key selection with native aggregate snapshots, independent
  `onAction(key)` item activation, select-all/clear keyboard commands,
  touch/pen long-press selection entry, collection keyboard navigation,
  hierarchical tree expansion, activation-ordered overlay dismissal, modal
  background inertness, nested focus containment/restoration, typed anchored
  overlay placement, inherited locale/direction, versioned capability
  reporting, shared native event-source state machines, and accessibility
  conformance checks. Versioned native-input manifests expand every role marked
  native into strict OS-automation cases; adapter and headless traces cannot
  satisfy that evidence gate.
  Populating those manifests with real platform automation, role-edge input
  parity, layout-aware collection page movement, IME/dead-key conformance,
  measured overlay collision/arrow conformance and scroll locking, remaining
  native focus conformance, and locale formatting are still in progress.
- `.rsx` component source modules with imports, local Rust types, hook
  registrations, Rust selector/reducer expressions, and a final `rsx!(...)`
  view.
- Static RSX lowering for elements, fragments, text, attributes, spreads,
  bindings, slots, action ids, and built-in semantic components.
- Native reducer loop that routes platform events back to stable action ids,
  with opt-in deterministic ancestor propagation control.
- Tailwind-compatible style parsing into platform-neutral `PortableStyle`.
  Runtime interaction state resolves supported pseudo-class and React Aria
  `data-*` variants and projects the resulting style to native widgets on
  AppKit, GTK4, and WinUI without a CSS engine.
- Built-in `rsx_ui` semantic components for controls, overlays, routing,
  collections, layout, feedback, color, date/time, drag/drop, and data views.
- Headless runtime and protocol tests for deterministic reducer and renderer
  coverage.

## Hook Model

`ComponentCx` uses the `use_*` prefix for Rust hook registrations, but not every
`use_*` API is a React hook. A3S keeps the React-aligned surface small and adds
native extensions where browser React does not map cleanly to a reducer-driven
desktop runtime.

| React hook | A3S equivalent |
| --- | --- |
| `useState` | `use_selector`, legacy `use_state`; `use_reactive` is the object-binding extension |
| `useReducer` | `use_reducer`; typed value/payload reducers are A3S extensions |
| `useContext` | `use_context` |
| `useRef` | `use_ref` |
| `useImperativeHandle` | `use_imperative_handle` |
| `useEffect` | `use_effect`, `use_effect_once`, `use_effect_with_deps` |
| `useLayoutEffect` | `use_layout_effect` variants |
| `useInsertionEffect` | `use_insertion_effect` variants |
| `useEffectEvent` | `use_effect_event` |
| `useMemo` | `use_memo`, `use_derived` |
| `useCallback` | `use_callback` |
| `useTransition` | `use_transition_reducer`, `use_transition_effect` |
| `useDeferredValue` | `use_deferred_value` |
| `useId` | `use_id` |
| `useDebugValue` | `use_debug_value`, `debug_values(state)` |
| `useSyncExternalStore` | `use_sync_external_store`, `SyncExternalStore` |
| `useOptimistic` | `use_optimistic` |
| `useActionState` | `use_action_state` |
| React DOM `useFormStatus` | `use_form_status` |

React's `use` entry is an API, not part of the built-in hook list. The closest
native A3S shape is `use_resource`, which exposes resource status and data to
RSX under a stable path.

Native-only hook families include:

| Family | Examples |
| --- | --- |
| State bindings | `use_reactive`, `use_selector_result`, `use_*_value` |
| Typed native actions | `use_value_reducer`, `use_payload_reducer`, `use_field` |
| Post-action effects | `use_action_effect`, `use_value_effect`, `use_payload_effect`, transition effects |
| Semantic UI hooks | `use_button`, `use_form`, `use_table`, `use_slider`, and related hooks |
| Runtime lifecycle | `use_mount`, `use_unmount`, `use_resource`, route hooks |

React does not provide a `use_reducer_effect` hook. Reducer-scoped work is
modeled with A3S post-action and transition effect hooks.

## RSX Component Example

```rust
use a3s_gui::{ComponentCx, RSX};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProfileViewState {
    name: String,
    status: String,
}

#[allow(non_snake_case)]
pub fn ProfileCard(cx: &mut ComponentCx<AppState>) -> RSX {
    let profile = cx.use_reactive("profile", |state: &AppState| ProfileViewState {
        name: state.profile.name.clone(),
        status: format!("{} tasks", state.profile.task_count),
    });
    let save = cx.use_reducer("saveProfile", |state: &mut AppState, _| {
        state.save_profile()
    });

    a3s_gui::rsx!(
        <UiCard key="profile" className="w-full">
          <UiCardHeader key="header">
            <UiCardTitle key="title">{profile.name}</UiCardTitle>
            <UiCardDescription key="status">{profile.status}</UiCardDescription>
          </UiCardHeader>
          <UiCardFooter key="footer">
            <UiButton key="save" onPress={save}>Save</UiButton>
          </UiCardFooter>
        </UiCard>
    )
}
```

Rust expressions belong in selectors, reducers, effects, resources, and other
hook registrations before the final `rsx!(...)` view. Template braces lower to
explicit native bindings or action ids; they do not execute arbitrary Rust or
JavaScript.

## Styling

A3S GUI accepts `class` and `className` and parses supported Tailwind-compatible
utilities directly in Rust. First-party components use the root `DESIGN.md`
visual language: quiet white surfaces, compact radii, strong hairline borders,
black primary actions, and blue only for inline/link semantics.

Prefer semantic components and tokenized classes first:

```rust
a3s_gui::rsx!(
    <UiButton
      key="run"
      variant="outline"
      size="lg"
      className="w-full justify-start border-hairline-strong bg-surface-card text-ink active:bg-surface-strong"
      onPress={runCommand}
    >
      Run command
    </UiButton>
)
```

Arbitrary values remain available for native views that need exact sizing or
platform-matched typography, for example `w-[396px]`, `rounded-[12px]`, and
`font-[Inter,-apple-system,system-ui,sans-serif]`.

## Status

| Area | Readiness |
| --- | --- |
| Component authoring | Usable with Rust `ComponentCx` hooks, `.rsx` component modules, component contracts, and native state bindings. |
| `rsx_ui` design system | Usable `DESIGN.md`-backed component set with Rust-owned hooks. |
| Headless runtime | Usable for protocol tests, command inspection, reducer loops, and accessibility snapshots. |
| Strict protocol v1 | Typed, versioned DTOs with ordered revisions/events, retained command-batch resend, exact ACK validation, and sensitive-value redaction. |
| Native execution | Typed widget IR with frame prepare/commit/ACK, explicit degraded state, and full replay through a fresh executor. |
| AppKit native surface | Usable for macOS smoke apps with role-aware press/hover/focus/key translation, controls, menus, close actions, and typed post-mount `autoFocus`. |
| GTK4 native surface | Usable for Linux smoke apps with role-aware press/hover/focus/key translation, controls, menus, dialogs, and scroll containers. |
| WinUI native surface | Usable for Windows smoke apps with role-aware press/hover/focus/key translation, typed programmatic focus and post-mount `autoFocus`, core controls, resize bounds, close actions, and root-window exit. |
| Native input conformance | Versioned requirement/run/report artifacts and a strict verifier are available. Real AppKit, GTK4, and WinUI automation evidence is not complete yet. |
| Product app shell | Dogfood-ready. Production distribution still needs signed installers and longer real-world focus/input hardening. |

## Roadmap

The delivery roadmap is tracked in [`docs/roadmap.md`](docs/roadmap.md). Its
next phases add a strict typed-message application profile, semantic automation
and replay, release AOT RSX artifacts, an application-layer capability broker,
frame performance budgets, and real-platform truth tests.

All new GUI product configuration and capability policy use ACL (`.acl`). The
application or host validates ACL and passes typed grants into GUI core; the
configuration parser is not a GUI-core dependency.

## Install

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

## Examples

The calculator dogfood app is a Windows Calculator-inspired native app authored
as split `.rsx` function components under
`examples/support/calculator/components/`. The root component uses
`use_reactive` for display state and reducers for native calculator actions.

From the monorepo root:

```bash
just calculator
```

From this crate directory:

```bash
cargo run --example protocol_session
cargo run --example state_loop
cargo run --example native_runtime_app
cargo run --example dogfood_session
```

Native smoke apps:

```bash
# macOS
cargo run --features appkit-native --example appkit_controls
cargo run --features appkit-native --example appkit_dogfood
cargo run --features appkit-native --example appkit_component_playground

# Linux
cargo run --features gtk4-native --example gtk4_controls
cargo run --features gtk4-native --example gtk4_dogfood
cargo run --features gtk4-native --example gtk4_component_playground

# Windows
cargo run --features winui-native --example winui_controls
cargo run --features winui-native --example winui_dogfood
cargo run --features winui-native --example winui_component_playground
```

The component playground opens on an Overview atlas by default and lets you
inspect Foundation, Controls, Collections, Data, Date/Color/Range, and
Overlays/Files from the left navigation.

## Development

The repository pins Rust 1.95.0 in `rust-toolchain.toml`; rustup selects it when
commands run from this directory. Reproducible Cargo commands use the committed
lockfile. Run the complete headless quality gate with:

```bash
just verify
```

This checks formatting, high-confidence clippy groups, rustdoc with warnings
denied, library and example tests, cross-platform planning adapters, and diff
whitespace. On macOS, Linux, or Windows, `just native-ci` additionally runs the
matching native feature's library tests and an all-target compile check.

Generate the exact input cases implied by a backend's native capability claims,
then verify an artifact emitted by an operating-system automation runner:

```bash
just native-input-manifest winui
just native-input-conformance path/to/winui-evidence.json
```

The verifier exits nonzero for missing or duplicate cases, incorrect semantic
event order, modality/target mismatches, incomplete environment identity, or
evidence produced only by an adapter kernel or the portable runtime.

The default feature set includes RSX authoring and the built-in design system.
The runtime/protocol core is independently buildable without SWC or `rsx_ui`:

```bash
cargo check --locked --no-default-features --lib
cargo check --locked --no-default-features --features authoring --lib
```

Use `authoring` for the RSX parser, `ComponentCx`, and explicit component
registries. Add `design-system` when the process should install the shared
built-in `rsx_ui` registry. Native platform features remain orthogonal to these
authoring layers.

## Documentation

- [Runtime and protocol architecture](docs/architecture.md)
- [Native style contract](docs/style-contract.md)
- [React Aria native direction and conformance status](docs/react-aria-native.md)
- [RSX language and hooks](docs/rsx.md)
- [RSX framework plan](docs/rsx-framework.md)
- [DESIGN.md](../../DESIGN.md)
