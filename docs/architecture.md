# a3s-gui Architecture

`a3s-gui` accepts structured UI input and emits native platform commands for
AppKit, WinUI, and GTK4 backends.

The input contract is structured element data: protocol element records,
semantic element names, `className`, inline style objects, `aria-*`, `data-*`,
and DOM-style event props. Supported React Aria component names are treated as
semantic identifiers. The renderer consumes A3S Native UI IR; host adapters
create platform widgets directly.

```text
JSX source
        |
        v
@a3s-lab/gui JSX runtime
        |
        v
Compiled element records
        |
        v
UiFrame protocol
        |
        v
ReactCompilerBridge
        |
        v
Semantic UI tree
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
Native events
        |
        v
HostEvent protocol
        |
        v
InteractionState
        |
        v
EventRouter
        |
        v
Action ids
```

## Contract

The bridge accepts semantic component names, HTML and SVG intrinsic names,
common Web props, and event names. The native renderer receives a typed,
portable protocol. Input records may come from `react-aria-components`, the
zero-dependency marker exports used by compiler fixtures, or any compiler that
emits the same protocol shape.

Allowed data:

- semantic component names, HTML intrinsic element names, and SVG intrinsic
  element names
- labels, values, placeholders, orientation, disabled state, selected state,
  checked state, expanded state, read-only state, multiple-selection state,
  autofocus state, text-entry hints, text-length hints, textarea sizing hints,
  and ranged values
- list, dialog, popover, tab, menu, and form structure
- stable keys for reconciliation
- `className`, Tailwind utility classes, inline `style` objects, and CSS text
  style strings as portable style input
- `aria-*`, `data-*`, and HTML attributes as metadata
- common JSX event prop names such as `onClick`, `onChange`, `onInput`,
  `onFocusChange`, `onExpandedChange`, `onKeyDown`, and `onKeyUp` with named
  callback functions, normalized into native actions

Non-portable runtime assumptions:

- direct DOM access, for example `document.querySelector`
- measuring browser layout APIs as the source of truth
- relying on arbitrary CSS selectors that cannot be expressed as native style
  tokens
- treating an `HTMLElement` instance as an application object

## NativeHost Boundary

Every platform adapter implements the same host operations:

- create a native widget for a `NativeElement`
- update native properties without remounting
- insert a native child at a stable index
- remove a native subtree
- set the root widget for a window or surface

The renderer owns reconciliation. Platform adapters own native object lifetime,
thread affinity, focus integration, accessibility integration, and event delivery.
The runtime does not require native hosts, command executors, event sources, or
widget drivers to be `Send`: real GUI handles are often confined to the main UI
thread.
Hosts model a tree, not a graph. Inserting an existing child reparents it from
any previous parent, and shared host implementations reject self-parenting or
ancestor cycles before recording a command.

The pure Rust `PlatformPlanningHost` implements the same `NativeHost` contract
without linking platform SDKs. It is an executable specification for native
adapters: given a `NativeElement`, it records the AppKit, WinUI, or GTK widget
class, accessibility role, action binding, style tokens, events, and metadata
that a platform backend applies.

