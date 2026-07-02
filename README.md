# a3s-gui

`a3s-gui` is a Rust crate that converts structured UI protocol frames into
native platform command streams. It lowers accepted element and style records
into a serializable Native UI IR and emits incremental create, update, insert,
remove, and root-setting commands. The public API accepts `UiFrame` protocol
frames, Rust `NativeElement` trees, and compiled element records.

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

## Source Layout

Runtime command execution lives under `src/backend/`. Platform blueprint
planning and widget-name mapping live under `src/platform/`. Compiler lowering,
React Aria mapping, and style normalization are split under `src/compiler/`,
`src/react_aria/`, and `src/style/`. Native surface implementations are grouped
by platform under `src/appkit_native/`, `src/gtk4_native/`, and
`src/winui_native/`.

## Compatibility Scope

The compiler bridge recognizes the HTML element surface exposed by the HTML
Living Standard plus common historical tags. It also recognizes common SVG
intrinsic element names used by JSX icon and vector trees. Known intrinsic
elements are mapped to native semantic roles where a matching role exists.
Document, metadata, template, slot, text, text-annotation, phrasing-text,
flow-text, legacy-text, legacy-frame, fallback-content, math, selected-content,
heading, heading-group, ruby annotation, landmark, sectioning, disclosure,
figure, description-list, form, form-grouping, option-group, output, meter,
list, dialog, menu, media, embedded-content, link, image-map, and
table-structure tags lower to dedicated native roles.
`input[type=range]` lowers to a native slider role, numeric `value` or
`defaultValue` props are projected as the ranged current value, and numeric
`step` is projected as native ranged-control step state.
`input[type=number]` lowers to the native text-field role while numeric
`value` or `defaultValue`, `min`, `max`, and `step` props are preserved as
native control state.
`input[type=button]`, `input[type=submit]`, `input[type=reset]`, and
`input[type=image]` lower to native button roles; `value`, default submit/reset
labels, and image `alt` text are used as fallback native labels.
`textarea` direct text children are projected as the native text-field value
when no explicit value is supplied.
Common form-control attributes such as `readonly`/`readOnly`, `multiple`,
`autofocus`/`autoFocus`, `autocomplete`/`autoComplete`, `inputmode`/`inputMode`,
`pattern`, `minlength`/`minLength`, `maxlength`/`maxLength`, `rows`, `cols`,
and `size` are projected into native control-state fields and remain available
as metadata for platform adapters.
Elements without a dedicated native role are represented as generic native
views or text nodes, and the original tag is preserved in metadata under
`data-a3s-html-tag` or `data-a3s-svg-tag`.
HTML `option` and `data` `value` attributes are projected as native value
state for selection items and machine-readable data text.

