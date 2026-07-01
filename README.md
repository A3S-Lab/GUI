# a3s-gui

`a3s-gui` is the native UI layer for A3S applications. It is designed to render
React Compiler output and React Aria applications directly into
platform-native controls, without a WebView.

The first principle is simple: **Web developers should author native apps with
normal Web-compatible syntax**. Application code should look like React/TSX, use
React Aria components and hooks, and keep familiar props such as `className`,
`style`, `aria-*`, `data-*`, `onClick`, and `onChange`. `a3s-gui` owns the
compiler/runtime bridge that lowers that Web-shaped authoring surface into
native controls.

The project is intentionally split into three layers:

1. **Web-compatible authoring layer**: accepts JSX/TSX-shaped React Aria trees,
   including DOM-style props, inline style objects, aria attributes, data
   attributes, and React-style event prop names with named callbacks.
2. **A3S Native UI IR**: a compact, accessibility-first tree of native roles,
   state, labels, values, Web props, style tokens, events, and children.
3. **Native host adapters**: create and update platform controls such as
   AppKit, WinUI, and GTK widgets.

The current crate contains the React Compiler bridge schema, `GuiRuntime`, core
IR, React Aria mapper, accessibility tree, keyed renderer diff engine, typed
portable style tokens, native event routing, action registration, a headless
host for validation, portable interaction state, and platform planning adapters
that map native IR to AppKit, WinUI, and GTK widget blueprints. The planning
adapters also emit a typed native command stream (`Create`, `Update`,
`InsertChild`, `Remove`, `SetRoot`). OS-bound adapters consume the same
commands to create and mutate real native objects. The command stream,
blueprints, native roles, accessibility roles, and portable style tokens are
serializable, so a Swift/AppKit, WinUI, or GTK process can consume the same
native protocol without seeing JSX or a browser DOM.

`CommandExecutingHost` is the bridge from renderer to backend. It wraps a
platform adapter plus a `PlatformCommandExecutor`, runs reconciliation, and
immediately executes every native command. `DriverCommandExecutor` contains the
shared command interpreter; OS-bound backends implement `NativeWidgetDriver` to
create, update, attach, remove, and set the root native widget on the UI thread.
Drivers that receive native callbacks also implement `NativeEventSource`; the
runtime drains those events and routes them through the same
`InteractionState`/`EventRouter` path as protocol `HostEvent` input.
For real platform bindings, `HandleWidgetDriver` stores thread-affine native
handles returned by a `NativeHandleAdapter`; the core traits intentionally do
not require `Send`, because AppKit, WinUI, and GTK widgets belong to their UI
thread. The feature modules expose `AppKitHandleAdapter`,
`WinUiHandleAdapter`, and `Gtk4HandleAdapter` as the handle-driver entry points
for platform bindings.
Backends that already have direct SDK calls can implement `NativeWidgetSurface`
instead: `SurfaceHandleAdapter` creates the platform object, applies the typed
`NativeWidgetSetter` operations produced from the blueprint/config diff, inserts
native children, removes objects, and sets the root on the UI thread.
The `appkit-native`, `winui-native`, and `gtk4-native` features use this surface
path for real in-process OS widgets.
The default `RecordingBackend` keeps a pure Rust object tree for tests. The
`appkit`, `winui`, and `gtk4` features add platform driver surfaces for macOS,
Windows, and Linux.

The JavaScript/Rust host boundary is explicit: a compiled React tree is submitted
as a `UiFrame`, optionally wrapped in a native `WindowOptions` surface, rendered
into native commands, and native input returns as `HostEvent` ->
`InteractionState` -> `ActionInvocation`.

## Why this is not a WebView

React Aria is a React/DOM accessibility system. A native renderer cannot reuse
DOM nodes, CSS, or browser focus APIs directly. Instead, `a3s-gui` preserves the
semantic contract: roles, labels, disabled/selected/checked state, text values,
orientation, list items, dialog structure, and accessibility metadata are mapped
to native controls and native accessibility roles.

Web syntax is still accepted. `className`, inline `style`, `aria-*`, `data-*`,
HTML state props such as `disabled` and `required`, ranged attributes such as
`min`/`max`/`aria-valuenow`, and DOM-style event props are normalized into the
native IR. Platform adapters then decide how to apply portable style tokens,
register native event handlers, and expose the corresponding accessibility
metadata.

## Minimal Example