The compiler bridge accepts the HTML element registry exposed by `HTML_ELEMENTS`.
Each recognized intrinsic tag lowers to the closest native semantic role, and
the original tag is preserved as `data-a3s-html-tag` metadata. Document,
metadata, template, slot, text, text-annotation, phrasing-text, flow-text,
legacy-text, legacy-frame, fallback-content, math, selected-content, heading,
heading-group, ruby annotation, landmark, sectioning, disclosure, figure,
description-list, form, form-grouping, option-group, output, meter, list,
dialog, menu, media, embedded-content, link, image-map, and table-structure
tags lower to dedicated native roles. `input[type=range]` lowers to a native
slider role, numeric `value` or `defaultValue` props are projected as the
ranged current value, and numeric `step` is projected as native ranged-control
step state. `input[type=number]` lowers to the native text-field role while
numeric `value` or `defaultValue`, `min`, `max`, and `step` props are preserved
as native control state; the text value is also retained for text-field
backends. GTK planning maps that number-shaped text field to `gtk::SpinButton`.
`input[type=button]`, `input[type=submit]`,
`input[type=reset]`, and `input[type=image]` lower to native button roles with
HTML fallback labels from `value`, default submit/reset labels, or image `alt`
text. `input[type=hidden]` keeps its form `name`, `form`, `type`, and `value`
state in the native IR but is marked hidden so platform widget config,
rendered accessibility projection, and native event routing treat it as
invisible. HTML dialog `open` state projects into a structured
`HtmlDialogProps` value and controls native widget visibility for intrinsic
dialog elements. `input` and `textarea` `placeholder` attributes, plus
`aria-placeholder`, project into native placeholder state. `textarea` direct
text children project into native text-field value state when no explicit value
is supplied. HTML
form-control attributes including `readonly`/`readOnly`, `multiple`,
`autofocus`/`autoFocus`, `autocomplete`/`autoComplete`,
`inputmode`/`inputMode`, `enterkeyhint`/`enterKeyHint`,
`autocapitalize`/`autoCapitalize`, `autocorrect`/`autoCorrect`,
`virtualkeyboardpolicy`/`virtualKeyboardPolicy`, `pattern`,
`minlength`/`minLength`, `maxlength`/`maxLength`, `rows`, `cols`, and `size`
project into native control-state fields and stay available in metadata.
ARIA relationship attributes including `aria-labelledby`, `aria-describedby`,
`aria-details`, `aria-controls`, `aria-owns`, `aria-flowto`,
`aria-errormessage`, and `aria-activedescendant` project into structured
native accessibility relationship hints and stay available in metadata.
ARIA description and value attributes including `aria-description`,
`aria-roledescription`, `aria-keyshortcuts`, and `aria-valuetext` project into
structured native accessibility description hints and stay available in metadata.
ARIA structural attributes including `aria-level`, `aria-posinset`,
`aria-setsize`, `aria-rowcount`, `aria-rowindex`, `aria-rowspan`,
`aria-colcount`, `aria-colindex`, `aria-colspan`, `aria-rowindextext`, and
`aria-colindextext`, plus `aria-sort`, project into structured native
accessibility structure hints and stay available in metadata.
ARIA state and live-region attributes including `aria-hidden`,
`aria-autocomplete`, `aria-multiline`, `aria-current`, `aria-haspopup`,
`aria-pressed`, `aria-live`, `aria-atomic`, `aria-busy`, `aria-relevant`, and
`aria-modal` project into structured native accessibility state hints and stay
available in metadata. `aria-hidden` is accessibility-only state and does not
change native widget visibility, but `aria-hidden="true"` omits the subtree
from rendered accessibility tree projection.
HTML global attributes including `title`, `hidden`, `lang`, `dir`,
`tabindex`/`tabIndex`, `role`, `accesskey`/`accessKey`,
`contenteditable`/`contentEditable`, `draggable`, `spellcheck`/`spellCheck`,
`translate`, `inert`, `popover`, `anchor`, `is`, and global `nonce` project
into native fields and stay available in metadata. Native surfaces map `title`
to platform tooltip/title hints where available. `hidden` also makes the native
widget config invisible. Shadow DOM distribution and style-part attributes such
as `slot`, `part`, and `exportparts`/`exportParts` project into a structured
`HtmlShadowProps` value for native composition and styling adapters. Microdata
attributes such as `itemscope`/`itemScope`, `itemprop`/`itemProp`,
`itemtype`/`itemType`, `itemid`/`itemID`, and `itemref`/`itemRef` project into
a structured `HtmlMicrodataProps` value for native metadata adapters. Form submission attributes and overrides project into
native fields for matching form and submit-control tags. Media and resource
attributes including `href`, `src`, `srcset`/`srcSet`,
`sizes`, `media`, resource `type`, intrinsic `width`/`height`, `loading`,
`decoding`, `fetchpriority`/`fetchPriority`, `crossorigin`/`crossOrigin`,
`referrerpolicy`/`referrerPolicy`, `poster`, `controls`, `autoplay`/
`autoPlay`, `loop`, `muted`, `playsinline`/`playsInline`, `preload`, `kind`,
`srclang`/`srcLang`, `label`, and `default` project into native fields for
matching image, media, source, track, link, script, embed, iframe, object, and
related resource tags. Resource policy attributes including anchor and area
`target`, `download`, `ping`, `rel`, `hreflang`/`hrefLang`, link `as`,
`integrity`, `blocking`, `nonce`, `imagesrcset`/`imageSrcSet`,
`imagesizes`/`imageSizes`, link `disabled`, script `async`, `defer`,
`nomodule`/`noModule`, iframe `name`, `allow`,
`allowfullscreen`/`allowFullScreen`, `sandbox`, and `srcdoc`/`srcDoc` project
into a structured `HtmlResourcePolicyProps` value for native resource loading,
navigation, and embedding adapters. Form association and range metadata such as
label `for`/`htmlFor`, output `for`/`htmlFor`, and meter `low`, `high`, and
`optimum` project into a structured `HtmlFormAssociationProps` value for native
accessibility and range adapters. Activation attributes such as button
`command`, `commandfor`/`commandFor`, `popovertarget`/`popoverTarget`, and
`popovertargetaction`/`popoverTargetAction`, plus popover target attributes on
button-like `input` controls, project into a structured `HtmlActivationProps`
value for native command, popover, and action adapters. Text annotation
attributes such as quote and change `cite`, change `datetime`/`dateTime`, and
time `datetime`/`dateTime` project into a structured
`HtmlTextAnnotationProps` value for native citation, provenance, and temporal
adapters. HTML `option` and `data` `value`
attributes project into native value state. Table and list structure attributes including
`colspan`/`colSpan`, `rowspan`/`rowSpan`, `headers`, `scope`, `abbr`, `span`,
`start`, `reversed`, list `type`, and list item `value` project into native
fields for table cells, columns, column groups, ordered lists, and list items.
The HTML prop lowering implementation is split under `src/compiler/props/` into
shared attribute parsing, control/form aliases, global attributes,
resource/media aliases, activation aliases, text annotation aliases, dialog
aliases, shadow/style-part aliases, microdata aliases, resource policy aliases,
semantic state aliases, form association aliases, table/list structure aliases,
and tag-specific native state rules. Shared activation, text annotation,
dialog, shadow/style-part, microdata, resource policy, form association, and
table/list hints are represented by `HtmlActivationProps`,
`HtmlTextAnnotationProps`, `HtmlDialogProps`, `HtmlShadowProps`,
`HtmlMicrodataProps`, `HtmlResourcePolicyProps`, `HtmlFormAssociationProps`,
and `HtmlCollectionProps` under `src/html/`; native and React Aria builder
methods for those grouped hints are split into dedicated submodules.
Generic HTML containers lower to `NativeRole::View`; unsupported custom elements
with a hyphenated tag name also lower to a generic native view.
The SVG element registry exposed by `SVG_ELEMENTS` follows the same lowering
path for vector and icon JSX trees. Recognized SVG tags lower to generic native
views or text nodes and preserve the original tag as `data-a3s-svg-tag`
metadata.

`GuiRuntime` is the public orchestration API. It accepts compiled JSX trees,
supported semantic component trees, or native IR trees and renders them into any
`NativeHost`. `InteractionState` updates platform-independent focus, value,
checked, selected, and expanded state from native events before action routing.
`EventRouter` maps native events such as press, change, focus, toggle,
selection change, key down, and key up back to serialized action identifiers
such as `onClick`, `onChange`, `onInput`, `onFocusChange`,
`onExpandedChange`, `onKeyDown`, and `onKeyUp`.
When a key-down event has no explicit `onKeyDown` binding on the target or its
route ancestors, Enter and Space fall back to the primary press action for
activatable controls such as buttons, links, and menu items. For stateful
controls without an explicit key-down handler on that route, keyboard
activation is normalized before routing: Space toggles checkboxes and switches,
Space selects radios, and Enter or Space toggles expanded controls and selects
listbox items or tabs.
Native event routing tries the target widget first, then mounted ancestors, so
child widget callbacks can reach container-level handlers. Selection events
without a value are normalized from the selected child's value or label.
Empty event action ids are ignored rather than dispatched.
`ActionRegistry` records and validates non-empty action ids before they are
handed to the JavaScript state bridge. Hosts that implement
`AccessibilityTreeHost` can expose the rendered tree through
`GuiRuntime::accessibility_tree()` for tests, protocol inspection, and native
accessibility integration. Runtime export overlays interaction state, so changed
values, checked state, selection, focus, read-only state, multiple-selection
mode, and host node ids are visible to protocol consumers.
Invisible target widgets, and descendants of invisible widgets, suppress native
events before interaction state or action routing and are omitted from rendered
accessibility tree projection. Invisibility currently includes HTML `hidden`,
CSS `display: none`, `visibility: hidden` / `collapse`,
`content-visibility: hidden`, and closed intrinsic dialogs. Inert widgets use
the same subtree event and accessibility suppression; both the HTML `inert`
attribute and CSS `interactivity: inert` feed that native inert state.
Disabled target widgets suppress user activation, value, selection, toggle, and
keyboard events before interaction state or action routing, while focus and blur
events can still update inspection state when a host reports them.
Read-only target widgets suppress value, selection, and toggle events after
keyboard activation normalization, while focus, blur, press, and explicit
keyboard events can still route to actions.
ListBox selection projection uses that multiple-selection flag to keep existing
selected children in multi-select lists while single-select lists follow the
latest selected child. On the first render without prior focus history, the
first renderable `autoFocus` control initializes runtime focus state for
accessibility projection. Real native surfaces also receive the same
`autoFocus` setter: AppKit and GTK request platform focus after the target is
attached to a root window, while the current WinUI binding records the target
and continues to rely on WinUI focus callbacks because `winio-winui3` 0.4.2
does not expose a safe programmatic focus method. Non-focus interaction state is
revision-scoped: after a successful rerender, controlled values from the new
blueprint supersede stale local event state while focus remains preserved until
a blur or another focus event arrives. Later native events on that node also
rebase their interaction baseline from the latest blueprint before applying
local changes.

