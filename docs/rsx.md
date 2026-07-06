# RSX Native UI Language

`.rsx` is the A3S native UI DSL. It keeps the familiar React component tag
shape, but
it compiles directly into `a3s-gui`'s native UI IR instead of a browser DOM.

## Contract

RSX is intentionally structural:

- Rust `ComponentCx` functions are the component authoring API for stateful UI.
- `.rsx` files are view templates. They may wrap the view in one
  `fn View(props: ViewProps) -> RSX` expression-body function so the template is
  easy to read and type at the syntax boundary.
- Elements, fragments, and text children are supported.
- Component tags such as `Toolbar`, `Button`, and `Text` are supported.
- Intrinsic HTML and SVG tags such as `div`, `button`, `input`, `svg`, and
  `path` are supported when they map to native semantics.
- `class` and `className` are aliases.
- Static string, boolean, and number attributes are supported.
- `style="property: value"` is supported for static CSS declarations.
- Event attributes accept action references: `onClick={saveDocument}`.
- Lowercase DOM event spellings such as `onclick={saveDocument}` normalize to
  the same native event action as `onClick`.
- Pure `state.*`, `props.*`, `derived.*`, `context.*`, and `resource.*` member paths are
  supported as explicit A3S bindings for scalar props and text children. Paths
  may use dot members plus static computed segments such as `[0]`,
  `["primary"]`, or ``[`primary`]``.
- Object spread attributes are supported when the spread expression is a pure
  `state.*`, `props.*`, `derived.*`, `context.*`, `resource.*`, or local item
  binding.
- Registered RSX subcomponents can be expanded by the Rust component layer.

RSX does not execute JavaScript. View-template functions are parsed for their
static RSX expression body, but arbitrary expression children, computed spread
expressions, unbounded ternaries, loops, and JavaScript hooks are dynamic
application logic. Represent that logic as explicit A3S state, bindings,
registered RSX subcomponents, Rust hooks on `ComponentCx`, or registered native
actions before rendering.

## View Templates

Author components as Rust functions that receive `&mut ComponentCx<S>`.
Use `.rsx` files only as view templates when the view is large enough to deserve
a separate file. A view template receives `props` and consumes `state.*`,
`props.*`, `derived.*`, `context.*`, `resource.*`, and action bindings; it does
not own business logic.

```rsx
fn CounterView(props: CounterViewProps) -> RSX {
  (
    <Button key="counter" onPress={props.onIncrement}>
      Count {state.count}
    </Button>
  )
}
```

Typed props are accepted at the syntax boundary so view files can read like Rust
without executing Rust code inside the template:

```rsx
fn BadgeView(props: BadgeViewProps) -> RSX {
  <Text key="badge" className={props.className} label={props.label} />
}
```

The function body is not a Rust or JavaScript runtime. `props.*`, `state.*`,
`derived.*`, `context.*`, and `resource.*` are native binding paths resolved by
the Rust `ComponentCx`/`RsxComponent` hook system. Calls, local mutation, async
work, and hooks belong in Rust selectors, reducers, effects, resources, and
registered actions.

## Dynamic State

Dynamic UI follows a native one-way flow:

```text
state -> RSX bindings -> native frame -> action -> reducer -> state
```

RSX bindings are member paths, not JavaScript evaluation:

```rsx
<Button key="counter" onPress={increment}>
  Count {state.count}
</Button>
```

Static computed path segments are allowed when they can be lowered to a native
binding path without executing JavaScript:

```rsx
<Text
  key="first"
  label={state.items[0].title}
  className={props.classes["primary"]}
  data-theme={context["theme"].name}
/>
```

Dynamic computed expressions such as `state.items[index]`,
`props.classes[getClassName()]`, or optional JavaScript fallbacks still belong in
Rust selectors. Expose their result as a named `state.*`, `props.*`,
`derived.*`, `context.*`, or `resource.*` binding before rendering.

Hosts render bound RSX with state:

```rust
use serde_json::json;
use a3s_gui::UiFrame;

let state = json!({ "count": 3 });
let frame = UiFrame::from_rsx_source_with_state(
    "counter",
    include_str!("ui/counter.rsx"),
    &state,
)?;
```

For reusable function-component templates, hosts can pass a full binding scope with
`state`, `props`, `derived`, read-only `context`, and `resource`:

```rust
use serde_json::json;
use a3s_gui::UiFrame;

let scope = json!({
    "state": { "count": 3 },
    "props": { "titleClass": "text-sm font-medium" },
    "derived": { "status": "Ready" },
    "context": { "theme": { "name": "dark" } },
    "resource": { "profile": { "status": "ready" } }
});
let frame = UiFrame::from_rsx_source_with_scope("counter", source, &scope)?;
```

Keys cannot be bound dynamically. Element identity must stay stable across
renders so the native reconciler can update existing widgets instead of
guessing which widget moved.

## Prop Spreads

RSX accepts spread attributes only when the expression is a binding to an object
in the native render scope:

```rsx
<Button
  key="save"
  {...props.primaryButton}
  label="Save"
  disabled={false}
/>
```

The object is resolved by the Rust component layer, then merged into the native
props before render. Explicit props written on the element win over spread props,
so a reusable component can keep local guarantees such as `disabled={false}` or
`label="Save"` while still accepting common visual and event props.

```rust
let mut cx = ComponentCx::<AppState>::new("save");
cx.use_prop("primaryButton", |_state: &AppState| {
    serde_json::json!({
        "className": "rounded-md border border-border bg-background",
        "onPress": "saveDocument",
        "data-kind": "primary"
    })
});
cx.use_reducer("saveDocument", save_document);
let button = cx.into_component(RSX::source(source))?;
```

The spread value must be a JSON object. `key` cannot come from a spread because
native widget identity must be explicit and stable. Calls such as
`{...buttonProps()}` are rejected; compute props with Rust selectors and expose
the result as `state.*`, `props.*`, `derived.*`, `context.*`, or `resource.*`.

## Control Flow

RSX control flow is native and explicit. It uses structural elements instead of
executing JavaScript expressions during render.

```rsx
<Toolbar key="root" orientation="vertical">
  <Text key="always" label="Always" />
  <Show key="ready-slot" when={state.ready}>
    <Text key="ready" label={state.message} />
  </Show>
  <When key="visible-slot" unless={state.hidden}>
    <Text key="visible" label="Visible" />
  </When>
</Toolbar>
```