```rust
use a3s_gui::{
    AppKitAdapter, AriaComponent, AriaElement, AriaProps, GuiRuntime,
    PlatformPlanningHost,
};

let tree = AriaElement::new("save", AriaComponent::Button)
    .with_props(
        AriaProps::new()
            .label("Save")
            .class_name("primary")
            .style("backgroundColor", "rebeccapurple")
            .dom_attribute("aria-label", "Save document")
            .on_click("saveDocument"),
    );

let host = PlatformPlanningHost::new(AppKitAdapter);
let mut runtime = GuiRuntime::new(host);
runtime.render_aria(&tree)?;
# Ok::<(), a3s_gui::GuiError>(())
```

The Rust example above mirrors the data shape produced by the compiler bridge.
The intended application authoring surface is ordinary TSX:

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {Button} from 'react-aria-components';
import {createAction, createUiFrame, defineAction} from '@a3s-lab/gui';

const saveDocument = createAction('saveDocument', 'Save document');

const root = (
  <Button
    className="primary"
    style={{backgroundColor: 'rebeccapurple'}}
    aria-label="Save document"
    onPress={saveDocument}
  >
    Save
  </Button>
);

export const frame = createUiFrame('save-frame', root, {
  actions: [defineAction(saveDocument)],
});
```

The prototype TypeScript SDK lives in `sdk/typescript`. It provides a zero-deps
JSX runtime and protocol helpers that produce the same `UiFrame` JSON consumed by
the Rust runtime. Event props can be authored as normal named functions or
`createAction` markers; the protocol boundary serializes those callbacks as
stable action ids. The SDK also exports React Aria-shaped component markers for
tests and compiler fixtures, while production authoring can stay compatible with
`react-aria-components`.

The compiler bridge can also consume a structured JSX tree produced after React
Compiler transforms:

```rust
use a3s_gui::{CompiledJsxNode, GuiRuntime, Gtk4Adapter, PlatformPlanningHost};

let compiled: CompiledJsxNode = serde_json::from_str(r#"{
  "kind": "element",
  "key": "save",
  "tag": "Button",
  "props": {
    "className": "primary",
    "events": {"onClick": "saveDocument"}
  },
  "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
}"#)?;

let host = PlatformPlanningHost::new(Gtk4Adapter);
let mut runtime = GuiRuntime::new(host);
runtime.render_compiled(&compiled)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Or submit the same tree as a protocol frame:

```rust
use a3s_gui::{Gtk4Adapter, HostEvent, NativeEvent, NativeEventKind, NativeProtocolSession};

let mut session = NativeProtocolSession::new(Gtk4Adapter);
let rendered = session.render_frame(&frame)?;
for command in &rendered.commands {
    // Apply the native command on the platform UI thread.
}

let response = session.dispatch_host_event(&HostEvent {
    frame_id: rendered.frame_id,
    event: NativeEvent::new(rendered.root, NativeEventKind::Press),
})?;
```

Frames can request a native window surface:

```json
{
  "frameId": "profile",
  "window": {"title": "Profile", "width": 640, "height": 480},
  "root": {"kind": "element", "key": "save", "tag": "Button"}
}
```

Native events route back to Web-authored action ids:

```rust
use a3s_gui::{EventRouter, NativeEvent, NativeEventKind};

let event = NativeEvent::new(root_id, NativeEventKind::Press);
let invocation = EventRouter::new().route(&button_blueprint, &event);
```

Inline styles are preserved and normalized:

```rust
let portable = button_blueprint.portable_style;
assert_eq!(portable.min_width.unwrap().points(), Some(280.0));

let config = button_blueprint.config();
assert!(config.enabled);
assert!(config.visible);

for setter in config.create_setters() {
    // Apply each initial setter to the native control.
}

let patch = previous_config.diff(&config);
for setter in patch.setters() {
    // Apply only changed setters to the native control.
}
```

Backends receive native commands:

```rust
use a3s_gui::{
    CommandExecutingHost, DriverCommandExecutor, HandleWidgetDriver,
    NativeEventSource, NativeHandleAdapter, WinUiAdapter,
};

let native_handles = HandleWidgetDriver::new(MyWinUiAdapter::default()); // implements NativeHandleAdapter
let host = CommandExecutingHost::new(
    WinUiAdapter,
    DriverCommandExecutor::new(native_handles),
);
let mut runtime = GuiRuntime::new(host);
runtime.render_compiled(&compiled)?;
let invocations = runtime.dispatch_pending_native_events()?;
```