CSS style attribute text is parsed by `src/css_text.rs` before it enters
`WebProps`. The parser splits declarations only on top-level CSS separators, so
URLs, strings, bracketed values, and function arguments can contain `:` or `;`
without corrupting the declaration map. It also strips top-level `!important`
priority markers from values and applies important declarations after normal
declarations.

For embedded Rust hosts, native backends can expose callbacks through
`NativeEventSource`. `GuiRuntime::dispatch_pending_native_events()` drains the
host event queue, applies interaction state updates for every event, and returns
the action invocations from events that have bound Web actions. State-only events
such as focus or blur do not interrupt the batch.
`GuiRuntime::handle_pending_native_event_results()` keeps the same drain boundary
but returns one result per native event, including optional action invocations
and the interaction state changes caused by that event. Those handled event
results are serializable for host logging and process boundaries. Protocol hosts
can still send explicit `HostEvent` records. Keyboard event values can carry the
platform key or shortcut token. Use the `handle_*` event APIs when the caller
needs per-event results, including events that only update runtime or
accessibility state.
`NativeRuntimeApp` builds on that embedded runtime path: it owns
`GuiRuntime<H>`, drains pending native events from hosts that implement
`NativeEventHost`, applies action invocations to application state through a
reducer, and renders the next `UiFrame` back into the same host after
state-changing events. `handle_pending_native_events_while` stops draining the
current native event batch as soon as the supplied state predicate returns
false, which keeps queued follow-up events from mutating state after an app-level
close action. Platform-native app specializations such as
`AppKitRuntimeApp`, `WinUiRuntimeApp`, and `Gtk4RuntimeApp` add the OS event
pump and use the same bounded drain while stopping their `run_*_while` loops
when the root window closes or the state predicate exits.
Window close lifecycle events use the same action path. `UiFrame.window.onClose`
wraps the rendered root in a native window with an `onClose` event binding, and
`NativeEventKind::Close` dispatches that action id. AppKit and GTK native
surfaces enqueue close events from native window, panel, and dialog callbacks.
WinUI installs a small HWND subclass and enqueues the same event from
`WM_CLOSE`, while still observing the root window handle for app-loop shutdown.

## Host Protocol

The JS/Rust bridge uses serializable protocol types:

- `UiFrame`: a compiled React tree plus action ids.
- `WindowOptions`: optional native window title, close action id, initial
  dimensions, min/max dimensions, and resizable flag for a frame.
- `RenderedFrame`: the native root node produced by rendering a frame.
- `NativeRenderResponse`: the native root plus the incremental platform
  commands and rendered accessibility tree emitted by a render pass.
- `HostEvent`: a native event emitted by a platform backend.
- `HostEventResponse`: the validated action invocation for that event plus
  any interaction state changes produced while handling it.
- `NativeHostEventResponse`: the optional action invocation, rendered
  accessibility tree, and interaction changes for host events that may only
  update runtime state.
- `NativeProtocolApp`: a reusable host-side state loop that builds `UiFrame`
  values from application state, applies action invocations through a reducer,
  and returns follow-up render responses after state-changing events.
- `NativeRuntimeApp`: the embedded equivalent for Rust-owned native hosts; it
  handles queued native events, applies reducer updates, and rerenders directly
  into the owned `GuiRuntime`.

The protocol decouples input generation from platform backend execution.
JavaScript does not see native widget handles; native backends receive
serialized protocol records. The same rule applies to native rendering
commands: platform hosts receive serializable command records and blueprints,
not component runtime instances.

`NativeProtocolSession` is a serializable host boundary for a native platform
process. It owns a `GuiRuntime<PlatformPlanningHost<_>>`, accepts `UiFrame`
values, returns only the new `PlatformCommand` records for that render pass,
and dispatches `HostEvent` values back to registered action ids. A native
host can keep one session per native surface or window and apply the returned
commands on its UI thread. Rendering a new `UiFrame` replaces the session's
registered action scope with that frame's `actions` list after the native render
succeeds, so stale action ids from earlier frames cannot be invoked by later
host events and failed renders keep the previous action scope. If raw protocol
JSON omits `actions`, Rust infers action ids from compiled event props and
optional `actionLabels`, matching the default TypeScript `createUiFrame`
behavior. Explicit `actions`, including an empty list, remain authoritative.

When `UiFrame.window` is present, the Rust core wraps the compiled React root in
a `NativeRole::Window`. The same command stream then creates `NSWindow`,
`Microsoft.UI.Xaml.Window`, or `gtk::ApplicationWindow` before inserting the
frame content as its child. Window title, initial dimensions, min/max
dimensions, and the resizable flag are projected into the native blueprint/config
path; the resizable value is also retained as Web metadata for hosts that still
read protocol attributes.

## Native Commands

`PlatformPlanningHost` records the exact commands a real platform backend needs
to execute:

- `{"type": "create", "id": ..., "blueprint": ...}`
- `{"type": "update", "id": ..., "blueprint": ...}`
- `{"type": "insertChild", "parent": ..., "child": ..., "index": ...}`
- `{"type": "remove", "id": ...}`
- `{"type": "setRoot", "id": ...}`

This keeps React reconciliation in the Rust core and keeps platform adapters
focused on native object lifetime and thread-affine UI work.
`NativeWidgetBlueprint` carries the platform family (`appKit`, `winUI`, `gtk4`),
native widget class, native role, accessibility role, label/value/action
bindings, semantic control state, Web metadata, event bindings, and parsed
portable style tokens. It is safe to send to a platform process or language
binding as JSON.
Portable style parsing stores normalized CSS declarations, CSS custom
properties, and Tailwind variant declarations before projecting supported values
into native style tokens. Recognized Tailwind utility classes are applied first
and inline style declarations second, preserving inline style precedence.
Unmapped CSS declarations are retained in `PortableStyle::unsupported` and in
the raw blueprint style map.
Container roles with `overflow`, `overflow-x`, `overflow-y`, logical overflow,
or Tailwind overflow utilities set to `auto` or `scroll` are planned as native
scroll shells. AppKit uses `NSScrollView` with an internal `NSStackView`, WinUI
uses `ScrollViewer` with an internal `StackPanel`, and GTK4 uses
`gtk::ScrolledWindow` with an internal `gtk::Box`. Children continue to mount
inside the internal container, while the outer scroll widget is what the parent
native view owns.
Native bindings can call `blueprint.config()` to derive a `NativeWidgetConfig`
with setter-oriented values such as `enabled`, `visible`, `placeholder`,
range bounds, range step state, selected/checked state, read-only state,
multiple-selection state, autofocus state, text-entry hints, text-length hints,
textarea sizing hints, window resizability, event action ids, metadata, and
portable style. This keeps AppKit, WinUI, and GTK bindings from reinterpreting
protocol fields differently.
`NativeWidgetConfig::diff()` returns a `NativeWidgetConfigPatch` for update
passes, and `HandleWidgetDriver` stores the last config for each handle so
`NativeHandleAdapter::update_handle_config()` can apply only changed setters.
`NativeWidgetConfig::create_setters()` and `NativeWidgetConfigPatch::setters()`
produce `NativeWidgetSetter` operations such as `SetLabel`, `SetEnabled`,
`SetVisible`, `SetPlaceholder`, `SetMinimum`, `SetMaximum`, `SetCurrent`,
`SetStep`, `SetWindowResizable`, `SetEvents`, `SetPortableStyle`, and
`SetMetadata`. Platform bindings can map those operations to the corresponding
AppKit, WinUI, or GTK property setters.
The feature-gated handle adapters keep a replayable setter log in their handle
state, so tests exercise the same create/update flow real native bindings will
map to OS controls.