The style layer accepts inline CSS declarations from style objects and CSS text.
It normalizes property names into a declaration map, preserves CSS custom
properties separately, and projects supported declarations into native style
tokens. CSS text parsing preserves delimiters inside functions, URLs, strings,
and brackets, ignores comments, and applies `!important` priority before values
reach portable parsers. The current portable token set includes the CSS display
mode family,
including legacy, representable multi-keyword, internal table, and ruby display
modes,
CSS cascade reset metadata, position, physical and logical inset, z-index,
visibility, box sizing, box decoration break, CSS Anchor Positioning metadata,
isolation, mix blend mode,
float, clear, vertical alignment, table layout, border collapse/spacing,
caption side, empty cells, flex direction/wrap/item
sizing/grow/shrink/order, reading flow/order metadata, alignment,
justification, place alignment, CSS Grid
shorthand/templates/auto tracks/auto flow/placement, physical and logical
sizing, intrinsic size interpolation metadata, gap/row-gap/column-gap, CSS containment, generated content,
CSS counters, quotes, string sets, and container query metadata, physical,
logical, margin-trim, and child spacing, uniform,
physical-edge, and logical-edge border width/style/color, uniform,
physical-corner, and logical-corner border radius, CSS border image metadata,
text color, background
shorthand/color/image/position/size/repeat/attachment/origin/clip/blend mode,
CSS clip, CSS clip path, CSS mask, CSS mask border,
CSS image rendering/orientation/resolution, object fit/position,
CSS Shapes shape-inside/outside/margin/padding/threshold, list style type/position/image and
marker side metadata,
columns, column rule/span/fill,
CSS page size/orientation/selection, bleed/marks, orphans/widows, bookmark and footnote metadata,
fragmentation breaks, font shorthand, font size and font size adjustment,
font weight, font family, font style, font stretch/width, font palette,
font language override, font kerning, font optical sizing, font smoothing hints,
font feature and variation settings, font variant and
font synthesis properties, line height, rhythmic sizing and line-grid metadata,
MathML math depth/shift/style metadata, baseline alignment metadata,
initial letter metadata, inline sizing, letter spacing, word spacing, tab size,
text alignment, final-line alignment, text justification, text size adjustment,
text direction, Unicode bidi, writing
mode, text orientation, text combine upright, text transform, text indent, text
wrapping, wrap control, text spacing, text autospace, word space transform,
text box trim/edge, line clamp,
line clamp longhands,
CSS Speech cue/pause/rest/speak/voice metadata,
SVG fill/stroke/marker presentation properties, SVG rendering hints, SVG paint
server and filter color properties, text decoration,
text decoration skip metadata, underline offset/position, text shadow, text overflow,
text emphasis, ruby layout, hanging punctuation, line breaking, whitespace,
whitespace collapse/trim, word breaking, hyphen handling and limits, overflow, opacity, aspect ratio, box shadow, ring shadow, divide
rule metadata, outline, transform, translate, rotate, scale, transform origin/style,
transform box, CSS Motion Path offset metadata,
perspective, backface visibility, filter, filter function components, backdrop
filter, backdrop filter function components, transition, animation,
scroll-driven animation timelines and ranges, CSS View Transitions metadata,
will-change, color scheme, forced color adjustment, print color adjustment,
field sizing, appearance, accent color, caret color and caret shape metadata,
resize, scroll behavior, physical and logical scroll
margin/padding, scroll snap, scroll initial target, scroll target groups,
scroll marker groups, scrollbar gutter/width/color, scroll anchoring,
logical overflow, overflow clip margin, overscroll behavior,
touch action, CSS UI directional navigation, CSS Spatial Navigation metadata,
CSS UI interactivity metadata, cursor, pointer events, and user selection.

CSS length values that cannot be converted to points or percentages, such as
`calc(...)`, `calc-size(...)`, `var(...)`, `clamp(...)`, `anchor(...)`,
`anchor-size(...)`, CSS math functions such as `round(...)`, `hypot(...)`, and
`abs(...)`, viewport units, and sizing keywords, are preserved as CSS length
tokens for platform adapters that can consume them.
CSS color values support hex, RGB/RGBA, HSL/HSLA, slash alpha syntax, keyword
preservation, and CSS color function preservation for `hwb(...)`, `lab(...)`,
`lch(...)`, `oklab(...)`, `oklch(...)`, `color(...)`, `color-mix(...)`,
`light-dark(...)`, `contrast-color(...)`, `alpha(...)`, and
`device-cmyk(...)`.