On macOS, the `appkit-native` feature includes a minimal real AppKit surface.
It creates `NSWindow`, `NSPanel`, `NSView`, `NSButton`, `NSTextField`,
`NSSwitch`, `NSStackView`, `NSComboBox`, `NSScrollView`, `NSTabView`,
`NSTabViewItem`, `NSBox(separator)`, `NSSlider`, and `NSProgressIndicator`
objects directly from the same React Aria command stream and applies typed
native setters such as label, value, checked, selected, orientation, enabled,
visible, placeholder, ranged min/max/current values, and frame size. It is
intentionally not a WebView wrapper.
Buttons are connected to Objective-C target/action callbacks that enqueue native
press events, which `GuiRuntime::dispatch_pending_native_events()` resolves back
to Web-authored action ids. Editable text fields are connected through an
`NSTextFieldDelegate` and enqueue native focus, change, and blur events, with
change events carrying the current control value. React Aria `onChange`,
`onFocus`, and `onBlur` handlers use the same action registry path. Checkboxes,
switches, and radio buttons use native checked state and enqueue toggle events
with the current boolean value for React Aria `onChange` handlers. Radio groups
use native `NSStackView` containers with `NSButton(radio)` children. Select
controls insert
`ListBoxItem` children into a native `NSComboBox` and enqueue selection-change
events with the selected AppKit item value. Independent React Aria `ListBox`
trees create a native `NSScrollView` containing AppKit row controls, and row
presses enqueue selection-change events with the selected item value. React Aria
`Toolbar` trees create horizontal native `NSStackView` containers for tool
controls. React Aria `Dialog` trees create native `NSPanel` windows with native
content views. React Aria `Popover` trees create native `NSPopover` overlays
with native AppKit content views. React Aria `Tabs` trees fold `TabList` and
ordered `TabPanel` children into native `NSTabViewItem` objects, and
`NSTabViewDelegate` selection changes enqueue native selection-change events.
React Aria `Menu` trees create native `NSMenu` objects with `NSMenuItem`
children; when a menu is the root, the AppKit surface installs it as the
application's main menu. Menu item target/action callbacks enqueue native press
events for Web-authored `onPress`/`onClick` actions. `Separator` creates native
`NSBox` separators. Native sliders enqueue
ranged change events with the current double value; progress indicators are
updated through the same setter-driven ranged state.

On Windows, the `winui-native` feature adds an in-process WinUI 3 surface behind
the same `NativeWidgetSurface` contract. It uses `winio-winui3` and the Windows
App SDK package bootstrap to create real `Microsoft.UI.Xaml.Window`,
`StackPanel`, `TextBlock`, `Button`, `TextBox`, `CheckBox`, `RadioButton`,
`ComboBox`, `ListBox`, `Grid`, `TabView`, `TabViewItem`, `ContentDialog`,
`ToolTip`, menu `StackPanel`/`Button` fallback controls, `Border(separator)`,
`Slider`, and `ProgressBar` objects directly.
It does not enable WebView2 and does not render HTML. WinUI callbacks enqueue
native press, text change, focus, blur, toggle, selection-change, and ranged
change events for the shared `GuiRuntime::dispatch_pending_native_events()`
path. React Aria `Tabs` trees fold `TabList` and ordered `TabPanel` children
into native `TabViewItem` objects whose content is populated with native panel
widgets. `Separator` creates a native XAML `Border` separator without
WebView/WebView2. React Aria `Toolbar` trees create native horizontal
`StackPanel` containers, so toolbar children are still real XAML controls.
React Aria `Dialog` trees create native `ContentDialog` controls with native
XAML content. React Aria `Popover` trees create native ToolTip-backed overlay
surfaces with native XAML content because `winio-winui3` 0.4.2 does not expose
a strong `Flyout` binding. React Aria `Menu` trees create a native XAML
`StackPanel` menu surface with native `Button` menu items because
`winio-winui3` 0.4.2 does not expose strong `MenuFlyout` or `MenuBar` bindings.
React Aria `Switch` currently uses a native CheckBox-backed toggle path
because `winio-winui3` 0.4.2 does not expose `ToggleSwitch`; the planning IR
keeps the `Switch`/`ToggleSwitch` semantic so the binding can be swapped when
the generated API supports it. The feature is compiled only on Windows and
requires a Windows App SDK runtime/development environment.