`CommandExecutingHost` wraps this command stream around a
`PlatformCommandExecutor`. `DriverCommandExecutor` is the reusable executor for
real native backends: it validates that a blueprint targets the driver's backend
and delegates command effects to `NativeWidgetDriver`. If a backend command
fails, the host restores the planning snapshot for that command so failed
creates or updates do not leave the planned tree ahead of the native backend.
Create commands must introduce a new host id; duplicate create ids are rejected
before native handles or recorded backend objects are replaced.

`NativeWidgetDriver` is the OS-binding contract. A platform layer implements:

- `create_widget(id, blueprint)`
- `update_widget(id, blueprint)`
- `insert_child(parent, child, index)`
- `remove_widget(id)`
- `set_root_widget(id)`

If the platform layer owns event callbacks, it also implements
`NativeEventSource::take_native_events()` and queues `NativeEvent` records such
as press, change, focus, key down, key up, and selection change.

That keeps command decoding, backend validation, and render order in shared Rust
while letting AppKit, WinUI, and GTK bindings own their thread-affine handles and
native callback registration.

For platform bindings that already expose clonable native handle wrappers,
`HandleWidgetDriver` provides the storage and command glue. The platform only
implements `NativeHandleAdapter`, returning handles from `create_handle` and
applying updates, child attachment, removal, and root assignment to those
handles. This path supports `NSView`, WinUI object, and GTK object wrappers.
Stored handles, configs, and root state are changed only after the adapter
operation succeeds, so backend failures do not desynchronize the shared driver.
Adapters can implement `update_handle_config` to receive the
typed config patch for native setter updates, or use the default full-blueprint
update method while bootstrapping. The feature modules expose `AppKitHandleAdapter`,
`WinUiHandleAdapter`, and `Gtk4HandleAdapter` surfaces for that path.
For platform bindings that prefer a setter-first SDK boundary,
`NativeWidgetSurface` is the lower-level contract. It creates one native widget
handle from a blueprint, applies each `NativeWidgetSetter`, inserts native
children, removes native widgets, and sets the root surface. `SurfaceHandleAdapter`
wraps that lower-level surface in `NativeHandleAdapter`, so real SDK bindings can
focus on calls such as `setTitle`, `setEnabled`, `setText`, `addSubview`,
`Append`, or `append` while the shared Rust driver still owns reconciliation,
config diffing, command order, and event routing.
The included `RecordingBackend` applies commands to a pure Rust object tree.

Feature-gated platform executor surfaces:

- `appkit`: maps classes such as `NSButton`, `NSTextField(input)`,
  `NSSearchField`, `NSSecureTextField`, and `NSTextField(textarea)` into
  AppKit object kinds behind `AppKitWidgetDriver` and replays native setter
  operations through `AppKitHandleAdapter`.
- `appkit-native`: uses `objc2` AppKit bindings on macOS to create real
  `NSWindow`, `NSPanel`, `NSView`, `NSButton`, `NSTextField`, `NSSwitch`, `NSStackView`,
  `NSComboBox`, `NSScrollView`, `NSTabView`, `NSTabViewItem`,
  `NSBox(separator)`, `NSSlider`, and `NSProgressIndicator` objects through
  `AppKitNativeSurface`.
  Window and panel setters apply content size plus min/max size constraints
  from `UiFrame.window` and portable style tokens.
  The shared renderer still owns
  reconciliation and config diffs; the surface applies typed
  `NativeWidgetSetter` operations directly to AppKit controls. The backend
  creates in-process AppKit controls directly. `NSButton` controls are
  wired to Objective-C
  target/action callbacks that enqueue `NativeEventKind::Press` records, and
  editable `NSTextField` controls, including native `NSSearchField` search
  inputs and textarea-shaped fields, use an `NSTextFieldDelegate` to apply
  max-length hints and enqueue focus, change, and blur records, with change
  events carrying the current native string value. AppKit text fields also map
  portable text-entry hints into native spell-checking, correction, completion,
  inline prediction, automatic text completion, and character-picker settings
  when the control responds to the matching selector. `NSButton(checkbox)` and
  `NSSwitch` controls apply native
  checked state and enqueue `NativeEventKind::Toggle` records with the current
  boolean value. `RadioGroup` uses native `NSStackView` containers with
  `NSButton(radio)` children, and selected radios apply native checked state.
  `NSComboBox` controls receive `ListBoxItem` children as native
  AppKit object values and enqueue `NativeEventKind::SelectionChange` records.
  Independent semantic `ListBox` trees create native `NSScrollView`
  containers with AppKit row controls and enqueue selection-change records with
  the selected item value.
  Semantic `Toolbar` trees create native `NSStackView` containers for tool
  controls.
  Semantic `Dialog` trees create native `NSPanel` windows whose content views
  are populated with native children.
  Semantic `Popover` trees create native `NSPopover` overlays whose content
  view controllers hold native children.
  Semantic `Tabs` trees fold `TabList` and ordered `TabPanel` children into
  native `NSTabViewItem` objects whose content views are the panel views;
  `NSTabViewDelegate` callbacks enqueue tab selection-change records.
  Semantic `Menu` trees create native `NSMenu` objects with `NSMenuItem`
  children; root menus are installed as the application main menu, and menu item
  target/action callbacks enqueue press records.
  `Separator` creates native `NSBox` separators. `NSSlider` controls apply
  native orientation and step hints and enqueue ranged `NativeEventKind::Change`
  records with the current double value, while `NSProgressIndicator` is updated
  by setter-driven ranged state.
  `autoFocus` requests are deferred until the target view is attached to a
  window, then applied with `makeFirstResponder` so the opened window starts on
  the intended native control when AppKit accepts the responder.
  The AppKit event pump also maps key-down and key-up `NSEvent` records to the
  focused native node, normalizes common AppKit key codes, and falls back to the
  root node for window-level keyboard routes.
  `AppKitRuntimeApp` provides the embedded app loop for this backend: it
  renders into an `AppKitNativeSurface`, pumps AppKit events, drains queued A3S
  native events, runs the application reducer, and rerenders the next frame.
  The loop also observes root AppKit window and panel close notifications,
  enqueues `NativeEventKind::Close` for `window.onClose` actions, and lets
  `run_appkit_while` stop when the user closes the surface.
  The `appkit_controls` example is the AppKit instance of the shared native
  controls smoke harness for text input, toggles, sliders, selects, tabs,
  actions, rerenders, and root-window close exit.
  The `appkit_dogfood` example runs the shared task editor and review workflow
  with menu commands, dialog visibility, checklists, disabled gates, read-only
  status, keyboard shortcuts, window close lifecycle actions, and
  state-driven app loop exit through `run_appkit_while`.
  These event paths flow through the existing `NativeEventSource` boundary.