`<Show>` and `<When>` are compile-time control nodes. They do not become native
widgets. When the condition is true, their children are inserted into the parent
at the same position; when it is false, the subtree is skipped and child
bindings are not resolved.

The condition must be a boolean literal or a `state.*` / `props.*` /
`derived.*` / `context.*` / `resource.*` binding:

- `when={state.ready}` renders when the value is true.
- `unless={state.hidden}` renders when the value is false.
- Supplying both means `when && !unless`.

Use these controls instead of JavaScript ternaries or short-circuit expressions.
That keeps the authoring model close to React while preserving the pure native
one-way data flow.

RSX also accepts a small static control sugar subset and lowers it to the same
control nodes:

```rsx
<Toolbar key="root">
  {state.ready && <Text key="ready" label="Ready" />}
  {!state.hidden && <Text key="visible" label="Visible" />}
  {state.ready
    ? <Text key="ready-branch" label="Ready" />
    : <Text key="waiting-branch" label="Waiting" />}
</Toolbar>
```

Only boolean literals, `state.*`, `props.*`, `derived.*`, `context.*`,
`resource.*`, and their `!` forms are accepted as control conditions. Calls,
comparisons, loops,
computed spreads, and arbitrary expressions are still application logic. Apart
from the narrow list `map` sugar documented below, dynamic transforms should be
modeled with native state selectors, derived selectors, context selectors,
resource selectors, actions, reducers, and explicit RSX controls.

## Lists

Lists use the native `<For>` control. Like `<Show>`, it is a structural control
node and does not become a rendered widget.

```rsx
<Toolbar key="commands" orientation="vertical">
  <For key="command-list" each={state.commands} as="command" indexAs="index" keyBy="id">
    <Text key="row" label={command.title} data-index={index} />
  </For>
</Toolbar>
```

`each` must resolve to an array. Every item is exposed as the local name from
`as`, which defaults to `item`; `indexAs` optionally exposes the zero-based
array index. `keyBy` is a static item path used to build stable native keys for
the whole repeated subtree. If `keyBy` is omitted, RSX falls back to the item
index, which is only appropriate for append-only lists.

More complex transforms should be computed in Rust selectors before rendering
and exposed as `state.*`, `props.*`, `derived.*`, `context.*`, or `resource.*`
arrays.

## Component Hooks

`ComponentCx` is the preferred Rust-native component authoring layer for RSX.
It makes the data flow explicit:

```text
logic hooks -> state/props/action handles -> RSX view bindings -> native frame
```

Hooks live on the explicit component context. This keeps Rust honest: there is
no hidden JavaScript-like hook runtime, and the view only consumes data returned
by hooks.

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

Semantic hooks return props for the view to spread, mirroring the role of
interaction hooks without importing a JavaScript runtime model:

```rust
use a3s_gui::{rsx, ComponentCx, RSX, UsePressProps};

fn pressable(cx: &mut ComponentCx<CounterState>) -> RSX {
    let count = cx.use_state("count", |state: &CounterState| state.count);
    let increment = cx.use_reducer("increment", |state: &mut CounterState, _action| {
        state.count += 1;
        Ok(())
    });
    let action = increment.clone();
    cx.use_press(move |_state| UsePressProps::new().on_press(Some(&action)));
    rsx!(
        <button key="root" {...props.pressProps}>
          Count {count}
        </button>
    )
}
```

`RsxComponent` remains the lower-level template API used by routers,
design-system presets, and tooling. App components with state or actions should
prefer `ComponentCx`; use `RsxComponent` directly only when a caller already has
a prepared template and a small registration bundle:

```rust
use a3s_gui::{ActionInvocation, GuiResult, RsxComponent};

let counter = RsxComponent::new(
    "counter",
    r#"
    fn Counter() -> RSX {
      (
        <Button key="counter" onPress={increment}>
          Count {state.count}
        </Button>
      )
    }
    "#,
)?
.use_state("count", |state: &CounterState| state.count)
.use_derived("summary", |state: &CounterState| format!("Count {}", state.count))
.use_reducer("increment", |state: &mut CounterState, _action: &ActionInvocation| {
    state.count += 1;
    Ok(())
});
```

For file-based authoring, preserve the source name when compiling the template:

```rust
use a3s_gui::{ActionInvocation, GuiResult, RsxComponent};

let counter = RsxComponent::from_source(
    "counter",
    "ui/counter.rsx",
    include_str!("ui/counter.rsx"),
)?
.use_state("count", |state: &CounterState| state.count)
.use_reducer("increment", |state: &mut CounterState, _action: &ActionInvocation| {
    state.count += 1;
    Ok(())
});
```

`RsxComponent::from_file`, `RsxTemplate::from_file`, and
`parse_rsx_file` are available for development tools that read `.rsx` files from
disk at runtime. `include_str!` remains the recommended production path because
the RSX source is bundled into the Rust binary.

The hook names are intentionally familiar to React developers, but in A3S they
live on `ComponentCx` and execute as Rust registrations, not JavaScript:

- `cx.use_state(path, selector)` exposes a serializable Rust state selector as
  `state.<path>` in RSX. Pair it with `cx.use_value_reducer(...)` when a native
  control needs a setter-like action.
- `cx.use_state_result(path, selector)` is the fallible typed form for state
  selectors that can fail while preparing a render.
- `cx.use_prop(path, selector)` exposes derived component props as
  `props.<path>`.
- `cx.use_press(selector)` exposes semantic press data as
  `props.pressProps` and `props.isPressed`.
- `cx.use_button(selector)` composes button semantics on top of the same
  press-data shape.
- `cx.use_prop_result(path, selector)` is the fallible typed form for derived
  props.
- `cx.use_derived(path, selector)` exposes computed page state as
  `derived.<path>`.
- `cx.use_derived_result(path, selector)` is the fallible typed form for
  computed page state.
- `cx.use_context(path, selector)` exposes read-only environment data, such as
  theme, workspace, session, or feature flags, as `context.<path>`.
- `cx.use_context_result(path, selector)` is the fallible typed form for
  read-only environment data.
- `cx.use_resource(path, selector)` exposes a standard resource state as
  `resource.<path>`, including `status`, `data`, `error`, `isLoading`,
  `isReady`, and `isError`.
- `cx.use_resource_result(path, selector)` is the fallible typed form for
  resources that may fail while preparing a render.
- `cx.use_memo(path, selector)` is an authoring alias for `cx.use_derived`;
  selectors are still Rust functions evaluated during render, not JavaScript
  hooks.
- `cx.use_reducer(action, handler)` registers the native action handler that
  mutates Rust state after a native event.
