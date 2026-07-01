# a3s-gui

`a3s-gui` is a Rust crate that converts structured UI records into a
serializable native UI intermediate representation and native command stream. It
accepts `UiFrame` protocol frames, Rust `NativeElement` trees, and
JSX-generated element records.

The core renderer is backend-independent. Platform adapters translate the native
command stream into AppKit, WinUI, or GTK4 widgets.

## Supported Inputs

The protocol accepts the following input data:

- JSX-generated element records
- semantic component names, supported React Aria component names, HTML intrinsic
  element names, and SVG intrinsic element names
- stable keys for reconciliation
- `className`, Tailwind utility classes, inline style objects, and CSS text
  style strings
- `aria-*`, `data-*`, and common HTML attributes
- common HTML state props such as `disabled`, `required`, `checked`, and
  `selected`
- common JSX event prop names such as `onClick`, `onChange`, `onFocus`, `onBlur`,
  and `onPress`

The renderer normalizes these fields into native roles, control state, metadata,
portable style tokens, and action bindings.

## Compatibility Scope

The compiler bridge recognizes the HTML element surface exposed by the HTML
Living Standard plus common historical tags. It also recognizes common SVG
intrinsic element names used by JSX icon and vector trees. Known intrinsic
elements are mapped to native semantic roles where a matching role exists.
Elements without a dedicated native role are represented as generic native views
or text nodes, and the original tag is preserved in metadata under
`data-a3s-html-tag` or `data-a3s-svg-tag`.

The style layer accepts inline CSS declarations from style objects and CSS text.
It normalizes property names into a declaration map, preserves CSS custom
properties separately, and projects supported declarations into native style
tokens. The current portable token set includes display, position, physical and
logical inset, z-index, visibility, box sizing, box decoration break,
isolation, mix blend mode, float, clear, vertical alignment, table layout, border
collapse/spacing, caption side, flex direction/wrap/item
sizing/grow/shrink/order, alignment, justification, place alignment, CSS Grid
shorthand/templates/auto tracks/auto flow/placement, sizing,
gap/row-gap/column-gap, CSS containment and container query metadata, physical
and logical spacing, uniform, physical-edge, and logical-edge border
width/style/color, uniform, physical-corner, and logical-corner border radius,
text color, background
color/image/position/size/repeat/attachment/origin/clip/blend mode,
CSS clip path, CSS mask, CSS mask border, object fit/position,
list style, columns, column rule/span/fill, fragmentation breaks, font size,
font weight, font family, font style, font
stretch, font kerning, font feature and variation settings, font variant and
font synthesis properties, line height, letter spacing, word spacing, tab size,
text alignment, text direction, Unicode bidi, writing mode, text orientation,
text transform, text indent, text
wrapping, line clamp, SVG fill/stroke presentation properties, text decoration,
text shadow, text overflow, line breaking, whitespace, word breaking, hyphen
handling, overflow, opacity, aspect ratio, box shadow, ring shadow, divide
rule metadata, outline, transform, translate, rotate, scale, transform origin/style,
perspective, backface visibility, filter, filter function components, backdrop
filter, backdrop filter function components, transition, animation,
will-change, appearance, accent color, caret color, resize, scroll behavior,
physical and logical scroll margin/padding, scroll snap, overscroll behavior,
touch action, cursor, pointer events, and user selection.

CSS length values that cannot be converted to points or percentages, such as
`calc(...)`, `var(...)`, `clamp(...)`, viewport units, and sizing keywords, are
preserved as CSS length tokens for platform adapters that can consume them.
CSS color values support hex, RGB/RGBA, HSL/HSLA, slash alpha syntax, and
keyword preservation.