- `winui`: maps classes such as `Microsoft.UI.Xaml.Controls.Button`,
  `Microsoft.UI.Xaml.Controls.TextBox`,
  `Microsoft.UI.Xaml.Controls.TextBox(search)`,
  `Microsoft.UI.Xaml.Controls.PasswordBox`, and
  `Microsoft.UI.Xaml.Controls.TextBox(textarea)` into WinUI object kinds behind
  `WinUiWidgetDriver` and replays native setter operations through
  `WinUiHandleAdapter`.
- `winui-native`: uses `winio-winui3` and the Windows App SDK on Windows to
  create real WinUI 3 `Window`, `StackPanel`, `TextBlock`, `Button`, `TextBox`,
  `CheckBox`, `RadioButton`, `ComboBox`, `ListBox`, `ContentDialog`, `ToolTip`,
  `Grid`, `TabView`, `TabViewItem`, `Border(separator)`, `Slider`, and
  `ProgressBar` objects through
  `WinUiNativeSurface`. The backend creates WinUI controls directly. WinUI
  windows apply initial `UiFrame.window.width`/`height` with `SetWindowPos`,
  min/max resize bounds through `WM_GETMINMAXINFO`, and
  `UiFrame.window.resizable` through HWND style updates so fixed and bounded
  windows match AppKit and GTK behavior.
  textarea-shaped text boxes enable return input and wrapping. WinUI text boxes
  map portable text-entry hints into native spell-check, text-prediction,
  programmatic keyboard display, and color-font settings where the current
  `winio-winui3` bindings expose those APIs. WinUI callbacks enqueue press,
  change,
  focus, blur, toggle, selection-change, and ranged value events through the
  same `NativeEventSource` and action routing path as AppKit and GTK. Semantic
  `Tabs` trees fold `TabList` and ordered `TabPanel` children into native
  `TabViewItem` objects and route WinUI tab selection changes with the
  serialized selection value when one is available. `Separator` uses a native
  XAML `Border` loaded through WinUI's markup runtime. Semantic `Toolbar` trees
  create horizontal
  native `StackPanel` containers, keeping toolbar children as real XAML
  controls. Semantic `Dialog` trees create native `ContentDialog` controls
  whose content is populated with real XAML children. Semantic `Popover`
  trees create ToolTip-backed native overlay surfaces with real XAML children
  because `winio-winui3` 0.4.2 does not expose a strong `Flyout` binding yet.
  Semantic `Menu` trees use native XAML `StackPanel` menu surfaces with native
  `Button` menu items while `winio-winui3` 0.4.2 lacks strong `MenuFlyout` and
  `MenuBar` bindings.
  The semantic `Switch` role keeps
  its native `Switch` semantic in the IR; with `winio-winui3` 0.4.2, the native
  surface temporarily backs that state with a WinUI `CheckBox` because the
  generated bindings do not expose `ToggleSwitch` yet.
  The WinUI message pump also maps `WM_KEYDOWN`, `WM_KEYUP`,
  `WM_SYSKEYDOWN`, and `WM_SYSKEYUP` records into A3S keyboard events. Since
  `winio-winui3` 0.4.2 does not expose strong `KeyRoutedEvent` registration
  methods, keyboard events target the currently focused A3S node tracked from
  WinUI focus callbacks, falling back to the root surface for window-level
  routes. The same binding version also leaves programmatic `Focus` unwrapped,
  so `autoFocus` is tracked as a pending target and cleared only when WinUI
  reports matching native focus.
  Window close requests are observed through the HWND message path: the surface
  installs a close-event subclass on each WinUI window and enqueues
  `NativeEventKind::Close` when `WM_CLOSE` arrives.
  `WinUiRuntimeApp` provides the embedded app loop for this backend: it renders
  into `WinUiNativeSurface`, pumps the Windows message queue, drains queued A3S
  native events, runs the application reducer, rerenders the next frame, and
  observes the root WinUI window handle so `run_winui_while` can stop when the
  user closes the surface.
  The `winui_controls` example runs the same shared native controls smoke
  frame against real WinUI widgets.
  The `winui_dogfood` example runs the shared task editor and review workflow
  through the same reducer loop and WinUI event queue, including window close
  lifecycle actions and state-driven app loop exit through `run_winui_while`.
- `gtk4`: maps classes such as `gtk::Button`, `gtk::Entry`,
  `gtk::SearchEntry`, `gtk::PasswordEntry`, `gtk::SpinButton`, and
  `gtk::TextView` into GTK object kinds behind `Gtk4WidgetDriver` and replays
  native setter operations through `Gtk4HandleAdapter`.
- `gtk4-native`: uses `gtk4-rs` on Linux to create real
  `gtk::ApplicationWindow`, `gtk::Box`, `gtk::Label`, `gtk::Button`,
  `gtk::Entry`, `gtk::SearchEntry`, `gtk::PasswordEntry`, `gtk::SpinButton`,
  `gtk::TextView`, `gtk::CheckButton`, `gtk::Switch`, `gtk::DropDown`,
  `gtk::ListBox`, `gtk::ListBoxRow`, `gtk::Dialog`, `gtk::Popover`,
  `gtk::PopoverMenuBar`, `gio::Menu`, `gio::MenuItem`, `gtk::Notebook`,
  `gtk::Separator`, `gtk::Scale`, and `gtk::ProgressBar` objects through
  `Gtk4NativeSurface`.
  Semantic `Toolbar` trees use native `gtk::Box(toolbar)` containers.
  Semantic `Dialog` trees use native `gtk::Dialog` windows and content areas.
  Semantic `Popover` trees use native `gtk::Popover` overlays with native
  GTK children.
  Semantic `Menu` trees use native `gio::Menu` models, `gio::MenuItem`
  children, `gtk::PopoverMenuBar` surfaces, and `gio::SimpleAction` activation
  callbacks.
  Semantic `Tabs`
  trees become native notebook pages with source tab labels, native panel
  widgets, and selection-change events carrying the selected tab value when
  available. GTK text entries, search entries, password entries, spin buttons,
  and textarea-shaped `TextView` controls apply the relevant read-only and
  sizing setters. Search/password/text entries and `TextView` controls also
  apply placeholder and max-length where GTK exposes that behavior. GTK spin
  buttons preserve numeric range, current, and step state and emit change
  events with the current numeric value. GTK `Entry` and `TextView` controls
  also map portable text-entry hints such as `inputMode`, `input type`,
  `autoCapitalize`, `autoCorrect`,
  `virtualKeyboardPolicy`, and `spellCheck` into GTK input purpose and input
  hint flags. It is a Linux-only feature so macOS and Windows builds can enable
  all portable features without linking GTK. Linux builds require GTK4
  development libraries and `pkg-config`.
  GTK callbacks enqueue press, text change, focus, blur, toggle, and
  selection-change events through the same `NativeEventSource` and action
  routing path as AppKit. GTK widgets also attach `EventControllerKey`
  controllers so native key-down and key-up events carry normalized key values
  through that same queue. `autoFocus` requests are deferred until the target
  widget is attached, then applied with `grab_focus` when GTK accepts the focus
  change.
  `Gtk4RuntimeApp` provides the embedded app loop for this backend: it registers
  a GTK application, renders into `Gtk4NativeSurface`, pumps the default GLib
  main context, drains queued A3S native events, runs the application reducer,
  rerenders the next frame, and observes root window/dialog close notifications.
  Those close callbacks enqueue `NativeEventKind::Close` for `window.onClose`
  actions and let `run_gtk4_while` stop when the user closes the surface.
  The `gtk4_controls` example runs the same shared native controls smoke frame
  against real GTK4 widgets.
  The `gtk4_dogfood` example runs the shared task editor and review workflow
  through the same reducer loop and GTK event queue, including window close
  lifecycle actions and state-driven app loop exit through `run_gtk4_while`.

