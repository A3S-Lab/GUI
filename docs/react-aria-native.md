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
- `NativeEventContext` carries modality, key modifiers, local position, and
  repeat state. Its fields are optional, so older serialized native events
  remain valid.
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
  end.
- `use_move` and `UiMovable` emit a real cross-platform move lifecycle. Primary
  pointer motion is incremental, starts only on the first non-zero delta,
  retains the initiating pointer identity, and ends on release or cancellation.
  Arrow keys emit a complete one-unit keyboard lifecycle and suppress native
  default or collection navigation. Move callbacks receive normalized
  modality, modifiers, position, repeat state, and `context.delta`.
- `InteractionState` tracks pressed, long-pressed, hovered, focused, and focus-visible state.
  It keeps transient state across keyed blueprint synchronization.
- Keyboard and virtual focus display focus-visible state; pointer presses clear
  it. Touch does not create hover state.
- `use_press`, press-capable semantic hooks, built-in RSX components, and the
  RSX compiler expose `onPressUp` alongside the existing press lifecycle.
- Event routing supports explicit hover lifecycle handlers and falls back to
  `onHoverChange` with canonical boolean values.
- `FocusManager` derives focusable and tabbable order from the mounted keyed
  tree, models nested focus scopes, provides first/last/next/previous
  navigation, resolves containment, and directs scope autofocus to a
  descendant instead of the scope wrapper. Restore-enabled scopes retain the
  focus owner that preceded their mount and unwind nested restoration targets
  when they unmount.
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
  `onKeyDown` on the target route retains ownership of the key. Until native
  layout geometry is available to the portable layer, PageUp and PageDown use
  deterministic collection-boundary behavior.
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
  RTL inference.
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
- Mounted native interaction profiles follow `SetAction` and `SetEvents`
  updates without remounting, so callbacks added or removed during a keyed
  rerender immediately change native event capture.
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
| Native menu activation | AppKit and GTK4 menu items emit terminal press only because their menu models do not expose a mounted generic view event source. |
| Hover and typed modality | View-backed widgets; explicit exceptions are reported for AppKit non-view wrappers/items, GTK4 menu items, and the WinUI window wrapper. |
| Focus events, scopes, and `autoFocus` | Native focusable control roles listed in the capability manifest. Runtime navigation, restoration, and post-mount `autoFocus` all emit typed `requestFocus` commands; contained scopes redirect escaping native focus. AppKit uses `makeFirstResponder`, GTK4 uses `grab_focus`, and WinUI calls the fixed `IUIElement::Focus(Programmatic)` ABI through an isolated adapter because the generated binding leaves that method unwrapped. |
| Selection and item action | Select/combo box, list box/tree, and tabs/tab list. GTK4 and WinUI ListBox callbacks provide complete native selection snapshots; AppKit modifier-aware row activation and all stable-key aggregation remain in the portable keyed-runtime layer. ListBox/Tree item `onAction(key)` separation and collection keyboard navigation are shared across adapters. |

`NativeCapabilities` is the executable source of truth. Global entries are
conservative and role overrides opt into verified behavior. This prevents a
native wrapper, menu model, or logical collection item from being advertised as
interactive merely because another role on that backend is interactive.

## Known Gaps

The following behavior systems are still incomplete or only represented as
props:

| Priority | Area | Required outcome |
| --- | --- | --- |
| P0 | Native input conformance | Add platform-run pointer, keyboard, assistive activation, and cancellation fixtures for every role currently marked native, then close or document remaining menu/item exceptions. |
| P1 | Event propagation | Add platform-run conformance fixtures for conditional `Stop`/`Continue` across nested native controls. |
| P1 | Focus management | Add platform-run conformance fixtures for post-mount `autoFocus`, nested containment, and restoration. |
| P1 | Collections and selection | Add layout-aware page navigation and complete IME/dead-key typeahead conformance. |
| P1 | Internationalization | Add message formatting, number/date formatting, locale-aware collation, and localized interaction behavior on top of inherited locale/direction. |
| P1 | Accessibility conformance | Complete OS accessibility API projection, relationships, live regions, value announcements, and role-specific native adapter coverage. |
| P2 | Overlays | Add dismissal, focus restoration, modal containment, outside interaction, and nested overlay ordering. |
| P2 | Capability enforcement | Turn reported capability gaps into adapter policy and conformance gates where portable fallback is not sufficient. |

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