Tailwind utility classes are resolved into the same declaration model. Base
utilities are projected into supported native style tokens; variant utilities such
as `hover:`, `focus:`, and responsive prefixes are preserved in
`variant_declarations`. Within a `className` value, Tailwind important utilities
using the `!` modifier are applied after normal utilities while preserving
relative order inside each priority group. Tailwind arbitrary values decode `_`
as a space, preserve escaped `\_` as an underscore, and keep underscores inside
`url(...)` values. The same bracketed-segment decoding is applied to arbitrary
variant keys. Tailwind color opacity modifiers such as `/50` are preserved in
the generated declarations and portable color tokens, including arbitrary color
functions. Display
utilities such as `inline`, `inline-block`, `flow-root`, `contents`,
`list-item`, `table-*`, `inline-table`, `flex`, `inline-flex`, `grid`,
`inline-grid`, and `hidden` are projected into portable display tokens.
Arbitrary `display` properties are projected into the same token when the
display value has an equivalent portable mode.
Screen-reader utilities such as `sr-only` and `not-sr-only` are projected into
their generated declaration groups. Formatting and table utilities such as
`box-*`, `box-decoration-*`, `isolate`, `isolation-auto`, `float-*`,
`clear-*`, `align-*`, `border-collapse`, `border-separate`,
`border-spacing-*`, `caption-*`, arbitrary `empty-cells` properties, and
arbitrary `border-image*` properties are projected into portable style tokens.
SVG presentation utilities such as `fill-*`, `stroke-*`, and `stroke-{width}`,
plus arbitrary SVG marker, rendering, paint server, and filter color
properties, are projected into portable style tokens. Common
visual-effect utilities such as `shadow-*`, `ring-*`, `inset-ring-*`,
`outline-*`, `cursor-*`,
`pointer-events-*`, `select-*`, `aspect-*`, `mix-blend-*`, `bg-blend-*`, and
`mask-*`, transform utilities such as `translate-*`, `scale-*`, `rotate-*`,
`skew-*`, `origin-*`, `perspective-*`, arbitrary `transform-box` and CSS
Motion Path properties, and composable filter utilities such as
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
`justify-items-*`, `justify-self-*`, `place-*`, and arbitrary `reading-*`
properties are projected as well.
Sizing and child-spacing utilities such as `size-*`, `space-x-*`,
`space-y-*`, `space-x-reverse`, and `space-y-reverse` are projected into
portable layout tokens.
Typography utilities such as `font-*`, `italic`, `not-italic`,
`antialiased`, `subpixel-antialiased`, `tracking-*`, `font-stretch-*`,
`font-features-*`, arbitrary `font`, `font-width`, `font-size-adjust`,
`font-palette`, and `font-language-override` properties,
font variant numeric utilities,
arbitrary rhythmic sizing and line-grid properties such as `line-height-step`,
`block-step*`, `line-grid`, `line-snap`, and `box-snap`,
arbitrary MathML math properties such as `math-depth`, `math-shift`, and
`math-style`,
`tab-*`, text transform utilities, text decoration utilities,
`underline-offset-*`, arbitrary `text-decoration-skip*` and
`text-underline-position` properties, arbitrary `text-emphasis-*` properties,
arbitrary `text-size-adjust`, `text-combine-upright`, `text-align-last`,
`text-align-all`, `text-group-align`, `text-justify`, baseline and
initial-letter properties,
`text-wrap-*`, arbitrary `wrap-*`, `line-padding`, `text-spacing`,
`text-spacing-trim`, `text-autospace`, `word-space-transform`,
`text-box*`, `white-space-collapse`, `white-space-trim`,
`hanging-punctuation`, hyphenation limit properties, and line-clamp
longhand properties,
arbitrary CSS Speech properties such as `speak`, `speak-as`, `pause`, `rest`,
`cue`, and `voice-*`,
arbitrary `ruby-*`
properties, `indent-*`, `line-clamp-*`, `text-shadow-*`, `text-wrap`, `text-nowrap`,
`text-balance`, `text-pretty`, `truncate`,
`text-ellipsis`, `text-clip`, `whitespace-*`, `wrap-*`, word-break utilities,
`hyphens-*`, generated-content utilities such as `content-[...]`,
`content-(...)`, and `content-none`, and arbitrary `counter-*`, `quotes`, and
`string-set` properties are projected into portable style tokens.
Arbitrary property utilities for CSS writing modes, CSS Anchor Positioning
properties such as `anchor-name` and `position-area`, and `ltr:`/`rtl:`
variants are preserved in the same declaration model.
Arbitrary `all` properties are projected as CSS cascade reset metadata.
Background, object, list, columns, and fragmentation utilities such as `bg-*`,
`object-*`, `list-*`, `list-image-*`, `columns-*`, `break-before-*`,
`break-after-*`, and `break-inside-*`, plus arbitrary CSS image and shape
properties such as `image-rendering`, `shape-outside`, `shape-inside`, and
`shape-padding`, and arbitrary paged
media, background shorthand, shape, and list properties such as `size`, `page`,
`page-orientation`, `bleed`, `marks`, `orphans`, `widows`, and `marker-side`,
plus bookmark and footnote properties, are
projected into portable style tokens.
Motion, interaction, and scroll utilities such as `transition-*`, `duration-*`,
`delay-*`, `ease-*`, `animate-*`, arbitrary scroll-driven animation
properties such as `animation-composition`, `animation-timeline`,
`scroll-timeline`, and `view-timeline`, top-layer `overlay` metadata,
arbitrary CSS View Transitions properties, `will-change-*`, `appearance-*`,
`scheme-*`, `forced-color-adjust-*`, arbitrary `print-color-adjust`,
`field-sizing-*`, `accent-*`,
`caret-*`, arbitrary `caret`, `caret-animation`, and `caret-shape`
properties, `resize-*`, `scroll-*`, `snap-*`, `scrollbar-*`,
`scrollbar-gutter-*`, `scrollbar-thumb-*`, `scrollbar-track-*`,
`overscroll-*`, arbitrary logical overflow, overflow clip margin, scroll
anchoring, scroll initial target, scroll target groups, scroll marker groups,
logical overscroll, CSS UI directional navigation, CSS Spatial Navigation, CSS
UI interactivity, and `touch-*` properties are projected into portable style
tokens.
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

