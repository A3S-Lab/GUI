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
- `.rsx` component source modules with imports, local Rust types, hook
  registrations, Rust selector/reducer expressions, and a final `rsx!(...)`
  view.
- Static RSX lowering for elements, fragments, text, attributes, spreads,
  bindings, slots, action ids, and built-in semantic components.
- Native reducer loop that routes platform events back to stable action ids.
- Tailwind-compatible style parsing into platform-neutral `PortableStyle`.
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
      className="w-full justify-start border-hairline-strong bg-canvas text-ink active:bg-surface-strong"
      onPress={runCommand}
    >
      Run command
    </UiButton>
)
```

Arbitrary values remain available for native views that need exact sizing or
platform-matched color, for example `w-[396px]`, `bg-[#f3f3f3]`, and
`font-[Segoe_UI,Inter,-apple-system,system-ui,sans-serif]`.

## Status

| Area | Readiness |
| --- | --- |
| Component authoring | Usable with Rust `ComponentCx` hooks, `.rsx` component modules, component contracts, and native state bindings. |
| `rsx_ui` design system | Usable shadcn-like semantic component set backed by `DESIGN.md` tokens and Rust-owned hooks. |
| Headless runtime | Usable for protocol tests, command inspection, reducer loops, and accessibility snapshots. |
| AppKit native surface | Usable for macOS smoke apps with windows, controls, menus, keyboard events, close actions, and native `autoFocus`. |
| GTK4 native surface | Usable for Linux smoke apps with controls, menus, dialogs, close actions, and scroll containers. |
| WinUI native surface | Usable for Windows smoke apps with core controls, size hints, resize bounds, focus callbacks, keyboard routing, close actions, and root-window exit. |
| Product app shell | Dogfood-ready. Production distribution still needs signed installers and longer real-world focus/input hardening. |

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

# Linux
cargo run --features gtk4-native --example gtk4_controls
cargo run --features gtk4-native --example gtk4_dogfood

# Windows
cargo run --features winui-native --example winui_controls
cargo run --features winui-native --example winui_dogfood
```

## Documentation

- [RSX language and hooks](docs/rsx.md)
- [RSX framework plan](docs/rsx-framework.md)
- [DESIGN.md](../../DESIGN.md)
