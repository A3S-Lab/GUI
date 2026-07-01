# a3s-gui

`a3s-gui` is a Rust-native, cross-platform GUI renderer for
Web-compatible React/JSX UI trees. It lowers semantic UI input into AppKit,
WinUI, and GTK controls without embedding a WebView.

The crate exposes a native rendering pipeline and protocol boundary rather than
a browser runtime. Inputs can come from the TypeScript JSX runtime, React
Compiler output, React Aria-compatible components, direct protocol frames, or
direct Rust `NativeElement` trees. All supported inputs lower into the same A3S
Native UI IR before platform adapters create native widgets.

React Aria is supported as an optional semantic source. It is not required by
the core renderer; hosts can use any compiler or SDK that emits the A3S Native
UI IR or the serializable `UiFrame` protocol.

## Supported Inputs

`a3s-gui` accepts Web-compatible UI data:

- JSX/TSX-shaped element trees
- semantic component or element names
- stable keys for reconciliation
- `className` and inline `style` values
- `aria-*`, `data-*`, and common HTML attributes
- common HTML state props such as `disabled`, `required`, `checked`, and
  `selected`
- React-style event props such as `onClick`, `onChange`, `onFocus`, `onBlur`,
  and `onPress`

The renderer normalizes those inputs into native roles, control state, metadata,
portable style tokens, and action bindings.

## Architecture

`a3s-gui` is split into four layers:

1. **Web-compatible authoring layer**: accepts JSX/TSX-shaped trees and
   DOM-shaped props from the TypeScript SDK, React Compiler bridge, React Aria
   adapters, or another compiler that emits the same protocol.
2. **A3S Native UI IR**: stores a semantic, accessibility-first tree of native
   roles, state, labels, values, Web metadata, portable style tokens, event
   bindings, and children.
3. **Renderer and protocol runtime**: reconciles keyed trees, emits incremental
   native commands, tracks interaction state, and routes native events back to
   registered Web action ids.
4. **Native host adapters**: create and update platform controls such as AppKit,
   WinUI, and GTK widgets.

```text
Web-compatible source
        |
        v
JSX runtime / compiler / protocol producer
        |
        v
UiFrame or NativeElement
        |
        v
A3S Native UI IR
        |
        v
Keyed renderer diff engine
        |
        v
Native command stream
        |
        v
NativeHost adapter
  - AppKit on macOS
  - WinUI on Windows
  - GTK4 on Linux
        |
        v
Native event callbacks
        |
        v
InteractionState + EventRouter
        |
        v
Web-authored action ids
```

The command stream, native widget blueprints, accessibility roles, control
state, portable style tokens, and host protocol types are serializable. A Swift,
WinUI, GTK, Rust, or multi-process host can consume the same protocol without
seeing JSX or a browser DOM.

## Native Rendering Model

The Web-compatible surface is intentionally source-level only. `a3s-gui` does
not reuse DOM nodes, CSS layout engines, browser focus APIs, or WebView event
loops. It preserves the semantic contract and then creates real native controls.

Accepted Web-compatible inputs include `className`, inline `style`, `aria-*`,
`data-*`, HTML state props such as `disabled` and `required`, ranged attributes
such as `min`/`max`/`aria-valuenow`, stable keys, and DOM-style event props.
Those values are normalized into native props and portable style tokens.
Platform adapters decide how to apply the resulting setters and how to expose
the matching platform accessibility metadata.

## Authoring Surfaces

The Rust API can render native IR directly:

```rust
use a3s_gui::{
    AppKitAdapter, GuiRuntime, NativeElement, NativeProps, NativeRole,
    PlatformPlanningHost, WebProps,
};

let tree = NativeElement::new("save", NativeRole::Button).with_props(
    NativeProps::new()
        .label("Save")
        .action("saveDocument")
        .web(
            WebProps::new()
                .attribute("aria-label", "Save document")
                .style("backgroundColor", "rebeccapurple"),
        ),
);

let host = PlatformPlanningHost::new(AppKitAdapter);
let mut runtime = GuiRuntime::new(host);
runtime.render_native(&tree)?;
# Ok::<(), a3s_gui::GuiError>(())
```

The same shape can come from ordinary TSX through the prototype TypeScript SDK:

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {Button} from '@a3s-lab/gui';
import {createAction, createUiFrame, defineAction} from '@a3s-lab/gui';

const saveDocument = createAction('saveDocument', 'Save document');

const root = (
  <Button
    className="primary"
    style={{backgroundColor: 'rebeccapurple'}}
    aria-label="Save document"
    onClick={saveDocument}
  >
    Save
  </Button>
);

export const frame = createUiFrame('save-frame', root, {
  actions: [defineAction(saveDocument)],
});
```

React Aria remains a first-class compatibility path when an application wants
its component vocabulary:

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

export const frame = createUiFrame('react-aria-save-frame', root, {
  actions: [defineAction(saveDocument)],
});
```

The compiler bridge can consume a structured JSX tree produced after React
Compiler transforms:

```rust
use a3s_gui::{CompiledJsxNode, Gtk4Adapter, GuiRuntime, PlatformPlanningHost};

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
use a3s_gui::{
    Gtk4Adapter, HostEvent, NativeEvent, NativeEventKind, NativeProtocolSession,
};

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

## Runtime And Backend Contracts

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
`WinUiHandleAdapter`, and `Gtk4HandleAdapter` as handle-driver entry points for
platform bindings.

Backends that already have direct SDK calls can implement `NativeWidgetSurface`
instead. `SurfaceHandleAdapter` creates the platform object, applies typed
`NativeWidgetSetter` operations produced from the blueprint/config diff, inserts
native children, removes objects, and sets the root on the UI thread. The
`appkit-native`, `winui-native`, and `gtk4-native` features use this surface
path for real in-process OS widgets.

The default `RecordingBackend` keeps a pure Rust object tree for tests. The
`appkit`, `winui`, and `gtk4` features add platform driver surfaces for macOS,
Windows, and Linux.

## Native Platform Support

On macOS, the `appkit-native` feature includes a minimal real AppKit surface.
It creates `NSWindow`, `NSPanel`, `NSView`, `NSButton`, `NSTextField`,
`NSSwitch`, `NSStackView`, `NSComboBox`, `NSScrollView`, `NSTabView`,
`NSTabViewItem`, `NSMenu`, `NSMenuItem`, `NSBox(separator)`, `NSSlider`, and
`NSProgressIndicator` objects directly from the native command stream. Buttons,
editable text fields, checkboxes, switches, radio buttons, lists, selects,
toolbars, dialogs, popovers, tabs, menus, separators, sliders, and progress
indicators use native setters and native callbacks.

On Windows, the `winui-native` feature adds an in-process WinUI 3 surface behind
the same `NativeWidgetSurface` contract. It uses `winio-winui3` and the Windows
App SDK package bootstrap to create real `Microsoft.UI.Xaml.Window`,
`StackPanel`, `TextBlock`, `Button`, `TextBox`, `CheckBox`, `RadioButton`,
`ComboBox`, `ListBox`, `Grid`, `TabView`, `TabViewItem`, `ContentDialog`,
`ToolTip`, menu `StackPanel`/`Button` fallback controls, `Border(separator)`,
`Slider`, and `ProgressBar` objects directly. It does not enable WebView2 and
does not render HTML.

On Linux, the `gtk4-native` feature adds an in-process GTK4 surface behind the
same `NativeWidgetSurface` contract. It creates `gtk::ApplicationWindow`,
`gtk::Box`, `gtk::Label`, `gtk::Button`, `gtk::Entry`, `gtk::CheckButton`,
`gtk::Switch`, `gtk::DropDown`, `gtk::ListBox`, `gtk::ListBoxRow`,
`gtk::Dialog`, `gtk::Popover`, `gtk::PopoverMenuBar`, `gio::Menu`,
`gio::MenuItem`, `gtk::Notebook`, `gtk::Separator`, `gtk::Scale`, and
`gtk::ProgressBar` objects directly. The feature is compiled only on Linux and
requires GTK4 development libraries plus `pkg-config`.

```rust
use a3s_gui::{AppKitNativeSurface, GuiRuntime};

let surface = AppKitNativeSurface::new()?;
let host = surface.into_host();
let mut runtime = GuiRuntime::new(host);
runtime.render_compiled(&compiled)?;
# Ok::<(), a3s_gui::GuiError>(())
```

The same embedded-host event path is used by protocol hosts and native platform
callbacks: native events update `InteractionState` and resolve through the
registered Web action ids.

## Supported Semantic Roles

| Semantic UI input | Native IR role |
| --- | --- |
| `Button` | `Button` |
| `TextField` / `Label` / `Input` | one native `TextField` |
| `Checkbox` | `Checkbox` |
| `Switch` | `Switch` |
| `RadioGroup` / `Radio` | native radio group with native radio buttons |
| `Select` / `ListBox` / `ListBoxItem` | native select with native options |
| `ListBox` / `ListBoxItem` | native list |
| `Dialog` | native dialog surface |
| `Popover` | native popover surface |
| `Tabs` / `TabList` / `Tab` / `TabPanel` | native tabs |
| `Menu` / `MenuItem` | native menu |
| `Separator` | native separator |
| `Slider` / `ProgressBar` | native ranged controls |
| `Toolbar` | native toolbar container |

## Platform Plan

| Platform | Target adapter |
| --- | --- |
| macOS | AppKit (`NSWindow`, `NSButton`, `NSTextField`, `NSSwitch`, `NSStackView`, `NSComboBox`, `NSScrollView`, `NSPanel`, `NSPopover`, `NSTabView`, `NSTabViewItem`, `NSMenu`, `NSMenuItem`, `NSBox(separator)`, `NSSlider`, `NSProgressIndicator`) |
| Windows | WinUI 3 (`Window`, `Button`, `TextBox`, `CheckBox`, `RadioButton`, `ComboBox`, `ListBox`, `ContentDialog`, `ToolTip`, `TabView`, `TabViewItem`, `StackPanel(menu)`, `Button(menu-item)`, `Border(separator)`, `Slider`, `ProgressBar`) |
| Linux | GTK4 (`gtk::ApplicationWindow`, `gtk::Button`, `gtk::Entry`, `gtk::DropDown`, `gtk::ListBox`, `gtk::Dialog`, `gtk::Popover`, `gtk::PopoverMenuBar`, `gio::Menu`, `gio::MenuItem`, `gtk::Notebook`, `gtk::Separator`, `gtk::Scale`) |

## TypeScript SDK

The prototype TypeScript SDK lives in `sdk/typescript`. It provides a zero-deps
JSX runtime and protocol helpers that produce the same `UiFrame` JSON consumed
by the Rust runtime. Event props can be authored as normal named functions or
`createAction` markers; the protocol boundary serializes those callbacks as
stable action ids. The SDK exports generic semantic component markers and
React Aria-compatible marker shapes for tests and compiler fixtures.

## Validation

```bash
cargo fmt --all --check
cargo test
cargo test --features appkit
cargo check --features appkit-native
cargo test --features winui
cargo check --target x86_64-pc-windows-msvc --features winui-native
cargo test --features gtk4
cargo check --features gtk4-native # Linux with GTK4 development libraries
cargo test --all-features
npm test --prefix sdk/typescript
```

## License

`a3s-gui` is released under the MIT License. See [LICENSE](LICENSE).