The crate is organized into four layers:

1. **Input layer**: accepts compiled element records, direct
   `NativeElement` trees, and `UiFrame` protocol frames.
2. **A3S Native UI IR**: stores native roles, state, labels, values,
   accessibility metadata, source metadata, portable style tokens, event
   bindings, and children.
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
state, portable style tokens, and host protocol types are serializable. Swift,
WinUI, GTK, Rust, and multi-process hosts can consume the same protocol records.

## Rendering Model

The renderer maps accepted element records and props to native IR, then creates
native controls through the selected platform adapter. It does not expose DOM
nodes, browser CSS layout, or browser focus APIs as runtime primitives.

Accepted input fields include `className`, inline `style`, `aria-*`,
`data-*`, HTML state props such as `disabled` and `required`, ranged attributes
such as `min`/`max`/`step`/`aria-valuenow`, `input[type=range]` and
`input[type=number]` numeric values, `placeholder`/`aria-placeholder`,
HTML form-control hints such as `readOnly`, `multiple`, `autoFocus`,
`autoComplete`, `inputMode`, `pattern`, `minLength`, `maxLength`, `rows`,
`cols`, and `size`, stable keys, and DOM-style event props.
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
`NSTabViewItem`, `NSMenu`, `NSMenuItem`, `NSBox(separator)`, `NSSlider`,
and `NSProgressIndicator` objects directly from the native command stream.
Buttons, editable text fields, checkboxes, switches, radio buttons, lists,
selects, toolbars, dialogs, popovers, tabs, menus, separators, sliders, and
progress indicators use native setters and native callbacks. Image, media,
embedded-content, and table roles are present in the native IR and planning
target map; native surfaces can accept them through the same command path and
may use generic native container fallbacks until dedicated controls are wired.

On Windows, the `winui-native` feature adds an in-process WinUI 3 surface behind
the same `NativeWidgetSurface` contract. It uses `winio-winui3` and the Windows
App SDK package bootstrap to create real `Microsoft.UI.Xaml.Window`,
`StackPanel`, `TextBlock`, `Button`, `TextBox`, `CheckBox`, `RadioButton`,
`ComboBox`, `ListBox`, `Grid`, `TabView`, `TabViewItem`, `ContentDialog`,
`ToolTip`, menu `StackPanel`/`Button` fallback controls, `Border(separator)`,
`Slider`, and `ProgressBar` objects directly. Image, media, canvas,
embedded-content, and table roles are present in the native IR and planning
target map; native surfaces can accept them through the same command path and
may use generic WinUI container fallbacks until dedicated controls are wired.