Tailwind utility classes are resolved into the same declaration model. Base
utilities are projected into supported native style tokens; variant utilities such
as `hover:`, `focus:`, and responsive prefixes are preserved in
`variant_declarations`. Tailwind color opacity modifiers such as `/50` are
preserved in the generated declarations and portable color tokens. Formatting
and table utilities such as `box-*`, `box-decoration-*`, `isolate`,
`isolation-auto`, `float-*`, `clear-*`, `align-*`, `table-*`,
`border-collapse`, `border-separate`, `border-spacing-*`, and `caption-*` are
projected into portable style tokens. SVG presentation utilities such as
`fill-*`, `stroke-*`, and `stroke-{width}` are projected into portable style
tokens. Common
visual-effect utilities such as `shadow-*`, `ring-*`, `inset-ring-*`,
`outline-*`, `cursor-*`,
`pointer-events-*`, `select-*`, `aspect-*`, `mix-blend-*`, `bg-blend-*`, and
`mask-*`, transform utilities such as `translate-*`, `scale-*`, `rotate-*`,
`skew-*`, `origin-*`, `perspective-*`, and composable filter utilities such as
`blur-*`, `brightness-*`, `contrast-*`, `drop-shadow-*`, `grayscale`,
`hue-rotate-*`, `invert-*`, `saturate-*`, `sepia-*`, and `backdrop-*` are
projected into the same declaration model. Container marker utilities such as
`@container`, `@container-size`, named container forms, and container query
variants such as `@md:` are resolved or preserved in the same declaration
model. Core
CSS Grid utilities such as `grid-cols-*`, `grid-rows-*`, `auto-cols-*`,
`auto-rows-*`, `grid-flow-*`, `col-*`, and `row-*` are also projected into
portable style tokens. Flex item and box-alignment utilities such as `flex-*`,
`basis-*`, `grow-*`, `shrink-*`, `order-*`, `content-*`, `self-*`,
`justify-items-*`, `justify-self-*`, and `place-*` are projected as well.
Typography utilities such as `font-*`, `italic`, `not-italic`, `tracking-*`,
`font-stretch-*`, `font-features-*`, font variant numeric utilities,
`tab-*`, text transform utilities, text decoration utilities,
`underline-offset-*`, `indent-*`, `line-clamp-*`, `text-shadow-*`,
`text-wrap`, `text-nowrap`, `text-balance`, `text-pretty`, `truncate`,
`text-ellipsis`, `text-clip`, `whitespace-*`, word-break utilities, and
`hyphens-*` are
projected into portable style tokens. Arbitrary property utilities for CSS
writing modes and `ltr:`/`rtl:` variants are preserved in the same declaration
model.
Background, object, list, columns, and fragmentation utilities such as `bg-*`,
`object-*`, `list-*`, `columns-*`, `break-before-*`, `break-after-*`, and
`break-inside-*` are projected into portable style tokens.
Motion, interaction, and scroll utilities such as `transition-*`, `duration-*`,
`delay-*`, `ease-*`, `animate-*`, `will-change-*`, `appearance-*`,
`accent-*`, `caret-*`, `resize-*`, `scroll-*`, `snap-*`, `overscroll-*`, and
`touch-*` are projected into portable style tokens.
Border radius utilities such as `rounded-*`, `rounded-t-*`, `rounded-r-*`,
`rounded-b-*`, `rounded-l-*`, `rounded-tl-*`, `rounded-tr-*`,
`rounded-br-*`, `rounded-bl-*`, `rounded-s-*`, `rounded-e-*`,
`rounded-ss-*`, `rounded-se-*`, `rounded-ee-*`, and `rounded-es-*` are
projected into physical or logical corner radius tokens.
Border width, color, and divide utilities such as `border-*`, `border-x-*`,
`border-y-*`, `border-t-*`, `border-r-*`, `border-b-*`, `border-l-*`,
`border-s-*`, `border-e-*`, `border-bs-*`, `border-be-*`, `divide-x-*`,
`divide-y-*`, `divide-*-reverse`, `divide-{color}`, and `divide-{style}` are
projected into physical, logical, or native child-divider tokens according to
their generated CSS property.
Logical direction utilities such as `start-*`, `end-*`, `ms-*`, `me-*`,
`mbs-*`, `mbe-*`, `mis-*`, `mie-*`, `ps-*`, `pe-*`, `pbs-*`, `pbe-*`,
`pis-*`, `pie-*`, `scroll-ms-*`, `scroll-me-*`, `scroll-mbs-*`,
`scroll-mbe-*`, `scroll-ps-*`, `scroll-pe-*`, `scroll-pbs-*`, and
`scroll-pbe-*` are projected into logical portable style tokens.
Inline styles are applied after class utilities so they keep normal inline-style
precedence.