- `cx.use_value_reducer(action, handler)` decodes the native event value into a
  typed Rust argument before running the reducer. Use it for controlled text
  fields, sliders, selections, and toggles.
- `cx.use_payload_reducer(action, handler)` decodes `actionValue` /
  `actionPayload` into a typed Rust payload before running the reducer.
- `cx.use_action_disabled(action, selector)` and
  `cx.use_action_enabled(action, selector)` expose state-derived command
  availability. Disabled actions remain registered for menus, buttons, and
  shortcuts, but the runtime rejects them before the reducer mutates state.
- `cx.use_mount(handler)` runs synchronous Rust initialization when the component
  is mounted into a runtime app. Use it for restoring local state, seeding a
  loading resource, or initializing a page-scoped counter before the first native
  frame.
- `cx.use_mount_result(handler)` is the fallible form for initialization that
  may fail, such as restoring serialized page state or validating a persisted
  workspace snapshot. Prefer `try_into_protocol_app` or `try_into_runtime_app`
  when a component registers fallible mount hooks so the error is returned
  before the first frame is built.
- `cx.use_unmount(handler)` runs synchronous Rust cleanup when a component is
  explicitly unmounted or when a router leaves that page. Use it for clearing
  page-local selections, flushing small state snapshots, or releasing host-side
  handles tracked in app state.
- `cx.use_unmount_result(handler)` is the fallible cleanup form. Router
  transitions return this error before the next page's mount hooks run.
- `cx.use_effect(handler)` runs deterministic state effects after a reducer
  succeeds and before the next native render.
- `cx.use_action_effect(action, handler)` scopes an effect to one action id.
- `cx.use_value_effect(action, handler)` and
  `cx.use_payload_effect(action, handler)` decode action values or payloads
  before running typed post-reducer effects.

The lower-level `RsxComponent` template API still exposes some advanced hooks
used by router internals and older tests:

- `use_memo_result(path, selector)` is the fallible typed alias for
  `use_derived_result`.
- `use_state_value`, `use_prop_value`, `use_derived_value`,
  `use_context_value`, and `use_memo_value` are the raw `serde_json::Value`
  forms for selectors that already produce a structured render value.
- `use_field(path, action, selector, reducer)` registers the state selector and
  the typed value reducer for a controlled field in one hook.
- `use_transition_reducer(action, reducer, effect)` runs one action reducer with
  access to a `RsxActionTransition` containing the state before the reducer. Use
  it when post-reducer logic needs to compare previous and current state.
- `use_value_transition_reducer` and `use_payload_transition_reducer` are the
  typed value and payload forms of transition reducers.
- `use_hooks(bundle)` and `try_use_hooks(bundle)` compose reusable groups of
  selectors, reducers, effects, and registered templates.
- `use_transition_effect(handler)` and
  `use_action_transition_effect(action, handler)` run post-reducer effects with
  access to a `RsxActionTransition` containing the state before the reducer.
- `use_value_transition_effect` and `use_payload_transition_effect` are the
  typed value and payload forms of transition effects.

Hook registrations are intentionally strict:

- A `state.*`, `props.*`, `derived.*`, `context.*`, or `resource.*` hook path may be
  registered only once in its scope object.
- Non-conditional `state.*`, root `props.*`, `derived.*`, `context.*`, and
  `resource.*` template bindings must be covered by a matching hook path. A hook
  can expose either the exact leaf, such as `use_state("profile.title", ...)`,
  or an ancestor object, such as `use_state("profile", ...)`.
- Fallible selector hooks return their `GuiError` before the native frame is
  rendered. They do not partially update the template scope.
- Each native action id may have only one reducer/action hook. `use_action`,
  `use_reducer`, `use_field`, `use_value_reducer`, and `use_payload_reducer`
  all share the same action namespace.
- Static action references in the RSX template, registered component templates,
  and window options must have a matching reducer/action hook.
- Duplicate action hooks are rejected before render or direct reducer dispatch
  so hook bundles cannot silently overwrite each other.
- Action disabled/enabled hooks must target a registered action id and may be
  registered only once per action.
- Multiple effects may observe the same action id because effects are
  post-reducer observers, not action owners.
- Action-scoped effects must observe a registered action id; typos are rejected
  before render or direct reducer dispatch.

Use `validate()` as a preflight check after composing hooks and before handing a
component to an application shell. `try_into_protocol_app` and
`try_into_runtime_app` run the same preflight before mounting, then return any
`use_mount_result` error before the app is handed to the host:

```rust
counter.validate()?;
let mut app = counter.try_into_protocol_app(Gtk4Adapter, CounterState::default())?;
```

The component can be mounted into the existing protocol loop:

```rust
use a3s_gui::Gtk4Adapter;

let mut app = counter.into_protocol_app(Gtk4Adapter, CounterState::default());
let rendered = app.render()?;
```

or into an embedded native runtime host with `into_runtime_app`. Non-`try`
constructors keep their infallible signatures for simple app shells; if a
fallible mount hook fails, the first `render()` returns that mount error instead
of panicking or silently dropping it. In both cases, the flow remains the same:

```text
mount hooks -> Rust state -> state/prop/derived/context/resource selectors
-> RSX bindings -> native frame -> native action -> reducer hook -> effect hooks
-> Rust state -> rerender
```

`context.*` participates in render like state and derived data, but reducers do
not mutate it directly. Treat context as read-only host or session input that can
be refreshed by the outer application state before the next render.

### Resource Hooks

Use `RsxResource` for page data that has loading and error states. A resource
hook is still a Rust selector; it does not run async JavaScript in the template:

```rsx
<Toolbar key="profile" orientation="vertical">
  <Show key="loading" when={resource.profile.isLoading}>
    <Text key="loading-text" label="Loading profile" />
  </Show>
  <Show key="ready" when={resource.profile.isReady}>
    <Text key="name" label={resource.profile.data.name} />
  </Show>
  <Show key="error" when={resource.profile.isError}>
    <Text key="error-text" label={resource.profile.error} />
  </Show>
</Toolbar>
```

```rust
let profile = RsxComponent::new("profile", source)?
    .use_resource("profile", |state: &ProfileState| {
        match &state.profile {
            ProfileLoad::Loading => RsxResource::loading(),
            ProfileLoad::Ready(profile) => RsxResource::ready(profile.clone()),
            ProfileLoad::Failed(error) => RsxResource::failed(error.clone()),
        }
    });
```

### Page Router

