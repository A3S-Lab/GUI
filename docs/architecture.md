# a3s-gui Architecture

`a3s-gui` accepts structured UI input and emits native platform commands for
AppKit, WinUI, and GTK4 backends.

The input contract is structured RSX element data: protocol element records,
semantic element names, `className`, inline style objects, `aria-*`, `data-*`,
and DOM-style event props. Supported `semantic_ui` component names are treated
as semantic identifiers. The renderer consumes A3S Native UI IR; host adapters
create platform widgets directly.

```text
Rust ComponentCx function + optional RSX view template
        |
        v
parse_rsx / RsxComponent
        |
        v
CompiledRsxNode records
        |
        v
UiFrame protocol
        |
        v
RsxCompilerBridge
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
portable protocol. Input records come from Rust `ComponentCx` functions,
registered `.rsx` view templates, `RsxComponent` hook registrations, or Rust
code that builds the same protocol shape.

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
- common RSX event prop names such as `onClick`, `onChange`, `onInput`,
  `onFocusChange`, `onExpandedChange`, `onKeyDown`, and `onKeyUp` with named
  callback functions, normalized into native actions

Non-portable runtime assumptions:

- direct DOM access, for example `document.querySelector`
- measuring browser layout APIs as the source of truth
- relying on arbitrary CSS selectors that cannot be expressed as native style
  tokens
- treating an `HTMLElement` instance as an application object

## Component Definition Registry

Built-in `rsx_ui` definitions are compiled once per process. The default
registry is a `LazyLock<GuiResult<ComponentRegistry>>`; ordinary
`RsxComponent` and `ComponentCx` constructors receive a cheap clone after the
first initialization instead of recompiling every built-in template for every
page. `ComponentRegistry` stores templates, contracts, and class variants in
separate immutable `Arc` maps. Clones share those maps, and registering an
application-specific definition uses `Arc::make_mut` to detach only the map
being changed. The built-in registry builder itself uses bare compilation, so
initialization cannot recursively request the default registry.

The constructor family makes registry ownership explicit:

- With `design-system` enabled, `RsxComponent::new`, `from_source`,
  `from_file`, and `from_template`, and `ComponentCx::compile`, install the
  shared default registry. An `authoring`-only build uses an empty default and
  therefore has no hidden `rsx_ui` dependency.
- The corresponding `*_bare` APIs, including `ComponentCx::compile_bare`, do
  not install built-in `rsx_ui` definitions.
- The `*_with_registry` APIs and `ComponentCx::compile_with_registry` accept an
  explicitly assembled `ComponentRegistry`. `builtin_component_registry()` is
  available when an application wants to extend the shared defaults once and
  reuse the resulting copy-on-write registry across its pages.

This is a runtime ownership boundary, not a second component model. All three
paths produce the same compiled RSX tree and enter the same semantic/native
lowering pipeline.

## Dependency Direction

Cargo features enforce the authoring/runtime split inside the current crate.
`authoring` enables SWC-backed RSX parsing and `ComponentCx`; `design-system`
depends on `authoring` and enables the built-in `rsx_ui` registry. The default
feature set keeps the existing authoring experience, while
`cargo check --no-default-features --lib` proves that the runtime, protocol,
semantic mapper, renderer, and planning core compile without SWC or `rsx_ui`.

The remaining dependencies stay one-way:

- `ComponentCx`, RSX parsing, and `rsx_ui` authoring compile outward-facing
  syntax into `CompiledRsxNode`; native backends never execute component
  functions.
- `rsx_ui` depends on semantic component contracts. The semantic mapper,
  native IR, renderer, and platform planning layers do not depend on the
  built-in design-system registry.
- `src/native.rs`, reconciliation, and `src/platform/` remain independent of OS
  widget handles. AppKit, GTK4, and WinUI surfaces depend on those portable
  types and are selected behind their target/feature boundaries.
- `src/backend/` executes planned commands but does not own product state or
  product I/O. The serializable host boundary carries data and command records,
  never component runtime instances or thread-affine native handles.
- `src/effect.rs` defines an executor seam without depending on Tokio or another
  application runtime. An application may inject such an executor at the outer
  edge.

Product applications depend on these layers; the GUI core must not import
product modules, storage clients, network clients, or process runners.
New product configuration and capability policy use ACL (`.acl`) exclusively.
The application or host owns ACL parsing and validation, then passes typed,
immutable capability grants across the outer boundary. The GUI core owns the
capability types but must not depend on the ACL parser or its AST.

## Native Input Evidence Boundary

`NativeCapabilities` describes the behavior a backend claims for each role.
`NativeInputConformanceManifestV1` turns native press claims into a canonical
role/scenario matrix, including activation, cancellation, disabled-state, and
keyed-rerender cases for each applicable input modality. This derivation keeps
capability declarations and test scope in one source of truth.

An operating-system runner records semantic events with
`NativeInputConformanceObservationV1::capture` and submits a versioned
`NativeInputConformanceRunV1`. Capture excludes event values and raw key data.
The verifier compares observations against the generated manifest, checks
provenance and environment identity, and produces a serializable
`NativeInputConformanceReportV1`. Headless and adapter-kernel traces are
explicitly ineligible to prove a native claim even when their event sequence is
identical.

File I/O remains outside the GUI core. The
`a3s-gui-native-input-conformance` binary is a thin adapter that prints current
manifests and reads evidence JSON for CI. Native backends own the OS automation
driver and event-loop integration; the portable verifier owns only the shared
semantic contract.

The Windows-only `a3s-gui-winui-input-smoke` harness keeps automation outside
the runtime library. It mounts each button-backed semantic role as a real WinUI
Button, confirms that the mounted native role matches the manifest case,
locates and verifies its enabled state through the OS UI Automation tree,
injects mouse and Enter-key input with `SendInput`, injects pen and touch with
`CreateSyntheticPointerDevice` / `InjectSyntheticPointerInput`, and invokes its
assistive `InvokePattern`. Mouse cancellation releases the real XAML pointer
capture after the injected press; pen and touch cancellation inject Windows'
cancelled-up pointer state. The production WinUI message loop records successful,
cancelled, keyed-rerender, and disabled responses. The harness validates all 70
observations for `Button`, `DisclosureSummary`, `Link`, `ImageMapArea`, and
`MenuItem` with the strict manifest verifier while allowing only `ListBoxItem`
and `TreeItem` to remain `MissingObservation`. Its output therefore remains an
explicitly partial run artifact rather than a conformance report. The harness
requires an interactive Windows desktop and the Windows App Runtime 1.7
framework package used by the WinUI backend's dynamic dependency.

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
that a platform backend applies. The adapter blueprint boundary applies the same
native prop normalization used by the renderer, so direct planning calls and
rendered command streams share value clamping, step snapping, and invalid
numeric-value omission semantics.

The compiler bridge accepts the HTML element registry exposed by `HTML_ELEMENTS`.
Each recognized intrinsic tag lowers to the closest native semantic role, and
the original tag is preserved as `data-a3s-html-tag` metadata. Document,
metadata, template, slot, text, text-annotation, phrasing-text, flow-text,
legacy-text, legacy-frame, fallback-content, math, selected-content, heading,
heading-group, ruby annotation, landmark, sectioning, disclosure, figure,
description-list, form, form-grouping, option-group, output, meter, list,
dialog, menu, media, embedded-content, link, image-map, and table-structure
tags lower to dedicated native roles. `input[type=range]` lowers to a native
slider role, numeric `value` or `defaultValue` props and attributes are
projected as the ranged current value, and numeric `step` is projected as
native ranged-control step state. `input[type=number]` lowers to the native
text-field role while numeric `value` or `defaultValue`, `min`, `max`, and
`step` props and attributes are preserved as native control state; the text
value is also retained for text-field backends. GTK planning maps that
number-shaped text field to `gtk::SpinButton`.
`input[type=button]`, `input[type=submit]`,
`input[type=reset]`, and `input[type=image]` lower to native button roles with
HTML fallback labels from `value`, default submit/reset labels, or image `alt`
text. `input[type=hidden]` keeps its form `name`, `form`, `type`, and `value`
state in the native IR but is marked hidden so platform widget config,
rendered accessibility projection, and native event routing treat it as
invisible. Value-bearing `input` and `textarea` `value` or `defaultValue`
attributes are projected as initial native values when no top-level protocol
value is supplied; button-like `input` controls keep using `value` as their
fallback label instead. HTML dialog `open` state projects into a structured
`HtmlDialogProps` value and controls native widget visibility for intrinsic
dialog elements through the same derived `visible` config used by the platform
surfaces; `HtmlDialogProps` remains the protocol metadata for replay and
authoring semantics. `input` and `textarea` `placeholder` attributes, plus
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
`rows`, `cols`, and `size` only affect native sizing when they are positive
integers, so zero-valued form sizing attributes remain metadata without
shrinking native controls.
Text-field values are clamped to `maxLength` before initial render, before
rerender updates, and when native change events arrive.
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
and `HtmlCollectionProps` under `src/html/`; native and semantic UI builder
methods for those grouped hints are split into dedicated submodules.
Generic HTML containers lower to `NativeRole::View`; unsupported custom elements
with a hyphenated tag name also lower to a generic native view.
The SVG element registry exposed by `SVG_ELEMENTS` follows the same lowering
path for vector and icon RSX trees. Recognized SVG tags lower to generic native
views or text nodes and preserve the original tag as `data-a3s-svg-tag`
metadata.

`GuiRuntime` is the public orchestration API. It accepts compiled RSX trees,
supported semantic component trees, or native IR trees and renders them into any
`NativeHost`. `InteractionState` updates platform-independent focus, press,
long-press, movement and its latest incremental delta, value, checked,
selected, and expanded state from native events before action routing.
`EventRouter` maps native events such as press, long-press, move lifecycle,
collection action, change, focus, toggle, selection change, key down, and key
up back to serialized action identifiers such as `onClick`, `onLongPress`,
`onMoveStart`, `onMove`, `onMoveEnd`, `onChange`, `onInput`, `onFocusChange`,
`onAction`, `onExpandedChange`, `onKeyDown`, and `onKeyUp`.
Focus and blur events always carry canonical `true` and `false` payloads for
`onFocusChange`. These direct focus callbacks are target-only. The runtime
separately derives `onFocusWithin`, `onBlurWithin`, and
`onFocusWithinChange` from the focused target and its `relatedTarget`, routing
them only across entered or exited subtree boundaries. Consecutive native
blur/focus events are linked before dispatch so focus movement between sibling
descendants preserves their ancestor's focus-within state. Toggle events for
checked or expanded controls canonicalize
native boolean strings such as `1`, `0`, `on`, and `off`; invalid toggle
payloads fall back to the next state derived from the current rendered or
interactive state. Boolean change events for checked controls use that same
canonical payload contract before action dispatch.
When a key-down event has no explicit `onKeyDown` binding on the target or its
route ancestors, Enter and Space fall back to the primary press action for
activatable controls such as buttons, links, and menu items. For stateful
controls without an explicit key-down handler on that route, keyboard
activation is normalized before routing: Space toggles checkboxes and switches,
Space selects radios, and Enter or Space toggles expanded controls and selects
listbox items or tabs.
Before that activation fallback, the mounted selection registry resolves
collection navigation for ListBox, Menu, Tree, Tabs/TabList, and RadioGroup.
Arrow keys follow the collection orientation and inherited text direction;
Home and PageUp move to the first focusable item, while End and PageDown move to
the last. Fully disabled items are skipped, items disabled only for selection
remain focusable, and `shouldFocusWrap` controls end-to-start navigation except
for Tabs and RadioGroup, which wrap by default. Focus movement is issued through
the same constrained `RequestFocus` capability as imperative navigation.
Replace-selection ListBox and Tree collections select as focus moves, toggle
collections move focus only, radio navigation selects, and Menu navigation is
focus-only. Tabs select on focus unless `keyboardActivation="manual"` is set.
Shift extends multiple selection; Control or Command moves focus without
selection. Multiple ListBox and Tree collections map Control/Command+A to the
explicit `all` selection and Escape to an empty selection. Escape defaults to
that clear behavior, can be released with `escapeKeyBehavior="none"`, and still
honors `disallowEmptySelection`. These commands route through the collection
root as complete stable-key snapshots, including normal action failure rollback.
Explicit `onKeyDown` bindings on the target route take precedence over all
built-in collection handling. Page movement deliberately uses collection
boundaries until portable native layout geometry is available.
Collection `onAction` is a separate semantic channel from selection. The
mounted registry projects an internal action marker onto owned ListBoxItem and
TreeItem rows so native adapters capture a press lifecycle without copying the
root callback onto each item. The runtime then bubbles `Action` from the item
to the collection root with the stable item key. Enter invokes action while
Space selects. Mouse replace behavior selects on the first click and invokes
action on the second; touch, pen, and virtual activation invoke action directly.
For touch and pen, holding through the long-press threshold selects the item
and enters a persistent touch-selection mode. Subsequent taps select instead
of acting until the selection becomes empty. The shared recognizer uses an
AppKit `NSTimer`, GTK main-loop timeout, or WinUI
`DispatcherQueueTimer` to deliver the terminal event at the threshold. Release
also evaluates elapsed time as a fallback if a platform timer cannot be
scheduled. Threshold recognition first ends long-press-start, then cancels the
active press and any active move lifecycle before routing the terminal
long-press event, so a later release cannot also activate the item.
When an action gesture also causes a native collection to select a row, a
single-use suppression token restores the mounted stable-key snapshot before
any selection callback is routed. `disabledBehavior="selection"` therefore
blocks selection while retaining action, whereas a fully disabled item blocks
both.
Tree adds a portable hierarchy over that collection kernel. Semantic nested
`TreeItem` nodes are flattened in preorder for native mounting, while stable
parent keys and accessibility level/position/set-size metadata preserve their
logical structure. Controlled `expandedKeys` is reapplied on every render;
`defaultExpandedKeys` seeds state once and then survives keyed rerenders.
Collapsed descendants are projected as hidden host rows and are excluded from
arrow navigation, boundaries, typeahead, focus candidates, and accessibility
projection. In LTR, Right expands a collapsed parent or enters its first visible
child, and Left collapses an expanded parent or focuses its parent. RTL mirrors
those keys. Expansion emits the complete key set through the Tree root's
`onExpandedChange`; host projection and mounted state roll back together if the
action fails. If controlled state hides the focused row during a rerender, focus
moves to the nearest visible ancestor.
Printable keys on ListBox, Menu, Tree, Select, and ComboBox items enter a
500 ms typeahead buffer owned by the mounted collection. Search uses explicit
`textValue` first, then the accessible label and value, and begins at the
current item before wrapping. ICU4X applies the inherited BCP 47 locale with
search collation at primary strength, matching React Aria's case- and
accent-insensitive behavior without reducing international text to ASCII.
The buffer is retained with stable collection identity across keyed rerenders.
Control and Command shortcuts do not enter it; fully disabled items are
skipped, while items disabled only for selection can still receive focus but
do not emit a selection action. AppKit reads the produced character, GTK4
prefers the GDK Unicode scalar over its symbolic key name, and WinUI uses
`ToUnicode` with the active keyboard layout and the non-mutating flag before
falling back to a stable virtual-key name. Full IME and dead-key composition
remains an adapter conformance gap.
AppKit keeps logical item identity separate from the collection selection
target: a row button is registered as the ListBoxItem or TreeItem responder,
while activation reports the selected value to the owning collection.
Programmatic item focus resolves that logical id to the current concrete row,
and row reconstruction restores the responder without changing logical focus.
GTK4 and WinUI mount Tree roles through focusable list/list-item primitives for
this flattened native projection. AppKit rebuilds its row list without hidden
descendants. Hierarchy and expansion semantics remain in the portable keyed
runtime, so adapter widget choice does not change the keyboard contract.
Native event routing tries the target widget first, then mounted ancestors, so
child widget callbacks can reach container-level handlers. Native `Close` is
target-scoped so closing a nested dialog cannot also close its containing
window. A selection-container
array is authoritative: it replaces the previous selection after native values
are resolved to stable element keys, then projects the complete state to every
affected row before action dispatch. GTK4 and WinUI native ListBox callbacks
produce these arrays from all currently selected rows. AppKit sends the row
value with its modifier context and the portable selection manager produces the
same aggregate snapshot. Scalar selection values remain compatibility input.
Selection events with missing or empty values are normalized from the selected
child's value or label; native selection-container events also infer the current
selected child when the host callback does not include a useful payload.
Text-field change values are clamped to `maxLength`. Initial, rerendered, and
event-provided number-input and ranged-control values are clamped to min/max
range bounds and snapped to step hints before platform rendering, interaction
state, or action dispatch. Missing, empty, non-finite, or otherwise
unparseable initial numeric values are omitted from native value state; matching
change payloads are treated as no-ops for number inputs and ranged controls,
preserving the last valid value. Protocol hosts and native backends therefore
share the same value contract.
Empty event action ids are ignored rather than dispatched.
`ActionRegistry` records and validates non-empty action ids before they are
handed to the Rust state reducer. Hosts that implement
`AccessibilityTreeHost` can expose the rendered tree through
`GuiRuntime::accessibility_tree()` for tests, protocol inspection, and native
accessibility integration. Runtime export overlays interaction state, so changed
values, checked state, selection, focus, read-only state, multiple-selection
mode, and host node ids are visible to protocol consumers.
Single-selection containers with no explicit value derive their rendered
accessibility value from the selected or checked child, keeping initial
selection state and later native selection callbacks on the same value shape.
Invisible target widgets, and descendants of invisible widgets, suppress native
events before interaction state or action routing and are omitted from rendered
accessibility tree projection. Invisibility currently includes HTML `hidden`,
CSS `display: none`, `visibility: hidden` / `collapse`,
`content-visibility: hidden`, and closed intrinsic dialogs. Inert widgets use
the same subtree event and accessibility suppression; both the HTML `inert`
attribute and CSS `interactivity: inert` feed that native inert state.
Disabled target widgets, and descendants of disabled widgets, suppress user
activation, value, selection, toggle, and keyboard events before interaction
state or action routing, while focus and blur events can still update
inspection state when a host reports them.
Read-only target widgets suppress value, selection, and toggle events after
keyboard activation normalization. Read-only selection containers also suppress
value-changing events from selectable descendants they own, while focus, blur,
press, and explicit keyboard events can still route to actions.
ListBox selection projection uses that multiple-selection flag to keep existing
selected children in multi-select lists while single-select lists follow the
latest selected child. On the first render without prior focus history, the
first `autoFocus` control in a renderable, non-disabled subtree initializes
runtime focus state for accessibility projection. After the complete tree is
mounted and its root is attached, the runtime resolves the actual focusable
target and emits the same typed `RequestFocus` command used by imperative
navigation. `autoFocus` is therefore not a native widget setter. AppKit uses
`makeFirstResponder`, GTK4 uses `grab_focus`, and WinUI calls the fixed
`IUIElement::Focus(Programmatic)` ABI through a small audited adapter because
`winio-winui3` 0.4.2 leaves that method unwrapped.
Imperative focus navigation uses a separate `ProgrammaticFocusHost` capability
and a serialized `PlatformCommand::RequestFocus` operation rather than a
widget-property setter. `GuiRuntime` validates mounted focusability, resolves
first/last/next/previous order, and constrains requests to the active contained
scope before issuing the command. AppKit maps it to `makeFirstResponder`, GTK4
maps it to `grab_focus`, and WinUI maps it to `IUIElement::Focus` with the
programmatic focus state. Native focus callbacks still authoritatively update
interaction state and route focus actions. When that callback omits modality,
the matching imperative request supplies a one-shot `virtual` modality so
focus-visible state remains deterministic. The runtime retains the last native
focus owner across the blur/focus transition, redirects focus events that
escape an active contained scope, and records the pre-mount focus owner for
each restore-enabled scope. When nested scopes unmount, valid restoration
targets unwind from the innermost scope outward through the same imperative
host capability.

## Mounted Overlay Runtime

`MountedOverlayRegistry` is the platform-independent source of truth for open
managed overlays. After keyed reconciliation, `GuiRuntime` synchronizes the
registry from `data-overlay` contracts, preserves already-open overlays in
activation order, and appends newly opened overlays. Persistent sibling nodes
therefore stack by when they became visible rather than by source order.

While any overlay is active, the runtime projects an internal capture marker
to mounted native nodes. The shared native interaction profile translates that
marker into pointer-lifecycle and key-down subscriptions on AppKit, GTK4, and
WinUI. Escape is claimed only by the topmost overlay unless
`isKeyboardDismissDisabled` is set. Outside dismissal records the topmost
overlay on press start and closes it only when release is also outside that
same overlay; intercepted background events do not reach application actions.
Close-on-blur requires a known `relatedTarget` outside the overlay subtree.
All successful dismissal paths synthesize a target-scoped native `Close`
event, which routes to the overlay's `onClose` action without closing an
ancestor layer.

For the topmost active modal, branches outside the modal and any overlays
opened after it are projected inert. Inert projection also removes those
branches from portable accessibility output. Ancestors needed to retain the
native hierarchy stay enabled, but the registry still suppresses interactions
targeted outside the modal foreground. Later portaled overlays are treated as
foreground and may receive focus through an earlier contained scope.

Overlay focus uses the existing `FocusManager` contracts. A newly opened
overlay with `autoFocus` focuses its first tabbable or focusable descendant;
contained focus remains inside the active scope; and unmounting a
restore-enabled overlay returns focus to its valid trigger. `UiDialog`,
`UiModal`, and the default `UiPopover` opt into these behaviors. A nonmodal
popover keeps background branches interactive and does not contain focus,
while still supporting autofocus, restoration, close-on-blur, and Escape.

Anchored overlay positioning is a separate typed host capability. Popover and
Tooltip authoring props serialize a `data-overlay-position` contract containing
placement, main/cross offsets, flip/update policy, boundary padding, arrow
geometry, and maximum height. After lifecycle projection, `GuiRuntime` resolves
each open Popover to an explicit mounted `anchor` id/key, its nearest marked
trigger context, a previous trigger sibling, or a non-window parent. Invalid,
ambiguous, self, and descendant anchors are rejected before platform commands
are emitted. `PlatformPlanningHost` retains the relationship for recovery
replay and suppresses identical commands when `shouldUpdatePosition` is false.
Protocol v1 carries the same validated options in `positionOverlay`.

Non-focus interaction state is
revision-scoped: after a successful rerender, controlled values from the new
blueprint supersede stale local event state while focus remains preserved until
a blur or another focus event arrives. Native focus history is retained even
when the focused node is later removed, so rerenders do not reapply `autoFocus`
after the platform has taken focus ownership. Later native events on that node
also rebase their interaction baseline from the latest blueprint before
applying local changes.

Stateful style projection uses that same revision model. The runtime resolves
supported variant declarations against transient interaction state and current
typed props, preserves declaration-level source order, and caches the last
resolved style per mounted node. A state transition batches focused
`update_portable_style` calls inside a host frame. Planning hosts translate
them to normal blueprint updates and `SetPortableStyle` setters, so AppKit,
GTK4, WinUI, command-executing, and headless hosts share one path. A failed
projection rolls the host frame, interaction state, selection state, and the
previous resolved styles back together. Style requirements also participate in
native event subscription planning, so a visual hover or press variant works
without a user action callback.

CSS style attribute text is parsed by `src/css_text.rs` before it enters
`WebProps`. The parser splits declarations only on top-level CSS separators, so
URLs, strings, bracketed values, and function arguments can contain `:` or `;`
without corrupting the declaration map. It also strips top-level `!important`
priority markers from values and applies important declarations after normal
declarations.

For embedded Rust hosts, native backends can expose callbacks through
`NativeEventSource`. `GuiRuntime::dispatch_pending_native_events()` drains the
host event queue, applies interaction state updates for every event, and returns
the action invocations from events that have bound RSX actions. State-only events
such as focus or blur do not interrupt the batch.
`GuiRuntime::handle_pending_native_event_results()` keeps the same drain boundary
but returns one result per native event, including optional action invocations
and the interaction state changes caused by that event. Those handled event
results are serializable for host logging and process boundaries. Events for
host node ids that no longer belong to the current render tree are treated as
empty handled events, so a stale native callback from a previous frame cannot
abort the remaining drain batch. Protocol hosts can still send explicit
`HostEvent` records. Keyboard event values can carry the platform key or
shortcut token; the runtime normalizes common native key names into canonical
payloads such as `Enter`, `Tab`, `Backspace`, `Escape`, arrow keys, and a single
space for Space. Use the `handle_*` event APIs when the caller needs per-event
results, including events that only update runtime or accessibility state.
`NativeRuntimeApp` builds on that embedded runtime path: it owns
`GuiRuntime<H>`, drains pending native events from hosts that implement
`NativeEventHost`, applies action invocations to application state through a
reducer, and renders the next `UiFrame` back into the same host after
state-changing events. Existing reducers return `GuiResult<()>` and therefore
continue through the complete target-to-ancestor route. Propagation-aware apps
use `new_with_propagation` and return `ActionPropagation` from the reducer. A
`Stop` result is sticky for that native event: every callback registered on the
same `currentTarget` still runs in deterministic order, then callbacks on
ancestors are discarded. The retained invocation prefix is reflected in both
the response and `ActionRegistry` history, and event responses expose
`propagationStoppedAt` for inspection. `NativeProtocolApp` implements the same
contract at the serialized host boundary.

`handle_pending_native_events_while` stops draining the
current native event batch as soon as the supplied state predicate returns
false, which keeps queued follow-up events from mutating state after an app-level
close action. `handle_pending_native_event_batch_while` uses the same behavior
but returns a `NativeRuntimeEventBatch` with host-drain, queued, handled,
buffered, and predicate-stop diagnostics for app-loop logging and native
automation assertions. Unprocessed events are kept inside the app and are
handled before new host events on the next drain. When the predicate is already
false before the drain starts, the host event queue is left untouched so callers
can pause and resume later.
Platform-native app specializations such as
`AppKitRuntimeApp`, `WinUiRuntimeApp`, and `Gtk4RuntimeApp` add the OS event
pump and use the same bounded drain while stopping their `run_*_while` loops
when the root window closes or the state predicate exits. Their
`pump_*_event_batch_while` helpers combine the pre-pump and post-pump A3S event
drains into one `NativeRuntimeEventBatch`, which gives native automation a
single assertion point for OS event pump cycles. Each backend also exposes
`*_with_propagation` constructors and propagation-aware pump/run variants so
the opt-in contract is preserved through the real OS event loop.
Window close lifecycle events use the same action path. `UiFrame.window.onClose`
wraps the rendered root in a native window with an `onClose` event binding, and
`NativeEventKind::Close` dispatches that action id. AppKit and GTK native
surfaces enqueue close events from native window, panel, and dialog callbacks.
WinUI installs a small HWND subclass for root windows and registers
`ContentDialog::Closing` for dialogs, then enqueues the same close event while
clearing the surface's open-dialog tracking so a dismissed dialog can be shown
again on a later state-driven render. When those queued dialog close events are
drained, the surface also releases the retained `ShowAsync` operation for that
dialog. The backend still observes the root window handle for app-loop shutdown.

## Effects and Application Scheduling

Reducers run synchronously on the UI thread and must remain fast state
transitions. They must not read files, call network services, wait for child
processes, create an async runtime, or use `block_on`. Application I/O belongs
outside the reducer in an application-owned `EffectRuntime` (or another
injected executor); completed work returns a message/result that is merged into
state on the UI thread.

`EffectRuntime` provides the implemented supervision boundary:

- in-flight work and the completion channel are both bounded; defaults are 32
  active effects and 256 queued completions, and `with_limits` configures both
- spawning beyond the in-flight limit returns a contextual error, while the
  synchronous completion channel provides backpressure
- `EffectCancellation` supports cooperative cancellation by id; cancelling or
  dropping the runtime prevents later completion delivery and signals all
  active tasks. A cancelled task keeps its in-flight slot until the worker
  reports termination, so a task that ignores cancellation cannot bypass the
  concurrency bound
- the default `ThreadEffectExecutor` catches task panics and converts them into
  `GuiError` completions. It returns a joinable `EffectWorker`; runtime shutdown
  disconnects the bounded completion channel and joins every worker, so even a
  non-cooperative task cannot escape the owning application lifetime
- `EffectExecutor` and `EffectWaker` let applications provide Tokio-backed work
  and wake their owning event loop without adding Tokio to the GUI core. A
  custom executor may return a detached worker only when an external owned task
  scope provides the equivalent shutdown guarantee

`NativeRuntimeApp::with_background_updates` is the UI-thread integration seam.
Its poller drains a batch of completed work, mutates application state, and
returns `BackgroundUpdate`. `poll_background_updates` renders exactly once when
that batch reports any state change, regardless of how many completions it
merged, and separately tracks whether more work remains pending.

The AppKit, GTK4, and WinUI `run_*_while` loops use that pending flag to choose
their event-pump policy. They poll native events while background work remains,
then park for at most one 16 ms frame instead of busy-spinning.
`background_effect_waker()` supplies the matching `EffectWaker`, so a completed
worker unparks the UI thread immediately. When no work is pending the loop uses
the platform's blocking wait. Each iteration polls background completions
before the OS event pump and rechecks the root-window and application exit
predicates before waiting.

## Host Protocol

The RSX/native host boundary uses serializable protocol types:

- `UiFrame`: a compiled RSX tree plus action ids.
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
- `NativeRuntimeEventBatch`: the diagnostic result for embedded pending-event
  drains, including per-event responses plus host-drain, handled, buffered, and
  predicate-stop counts.

The protocol decouples input generation from platform backend execution.
RSX components do not see native widget handles; native backends receive
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
optional `actionLabels`. Explicit `actions`, including an empty list, remain
authoritative.

Protocol v1 is the strict transport contract. Its request, command,
accessibility, control-state, event, and response types are dedicated `*V1`
DTOs with unknown-field rejection; they do not serialize runtime structs behind
opaque JSON values. A session permanently selects either legacy or strict-v1
mode on first use. Strict-v1 validates the protocol version, session id, render
revision, event sequence, command sequence, and acknowledgement before any
state transition. Each rendered response is retained until its exact ACK;
retrying the same revision returns the retained response, while later renders
and events are rejected until delivery is acknowledged. Password values remain
available to in-process reducers but are removed from commands, accessibility,
responses, session debug output, and retained diagnostics.

`NativeProtocolApp` remains the convenience state/reducer loop for the legacy
in-process API. Strict-v1 is deliberately a transport-owned
`NativeProtocolSession` primitive: the transport owns resend/ACK ordering and
the product owns reducer, effect, and next-frame orchestration.

When `UiFrame.window` is present, the Rust core wraps the compiled RSX root in
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

The planning host's command vector is a pending delivery queue, not cumulative
diagnostic history. `take_commands()` moves all commands produced since the
previous drain and leaves the queue empty. Legacy session calls therefore
return only the new commands for each render pass. Strict-v1 moves those
commands into its retained response and does not release that delivery record
until ACK.

Diagnostic histories owned by the runtime are bounded independently. Action
invocations, interaction changes, headless operations, successfully executed
driver/recording commands, and native setter diagnostics default to the most
recent 256 entries. Their limits are configurable, zero disables retention
without disabling dispatch/execution, and the corresponding `take_*` APIs
transfer retained records out and clear the history. Sensitive control values
are redacted before retention. Sensitivity is resolved defensively from the
explicit marker, typed password widget kind, structured input type, or legacy
`metadata["type"]`. Blueprint serialization, `Debug`, command histories, and
setter histories use the same decision and remove duplicated `value`,
`defaultValue`, ARIA-value, and data-value metadata channels. Low-level action
registry calls redact values by default unless the caller explicitly supplies a
public sensitivity classification, while runtime/app `Debug` output reports
types and queue counts rather than live state or event payloads. These histories
exist for inspection and testing; they are not application event stores.
Credential-bearing metadata and resource-policy fields, including CSP nonces
and inline `srcdoc`, are always removed from diagnostic projections regardless
of control value sensitivity; live rendering and protocol input retain them.

This keeps keyed reconciliation in the Rust core and keeps platform adapters
focused on native object lifetime and thread-affine UI work.
`NativeWidgetBlueprint` carries the platform family (`appKit`, `winUI`, `gtk4`),
typed `NativeWidgetKind`, a diagnostic legacy class name, native role,
accessibility role, label/value/action
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
native view owns. Native surface update paths keep the scroll policy, inner
container orientation, spacing, and explicit viewport size in sync across
rerenders.
Native bindings can call `blueprint.config()` to derive a `NativeWidgetConfig`
with setter-oriented values such as `enabled`, `visible`, `placeholder`,
range bounds, range step state, selected/checked state, read-only state,
multiple-selection state, autofocus state, text-entry hints, text-length hints,
textarea sizing hints, window resizability, event action ids, metadata, and
portable style. This keeps AppKit, WinUI, and GTK bindings from reinterpreting
protocol fields differently.
`NativeWidgetConfig::diff()` returns an ordered `NativeWidgetSetterBatch` for
update passes (`NativeWidgetConfigPatch` is only the compatibility alias), and
`HandleWidgetDriver` stores the last config for each handle so
`NativeHandleAdapter::update_handle_config()` can apply only changed setters.
`NativeWidgetConfig::create_setters()` and `NativeWidgetSetterBatch::setters()`
produce `NativeWidgetSetter` operations such as `SetLabel`, `SetEnabled`,
`SetVisible`, `SetPlaceholder`, `SetMinimum`, `SetMaximum`, `SetCurrent`,
`SetStep`, `SetWindowResizable`, `SetEvents`, `SetPortableStyle`, and
`SetMetadata`. Platform bindings can map those operations to the corresponding
AppKit, WinUI, or GTK property setters.
The feature-gated handle adapters keep a bounded, redacted setter history in
their handle state, so tests exercise the same create/update flow real native
bindings map to OS controls without accumulating secrets or unbounded logs.

`CommandExecutingHost` wraps this command stream around a
`PlatformCommandExecutor`. A render frame checkpoints planning, prepares its
complete immutable `PlatformCommandBatch`, commits it, validates the explicit
`PlatformBatchAck`, and only then consumes the acknowledged queue. Prepare
failure preserves the exact frame and native state. A commit or invalid ACK is
conservatively treated as potentially partial: the host enters
`DegradedNativeState`, rejects incremental work, rolls planning back to the last
acknowledged snapshot, and requires `recover_with_executor` to replay a full
create/insert/set-root snapshot into a fresh executor before resuming.

`DriverCommandExecutor` is the reusable executor for real native backends: it
validates that a blueprint targets the driver's backend and delegates command
effects to `NativeWidgetDriver`. Platform executors must own their native
resources and release them when dropped; recovery drops the old partial
executor before replaying into the replacement so two surfaces are not active
at once. Real-platform failure-injection and teardown automation remain a
hardening gate in addition to the deterministic Rust executor tests.
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
  max-length hints and textarea sizing hints, including rerenders that remove
  `rows`/`cols` and return to the native default size. They enqueue focus,
  change, and blur records, with change events carrying the current native
  string value. AppKit text fields also map portable text-entry hints into
  native spell-checking, correction, completion,
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
  are populated with native children; panels are presented from native
  visibility state instead of being inserted into parent stack layout.
  Semantic `Popover` trees create native `NSPopover` overlays whose content
  view controllers hold native children. Typed positioning selects an explicit
  anchor `NSView`, resolves logical placement against direction, and projects a
  positioning point, preferred edge, main offset, and cross offset.
  Semantic `Tabs` trees fold `TabList` and ordered `TabPanel` children into
  native `NSTabViewItem` objects whose content views are the panel views;
  `NSTabViewDelegate` callbacks enqueue tab selection-change records.
  Semantic `Menu` trees create native `NSMenu` objects with `NSMenuItem`
  children; top-level menus are installed as the application main menu instead
  of being inserted as views, and menu item target/action callbacks enqueue
  press records.
  `Separator` creates native `NSBox` separators. `NSSlider` controls apply
  native orientation and step hints and enqueue ranged `NativeEventKind::Change`
  records with the current double value, while `NSProgressIndicator` is updated
  by setter-driven ranged state.
  Runtime `autoFocus` commands arrive after the target view is attached to a
  window and use `makeFirstResponder`, so the opened window starts on the
  intended native control when AppKit accepts the responder.
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
  apply `maxLength` updates, including rerenders that remove the limit, clamp
  programmatic text writes and change payloads to the active limit, and saturate
  protocol values at WinUI's signed integer boundary. Text inputs track the
  last controlled value so read-only callbacks can roll native edits back
  without enqueueing change events. They also map portable text-entry hints into
  native spell-check, text-prediction,
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
  whose content is populated with real XAML children; dialog nodes are
  overlay-mounted and shown with `ShowAsync` from native visibility state after
  the root window exposes a `XamlRoot`. Semantic `Popover`
  trees create ToolTip-backed native overlay surfaces with real XAML children
  because `winio-winui3` 0.4.2 does not expose a strong `Flyout` binding yet.
  Position commands bind `PlacementTarget`, a direction-aware placement point,
  signed offsets, and optional maximum height. WinUI remains responsible for
  choosing the final collision side.
  Semantic `Menu` trees use native XAML `StackPanel` menu surfaces with native
  `Button` menu items while `winio-winui3` 0.4.2 lacks strong `MenuFlyout` and
  `MenuBar` bindings.
  The semantic `Switch` role keeps
  its native `Switch` semantic in the IR; with `winio-winui3` 0.4.2, the native
  surface temporarily backs that state with a WinUI `CheckBox` because the
  generated bindings do not expose `ToggleSwitch` yet.
  WinUI pointer handlers opt into handled routed events so Button-backed
  controls preserve mouse, pen, and touch press phases that XAML class handling
  would otherwise hide. Preview key-down and key-up handlers likewise feed the
  shared keyboard state machine before Button activation, avoiding duplicate
  semantic presses from the later `Click` callback. `winio-winui3` 0.4.2 does
  not generate these registration methods, so the native surface isolates the
  fixed WinRT ABI calls in a small adapter. The same binding version leaves
  programmatic `Focus` unwrapped; that call uses the same isolated ABI approach
  while WinUI focus callbacks remain the observable event source.
  Window close requests are observed through the HWND message path: the surface
  installs a close-event subclass on each WinUI window and enqueues
  `NativeEventKind::Close` when `WM_CLOSE` arrives. Content dialogs register
  WinUI's `Closing` callback and enqueue the same event for dialog `onClose`
  actions while clearing local open-dialog tracking. Draining those native close
  events releases the retained `ShowAsync` operation; programmatic hides are
  suppressed during render-driven teardown.
  WinUI surfaces are created inside the XAML-owned `Application::Start`
  lifecycle. `run_winui_application_staged_async` renders and activates the
  first window during `OnLaunched`, then polls the application future through
  `DispatcherQueue` turns after launch returns. `WinUiRuntimeApp` drains queued
  A3S native events, runs the reducer, rerenders the next frame, and observes
  the root window so `run_winui_while_async` can stop when the user closes the
  surface without blocking XAML layout or input dispatch.
  The `winui_controls` example runs the same shared native controls smoke
  frame against real WinUI widgets.
  The `winui_dogfood` example runs the shared task editor and review workflow
  through the same reducer loop and WinUI event queue, including window close
  lifecycle actions and state-driven app loop exit through
  `run_winui_while_async`.
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
  Semantic `Dialog` trees use native `gtk::Dialog` windows and content areas;
  dialog nodes are presented from native visibility state instead of being
  inserted into parent box layout.
  Semantic `Popover` trees use native `gtk::Popover` overlays with native
  GTK children. Position commands reparent the popover to the resolved anchor,
  set its physical side and pointing rectangle, and apply signed main/cross
  offsets; GTK performs final work-area collision handling.
  Semantic `Menu` trees use native `gio::Menu` models, `gio::MenuItem`
  children, `gtk::PopoverMenuBar` surfaces, and `gio::SimpleAction` activation
  callbacks.
  Semantic `Tabs`
  trees become native notebook pages with source tab labels, native panel
  widgets, and selection-change events carrying the selected tab value when
  available. GTK text entries, search entries, password entries, spin buttons,
  and textarea-shaped `TextView` controls apply the relevant read-only and
  sizing setters. `TextView` sizing rerenders clear removed `rows`/`cols` hints
  while preserving axes that are explicitly sized by portable style.
  Search/password/text entries and `TextView` controls also apply placeholder
  and max-length where GTK exposes that behavior. GTK spin
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
  through that same queue. Runtime `autoFocus` commands arrive after the target
  widget is attached and use `grab_focus` when GTK accepts the focus change.
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

Source files are grouped by layer:

- `src/native.rs` and `src/native/` define the platform-neutral native element
  IR: roles, keys, props, and prop builders. They do not create OS widgets.
- `src/platform/` owns native blueprint types, config diffs, platform adapters,
  planning, and widget-name mapping. It answers "what widget class should this
  native element become on this platform?"
- `src/appkit.rs`, `src/gtk4.rs`, and `src/winui.rs` contain lightweight
  platform mapping and handle-driver code. They are useful for planning,
  recording, and tests that should not instantiate real OS widgets.
- `src/appkit_native/`, `src/gtk4_native/`, and `src/winui_native/` are the real
  OS surface implementations. They implement `NativeWidgetSurface`, create
  AppKit/GTK/WinUI widgets, attach native events, and run the embedded app
  loops.
- `src/native_backends/` is private support code for those real native
  surfaces, not a competing backend layer. It currently holds menu-specific
  helpers: `native_backends/appkit/menu.rs` owns AppKit menu parent/child
  tracking, `native_backends/winui/menu.rs` owns the WinUI menu fallback policy,
  and `native_backends/gtk4/menu.rs` owns GTK menu models, menu item actions,
  and model rebuilds.

`src/backend/` owns the command executor, handle driver, surface adapter,
recording backend, and backend traits. Platform-specific native surface
directories keep their module entry point separate from the `NativeWidgetSurface`
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

The detailed portable CSS and Tailwind projection contract lives in
[`style-contract.md`](style-contract.md). Keeping it separate makes the runtime
and protocol architecture easier to review while preserving one authoritative
list of supported native style semantics.
