# Native App Shell

A3S GUI apps are reducer-driven native shells. The platform owns windows,
menus, focus, and input events; application code owns state and frame
construction. A shell is production-ready when those two halves can be started,
driven, tested, and stopped through the same lifecycle on each native backend.

## Shell Contract

Use `NativeRuntimeApp` when Rust owns the app state:

1. Build a `UiFrame` from state.
2. Render the frame into a native host.
3. Drain queued native events.
4. Reduce action invocations into state.
5. Rerender after state changes.
6. Stop the loop when the state predicate returns false.

`AppKitRuntimeApp`, `WinUiRuntimeApp`, and `Gtk4RuntimeApp` are type aliases for
that same contract with platform-native hosts. Their `run_*_while` methods render
the first frame if needed, pump platform events, drain queued A3S events, and
stop when either the root window closes or the state predicate returns false.
For embedded hosts and automation, `handle_pending_native_event_batch_while`
returns the same per-event responses plus batch diagnostics for how many native
events were drained, handled, and left buffered when the predicate stopped. The
platform event-pump helpers expose the same diagnostics through
`pump_appkit_event_batch_while`, `pump_gtk4_event_batch_while`, and
`pump_winui_event_batch_while`.

Window close should be modeled as state:

```tsx
createUiFrame('editor', <Editor />, {
  window: {title: 'Editor', onClose: 'closeEditor'},
  actions: [defineAction(createAction('closeEditor'))],
});
```

The reducer should set an exit flag. The platform loop should observe that flag:

```rust
app.run_appkit_while(|state| !state.close_requested)?;
```

This keeps menu close, root-window close, and explicit quit actions on the same
path.

## Local Workflow

Run commands from the `crates/gui` directory.

```bash
just verify
```

`just verify` runs formatting, Rust tests, example tests, platform planning
tests, TypeScript SDK tests, and whitespace checks.

The repository CI runs `just verify` on Linux, then runs host-native AppKit,
GTK4, and WinUI compile checks plus dogfood regression tests on the matching
operating systems. Pushes to `main` and manual workflow runs also build, stage,
validate, and upload compressed unsigned native dogfood bundles for manual
platform QA.

Use the headless dogfood session when changing protocol, reducer, or rendering
logic:

```bash
just dogfood
```

Run the dogfood regression tests when changing app-shell event flow, menus,
dialogs, keyboard routing, or close behavior:

```bash
just dogfood-regression
```

Use the native dogfood app when changing platform behavior:

```bash
just dogfood-native
```

The native recipe selects the matching backend for the current operating system:

| Host | Example |
|------|---------|
| macOS | `appkit_dogfood` with `appkit-native` |
| Linux | `gtk4_dogfood` with `gtk4-native` |
| Windows | `winui_dogfood` with `winui-native` |

For cross-target Windows checks from a configured non-Windows host:

```bash
just check-winui
```

Before handing a native dogfood build to another developer, build and stage a
host-native release artifact:

```bash
just release-native
just bundle-native
```

The staged artifacts and platform prerequisites are documented in
[`packaging.md`](packaging.md).

## Dogfood Coverage

The shared dogfood app exercises the shell features that need to keep working
before A3S GUI is considered generally usable:

- root window metadata, size hints, resize bounds, and `window.onClose`
- menus and menu item actions
- text input and textarea length/sizing hints, initial/rerendered and
  change-event max-length clamping, focus/blur routing, select, slider range
  bounds and step hints, tabs, switches, and checkboxes
- native focus ownership across conditional field removal and later `autoFocus`
  candidates
- overflow-aware root containers that become native scroll viewports
- reducer-driven rerendering after native events
- stale queued native events after reducer-driven node removal
- dialog open state plus native dialog close event routing
- keyboard down, key release, and review shortcut routing
- review gates that disable completion until state is valid and suppress
  disabled completion actions
- state-driven app loop exit after close actions

Headless dogfood tests cover both embedded `NativeRuntimeApp` handling and the
host/process boundary exposed by `NativeProtocolApp`. Native checks ensure the
matching backend compiles and can host the dogfood surface.

## Current Hardening Gaps

The app shell is usable for dogfood and smoke applications. Before treating it as
a stable production surface, keep hardening these areas:

- signed installers and automated app package generation for each product
- broader resize, focus, and text input edge cases under longer real-world forms
- native-platform automation for dogfood menu, dialog, and keyboard interaction
  flows beyond compile-time checks
- WinUI programmatic focus once the underlying safe API is available