Use `RsxRouter` when a desktop surface has multiple screens. The active page is
selected from Rust state; page actions mutate that state, then the next render
selects the new page. There is no browser URL parser or JavaScript router hidden
inside RSX.

```rust
let router = RsxRouter::new(|state: &AppState| state.route.clone())
    .layout(app_shell_component)
    .route("home", home_component)?
    .route("settings", settings_component)?
    .default_route("home")
    .use_route_context("title", |state: &AppState, route| {
        state.route_title(route)
    })
    .use_route_transition_effect(|state: &mut AppState, transition| {
        state.record_navigation(
            transition.from(),
            transition.to(),
            transition.action(),
        );
        Ok(())
    });

let mut app = router.try_into_runtime_app(host, AppState::default())?;
```

Use an optional layout component when the desktop surface has persistent app
chrome such as navigation, command bars, sidebars, or status areas. A router
layout renders once around the active route and must declare exactly one route
outlet with `<Slot />` or `<Slot name="route" />`:

```rsx
<Toolbar key="root" orientation="vertical">
  <Toolbar key="chrome" orientation="horizontal">
    <Button key="home" label="Home" onPress={openHome} />
    <Text key="route" label={context.route.title} />
  </Toolbar>
  <Slot key="content" name="route" />
</Toolbar>
```

Layout actions are registered together with the active route's actions. The
router dispatches layout actions to the layout component and page actions to the
active route component; either reducer may update the selected route. Layout and
route action ids must not collide because native action ids are app-global within
the rendered frame.

The router automatically provides `context.route.id` to both the layout and the
active page. Use `use_route_context(path, selector)` for additional app-shell
values exposed under `context.route.<path>`:

```rsx
<Toolbar key="root" orientation="vertical">
  <Text key="route" label={context.route.id} />
  <Text key="title" label={context.route.title} />
</Toolbar>
```

Route context selectors are Rust functions. They receive the app state and the
active route id, so route-specific labels, breadcrumbs, permissions, and layout
metadata stay in the same one-way data-flow model as `state.*` and
`derived.*`. The `id` route context field is reserved for the active route id.

Each route is a normal `RsxComponent` with its own template, selectors, resources,
actions, effects, and registered child components. Without a router layout, only
the active page's actions are registered with the native runtime. With a layout,
the runtime receives layout actions plus actions from the active page:

```rust
let home = RsxComponent::new("home", home_source)?
    .use_action("openSettings", |state: &mut AppState, _action| {
        state.route = "settings".to_string();
        Ok(())
    })
    .use_unmount(|state: &mut AppState| {
        state.clear_home_selection();
    });

let settings = RsxComponent::new("settings", settings_source)?
    .use_mount_result(|state: &mut AppState| {
        state.settings_resource = SettingsResource::Loading;
        state.restore_settings_view()?;
        Ok(())
    })
    .use_state("title", |state: &AppState| state.settings_title.clone())
    .use_action("renameSettings", rename_settings);
```

Layout mount hooks run once before the first frame. Route mount hooks run for the
initial active route after layout mount. If any layout or route action changes
the selected route, the previous route's unmount hooks run first, then the newly
active route's mount hooks run before that next route is rendered. The layout is
not remounted for route changes.

Router-level `use_route_transition_effect` hooks run after that page lifecycle
sequence and receive an `RsxRouteTransition` containing the previous route id,
the next route id, and the action invocation that caused the transition. Use them
for app-shell concerns such as navigation history, window titles, analytics
events, or route-scoped persistence that should not live inside an individual
page reducer. `use_route_effect` remains available as a legacy adapter when the
older `(from, to, action)` callback shape is enough.

Route lifecycle hooks are synchronous Rust hooks; represent long-running work by
updating state to a loading resource and completing it from the host side with
normal actions. Fallible route lifecycle and route-effect hooks propagate through
`try_into_protocol_app`, `try_into_runtime_app`, and route-changing action
dispatch, so a failed page cleanup, restore, or transition effect can stop the
next frame before it is committed.

### Hook Bundles

Large screens should group page logic by feature instead of growing one long
function body. Prefer small functions that receive `&mut ComponentCx<S>` and
register selectors, semantic hooks, actions, and effects for one concern.
Low-level `RsxComponent::use_hooks` remains useful for design-system presets and
router modules that register component templates.

```rust
fn profile_form_hooks(component: RsxComponent<ProfileState>) -> RsxComponent<ProfileState> {
    component
        .use_field(
            "email",
            "setEmail",
            |state: &ProfileState| state.email.clone(),
            |state: &mut ProfileState, email: String| {
                state.email = email;
                Ok(())
            },
        )
        .use_derived("summary", |state: &ProfileState| {
            format!("{} changes", state.change_count)
        })
        .use_value_effect("setEmail", |state: &mut ProfileState, _email: String| {
            state.change_count += 1;
            Ok(())
        })
}

let form = RsxComponent::new("profile", source)?
    .use_hooks(profile_form_hooks);
```

Fallible bundles can register reusable RSX templates:

```rust
fn command_row_hooks(component: RsxComponent<ListState>) -> GuiResult<RsxComponent<ListState>> {
    Ok(component
        .use_component("CommandRow", command_row_source)?
        .use_state("items", |state: &ListState| state.items.clone())
        .use_payload_reducer("selectItem", |state: &mut ListState, item: ItemPayload| {
            state.selected_id = Some(item.id);
            Ok(())
        }))
}

let list = RsxComponent::new("commands", source)?
    .try_use_hooks(command_row_hooks)?;
```

Routers have the same composition shape for app-shell route modules:

```rust
fn settings_routes(router: RsxRouter<AppState>) -> GuiResult<RsxRouter<AppState>> {
    Ok(router
        .route("settings", settings_component())?
        .use_route_transition_effect(|state: &mut AppState, transition| {
            state.record_navigation(
                transition.from(),
                transition.to(),
                transition.action(),
            );
            Ok(())
        }))
}

let router = RsxRouter::new(|state: &AppState| state.route.clone())
    .route("home", home_component())?
    .try_use_routes(settings_routes)?
    .use_routes(|router| router.default_route("home"));
```

### Action Effects

Action effects are native, deterministic post-reducer hooks. They are useful for
state bookkeeping that should happen after one or more actions, such as updating
history, clearing dependent fields, or maintaining counters. They do not run
after every render and they do not execute JavaScript.

