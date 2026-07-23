# React Aria Native Direction

The long-term goal of `a3s-gui` is to provide the native, cross-platform
equivalent of [React Aria](https://react-aria.adobe.com/getting-started): a
headless behavior and accessibility layer that applications can compose into a
design system without depending on a browser.

This is a behavioral compatibility target, not a DOM compatibility target.
React Aria's public concepts should have recognizable native equivalents, but
the implementation must use AppKit, GTK4, and WinUI controls, focus systems,
accessibility APIs, and input events directly.

The project does not claim React Aria parity yet.

## Target Architecture

```text
Rust component and semantic hook API
                |
                v
Headless behavior contracts
  press / hover / focus / selection / overlays / i18n
                |
                v
Portable semantic tree and typed Native UI IR
                |
                v
Behavior state machines and keyed reconciliation
                |
                v
Versioned platform capabilities
                |
                v
AppKit / GTK4 / WinUI native adapters

Native input and accessibility events travel upward through the same layers.
```

The behavior layer owns platform-independent semantics. Native adapters own
thread affinity, widget lifetime, raw event capture, accessibility projection,
and translation between platform event data and portable event context.

## Required Invariants

- Input modality is normalized once as keyboard, mouse, touch, pen, virtual,
  or unknown. Components must not infer it independently.
- Press is a lifecycle, not a click alias. Start, release, end, cancellation,
  keyboard activation, touch movement, and virtual activation must have
  deterministic semantics.
- Focus visibility depends on modality. Programmatic and keyboard focus can
  display a focus ring without making pointer focus do so automatically.
- Hover is unavailable to touch input and must not be synthesized from a touch
  press.
- Transient interaction state survives keyed rerenders and disappears when its
  node disappears.
- Collection identity is based on stable keys. Selection is a key set or an
  explicit `all` value, not one optional string.
- Focus containment, restoration, and autofocus are runtime behavior, not
  marker attributes.
- Locale and writing direction are inherited context values and can be scoped.
- Unsupported platform behavior is reported through capabilities or an error;
  native setters must not silently ignore a semantic contract.
- Tests assert semantic roles, accessible names, state, and interaction
  results. Platform class names alone are not conformance evidence.

## Implemented Foundation

The first shared interaction milestone is available in the portable runtime:

- `NativeInputModality` represents keyboard, mouse, touch, pen, virtual, and
  unknown input.
- `NativeEventContext` carries modality, key modifiers, local position,
  move/wheel delta, and repeat state. Its fields are optional, so older
  serialized native events remain valid.
- `ActionInvocation` preserves that typed context through action routing, so
  reducers can distinguish keyboard, pointer, touch, pen, and virtual input.
- One native event produces an ordered invocation batch. Lifecycle-specific
  callbacks run before their change callbacks, then the event bubbles from the
  target through its nearest ancestors. `node` and `currentTarget` preserve
  both sides of that relationship. Native `Close` remains target-scoped so a
  nested dialog does not invoke the containing window's close request.
- Propagation-aware reducers return `ActionPropagation::Continue` or `Stop`.
  Stopping completes every callback on the current target, then removes the
  ancestor suffix from the response and action history. Unit-returning reducers
  retain the original continue-all behavior. The protocol app and all three
  native event pumps expose the same opt-in contract.
- Native events include press start, press up, press end, press cancellation,
  long-press start, terminal long press, long-press end, hover start, and hover
  end, plus a signed cross-platform wheel event.
- `use_move` and `UiMovable` emit a real cross-platform move lifecycle. Primary
  pointer motion is incremental, starts only on the first non-zero delta,
  retains the initiating pointer identity, and ends on release or cancellation.
  Arrow keys emit a complete one-unit keyboard lifecycle and suppress native
  default or collection navigation. Move callbacks receive normalized
  modality, modifiers, position, repeat state, and `context.delta`.
- `InteractionState` tracks pressed, long-pressed, hovered, focused,
  focus-within, and focus-visible state. It keeps transient state across keyed
  blueprint synchronization.
- `GuiRuntime` resolves `hover`, `active`, `focus`, `focus-visible`, and
  `focus-within` style variants, plus the corresponding React Aria
  `data-[...]:*` states, against that interaction state. Declaration-level
  source order is retained, and resolved `PortableStyle` updates are committed
  transactionally through the native host rather than waiting for an action
  callback or rerender.
- Keyboard and virtual focus display focus-visible state; pointer presses clear
  it. Touch does not create hover state.
- `use_press`, press-capable semantic hooks, built-in RSX components, and the
  RSX compiler expose `onPressUp` alongside the existing press lifecycle.
- Event routing supports explicit hover lifecycle handlers and falls back to
  `onHoverChange` with canonical boolean values.
- Marked NumberField wheel sources use `context.delta` on AppKit, GTK4, and
  WinUI. Platform sources normalize their wheel signs to the portable
  DOM-style contract before the runtime derives a model-space change.
- Direct `onFocus`, `onBlur`, and `onFocusChange` handlers run only for the
  focused target. `use_focus_within`, `UiFocusWithin`, and the
  `onFocusWithin`/`onBlurWithin`/`onFocusWithinChange` handlers independently
  observe subtree entry and exit. Adjacent native blur/focus events are linked
  through `relatedTarget`, so moving between descendants does not churn an
  ancestor's focus-within state.
- `use_focus` is the React Aria-compatible direct-focus API name;
  `use_focusable` remains as a compatibility alias. `use_focus_ring` and
  `UiFocusRing` include descendant focus only when `within=true`, matching the
  direct-focus default.
- `FocusManager` derives focusable and tabbable order from the mounted keyed
  tree, models nested focus scopes, provides first/last/next/previous
  navigation, resolves containment, and directs scope autofocus to a
  descendant instead of the scope wrapper. Restore-enabled scopes retain the
  focus owner that preceded their mount and unwind nested restoration targets
  when they unmount.
- `MountedOverlayRegistry` discovers open managed overlays from the reconciled
  native tree and retains activation order independently of document order.
  Only the topmost overlay handles Escape or outside-press dismissal. Escape
  remains available to the focused control when keyboard dismissal is
  disabled, and outside dismissal requires both press start and release to
  occur outside the same topmost overlay.
- Modal overlays project `inert` onto background branches, which also removes
  them from the portable accessibility tree. Structural ancestors stay
  available for event routing, and overlays opened later through a separate
  portal branch remain interactive foreground layers.
- Overlay autofocus, contained focus, and restoration reuse `FocusManager`.
  Dismissal emits a target-scoped native `Close` event, so it invokes only the
  topmost overlay's `onClose`. `UiDialog` and `UiModal` expose
  `isDismissable` and `isKeyboardDismissDisabled`; `UiPopover` additionally
  exposes `isNonModal`, closes on focus leaving its subtree, and defaults to a
  modal dismissable popover.
- `use_overlay_position` accepts React Aria's 22 placement strings together
  with `offset`, `crossOffset`, `shouldFlip`, `shouldUpdatePosition`,
  `containerPadding`, arrow geometry, and `maxHeight`. Open mounted popovers
  resolve an explicit `anchor`/trigger reference or their trigger context and
  emit a typed `positionOverlay` command. Logical start/end placement resolves
  against inherited LTR/RTL direction.
- `GuiRuntime` exposes `request_focus`, `focus_first`, `focus_last`,
  `focus_next`, and `focus_previous`. These methods validate mounted
  focusability, apply active-scope containment, and send a typed platform
  command; native focus callbacks remain the source of truth for interaction
  state and focus actions. An incoming native focus event that escapes a
  contained scope is suppressed and redirected through the same host
  capability.
- `KeyedCollection` rejects duplicate identities without mutating the active
  collection. `Selection` represents an explicit key set or `all`, and
  `SelectionManager` implements single/multiple, toggle/replace, range,
  disabled-key, disallow-empty, and async collection-update behavior.
- `use_selection` now exposes `selectedKeys` while retaining the scalar
  `value`/`selectedValue` compatibility path. Selection action payloads decode
  legacy scalar keys, key arrays, and `all` through one typed API.
- Controlled `selectedKeys` and uncontrolled `defaultSelectedKeys` are distinct
  all the way through hook serialization, RSX expansion, reconciliation, and
  mounted state. Omitting both no longer serializes an accidental controlled
  empty selection. Collection components also carry `disabledKeys`,
  `selectionBehavior`, `disabledBehavior`, `disallowEmptySelection`, and
  `shouldFocusWrap`. ListBox and Tree also carry React Aria's
  `escapeKeyBehavior`, defaulting to `clearSelection`; `none` leaves Escape
  unclaimed by the selection contract.
- List box, grid list, tag, tabs, tree, menu, selection input, calendar picker,
  and color swatch picker components reuse the same selection contract.
  `RadioGroup` retains React Aria's scalar `value`/`defaultValue` contract.
- `MountedSelectionRegistry` discovers collection ownership from the mounted
  tree, uses declarative element keys as item identity, rejects duplicate keys
  before host mutation, preserves uncontrolled selection through keyed
  reorders, and projects controlled or `all` selection into native item state.
  Native display values remain compatibility aliases rather than identity.
- Mounted selection events implement toggle/replace and modifier range
  behavior, update every affected sibling, bubble an aggregate stable-key
  payload, and track the collection's focused key. Programmatic projection is
  applied before keyed reconciliation so it does not transiently clear native
  selection during rerenders.
- ListBox, GridList, Tag, and Tree collection roots expose `onAction(key)` as a
  distinct event from `onSelectionChange`. Enter invokes the item action while
  Space selects. Mouse replace behavior uses one click for selection and two
  clicks for action; touch, pen, and virtual taps prefer action. Toggle
  collections with an empty selection also prefer action. Native selection
  callbacks produced by those primary action gestures are suppressed and
  reverted before user callbacks run. Items disabled only for selection can
  still act, while fully disabled items cannot.
- Touch and pen long press select the held item and enter a persistent
  collection selection mode. Later taps select instead of invoking action
  until the selection is cleared. The generic `use_long_press` and
  `UiLongPressable` contracts expose start, terminal, and end callbacks, a
  bounded millisecond threshold (500 ms by default), and an accessibility
  description. AppKit `NSTimer`, GTK main-loop timeout, and WinUI
  `DispatcherQueueTimer`
  deliver the terminal event while the pointer remains held. At recognition,
  the native kernel ends long-press-start and cancels active press and move
  lifecycles before dispatching the terminal callback. Release-time evaluation
  remains as a fallback if platform timer registration fails.
- Native ListBox input is normalized to that same complete snapshot contract.
  GTK4 and WinUI enable their native single/multiple selection modes and report
  all selected row values; the runtime resolves those display aliases to stable
  keys and replaces the previous key set atomically. AppKit row activation
  retains Shift and Command modifiers, then the portable manager produces the
  complete stable-key snapshot. Host projection and mounted state roll back
  together if the selection action fails.
- Mounted ListBox, Menu, Tree, Tabs/TabList, and RadioGroup collections share
  one keyboard-navigation contract. Arrow, Home, End, PageUp, and PageDown
  navigation skips fully disabled items, follows orientation and RTL direction,
  respects optional focus wrapping, and sends typed `RequestFocus` commands on
  AppKit, GTK4, and WinUI. Replace and toggle selection behavior, Shift range
  extension, Control/Command focus-only movement, and automatic versus manual
  tab activation are resolved before selection actions are routed. Multiple
  ListBox and Tree collections use Control/Command+A for `all`; Escape clears
  selection unless `escapeKeyBehavior="none"` is configured, and
  `disallowEmptySelection` still prevents the clear. Both shortcuts emit the
  same complete stable-key payload as pointer selection. An explicit
  `onKeyDown` on the target route retains ownership of the key. PageUp and
  PageDown use a `CollectionLayoutSnapshot` containing the visible rectangle,
  content size, and stable-key item rectangles. Command hosts measure that
  snapshot from AppKit, GTK4, or WinUI immediately before navigation; custom
  and headless hosts can inject it through `GuiRuntime::set_collection_layout`.
  Variable-size rows move by one visible extent and fully disabled rows remain
  excluded. A host that cannot measure layout retains deterministic
  collection-boundary behavior.
- Tree owns controlled `expandedKeys` or uncontrolled `defaultExpandedKeys`
  independently from selection. Nested semantic `TreeItem` nodes lower to
  stable, same-level native rows with parent-key, level, position, and set-size
  metadata. Up, Down, Home, End, page movement, and typeahead operate on visible
  preorder rows only. In LTR, Right expands or enters the first child and Left
  collapses or returns to the parent; RTL mirrors those keys. Expansion routes
  the complete stable-key set through the Tree root's `onExpandedChange`, and a
  failed action rolls both mounted and host state back. Controlled collapse also
  restores focus from a hidden descendant to its nearest visible ancestor.
- ListBox, Menu, Tree, Select, and ComboBox collection items support buffered
  type-to-select using their explicit `textValue`, accessible label, or value.
  The 500 ms buffer survives keyed rerenders, starts at the current item and
  wraps, ignores Control/Command shortcuts, and skips only items disabled for
  all interaction. ICU4X search collation provides locale-sensitive,
  case-insensitive, and accent-insensitive prefix matching. ListBox and Tree
  follow replace-selection rules, while Menu and open Select/ComboBox lists move
  focus without committing a value. AppKit uses the produced character, GTK4
  uses the key's Unicode value, and WinUI translates virtual keys with the
  active keyboard layout without mutating dead-key state.
- Logical AppKit ListBox and Tree items resolve to their concrete row buttons
  for responder lookup and programmatic focus. Selection activation still
  targets the owning collection. Rebuilding a row preserves the focused item,
  and programmatic AppKit focus enqueues the matching blur/focus transition.
  Hidden AppKit tree descendants are removed when the row list is rebuilt.
  GTK4 and WinUI mount Tree through their native list primitives as well; the
  portable hierarchy layer supplies visible flattened rows consistently on all
  three backends.
- `I18nManager` projects inherited locale and writing direction through the
  keyed native tree. Scoped overrides and default locale changes are preserved
  across rerenders, and BCP 47 language/script subtags provide deterministic
  RTL inference. It creates reusable, thread-safe `LocaleCollator`,
  `LocaleNumberParser`, `LocaleNumberFormatter`, and `LocaleDateFormatter`
  values from the effective node locale. Collation covers search/sort
  sensitivity, case-first, numeric ordering, and locale-equivalent prefix,
  suffix, and substring filtering. Stable ICU4X decimal parsing covers
  localized signs and separators, partial-input validation, and automatic
  Latin, Arabic, Han decimal, Devanagari, Bengali, and full-width numbering
  system detection. Decimal and percent styles share typed formatting options
  for grouping, signs, and fraction digits. Percent formatting uses localized
  CLDR affix patterns, scales model values for display, and defaults NumberField
  stepping to `0.01`; parsing converts localized percent input back to model
  space. Number-shaped text fields reuse the parser and formatter for inherited
  locale display before canonical range/step normalization. Date/time formatting
  covers localized short through full styles, seconds, calendar,
  numbering-system, and hour-cycle locale extensions. Collection typeahead
  reuses the public collator filter.
- `use_number_field` exposes separate group, input, increment-button, and
  decrement-button prop contracts. `UiNumberField` projects that anatomy as a
  native group instead of collapsing it into one text field. Buttons are
  excluded from sequential focus, carry the next model-space value, use
  field-aware accessible labels (or explicit `incrementAriaLabel` and
  `decrementAriaLabel` overrides), and disable at step boundaries or for
  disabled/read-only fields. ArrowUp, ArrowDown, PageUp, and PageDown use the
  same minimum-anchored step algorithm, including decimal-noise cleanup;
  Home/End move to the explicit bounds. Modified key combinations are left to
  the application or platform. Focused vertical wheel input uses the same
  model-space stepping, rejects horizontal-dominant trackpad gestures and
  control-wheel zoom, and can be disabled independently with
  `isWheelDisabled`. Mouse presses on either stepper restore focus to the input.
  Mouse and pen steppers fire immediately, repeat after a 400-millisecond
  delay, and continue every 60 milliseconds. Touch steppers defer repeating
  for 600 milliseconds and preserve short-tap activation on release. Leaving,
  cancellation, disabled/read-only updates, and step boundaries stop the
  current cycle; re-entry starts a new one.
  AppKit, GTK4, and WinUI claim handled keys and wheels before the toolkit
  default runs and consume terminal stepper activation, preventing native
  controls from applying a duplicate change.
- Native IR capabilities are versioned. Every host exposes a feature manifest
  with unsupported, portable, or native support levels, role-specific
  overrides, and auditable capability issues. Protocol render responses carry
  both the manifest and concrete gaps.
- Accessibility conformance validates names, focus uniqueness, selection and
  checked states, exclusive-container selection, relationships, and duplicate
  node identity. The same semantic tree assertions run against AppKit, GTK4,
  and WinUI planning adapters. Mounted selection is the source of truth for
  accessible selected/checked state.
- AppKit, GTK4, and WinUI use the same press and keyboard state machines. Their
  view-backed widgets emit pointer press/re-entry/cancellation, hover, focus,
  key, modality, modifier, repeat, and local-position data through one portable
  event contract. A key-up completes the press on the original key-down node
  even if focus moved in between.
- Native control activation is normalized with pre-dispatch context so a
  platform click does not duplicate the portable keyboard lifecycle.
  Programmatic and assistive activation emit the complete virtual lifecycle.
- Mounted native interaction profiles follow `SetAction`, `SetEvents`, and
  `SetPortableStyle` updates without remounting. Callback changes and
  style-driven hover, press, long-press, move, and focus-modality requirements
  therefore update native event capture immediately.
- Native surfaces are split by responsibility: widget creation, updates,
  hierarchy mutation, interaction translation, platform delegates, types, and
  styling/layout no longer share monolithic backend files.

This foundation is covered by serialization, routing, state-machine, rerender,
and built-in RSX component tests.

## Native Capability Boundary

The generic interaction source is now present on all three native backends, but
support is deliberately reported by role rather than inferred from the mere
existence of a platform object:

| Contract | Native coverage |
| --- | --- |
| Complete press lifecycle | Button, disclosure summary, link, image-map area, ListBoxItem, and TreeItem on AppKit, GTK4, and WinUI; WinUI menu items also use the complete lifecycle. |
| Long press | Shared AppKit, GTK4, and WinUI press sources emit start/end and recognize terminal long press after the configured threshold. `NSTimer`, GTK main-loop timeout, and `DispatcherQueueTimer` provide threshold-time delivery, and release-time evaluation is the fallback. |
| Move | AppKit mouse/pen drag events, GTK4 `GestureDrag`, and WinUI mouse/touch/pen pointer capture use one incremental move state machine. All three normalize Arrow keys to a complete keyboard lifecycle and prevent the underlying native default. |
| NumberField stepping | The shared runtime maps ArrowUp/ArrowDown/PageUp/PageDown, Home/End, focused vertical wheels, and stepper presses to model-space `Change` events. Wheel input rejects horizontal-dominant gestures and control-wheel zoom and honors `isWheelDisabled`. Built-in decrement and increment buttons expose the same next values and boundary/read-only state through native Button controls, and mouse presses preserve input focus. AppKit, GTK4, and WinUI share cancellable pointer-hold stepping with immediate mouse/pen activation, delayed touch activation, and 60-millisecond repeats; handled toolkit defaults and terminal native clicks are suppressed. Automatic localized button messages and live value announcements remain incomplete. |
| Native menu activation | AppKit and GTK4 menu items emit terminal press only because their menu models do not expose a mounted generic view event source. |
| Hover and typed modality | View-backed widgets; explicit exceptions are reported for AppKit non-view wrappers/items, GTK4 menu items, and the WinUI window wrapper. |
| Focus within | Portable runtime routing on AppKit, GTK4, WinUI, and headless hosts. Native blur/focus batches are linked with `relatedTarget`; direct focus callbacks remain target-only while focus-within callbacks run only when a subtree boundary is crossed. |
| Interaction style projection | Runtime-resolved hover, press, long-press, move, focus, focus-visible, focus-within, selected, checked, expanded, disabled, validation, read-only, direction, and matching `data-*`/`aria-*` variants use the same transactional `SetPortableStyle` path on all three planning adapters. |
| Focus events, scopes, and `autoFocus` | Native focusable control roles listed in the capability manifest. Runtime navigation, restoration, and post-mount `autoFocus` all emit typed `requestFocus` commands; contained scopes redirect escaping native focus. AppKit uses `makeFirstResponder`, GTK4 uses `grab_focus`, and WinUI calls the fixed `IUIElement::Focus(Programmatic)` ABI through an isolated adapter because the generated binding leaves that method unwrapped. |
| Overlay stack | Activation ordering, topmost Escape and outside-press dismissal, modal background inertness/accessibility suppression, close-on-blur, portaled child overlays, containment, autofocus, and restoration run in the shared mounted runtime. AppKit, GTK4, and WinUI planning adapters receive the same projected props and event subscriptions. |
| Anchored overlay position | `Popover` and `Tooltip` expose one typed positioning contract and versioned command. AppKit maps it to `NSPopover` positioning rectangles and edges, GTK4 maps it to `gtk::Popover` parent/pointing rectangle/position/offset, and WinUI maps it to ToolTip placement target/rectangle/signed offsets. Headless and protocol hosts retain the same anchor relationship. Native collision behavior and exact arrow geometry remain backend-specific and are reported as portable capability coverage. |
| Selection and item action | Select/combo box, list box/tree, and tabs/tab list. GTK4 and WinUI ListBox callbacks provide complete native selection snapshots; AppKit modifier-aware row activation and all stable-key aggregation remain in the portable keyed-runtime layer. ListBox/Tree item `onAction(key)` separation and collection keyboard navigation are shared across adapters. |

`NativeCapabilities` is the executable source of truth. Global entries are
conservative and role overrides opt into verified behavior. This prevents a
native wrapper, menu model, or logical collection item from being advertised as
interactive merely because another role on that backend is interactive.

## Native Input Evidence Gate

`NativeInputConformanceManifestV1::from_capabilities` expands each role whose
press support is marked `Native` into a machine-readable automation matrix. A
complete press-lifecycle role requires separate mouse and pen activation,
cancellation, and disabled cases, plus keyboard, virtual assistive activation,
and keyed-rerender cancellation. GTK4 and WinUI also require touch activation,
cancellation, and disabled-touch cases. AppKit and GTK4 menu items retain an
explicit terminal-activation exception because their menu models do not expose
the generic view event source. The expected successful activation order follows
React Aria's [`usePress`](https://react-aria.adobe.com/usePress) contract:
press start, press up, press end, then terminal press.

The evidence boundary is intentionally strict:

- `NativeInputConformanceObservationV1::capture` retains only redacted semantic
  press events; raw key values and native event payload values are excluded.
- A verifier derives expectations from the current capability manifest rather
  than accepting expectations supplied by an evidence file.
- Exactly one observation is required per case. Event order, target identity,
  modality, keyboard activation deduplication, and click count are checked.
- Only `OperatingSystemAutomation` evidence with matching OS, OS version,
  toolkit version, and automation-driver identity is eligible. Adapter-kernel
  and portable-runtime traces remain useful tests but cannot prove native
  support.
- The native CI matrix publishes the generated AppKit, GTK4, and WinUI
  requirement artifacts so missing platform evidence is explicit and
  reviewable.

Generate or verify artifacts with:

```bash
just native-input-manifest appkit
just native-input-manifest gtk4
just native-input-manifest winui
just native-input-conformance path/to/evidence.json
just winui-input-smoke path/to/winui-smoke.json
```

The WinUI smoke harness covers XAML Button-backed `Button`,
`DisclosureSummary`, `Link`, `ImageMapArea`, and `MenuItem` roles plus
`ListBoxItem` and `TreeItem` inside real list containers. Mouse and keyboard use
Windows `SendInput`, pen and touch use synthetic pointer injection, and
assistive activation uses UI Automation `InvokePattern` or
`SelectionItemPattern` according to the native control contract. It exercises
successful activation, mouse/pen/touch cancellation, keyed-rerender
cancellation, and disabled input inside the production XAML application
lifecycle. Handled routed pointer events and preview key events feed the same
portable press state machine while the asynchronous dispatcher loop leaves
WinUI layout and input dispatch unblocked. The strict verifier accepts the
complete 98-case WinUI manifest with no semantic defect. Native automation
drivers still need to submit complete passing run artifacts on real macOS and
Linux runners.

## Known Gaps

The following behavior systems are still incomplete or only represented as
props:

| Priority | Area | Required outcome |
| --- | --- | --- |
| P0 | Native input conformance | WinUI's complete 98-case V1 manifest passes real OS automation. Populate the AppKit and GTK4 manifests with platform-run mouse, pen, touch where applicable, keyboard, assistive activation, disabled, cancellation, and keyed-rerender fixtures for every role currently marked native; then close or retain evidence-backed menu/item exceptions. |
| P1 | Event propagation | Add platform-run conformance fixtures for conditional `Stop`/`Continue` across nested native controls. |
| P1 | Focus management | Add platform-run conformance fixtures for post-mount `autoFocus`, nested containment, and restoration. |
| P1 | Collections and selection | Complete IME/dead-key typeahead conformance and add real-platform fixtures for layout-aware page navigation. |
| P1 | NumberField interaction | Add automatic localized stepper labels and live value announcements. Group/input/button anatomy, minimum-anchored button and continuous press-and-hold stepping, Arrow/Page/Home/End keyboard semantics, focused vertical-wheel stepping with horizontal/zoom rejection and `isWheelDisabled`, mouse input-focus preservation, decimal-noise cleanup, boundary disabling, cancellation/re-entry, and native-default suppression are implemented. |
| P1 | Internationalization | Add message formatting, currency/unit parsing and formatting, and date ranges/time zones. Reusable decimal/percent parsing and formatting, partial-input validation, locale-aware filtering, and localized NumberField model/display conversion now build on inherited locale/direction. |
| P1 | Accessibility conformance | Complete OS accessibility API projection, relationships, live regions, value announcements, and role-specific native adapter coverage. |
| P2 | Overlays | Complete measured boundary-driven collision and arrow projection, native scroll locking, configurable outside-interaction filters, multi-window layer coordination, and real-platform positioning conformance fixtures. |
| P2 | Capability enforcement | Turn reported capability gaps into adapter policy and conformance gates where portable fallback is not sufficient. |
| P2 | Environment style variants | Add native environment and ancestry evaluators for responsive/container, theme, group, peer, and structural selector variants. These remain preserved in the style IR but inactive at runtime today. |

Adding more component names before these systems exist does not improve
conformance. New components should be built by composing the shared behavior
contracts rather than reimplementing press, focus, hover, selection, or locale
logic.

## Conformance Gate

A behavior is complete only when all of the following are true:

1. Its portable contract and state machine are specified and tested.
2. AppKit, GTK4, and WinUI translate native input into the same observable
   behavior, with documented capability differences where exact parity is not
   possible.
3. Keyboard, pointer, touch, virtual accessibility activation, disabled state,
   cancellation, and rerender cases are covered where applicable.
4. Accessibility role, name, state, relationships, and focus behavior are
   asserted semantically.
5. The public documentation describes the supported contract without implying
   broader parity.
6. A real operating-system automation run satisfies the generated versioned
   manifest; portable or adapter-only tests do not count as native evidence.