On Linux, the `gtk4-native` feature adds an in-process GTK4 surface behind the
same `NativeWidgetSurface` contract. It creates `gtk::ApplicationWindow`,
`gtk::Box`, `gtk::Label`, `gtk::Button`, `gtk::Entry`, `gtk::CheckButton`,
`gtk::Switch`, `gtk::DropDown`, `gtk::ListBox`, `gtk::ListBoxRow`,
`gtk::Dialog`, `gtk::Popover`, `gtk::PopoverMenuBar`, `gio::Menu`,
`gio::MenuItem`, `gtk::Notebook`, `gtk::Separator`, `gtk::Scale`, and
`gtk::ProgressBar` objects directly. Image, media, canvas, embedded-content,
and table roles are present in the native IR and planning target map; native
surfaces can accept them through the same command path and may use generic GTK
container fallbacks until dedicated controls are wired. The feature is compiled
only on Linux and requires GTK4 development libraries plus `pkg-config`.

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
| `html` / `head` / `body` / `title` | native document roles |
| `base` / `link` / `meta` / `style` / `script` / `noscript` | native metadata and resource roles |
| `template` / `slot` | native template and slot roles |
| `abbr` / `cite` / `dfn` / `data` / `ins` / `del` / `mark` / `time` | native text annotation roles; `data[value]` preserves native value state |
| `em` / `strong` / `code` / `kbd` / `samp` / `var` / `q` / `sub` / `sup` / `small` | native phrasing text roles |
| `b` / `i` / `s` / `strike` / `u` / `bdi` / `bdo` | native typographic and bidi text roles |
| `p` / `pre` / `blockquote` / `address` / `br` / `wbr` | native flow text roles |
| `listing` / `plaintext` / `xmp` / `nobr` / `center` / `font` / `basefont` / `big` / `tt` | native legacy text roles |
| `applet` / `bgsound` / `frame` / `frameset` / `noembed` / `noframes` / `marquee` / `nextid` | native legacy document roles |
| `math` | native math role |
| `selectedcontent` | native selected-content role |
| `h1` / `h2` / `h3` / `h4` / `h5` / `h6` | native heading role |
| `hgroup` | native heading-group role |
| `ruby` / `rb` / `rt` / `rp` / `rtc` | native ruby annotation roles |
| `main` / `nav` / `header` / `footer` / `article` / `section` / `aside` / `search` | native landmark or sectioning role |
| `details` / `summary` | native disclosure roles |
| `figure` / `figcaption` | native figure roles |
| `dl` / `dt` / `dd` | native description-list roles |
| `form` / `fieldset` / `legend` | native form and fieldset roles |
| `input[type=range]` | native slider role with numeric current value and step state |
| `input[type=number]` | native text-field role with numeric current value, bounds, and step state |
| `input[type=button]` / `input[type=submit]` / `input[type=reset]` / `input[type=image]` | native button role with HTML fallback labels |
| `readOnly` / `multiple` / `autoFocus` / `autoComplete` / `inputMode` / `pattern` / `minLength` / `maxLength` / `rows` / `cols` / `size` | native control-state fields plus preserved metadata |
| `select` / `optgroup` / `option` | native select option-group and option roles; `option[value]` preserves native value state |
| `output` | native output role |
| `meter` / `progress` | native ranged indicator roles |
| `a[href]` | native link role |
| `map` / `area` | native image-map roles |
| `img` / `picture` | native image role |
| `audio` / `video` | native media role |
| `canvas` | native canvas role |
| `iframe` / `embed` / `object` | native embedded-content role |
| `table` / `thead` / `tbody` / `tfoot` / `tr` / `td` / `th` / `col` / `caption` | native table roles |
| `TextField` / `Label` / `Input` | one native `TextField` |
| `Checkbox` | `Checkbox` |
| `Switch` | `Switch` |
| `RadioGroup` / `Radio` | native radio group with native radio buttons |
| `Select` / `ListBox` / `ListBoxItem` | native select with native options |
| `dir` | native list role |
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
| macOS | AppKit (`NSWindow`, `NSButton`, `NSTextField`, `NSImageView`, `NSSwitch`, `NSStackView`, `NSComboBox`, `NSScrollView`, `NSPanel`, `NSPopover`, `NSTabView`, `NSTabViewItem`, `NSMenu`, `NSMenuItem`, `NSBox(separator)`, `NSSlider`, `NSProgressIndicator`, `NSTableView`) |
| Windows | WinUI 3 (`Window`, `Button`, `TextBlock`, `Image`, `TextBox`, `CheckBox`, `RadioButton`, `ComboBox`, `ListBox`, `Grid`, `ContentDialog`, `ToolTip`, `TabView`, `TabViewItem`, `StackPanel(menu)`, `Button(menu-item)`, `Border(separator)`, `Slider`, `ProgressBar`) |
| Linux | GTK4 (`gtk::ApplicationWindow`, `gtk::Button`, `gtk::Entry`, `gtk::Picture`, `gtk::Video`, `gtk::DrawingArea`, `gtk::DropDown`, `gtk::ListBox`, `gtk::Dialog`, `gtk::Popover`, `gtk::PopoverMenuBar`, `gio::Menu`, `gio::MenuItem`, `gtk::Notebook`, `gtk::Separator`, `gtk::Scale`, `gtk::Grid`) |

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