```rust
let component = RsxComponent::new("counter", source)?
    .use_action("increment", |state: &mut CounterState, _action| {
        state.count += 1;
        Ok(())
    })
    .use_effect(|state: &mut CounterState, action: &ActionInvocation| {
        state.last_action = Some(action.action.clone());
        Ok(())
    })
    .use_action_effect("increment", |state: &mut CounterState, _action| {
        state.increment_count += 1;
        Ok(())
    });
```

Effects run in registration order after the matching action reducer succeeds.
If an effect returns an error, the next render is skipped and the error is
reported to the runtime caller. Keep rendering data derivation in `use_derived`
or `use_memo`; use action effects only when state itself must change.
An action-scoped effect is validated against registered reducer/action hooks, so
`use_action_effect("saveDocment", ...)` fails early instead of becoming a silent
observer typo.

Typed effects mirror typed reducers. They keep post-reducer logic close to the
action contract without manually inspecting `ActionInvocation`:

```rust
let form = RsxComponent::new("profile", source)?
    .use_field(
        "email",
        "setEmail",
        |state: &ProfileState| state.email.clone(),
        |state: &mut ProfileState, email: String| {
            state.email = email;
            Ok(())
        },
    )
    .use_value_effect("setEmail", |state: &mut ProfileState, email: String| {
        state.audit_message = format!("Email changed to {email}");
        Ok(())
    });
```

Use `use_payload_effect` for actions carrying `actionPayload`, such as list row
selection, command palettes, and menus with structured item metadata.

Transition effects are for existing reducers that need before/after state. They
capture the state before the reducer, run after that reducer succeeds, and keep
their registration order relative to normal effects:

```rust
let counter = RsxComponent::new("counter", source)?
    .use_action("increment", |state: &mut CounterState, _action| {
        state.count += 1;
        Ok(())
    })
    .use_action_transition_effect(
        "increment",
        |state: &mut CounterState, transition: &RsxActionTransition<'_, CounterState>| {
            state.audit = format!(
                "{} changed {} -> {}",
                transition.action(),
                transition.before().count,
                state.count,
            );
            Ok(())
        },
    );
```

Transition effects require `S: Clone` because RSX snapshots the previous state.
Use `use_value_transition_effect` and `use_payload_transition_effect` when the
effect should decode the same typed action value or payload as the reducer.

### Transition Reducers

Use transition reducers when one action needs both a reducer and transition-aware
post-reducer state bookkeeping. The reducer receives the current mutable state.
The transition effect runs immediately after that reducer succeeds, with the
same mutable state now representing the after state and `transition.before()`
representing the state snapshot from before the reducer.

```rust
let counter = RsxComponent::new("counter", source)?
    .use_transition_reducer(
        "increment",
        |state: &mut CounterState, _action| {
            state.count += 1;
            Ok(())
        },
        |state: &mut CounterState, transition: &RsxActionTransition<'_, CounterState>| {
            state.audit = format!(
                "{} changed {} -> {}",
                transition.action(),
                transition.before().count,
                state.count,
            );
            Ok(())
        },
    );
```

Transition reducers are still normal action hooks, so they participate in the
same validation, labels, disabled checks, and native action registry as
`use_reducer`. They require `S: Clone` because RSX keeps a before-state snapshot
for the transition effect. Use `use_value_transition_reducer` for native event
values and `use_payload_transition_reducer` for structured `actionPayload` data.

### Controlled Values

Controlled fields use one-way data flow: Rust state renders the value, native UI
emits a value event, a reducer updates Rust state, and the next render sends the
new value back to the native control.

```rsx
<TextField
  key="email"
  label="Email"
  value={state.email}
  onChange={setEmail}
/>
```

```rust
let form = RsxComponent::new("profile", source)?
    .use_state("email", |state: &ProfileState| state.email.clone())
    .use_value_reducer("setEmail", |state: &mut ProfileState, email: String| {
        state.email = email;
        Ok(())
    });
```

For the common controlled field shape, use `use_field` to register the getter
and setter-style reducer together:

```rust
let form = RsxComponent::new("profile", source)?
    .use_field(
        "email",
        "setEmail",
        |state: &ProfileState| state.email.clone(),
        |state: &mut ProfileState, email: String| {
            state.email = email;
            Ok(())
        },
    );
```

`use_labeled_field` accepts the same arguments plus an action label for hosts
that expose native command metadata.

`use_value_reducer` decodes `ActionInvocation::value()` into the requested Rust
type. Plain strings are passed through, while JSON-like scalar values such as
`42`, `3.5`, and `true` can decode into numeric and boolean types. Missing
values produce a clear reducer error because controlled inputs should always
emit the value that changed.

The same hook works for selection, ranged, and boolean controls:

```rsx
<Slider
  key="volume"
  min={0}
  max={100}
  step={5}
  valueNumber={state.volume}
  onChange={setVolume}
/>

<Select
  key="theme"
  label="Theme"
  value={state.theme}
  onSelectionChange={setTheme}
>
  <option key="light" value="light">Light</option>
  <option key="dark" value="dark">Dark</option>
</Select>

<Switch
  key="notifications"
  label="Notifications"
  isChecked={state.notifications}
  onChange={setNotifications}
/>
```

Checked controls normalize toggle/change events to `true` or `false` before
the reducer runs, so a boolean value reducer can stay small:

```rust
let settings = RsxComponent::new("settings", source)?
    .use_state("notifications", |state: &SettingsState| state.notifications)
    .use_value_reducer(
        "setNotifications",
        |state: &mut SettingsState, enabled: bool| {
            state.notifications = enabled;
            Ok(())
        },
    );
```

Keep arbitrary JavaScript handler bodies out of RSX. Instead of
`onChange={(event) => setEmail(event.target.value)}`, bind `onChange={setEmail}`
and put the state transition in a Rust value reducer.

### Registered Components

RSX supports reusable native subtemplates through the Rust component layer. The
parent template references a PascalCase tag and Rust registers the RSX template
for that tag:

```rsx
<Toolbar key="commands" orientation="vertical">
  <For key="items" each={state.items} as="item" keyBy="id">
    <CommandRow
      key="row"
      title={item.title}
      selected={item.visible}
      onPress={selectItem}
      actionPayload={item}
    />
  </For>
</Toolbar>
```

```rsx
<Button
  key="root"
  onPress={props.onPress}
  isSelected={props.selected}
  actionPayload={props.actionPayload}
>
  {props.title}
</Button>
```

```rust
let commands = RsxComponent::new("commands", parent_source)?
    .use_component("CommandRow", row_source)?
    .use_state("items", |state: &ListState| state.items.clone())
    .use_reducer("selectItem", select_item);
```