Menu-specific native backend code lives under `src/native_backends/`:
`native_backends/appkit/menu.rs` owns AppKit menu parent/child tracking,
`native_backends/winui/menu.rs` owns the WinUI menu fallback policy, and
`native_backends/gtk4/menu.rs` owns GTK menu models, menu item actions, and
model rebuilds.

Source files are grouped by layer. `src/backend/` owns the command executor,
handle driver, surface adapter, recording backend, and backend traits.
`src/platform/` owns native blueprint types, config diffs, platform adapters,
planning, and widget-name mapping. Platform-specific native surfaces live in
`src/appkit_native/`, `src/gtk4_native/`, and `src/winui_native/`; each surface
directory keeps its module entry point separate from the `NativeWidgetSurface`
implementation, and WinUI keeps setter and event helpers in a dedicated helper
module.

Style normalization lives under `src/style/`. `src/style/mod.rs` is the module
entry point. `portable.rs`, `apply.rs`, `shorthands.rs`, `composition.rs`,
`tailwind_apply.rs`, and `declarations.rs` split the `PortableStyle` data model
from CSS property projection and declaration bookkeeping. `types.rs` owns the
portable style value types, and `parsing.rs` owns CSS property parsers.
Tailwind utility-to-declaration mapping is split under `src/style/tailwind_utilities/`
by utility family. `src/style/value_parsing.rs` owns length and time token
recognition, including CSS custom properties, CSS math functions, and units that
must be preserved instead of converted eagerly. `src/style/color_parsing.rs`
owns hex, RGB/HSL, modern CSS color function, and background-shorthand color
recognition.

## Style Contract

`WebProps` keeps the original Web style map. `PortableStyle` keeps normalized
CSS declarations and projects the subset that native adapters can apply
deterministically:

- `all`
- `display`, including inline, block, flow-root, contents, list-item, flex,
  grid, table, ruby, and representable multi-keyword display modes
- `boxSizing`, `boxDecorationBreak`, `isolation`, `mixBlendMode`
- `float`, `clear`, `verticalAlign`
- `tableLayout`, `borderCollapse`, `borderSpacing`, `captionSide`
- `position`, `anchorName`, `anchorScope`, `positionAnchor`,
  `positionArea`, `positionTry*`, `positionVisibility`, `inset`,
  `insetBlock*`, `insetInline*`, `start`, `end`, `top`, `right`, `bottom`,
  `left`, `zIndex`
- `visibility`
- `flexDirection`, `flexWrap`, `flex`, `flexBasis`, `flexGrow`,
  `flexShrink`, `order`, `readingFlow`, `readingOrder`
- `alignItems`, `alignContent`, `alignSelf`, `justifyContent`,
  `justifyItems`, `justifySelf`, `placeContent`, `placeItems`, `placeSelf`
- `grid`, `gridTemplate*`, `gridAutoColumns`, `gridAutoRows`, `gridAutoFlow`,
  `gridColumn*`, `gridRow*`, `gridArea`
- `contain`, `container*`, `content`, `counterReset`, `counterIncrement`,
  `counterSet`, `quotes`, `stringSet`, `contentVisibility`, and
  `containIntrinsic*`
- `width`, `height`, `inlineSize`, `blockSize`, `interpolateSize`, and
  physical/logical min/max sizes
- `gap`, `rowGap`, `columnGap`, physical and logical `padding*`, `margin*`,
  `marginTrim`, and Tailwind `space*` child-spacing metadata
- `border`, physical and logical `borderWidth`, `borderStyle`,
  `borderColor`, uniform, physical-corner, and logical-corner `borderRadius`
- `borderImage`, `borderImageSource`, `borderImageSlice`,
  `borderImageWidth`, `borderImageOutset`, and `borderImageRepeat`
- `color`, `background`, `backgroundColor`, `backgroundImage`, `backgroundPosition`,
  `backgroundSize`, `backgroundRepeat`, `backgroundAttachment`,
  `backgroundOrigin`, `backgroundClip`, `backgroundBlendMode`
- `clip`, `clipPath`, `mask*`, and `maskBorder*`
- `imageRendering`, `imageOrientation`, `imageResolution`, `objectFit`,
  `objectPosition`, `shapeOutside`, `shapeInside`, `shapeMargin`,
  `shapePadding`, `shapeImageThreshold`,
  `listStyleType`, `listStylePosition`, `listStyleImage`, `markerSide`,
  `columns`, `columnCount`, `columnWidth`, `columnRule*`, `columnSpan`,
  `columnFill`, `size`, `page`, `pageOrientation`, `bleed`, `marks`,
  `orphans`, `widows`, `bookmark*`, `footnote*`, `breakBefore`, `breakAfter`,
  `breakInside`