On Linux, the `gtk4-native` feature adds an in-process GTK4 surface behind the
same `NativeWidgetSurface` contract. It creates `gtk::ApplicationWindow`,
`gtk::Box`, `gtk::Label`, `gtk::Button`, `gtk::Entry`, `gtk::CheckButton`,
`gtk::Switch`, `gtk::DropDown`, `gtk::ListBox`, `gtk::ListBoxRow`,
`gtk::Dialog`, `gtk::Popover`, `gtk::PopoverMenuBar`, `gio::Menu`,
`gio::MenuItem`, `gtk::Notebook`, `gtk::Separator`, `gtk::Scale`, and
`gtk::ProgressBar` objects directly.
React Aria `Menu` trees create `gio::Menu` models with `gio::MenuItem` children
and expose them through native `gtk::PopoverMenuBar`; menu item activation uses
native `gio::SimpleAction` callbacks. React Aria `Tabs` trees become native
`gtk::Notebook` pages with Web-authored tab labels and native panel widgets.
React Aria `Dialog` trees create native `gtk::Dialog` windows with native
content areas. React Aria `Popover` trees create native `gtk::Popover`
overlays with native GTK children. Native GTK callbacks enqueue press, text
change, focus, blur, toggle, and selection-change events for the shared
`GuiRuntime::dispatch_pending_native_events()` path. The feature is compiled
only on Linux and requires GTK4 development libraries plus
`pkg-config`.

```rust
use a3s_gui::{AppKitNativeSurface, GuiRuntime};

let surface = AppKitNativeSurface::new()?;
let host = surface.into_host();
let mut runtime = GuiRuntime::new(host);
runtime.render_compiled(&compiled)?;
# Ok::<(), a3s_gui::GuiError>(())
```

The same embedded-host event path is used by protocol hosts and native AppKit
callbacks: native events update `InteractionState` and resolve through the
registered Web action ids.

The same commands can cross a process or language boundary as JSON:

```json
{
  "type": "create",
  "id": 42,
  "blueprint": {
    "backend": "gtk4",
    "widgetClass": "gtk::Entry",
    "role": "textField",
    "accessibilityRole": "textField",
    "label": "Email",
    "controlState": {
      "placeholder": "you@example.com",
      "disabled": false,
      "required": true,
      "invalid": false,
      "selected": false,
      "checked": null,
      "expanded": null,
      "orientation": null,
      "min": null,
      "max": null,
      "current": null
    },
    "events": {"onChange": "setEmail"}
  }
}
```

On macOS, the AppKit executor surface is available behind a feature:

```bash
cargo test --features appkit
cargo check --features appkit-native
cargo test --features winui
cargo check --features winui-native # Windows with Windows App SDK; non-Windows validates cfg gating
cargo test --features gtk4
cargo check --features gtk4-native # Linux with GTK4 development libraries
cargo test --all-features
```

## Supported Semantic Components

| React Aria semantic part | Native IR role |
| --- | --- |
| `Button` | `Button` |
| `TextField` + `Label` + `Input` | one native `TextField` |
| `Checkbox` | `Checkbox` |
| `Switch` | `Switch` |
| `RadioGroup` + `Radio` | native radio group with native radio buttons |
| `Select` + `ListBox` + `ListBoxItem` | native select with native options |
| `ListBox` + `ListBoxItem` | native list |
| `Dialog` | native dialog surface |
| `Popover` | native popover surface |
| `Tabs` / `TabList` / `Tab` / `TabPanel` | native tabs |
| `Menu` + `MenuItem` | native menu |
| `Separator` | native separator |
| `Slider` / `ProgressBar` | native ranged controls |
| `Toolbar` | native toolbar container |

## Platform Plan

| Platform | Target adapter |
| --- | --- |
| macOS | AppKit (`NSWindow`, `NSButton`, `NSTextField`, `NSSwitch`, `NSStackView`, `NSComboBox`, `NSScrollView`, `NSPanel`, `NSPopover`, `NSTabView`, `NSTabViewItem`, `NSMenu`, `NSMenuItem`, `NSBox(separator)`, `NSSlider`, `NSProgressIndicator`) |
| Windows | WinUI 3 (`Window`, `Button`, `TextBox`, `CheckBox`, `RadioButton`, `ComboBox`, `ListBox`, `ContentDialog`, `ToolTip`, `TabView`, `TabViewItem`, `StackPanel(menu)`, `Button(menu-item)`, `Border(separator)`, `Slider`, `ProgressBar`) |
| Linux | GTK4 (`gtk::ApplicationWindow`, `gtk::Button`, `gtk::Entry`, `gtk::DropDown`, `gtk::ListBox`, `gtk::Dialog`, `gtk::Popover`, `gtk::PopoverMenuBar`, `gio::Menu`, `gio::MenuItem`, `gtk::Notebook`, `gtk::Separator`, `gtk::Scale`) |

The public `NativeHost` trait is the renderer boundary. Real platform bindings
should usually implement `NativeHandleAdapter` or `NativeWidgetSurface` and run
through `HandleWidgetDriver`.