Registered components are expanded before native rendering. Props are resolved
in the parent scope, exposed as `props.*` in the child template, and the
expanded subtree receives stable keys based on the component invocation key.
Event props such as `onPress={selectItem}` can be forwarded with
`onPress={props.onPress}`; they remain native action ids rather than JavaScript
closures.

Component names are registered once. If two hook bundles both register
`CommandRow`, the second registration fails instead of silently replacing the
first template.

For reusable components with a stable public surface, register a prop contract:

```rust
let commands = RsxComponent::new("commands", parent_source)?
    .use_component_with_contract(
        "CommandRow",
        row_source,
        RsxComponentContract::new()
            .required(["title", "onPress"])
            .optional(["selected", "actionPayload"])
            .default_prop("tone", "neutral")?,
    )?
    .use_state("items", |state: &ListState| state.items.clone())
    .use_reducer("selectItem", select_item);
```

Contracts are checked by `validate()`, `render()`, `reduce()`, and the
`try_into_*` mount helpers. Required props must be written at the component
call site, and closed contracts reject unknown props such as `titel="..."`.
Use `allow_extra_props()` for pass-through or wrapper components that accept
additional `data-*`, styling, or host-specific props.

Use contract default props for design-system components with stable defaults.
Defaults are applied before component invocation props are resolved, so explicit
props and spread props still override them:

```rsx
<Badge key="status" />
<Badge key="danger" tone="danger" />
```

```rust
let component = RsxComponent::new("badges", source)?
    .use_component_with_contract(
        "Badge",
        r#"<Text key="root" label={props.tone} />"#,
        RsxComponentContract::new()
            .default_prop("tone", "neutral")?,
    )?;
```

A defaulted prop is part of the component contract and can be used by the child
template without requiring every call site to repeat it.

When a registered component has a contract, its template `props.*` bindings must
also be declared in that contract. For example, `props.title` is valid when
`title` is required or optional, while `props.detail` is rejected until the
contract declares `detail`.

Registered components also support structural slots. Parent children are
resolved in the parent scope, then inserted wherever the registered template
declares `<Slot />`:

```rsx
<Panel key="panel" title={state.title}>
  <Text key="body" label={state.message} />
  <Button key="save" slot="footer" onPress={saveDocument}>
    Save
  </Button>
</Panel>
```

```rsx
<Toolbar key="root" orientation="vertical">
  <Text key="title" label={props.title} />
  <Slot key="content" />
  <Toolbar key="footer" orientation="horizontal">
    <Slot key="footer-items" name="footer" />
  </Toolbar>
</Toolbar>
```

Unmarked direct children go to the default slot. Direct children with
`slot="footer"` go to `<Slot name="footer" />`. The structural `slot` attribute
is removed from slotted children before native rendering; outside registered
component expansion, `slot` remains a normal HTML/native attribute.

`<Slot />` is structural only while a registered component template is being
expanded. Outside that context it remains the native slot element. Slot content
keeps lexical parent scope, receives stable keys based on the component
invocation key and slot key, and does not become a JavaScript `props.children`
value.

This is intentionally not a Rust or JavaScript function runtime inside the RSX
file. View files use `fn ComponentView(props: ComponentViewProps) -> RSX` so the
UI is easy to read, but hooks and dynamic logic run in Rust `ComponentCx`
functions; pass data through props and slots, and keep page logic in Rust
selectors, reducers, effects, resources, and registered actions.

### Action Values

RSX does not execute event handler closures such as
`onPress={() => selectItem(item.id)}`. Use `actionValue` to attach a scalar
native value, or `actionPayload` to attach a JSON payload, to the action
invocation instead:

```rsx
<For key="items" each={state.items} as="item" keyBy="id">
  <Button key="select" onPress={selectItem} actionValue={item.id}>
    {item.title}
  </Button>
</For>
```

The reducer receives the value through `ActionInvocation.value`:

```rust
let list = RsxComponent::new("items", source)?
    .use_state("items", |state: &ListState| state.items.clone())
    .use_reducer("selectItem", |state: &mut ListState, action: &ActionInvocation| {
        state.selected_id = action.value.clone();
        Ok(())
    });
```

Native event values, such as text input changes, take precedence over static
`actionValue`. Static action values are therefore best for buttons, menu items,
tabs, and list rows that need to identify the item being acted on.

Action availability belongs in Rust state selectors, not inline JavaScript. Use
the same selector for visual disabled bindings and command metadata when they
should move together:

```rsx
<Button key="save" onPress={saveDocument} isDisabled={derived.saveDisabled}>
  Save
</Button>
```

```rust
let editor = RsxComponent::new("editor", source)?
    .use_derived("saveDisabled", |state: &EditorState| !state.has_changes)
    .use_action("saveDocument", save_document)
    .use_action_disabled("saveDocument", |state: &EditorState| !state.has_changes);
```

For richer event arguments, bind `actionPayload` to a serializable object. The
typed payload reducer hook decodes it before running the reducer:

```rsx
<For key="items" each={state.items} as="item" keyBy="id">
  <Button key="select" onPress={selectItem} actionPayload={item}>
    {item.title}
  </Button>
</For>
```

```rust
#[derive(serde::Deserialize)]
struct ItemPayload {
    id: String,
    title: String,
}

let list = RsxComponent::new("items", source)?
    .use_state("items", |state: &ListState| state.items.clone())
    .use_payload_reducer("selectItem", |state: &mut ListState, item: ItemPayload| {
        state.selected_id = Some(item.id);
        Ok(())
    });
```

`use_payload_reducer` also works with scalar `actionValue`, for example
`actionValue={item.id}` can decode into `String`. `ActionInvocation::payload()`
and `ActionInvocation::payload_json()` remain available when reducers need
optional payloads, event kind inspection, node ids, or dynamic JSON handling.

## Design System Components

`rsx_ui` provides a React-inspired Rust component set backed by the Vercel/Geist
tokens in the repository root `DESIGN.md`. Built-in components are available by
default when a page component is created with `RsxComponent::new`,
`RsxComponent::from_source`, `RsxComponent::from_file`, or
`RsxComponent::from_template`:

```rust
use a3s_gui::RsxComponent;

let page = RsxComponent::from_source(
    "settings",
    "ui/settings.rsx",
    include_str!("ui/settings.rsx"),
)?
.use_state("email", |state: &SettingsState| state.email.clone())
.use_value_reducer("setEmail", set_email)
.use_reducer("saveProfile", save_profile);
```