- `font`, `fontFamily`, `fontStyle`, `fontSize`, `fontSizeAdjust`,
  `fontWeight`, `fontStretch`, `fontWidth`, `fontPalette`, `fontLanguageOverride`,
  `fontKerning`, `fontOpticalSizing`, `WebkitFontSmoothing`,
  `MozOsxFontSmoothing`, `fontFeatureSettings`, `fontVariationSettings`,
  `fontVariant*`, `fontSynthesis*`, `lineHeight`, `lineHeightStep`,
  `blockStep*`, `lineGrid`, `lineSnap`, `boxSnap`, `mathDepth`, `mathShift`,
  `mathStyle`, `dominantBaseline`,
  `baselineSource`, `alignmentBaseline`, `baselineShift`, `lineFitEdge`,
  `inlineSizing`, `initialLetter*`,
  `letterSpacing`, `wordSpacing`, `tabSize`, `textAlign`, `textAlignAll`,
  `textAlignLast`, `textGroupAlign`, `textJustify`, `wordSpaceTransform`,
  `textSizeAdjust`, `WebkitTextSizeAdjust`,
  `MozTextSizeAdjust`, `MsTextSizeAdjust`, `direction`, `unicodeBidi`,
  `writingMode`, `textOrientation`,
  `textCombineUpright`, `textTransform`, `textIndent`, `textWrap`,
  `textWrapMode`, `textWrapStyle`, `wrapBefore`, `wrapAfter`, `wrapInside`,
  `linePadding`, `textSpacing`, `textSpacingTrim`, `textAutospace`,
  `textBox*`, `hangingPunctuation`, `lineClamp`, `blockEllipsis`,
  `continue`, `maxLines`, `boxOrient`
- `speak`, `speakAs`, `pause*`, `rest*`, `cue*`, and `voice*`
- SVG presentation properties such as `fill`, `fillOpacity`, `fillRule`,
  `clipRule`, `stroke`, `strokeWidth`, `strokeLinecap`, `strokeLinejoin`,
  `strokeMiterlimit`, `strokeDasharray`, `strokeDashoffset`, `strokeOpacity`,
  `vectorEffect`, `paintOrder`, `shapeRendering`, `textRendering`,
  `colorRendering`, `colorInterpolation`, `colorInterpolationFilters`,
  `marker*`, `stopColor`, `stopOpacity`, `floodColor`, `floodOpacity`, and
  `lightingColor`
- `textDecorationLine`, `textDecorationColor`, `textDecorationStyle`,
  `textDecorationThickness`, `textDecorationSkip*`, `textUnderlineOffset`,
  `textUnderlinePosition`, `textEmphasisStyle`, `textEmphasisColor`,
  `textEmphasisPosition`, `textEmphasisSkip`, `rubyAlign`, `rubyPosition`, `rubyMerge`,
  `rubyOverhang`, `textShadow`, `textOverflow`, `lineBreak`, `whiteSpace`,
  `whiteSpaceCollapse`, `whiteSpaceTrim`, `wordBreak`, `overflowWrap`,
  `hyphens`, `hyphenateCharacter`, and `hyphenateLimit*`
- `overflow`, `overflowX`, `overflowY`, `overflowBlock`, `overflowInline`,
  `overflowClipMargin`, `overflowAnchor`
- `emptyCells`
- `aspectRatio`, `boxShadow`, Tailwind `ring*` and `divide*` metadata,
  `outline*`, `transform`, `filter`,
  `backdropFilter`
- `translate`, `rotate`, `scale`, `transformOrigin`, `transformStyle`,
  `transformBox`, `offset*`, `backfaceVisibility`, `perspective`,
  `perspectiveOrigin`
- filter function components such as blur, brightness, contrast, drop shadow,
  grayscale, hue rotate, invert, saturate, and sepia
- backdrop-filter function components such as backdrop blur, brightness,
  contrast, grayscale, hue rotate, invert, opacity, saturate, and sepia
- `transition*`, `animation*`, `animationTimeline`, `animationRange*`,
  `viewTransition*`, `willChange`
- `colorScheme`, `forcedColorAdjust`, `printColorAdjust`, `colorAdjust`,
  `fieldSizing`, `appearance`, `accentColor`, `caretColor`, `caret`,
  `caretAnimation`, `caretShape`, `resize`
- `scrollBehavior`, `scrollTimeline*`, `viewTimeline*`, `timelineScope`,
  physical and logical `scrollMargin*`,
  `scrollPadding*`, `scrollSnap*`, `scrollbarGutter`, `scrollbarWidth`,
  `scrollbarColor`, `scrollInitialTarget`, `scrollTargetGroup`,
  `scrollMarkerGroup`, `overscrollBehavior*`, `overscrollBehaviorBlock`,
  `overscrollBehaviorInline`, `touchAction`, `nav*`, `spatialNavigation*`, and
  `interactivity`
- `cursor`, `pointerEvents`, `userSelect`
- `opacity`