CSS properties and Tailwind classes that do not yet have a portable native token
remain available as raw `className`, style declarations, metadata, or
`PortableStyle::unsupported` entries for platform adapters and future mappings.

## Architecture

`a3s-gui` is organized into four layers:

1. **Input layer**: accepts JSX-generated element records, direct
   `NativeElement` trees, and `UiFrame` protocol frames.
2. **A3S Native UI IR**: stores native roles, state, labels, values,
   accessibility metadata, Web metadata, portable style tokens, event bindings,
   and children.
3. **Renderer and protocol runtime**: reconciles keyed trees, emits incremental
   native commands, tracks interaction state, and routes native events back to
   registered action ids.
4. **Native host adapters**: create and update platform controls such as AppKit,
   WinUI, and GTK widgets.

```text
Structured UI source
        |
        v
SDK / compiler / protocol producer
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
Action ids
```

The command stream, native widget blueprints, accessibility roles, control
state, portable style tokens, and host protocol types are serializable. A Swift,
WinUI, GTK, Rust, or multi-process host can consume the same protocol as
serialized records, without source-level JSX or DOM nodes crossing the host
boundary.

## Rendering Model

The renderer maps accepted element records and props to native IR, then creates
native controls through the selected platform adapter. It does not expose DOM
nodes, browser CSS layout, or browser focus APIs as runtime primitives.

Accepted input fields include `className`, inline `style`, `aria-*`,
`data-*`, HTML state props such as `disabled` and `required`, ranged attributes
such as `min`/`max`/`aria-valuenow`, stable keys, and DOM-style event props.
Those values are normalized into native props and portable style tokens.
Platform adapters decide how to apply the resulting setters and how to expose
the matching platform accessibility metadata.

## Input Examples

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

The TypeScript SDK can produce the same frame shape from TSX:

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

Supported React Aria component names and props can be lowered through the same
JSX runtime:

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

Compiled JSX node records can be submitted through `render_compiled`:

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

The same tree can also be submitted as a protocol frame:

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
handles returned by a `NativeHandleAdapter`; the core traits do not require
`Send`, because AppKit, WinUI, and GTK widgets belong to their UI thread. The
feature modules expose `AppKitHandleAdapter`,
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
`Slider`, and `ProgressBar` objects directly. It uses WinUI controls as the
rendered surface.

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
registered action ids.

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

## Platform Targets

| Platform | Target adapter |
| --- | --- |
| macOS | AppKit (`NSWindow`, `NSButton`, `NSTextField`, `NSSwitch`, `NSStackView`, `NSComboBox`, `NSScrollView`, `NSPanel`, `NSPopover`, `NSTabView`, `NSTabViewItem`, `NSMenu`, `NSMenuItem`, `NSBox(separator)`, `NSSlider`, `NSProgressIndicator`) |
| Windows | WinUI 3 (`Window`, `Button`, `TextBox`, `CheckBox`, `RadioButton`, `ComboBox`, `ListBox`, `ContentDialog`, `ToolTip`, `TabView`, `TabViewItem`, `StackPanel(menu)`, `Button(menu-item)`, `Border(separator)`, `Slider`, `ProgressBar`) |
| Linux | GTK4 (`gtk::ApplicationWindow`, `gtk::Button`, `gtk::Entry`, `gtk::DropDown`, `gtk::ListBox`, `gtk::Dialog`, `gtk::Popover`, `gtk::PopoverMenuBar`, `gio::Menu`, `gio::MenuItem`, `gtk::Notebook`, `gtk::Separator`, `gtk::Scale`) |

## TypeScript SDK

The TypeScript SDK lives in `sdk/typescript`. It provides a zero-dependency JSX
runtime and protocol helpers that produce the same `UiFrame` JSON consumed by
the Rust runtime. Event props can be named functions or `createAction` markers;
the protocol boundary serializes those callbacks as stable action ids. The SDK
exports generic semantic component markers and React Aria marker shapes for
tests and compiler fixtures.

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