The default component set includes:

- `UiButton`
- `UiBadge`
- `UiAutocomplete`
- `UiBreadcrumbs`
- `UiBreadcrumb`
- `UiCard`
- `UiCardHeader`
- `UiCardTitle`
- `UiCardDescription`
- `UiCardContent`
- `UiCardFooter`
- `UiCheckbox`
- `UiCheckboxGroup`
- `UiComboBox`
- `UiDialog`
- `UiDisclosure`
- `UiDisclosureGroup`
- `UiDisclosureSummary`
- `UiDropZone`
- `UiFieldSet`
- `UiFileTrigger`
- `UiForm`
- `UiGridList`
- `UiGridListItem`
- `UiGroup`
- `UiHeading`
- `UiInput`
- `UiLabel`
- `UiLegend`
- `UiLink`
- `UiListBox`
- `UiListBoxItem`
- `UiMenu`
- `UiMenuItem`
- `UiMeter`
- `UiModal`
- `UiNumberField`
- `UiPopover`
- `UiProgressBar`
- `UiRadio`
- `UiRadioGroup`
- `UiSearchField`
- `UiSelect`
- `UiSelectValue`
- `UiSeparator`
- `UiSlider`
- `UiSwitch`
- `UiTagGroup`
- `UiTag`
- `UiTable`
- `UiTableHeader`
- `UiTableBody`
- `UiTableRow`
- `UiTableColumn`
- `UiTableCell`
- `UiTableCaption`
- `UiTabs`
- `UiTabsList`
- `UiTabsTrigger`
- `UiTabsContent`
- `UiText`
- `UiTextField`
- `UiTextarea`
- `UiToastRegion`
- `UiToast`
- `UiToolbar`
- `UiToggleButton`
- `UiToggleButtonGroup`
- `UiTooltip`
- `UiTree`
- `UiTreeItem`
- `UiTreeItemContent`
- `UiVirtualizer`

Example:

```rsx
<UiCard key="card" className="w-full">
  <UiCardHeader key="header">
    <UiCardTitle key="title">Settings</UiCardTitle>
    <UiCardDescription key="description">
      Native RSX controls
    </UiCardDescription>
  </UiCardHeader>
  <UiCardContent key="content">
    <UiInput
      key="email"
      value={state.email}
      placeholder="Email"
      onChange={setEmail}
    />
  </UiCardContent>
  <UiCardFooter key="footer">
    <UiButton
      key="save"
      variant="secondary"
      size="sm"
      onPress={saveProfile}
    >
      Save
    </UiButton>
    <UiBadge key="status" variant="outline">
      Preview
    </UiBadge>
  </UiCardFooter>
</UiCard>
```

Tabs use shadcn-style visual wrappers while preserving native tab semantics:

```rsx
<UiTabs key="settings-tabs" value={state.tab} onSelectionChange={setTab}>
  <UiTabsList key="list" className="grid w-full grid-cols-2">
    <UiTabsTrigger
      key="profile-trigger"
      value="profile"
      isSelected={state.profileSelected}
    >
      Profile
    </UiTabsTrigger>
    <UiTabsTrigger
      key="billing-trigger"
      value="billing"
      isSelected={state.billingSelected}
    >
      Billing
    </UiTabsTrigger>
  </UiTabsList>
  <UiTabsContent key="profile-panel" value="profile">
    Profile settings
  </UiTabsContent>
  <UiTabsContent key="billing-panel" value="billing">
    Billing settings
  </UiTabsContent>
</UiTabs>
```

Select follows a React-like composition model: the label, trigger, selected
value, popover, list box, and options are separate RSX components that lower to
one native select control.

```rsx
<UiSelect key="density" value={state.density} onSelectionChange={setDensity}>
  <UiLabel key="density-label">Density</UiLabel>
  <UiButton key="density-trigger" variant="outline" onPress={openDensity}>
    <UiSelectValue
      key="density-value"
      value={state.density}
      placeholder="Density"
    />
  </UiButton>
  <UiPopover key="density-popover">
    <UiListBox key="density-options">
      <UiListBoxItem key="compact" value="compact" textValue="Compact">
        Compact
      </UiListBoxItem>
      <UiListBoxItem
        key="comfortable"
        value="comfortable"
        textValue="Comfortable"
      >
        Comfortable
      </UiListBoxItem>
    </UiListBox>
  </UiPopover>
</UiSelect>
```

ComboBox and Table follow the same composed native pattern:

```rsx
<UiComboBox
  key="assignee"
  label="Assignee"
  value={state.assignee}
  inputValue={state.assigneeQuery}
  onChange={setAssigneeQuery}
  onSelectionChange={setAssignee}
>
  <UiPopover key="assignee-popover">
    <UiListBox key="assignee-list">
      <UiListBoxItem key="ada" value="ada" textValue="Ada">
        Ada
      </UiListBoxItem>
    </UiListBox>
  </UiPopover>
</UiComboBox>

<UiTable key="members" label="Members">
  <UiTableHeader key="header">
    <UiTableRow key="header-row">
      <UiTableColumn key="name-column">Name</UiTableColumn>
    </UiTableRow>
  </UiTableHeader>
  <UiTableBody key="body">
    <UiTableRow key="ada-row">
      <UiTableCell key="ada-name">Ada</UiTableCell>
    </UiTableRow>
  </UiTableBody>
</UiTable>
```

Structural, collection, file, and notification primitives stay in the same
one-way data model:

```rsx
<UiBreadcrumbs key="path" label="Project path">
  <UiBreadcrumb key="home" href="/" onPress={openHome}>Home</UiBreadcrumb>
  <UiBreadcrumb key="repo" href="/repo" onPress={openRepo}>A3S</UiBreadcrumb>
</UiBreadcrumbs>

<UiGridList key="files" label="Files" onSelectionChange={selectFile}>
  <UiGridListItem key="main" value="main" textValue="main.rs">
    main.rs
  </UiGridListItem>
</UiGridList>

<UiFileTrigger
  key="import"
  acceptedFileTypes=".rsx,.rs"
  onSelect={importFiles}
  allowsMultiple
>
  Import
</UiFileTrigger>

<UiToastRegion key="toasts">
  <UiToast key="saved" title="Saved" onClose={closeToast} />
</UiToastRegion>
```

Forms and feedback primitives follow the same model. The component files keep
logic in Rust hooks, props as data, and the view as a semantic RSX tree:

```rsx
<UiForm key="profile" label="Profile" onSubmit={saveProfile}>
  <UiHeading key="title" level="2">Profile</UiHeading>
  <UiTextField
    key="name"
    label="Name"
    value={state.name}
    placeholder="Name"
    onChange={setName}
  />
  <UiToolbar key="actions" label="Actions">
    <UiLink key="docs" href="/docs" onPress={openDocs}>Docs</UiLink>
    <UiButton key="save" onPress={saveProfile}>Save</UiButton>
  </UiToolbar>
  <UiProgressBar
    key="sync"
    label="Sync"
    valueNumber={state.syncProgress}
    minValue="0"
    maxValue="100"
  />
</UiForm>
```

These are Rust function components in `.rsx` source modules, written with
`ComponentCx` hooks and `rsx!` views. They do not execute React components. Each
component owns a static shadcn/Vercel base class and merges a caller-provided
`className` onto that base class, matching the useful part of shadcn's
`cn(base, className)` pattern without JavaScript.

The visual wrappers lower to the semantic layer before reaching the native host.
For example, `UiTabs`, `UiTabsList`, `UiTabsTrigger`, and `UiTabsContent` expand
to `Tabs`, `TabList`, `Tab`, and `TabPanel`, then the semantic mapper projects
them to native tab roles.

Variant classes are registered in Rust with `ComponentClassVariants`, not by
running JavaScript `cva` helpers. `UiButton` currently supports `variant`
(`default`, `secondary`, `outline`, `ghost`, `link`, `destructive`) and `size`
(`default`, `sm`, `lg`, `icon`). `UiBadge` supports `variant` (`default`,
`secondary`, `destructive`, `outline`). Unknown variant values fail during RSX
rendering so invalid component states are caught before they reach the native
host.

## Tailwind

RSX preserves Tailwind-style `class` / `className` strings and the Rust style
pipeline parses the portable static subset into `PortableStyle`. Common layout,
spacing, typography, border, radius, color, ring, shadow, and arbitrary-value
utilities are supported.

```rsx
<div
  key="root"
  class="min-w-[920px] bg-background text-foreground p-3"
>
  <button
    key="save"
    class="rounded-md border border-border bg-primary text-primary-foreground px-3"
    onclick={saveDocument}
  >
    Save
  </button>
</div>
```

The parser also keeps Tailwind variants as native style variant declarations.
This covers the shadcn patterns used by Button, Input, Card, Dialog, and
Dropdown Menu components:

- State variants such as `hover:*`, `focus-visible:*`, `disabled:*`, and
  `aria-invalid:*`.
- Theme variants such as `dark:*`.
- Attribute variants such as `data-[state=open]:*`.
- Structural variants such as `has-[>svg]:*` and arbitrary selector variants
  such as `[&_svg]:*`.
- Arbitrary values such as `ring-[3px]`, `transition-[color,box-shadow]`, and
  `w-[calc(100%_-_2rem)]`.
- Tailwind container width tokens such as `sm:max-w-lg`.
- Common `tailwindcss-animate` shadcn classes such as `animate-in`,
  `animate-out`, `fade-in-0`, `fade-out-0`, `zoom-in-95`, `zoom-out-95`, and
  `slide-in-from-top-2`.

```rsx
<button
  key="save"
  class="
    inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md
    bg-primary text-primary-foreground px-4 py-2 text-sm font-medium shadow-xs
    transition-[color,box-shadow] outline-none
    hover:bg-primary/90 disabled:pointer-events-none disabled:opacity-50
    focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]
    aria-invalid:border-destructive aria-invalid:ring-destructive/20
    [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4
  "
  onclick={saveDocument}
>
  Save
</button>
```

Those variants are not browser CSS selectors. They are preserved as explicit
style metadata so native hosts can apply supported states without embedding a
DOM, CSSOM, or React runtime.

The contract is covered by focused style tests for representative shadcn
Button, Input, Card, Dialog, and Dropdown Menu class strings. Radix-style
variants such as `data-[state=open]:*`, `data-[side=bottom]:*`,
`data-[disabled]:*`, and `data-[variant=destructive]:focus:*` are preserved as
variant keys in the native style IR.

## Design Tokens

The RSX style parser supports shadcn-compatible semantic color utilities, backed
by the Vercel/Geist palette in `DESIGN.md`.

| Class token | Vercel value |
| --- | --- |
| `background`, `canvas` | `#fafafa` |
| `foreground`, `ink`, `primary` | `#171717` |
| `card`, `popover`, `elevated` | `#ffffff` |
| `border`, `input`, `hairline` | `#ebebeb` |
| `secondary`, `muted`, `accent` | `#f2f2f2` |
| `body` | `#4d4d4d` |
| `muted-foreground`, `mute` | `#8f8f8f` |
| `faint` | `#a1a1a1` |
| `ring`, `link`, `success` | `#0070f3` |
| `destructive`, `error` | `#ee0000` |

The usual shadcn foreground pairs are also available, including
`primary-foreground`, `card-foreground`, `popover-foreground`,
`secondary-foreground`, `accent-foreground`, `destructive-foreground`, and the
`sidebar-*` token family.

Use these names with Tailwind color prefixes:

- `bg-background`
- `text-foreground`
- `border-border`
- `ring-ring`
- `bg-card`
- `text-muted-foreground`
- `bg-sidebar`
- `border-sidebar-border`

Opacity modifiers work with semantic colors, so shadcn classes such as
`bg-primary/90`, `ring-ring/50`, and `aria-invalid:ring-destructive/20` resolve
to native RGBA colors from the same Vercel palette.

## Rust API

```rust
use a3s_gui::UiFrame;

let frame = UiFrame::from_rsx_source("desktop", include_str!("ui/app.rsx"))?;
```

`parse_rsx` returns a `CompiledRsxNode` when lower-level access is needed. Use
`parse_rsx_source("ui/app.rsx", source)` when parser and lowering errors should
report the original `.rsx` file name. Use `RsxTemplate::parse_source` or
`RsxComponent::from_source` for app code that keeps page templates and Rust page
logic in separate files.

Reusable component templates can also carry source names:

```rust
use a3s_gui::{RsxComponent, RsxComponentContract};

let page = RsxComponent::from_source(
    "commands",
    "ui/commands.rsx",
    include_str!("ui/commands.rsx"),
)?
.use_component_source_with_contract(
    "CommandRow",
    "ui/components/command-row.rsx",
    include_str!("ui/components/command-row.rsx"),
    RsxComponentContract::new().required(["title", "onPress"]),
)?;
```

The public Rust API uses RSX names only.