CSS custom properties are stored separately. Tailwind variant utilities are
stored under `variant_declarations` so state or responsive processing can apply
them without reparsing `className`. Tailwind important utilities using the `!`
modifier are evaluated after normal utilities within the same `className`, with
the original relative order preserved inside each priority group. Tailwind
arbitrary values decode `_` as a space, preserve escaped `\_` as an underscore,
keep underscores inside `url(...)` values, and apply the same bracketed-segment
decoding to arbitrary variant keys. Unsupported style declarations are preserved
so callers can report unmapped declarations without dropping source data.
CSS length values that cannot be converted to numeric points or percentages are
kept as `StyleLength::Css`, including `calc(...)`, `calc-size(...)`,
`var(...)`, `clamp(...)`, `anchor(...)`, `anchor-size(...)`,
CSS math functions such as `round(...)`, `hypot(...)`, and `abs(...)`,
viewport/container units, and sizing keywords such as `min-content`.
CSS time values that cannot be converted to milliseconds are kept as
`StyleTime::Css`, including custom properties and CSS math functions.
CSS colors parse hex, RGB/RGBA, HSL/HSLA, and slash alpha syntax into portable
RGBA tokens when possible. Native CSS color functions such as `hwb(...)`,
`lab(...)`, `lch(...)`, `oklab(...)`, `oklch(...)`, `color(...)`,
`color-mix(...)`, `light-dark(...)`, `contrast-color(...)`, and
`alpha(...)`, and `device-cmyk(...)` are preserved as function color tokens.
Tailwind color opacity modifiers are preserved for both base and variant
utilities, including arbitrary color functions.
Common Tailwind visual-effect and interaction utilities such as `shadow-*`,
`shadow-(...)`, `shadow-(color:...)`, `inset-shadow-*`, `ring-*`,
`inset-ring-*`, `outline-*`, `cursor-*`, `pointer-events-*`, `select-*`,
`aspect-*`, `mix-blend-*`, `bg-blend-*`, and `mask-*` project into the same
declaration model.
Tailwind container marker utilities such as `@container`, `@container-size`,
and named container forms project into container declarations. Container query
variants such as `@md:` are stored in `variant_declarations`.
Common Tailwind formatting and table utilities such as `box-*`,
`box-decoration-*`, `isolate`, `isolation-auto`, `float-*`, `clear-*`,
`align-*`, `border-collapse`, `border-separate`, `border-spacing-*`, and
`caption-*`, plus arbitrary `empty-cells` and `border-image*` properties,
project into the same declaration model. Tailwind display
utilities such as `inline-block`, `flow-root`, `contents`, `list-item`,
`table-*`, `inline-table`, `inline-flex`, and `inline-grid` project into
portable display tokens. Arbitrary `display` properties project into the same
display token when the display value has an equivalent portable mode. Tailwind
screen-reader utilities such as `sr-only` and `not-sr-only` project into their
generated declaration groups.
Common Tailwind SVG presentation utilities such as `fill-*`, `stroke-*`, and
`stroke-{width}`, plus arbitrary SVG marker, rendering, paint server, and
filter color properties, project into the same declaration model.
Common Tailwind transform utilities such as `translate-*`, `scale-*`,
`rotate-*`, `skew-*`, `origin-*`, `perspective-*`, `backface-*`, and
`transform-*`, plus arbitrary `transform-box` and CSS Motion Path properties,
project into individual transform properties or the transform function
pipeline.
Common Tailwind filter and backdrop-filter utilities such as `blur-*`,
`brightness-*`, `contrast-*`, `drop-shadow-*`, `grayscale`, `hue-rotate-*`,
`invert-*`, `saturate-*`, `sepia-*`, and `backdrop-*` project into composable
filter tokens.
Common Tailwind Grid utilities such as `grid-cols-*`, `grid-rows-*`,
`auto-cols-*`, `auto-rows-*`, `grid-flow-*`, `col-*`, and `row-*` project into
the same declaration model.
Common Tailwind Flexbox item and box-alignment utilities such as `flex-*`,
`basis-*`, `grow-*`, `shrink-*`, `order-*`, `content-*`, `self-*`,
`justify-items-*`, `justify-self-*`, `place-*`, and arbitrary `reading-*`
properties project into the same declaration model.
Common Tailwind sizing and child-spacing utilities such as `size-*`,
`space-x-*`, `space-y-*`, `space-x-reverse`, and `space-y-reverse` project
into the same declaration model.
Common Tailwind typography and text utilities such as `font-*`, `italic`,
`not-italic`, `antialiased`, `subpixel-antialiased`, `tracking-*`,
`font-stretch-*`, `font-features-*`, arbitrary `font`, `font-width`,
`font-size-adjust`, `font-palette`, and `font-language-override` properties,
font variant numeric utilities, arbitrary rhythmic sizing and line-grid
properties such as `line-height-step`, `block-step*`, `line-grid`,
`line-snap`, and `box-snap`, arbitrary MathML math properties such as
`math-depth`, `math-shift`, and `math-style`, `tab-*`, text transform utilities, text decoration
utilities, `underline-offset-*`, arbitrary `text-decoration-skip*` and
`text-underline-position` properties, arbitrary `text-emphasis-*` properties,
arbitrary `text-size-adjust`, `text-combine-upright`, `text-align-last`,
`text-align-all`, `text-group-align`, `text-justify`, baseline and
initial-letter properties, `text-wrap-*`, arbitrary `wrap-*`,
`line-padding`, `text-spacing`, `text-spacing-trim`, `text-autospace`,
`word-space-transform`,
`text-box*`, `white-space-collapse`, `white-space-trim`,
`hanging-punctuation`, hyphenation limit properties, and line-clamp
longhand properties,
arbitrary CSS Speech properties such as `speak`, `speak-as`, `pause`, `rest`,
`cue`, and `voice-*`,
arbitrary `ruby-*`
properties, `truncate`, `text-ellipsis`, `text-clip`, `indent-*`, `line-clamp-*`,
`text-shadow-*`, `text-wrap`, `text-nowrap`,
`text-balance`, `text-pretty`, `whitespace-*`, `wrap-*`, word-break utilities,
`hyphens-*`, generated-content utilities such as `content-[...]`,
`content-(...)`, and `content-none`, and arbitrary `counter-*`, `quotes`, and
`string-set` properties project into the same declaration model.
CSS writing-mode arbitrary property utilities, CSS Anchor Positioning arbitrary
properties such as `anchor-name` and `position-area`, and `ltr:`/`rtl:`
variants are stored in the same declaration model.
Arbitrary `all` properties project into cascade reset metadata.
Common Tailwind background, object, list, columns, and fragmentation utilities
such as `bg-*`, `object-*`, `list-*`, `list-image-*`, `columns-*`,
`break-before-*`, `break-after-*`, and `break-inside-*`, plus arbitrary CSS
background shorthand, image, and shape properties such as `background`,
`image-rendering`, `shape-outside`, `shape-inside`, and `shape-padding`,
and arbitrary paged media and list
properties such as `page`, `orphans`, `widows`, and `marker-side`, plus bookmark
and footnote properties, plus paged media `size`, `page-orientation`, `bleed`,
and `marks`, project into the same declaration model.
Tailwind border radius utilities such as `rounded-*`, `rounded-t-*`,
`rounded-r-*`, `rounded-b-*`, `rounded-l-*`, `rounded-tl-*`,
`rounded-tr-*`, `rounded-br-*`, `rounded-bl-*`, `rounded-s-*`,
`rounded-e-*`, `rounded-ss-*`, `rounded-se-*`, `rounded-ee-*`, and
`rounded-es-*` project into physical or logical corner radius tokens.
Tailwind border width, color, and divide utilities such as `border-*`, `border-x-*`,
`border-y-*`, `border-t-*`, `border-r-*`, `border-b-*`, `border-l-*`,
`border-s-*`, `border-e-*`, `border-bs-*`, `border-be-*`, `divide-x-*`,
`divide-y-*`, `divide-*-reverse`, `divide-{color}`, and `divide-{style}`
project into physical, logical, or native child-divider tokens.
Common Tailwind motion, interaction, and scroll utilities such as
`transition-*`, `duration-*`, `delay-*`, `ease-*`, `animate-*`,
arbitrary animation and scroll-driven animation properties such as
`animation-composition`, `animation-timeline`, `scroll-timeline`, and
`view-timeline`, top-layer `overlay` metadata, arbitrary CSS View Transitions
properties, `will-change-*`, `appearance-*`,
`accent-*`, `caret-*`, arbitrary `caret`, `caret-animation`, and
`caret-shape` properties, `resize-*`,
`scheme-*`, `forced-color-adjust-*`, arbitrary `print-color-adjust`,
`field-sizing-*`, `scroll-*`, `snap-*`,
`scrollbar-*`, `scrollbar-gutter-*`, `scrollbar-thumb-*`,
`scrollbar-track-*`, `overscroll-*`, arbitrary logical overflow, overflow clip
margin, scroll anchoring, scroll initial target, scroll target groups, scroll
marker groups, logical overscroll, CSS UI directional navigation, CSS Spatial
Navigation, CSS UI interactivity, and `touch-*` properties project into the
same declaration model.
Tailwind logical direction utilities such as `start-*`, `end-*`, `ms-*`,
`me-*`, `mbs-*`, `mbe-*`, `mis-*`, `mie-*`, `ps-*`, `pe-*`, `pbs-*`,
`pbe-*`, `pis-*`, `pie-*`, `scroll-ms-*`, `scroll-me-*`, `scroll-mbs-*`,
`scroll-mbe-*`, `scroll-ps-*`, `scroll-pe-*`, `scroll-pbs-*`, and
`scroll-pbe-*` project into logical portable style tokens.
