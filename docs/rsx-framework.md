# RSX Framework Plan

RSX is a Rust-owned UI language with `ComponentCx` function components and
`.rsx` component source modules. It should feel familiar to React authors without
inheriting the JavaScript runtime, DOM, CSSOM, or browser routing model.

## Direction

The framework should keep these boundaries stable:

- RSX syntax is declarative and statically analyzable.
- Page logic lives in Rust `ComponentCx` hooks: selectors, reducers, effects,
  resources, and router hooks.
- Components are authored as Rust functions that receive
  `&mut ComponentCx<State>` and return `RSX`.
- `.rsx` files are Rust component modules or view templates. They consume
  `state.*`, `props.*`, derived values, context, resources, and action ids
  produced by Rust hooks.
- Native hosts receive compiled UI IR, actions, style tokens, accessibility
  metadata, and platform-neutral control state.
- Familiar intrinsic names are an authoring affordance, not a compatibility
  target or runtime dependency.

## Layers

### Language

The language layer owns parsing `.rsx` sources into `CompiledRsxNode` trees.
It supports view-template wrappers, RSX tags, props, children, fragments, static
bindings, `<Show>`, `<When>`, `<For>`, `<Each>`, slots, spread objects, and
named source diagnostics.

It intentionally rejects arbitrary JavaScript expressions. Dynamic behavior must
be represented as named bindings such as `state.route`, `derived.title`,
`resource.profile.status`, `context.route.id`, `item.title`, or action ids.

### App Model

The app model is the Rust equivalent of React's one-way data flow:

- `ComponentCx` is the preferred function-component authoring API; it records
  hook registrations and compiles the returned `RSX` into an `RsxComponent`.
- `RsxComponent` owns the compiled template and registered hooks used by the
  runtime.
- `cx.use_*` APIs provide the hook vocabulary in explicit Rust context form.
  Some APIs are React-aligned equivalents, while others are A3S native
  extensions for state projection, typed native actions, semantic component
  props, router lifecycle, resources, and accessibility metadata.
- React hook alignment is a naming and mental-model target, not a JavaScript
  runtime contract: `use_selector` is the exact state-selector spelling,
  `use_state` remains compatible, `use_reactive` is an A3S object-binding
  extension, and React commit hooks map to insertion, layout, and passive native
  effect phases.
- State, props, derived, context, resource, and memo hooks build the render
  scope consumed by `.rsx` view-template props and bindings.
- Reducer/action hooks mutate Rust state from native events.
- `use_effect` hooks run after a native frame is committed. Action and
  transition effect hooks run after reducers.
- Mount and unmount hooks initialize and clean up page-local state.
- `RsxRouter` selects active pages, provides route context, runs route
  lifecycle hooks, and supports a persistent layout outlet.

Reducers are synchronous UI-thread state transitions. A reducer must not do
filesystem or network I/O, wait for a child process, create an async runtime,
or call `block_on`. Long-running work belongs in an application-owned
`EffectRuntime` or another injected executor, and its completion is merged back
into state on the UI thread.

`EffectRuntime` bounds both active work and its completion channel (32 and 256
by default), rejects new work at the in-flight limit, and provides cooperative
cancellation. Dropping it cancels every active task. The default
`ThreadEffectExecutor` catches task panics and reports them as error
completions; applications can implement `EffectExecutor` for Tokio or another
runtime and supply an `EffectWaker` for their event loop.

### Component Registry

The built-in component registry is initialized once per process through
`LazyLock`. A `ComponentRegistry` clone shares immutable `Arc` maps for compiled
templates, contracts, and variants. Adding an application definition uses
copy-on-write and detaches only the map that changes, so creating a page does
not recompile or deep-copy the full design system.

Registry selection is part of the authoring API:

- with `design-system` enabled, the standard `RsxComponent` constructors and
  `ComponentCx::compile` use the shared default registry; an `authoring`-only
  build starts with an empty registry
- `*_bare` constructors and `ComponentCx::compile_bare` install no built-in
  `rsx_ui` definitions
- `*_with_registry` constructors and `ComponentCx::compile_with_registry` use
  an explicitly supplied registry, including a cloned and application-extended
  `builtin_component_registry()`

All variants produce the same `RsxComponent`; the choice controls definition
ownership, not rendering semantics.

### Design System

The first design-system layer is `rsx_ui`, a React-inspired Rust RSX component set
backed by the repository root `DESIGN.md`.

This layer makes reusable `.rsx` Rust function components available by default
instead of asking every page to copy long class strings:

Related primitives are grouped by component family in the source tree, such as
`components/card/`, `components/checkbox/`, `components/color/`,
`components/combo_box/`, `components/form/`, `components/radio/`,
`components/slider/`, `components/select/`, `components/tag/`,
`components/tabs/`, `components/text_field/`, `components/toggle/`,
`components/menu/`, `components/collection/`, `components/breadcrumb/`,
`components/feedback/`, `components/typography/`, `components/structure/`,
`components/interaction/`, `components/file/`, and `components/layout/`.

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
- `UiClipboardTarget`
- `UiCollection`
- `UiColorPicker`
- `UiColorArea`
- `UiColorThumb`
- `UiColorField`
- `UiColorSlider`
- `UiColorWheel`
- `UiColorWheelTrack`
- `UiColorSwatch`
- `UiColorSwatchPicker`
- `UiColorSwatchPickerItem`
- `UiComboBox`
- `UiComboBoxValue`
- `UiDateField`
- `UiDateInput`
- `UiDateSegment`
- `UiDatePicker`
- `UiDateRangePicker`
- `UiDescription`
- `UiDialog`
- `UiDialogTrigger`
- `UiDisclosure`
- `UiDisclosureGroup`
- `UiDisclosureSummary`
- `UiDisclosurePanel`
- `UiDraggable`
- `UiDropIndicator`
- `UiDropZone`
- `UiDroppable`
- `UiArticle`
- `UiAside`
- `UiFieldSet`
- `UiFieldError`
- `UiFileTrigger`
- `UiFocusable`
- `UiFooter`
- `UiForm`
- `UiGridList`
- `UiGridListSection`
- `UiGridListHeader`
- `UiGridListItem`
- `UiGroup`
- `UiHeader`
- `UiHeading`
- `UiHoverable`
- `UiInput`
- `UiKeyboard`
- `UiKeyboardTarget`
- `UiLabel`
- `UiLegend`
- `UiLink`
- `UiListBox`
- `UiListBoxSection`
- `UiListBoxHeader`
- `UiListBoxItem`
- `UiLongPressable`
- `UiMain`
- `UiMenu`
- `UiMenuTrigger`
- `UiMenuSection`
- `UiMenuItem`
- `UiSubmenuTrigger`
- `UiMeter`
- `UiModal`
- `UiMovable`
- `UiNavigation`
- `UiNumberField`
- `UiPopover`
- `UiPressable`
- `UiProgressBar`
- `UiRadio`
- `UiRadioGroup`
- `UiRangeCalendar`
- `UiSearch`
- `UiSearchField`
- `UiSelect`
- `UiSelectValue`
- `UiSelectionIndicator`
- `UiSeparator`
- `UiSection`
- `UiSharedElement`
- `UiSharedElementTransition`
- `UiSlider`
- `UiSliderTrack`
- `UiSliderFill`
- `UiSliderThumb`
- `UiSliderOutput`
- `UiSwitch`
- `UiTagGroup`
- `UiTagList`
- `UiTag`
- `UiTable`
- `UiTableHeader`
- `UiTableBody`
- `UiTableRow`
- `UiTableColumn`
- `UiTableCell`
- `UiTableFooter`
- `UiTableCaption`
- `UiTabs`
- `UiTabsList`
- `UiTabsTrigger`
- `UiTabsContent`
- `UiTabPanels`
- `UiText`
- `UiTextField`
- `UiTextarea`
- `UiTextArea`
- `UiTimeField`
- `UiToastRegion`
- `UiToast`
- `UiToolbar`
- `UiToggleButton`
- `UiToggleButtonGroup`
- `UiTooltip`
- `UiTooltipTrigger`
- `UiTree`
- `UiTreeSection`
- `UiTreeHeader`
- `UiTreeItem`
- `UiTreeItemContent`
- `UiTreeLoadMoreItem`
- `UiVirtualizer`
- `UiVisuallyHidden`
- `UiCalendar`
- `UiCalendarHeading`
- `UiCalendarGrid`
- `UiCalendarGridHeader`
- `UiCalendarGridBody`
- `UiCalendarHeaderCell`
- `UiCalendarCell`
- `UiCalendarMonthPicker`
- `UiCalendarYearPicker`

The component files use native semantic tags, familiar intrinsic names, and
Tailwind-compatible class strings. Each component is a Rust function component
stored in a `.rsx` source module, using `ComponentCx` hooks and an `rsx!` view.
The file parser can extract the final `rsx!(...)` view from these Rust modules
and rewrite common hook aliases such as `let title = cx.use_prop("title", ...)`
or `let save = cx.use_reducer("save", ...)` into static native bindings/actions.
Static base classes and Rust-side `ComponentClassVariants` merge with caller
`className`, giving each component one `DESIGN.md` base contract plus explicit
Rust variants without JavaScript.

`rsx_ui` sits above the semantic mapper. It should not own accessibility or
platform-control behavior directly; components such as `UiTabs` expand to
semantic `Tabs`, `TabList`, `Tab`, and `TabPanel` nodes, which are then lowered
through the native semantic layer.

### Style

The style layer parses Tailwind-style class strings into `PortableStyle`. It
should keep expanding toward the subset used by real component contracts:

- semantic colors and opacity modifiers
- spacing, sizing, radius, border, ring, shadow, typography, and layout tokens
- state variants such as `hover`, `focus-visible`, `disabled`, and
  `aria-invalid`
- data variants used by Radix-style components
- structural variants used by SVG and slot patterns

### Native Runtime

The runtime remains native-first. It registers actions, routes native events,
maintains interaction state, exposes accessibility trees, and sends platform
commands to AppKit, GTK, WinUI, or test backends.

`NativeRuntimeApp::with_background_updates` integrates completed effects with
the UI loop. One poll may merge many completions into state, but it produces at
most one render for that batch. `BackgroundUpdate` reports state change and
pending work separately. The three native loops poll instead of blocking while
work remains, briefly yield between polls, and return to a platform blocking
wait when idle.

Diagnostic retention is deliberately bounded. Action invocations, interaction
changes, and executed driver commands keep at most 256 entries by default and
offer configurable limits plus take/drain APIs. `PlatformPlanningHost` commands
have different semantics: they are pending commands since the previous
`take_commands()` call, not an accumulating history.

### Dependency Rules

- `authoring` owns the SWC-backed parser and `ComponentCx`; `design-system`
  adds `rsx_ui`. The no-default-features library build is the CI-enforced core
  boundary and contains neither dependency.
- RSX/`ComponentCx` and `rsx_ui` compile into `CompiledRsxNode`; native
  surfaces do not execute component code.
- `rsx_ui` sits above semantic contracts. Semantic/native IR, reconciliation,
  and platform planning do not depend on the default design-system registry.
- Platform-neutral IR and planning contain no OS handles. Feature-gated native
  surfaces depend inward on those records and own thread-affine widgets.
- The backend layer executes commands but does not own product state or I/O.
- The effect core depends only on its executor trait, not on Tokio. Product
  applications inject I/O and executor implementations at the outer boundary.
- Protocol and command boundaries carry serializable records, never component
  runtime objects or native widget handles.

## Framework Milestones

The cross-cutting runtime, automation, capability, performance, and platform
delivery plan is tracked in [`roadmap.md`](roadmap.md). These milestones cover
the RSX framework and design-system portion of that plan.

### M1 - Core RSX App Model

Status: in progress and largely implemented.

- `.rsx` parsing and named source errors
- Rust `ComponentCx` component functions with optional `.rsx` view templates
- component templates, contracts, default props, and slots
- state, prop, derived, context, resource, and memo hooks
- action, value, payload, reducer, and disabled hooks
- effect and transition hooks
- router pages, route context, route lifecycle, and layout outlet

### M2 - Design System Foundation

Status: initial implementation.

- `rsx_ui` module
- default built-in component availability for `RsxComponent::new`,
  `from_source`, `from_file`, and `from_template`
- `DESIGN.md` class contracts for Button, Input, TextField, SearchField,
  NumberField, Card, Badge, Separator, Checkbox, CheckboxGroup, Switch, Radio,
  ComboBox, Select/ListBox, Menu, Slider, Tabs, Dialog, Modal, Disclosure,
  ProgressBar, Meter, Toolbar, ToggleButton, Tooltip, Table, Breadcrumbs,
  GridList, TagGroup, Tree, FileTrigger, DropZone, Draggable, Droppable, Toast, Virtualizer,
  DateField, TimeField, DatePicker, DateRangePicker, Calendar, RangeCalendar,
  ColorPicker, ColorArea, ColorField, ColorSlider, ColorWheel, ColorSwatch,
  Link, and text primitives
- Rust function components stored as `.rsx` source modules and written with
  `ComponentCx` and `rsx!`
- `UiTextarea`
- class merging for static base classes plus caller `className`
- focused module split for classes, component source modules, variants, and
  tests
- tests that render built-in components and verify action/style/native
  semantic integration

### M3 - Variants

Status: initial implementation.

- `ComponentClassVariants` for Rust-side variant contracts
- component-specific variant presets for `UiButton` and `UiBadge`
- validation for unknown variant values during RSX render
- generated or precompiled class maps, not JavaScript conditionals

Next work:

- variant presets for `UiInput`, card surfaces, `tone`, and `density`

### M4 - Component Expansion

Next after the current primitive set.

- `UiDropdownMenu`
- `UiScrollArea`
- `UiSidebar`
- `UiCommand`

Radix-style behavior should be modeled as Rust state and actions. RSX templates
can carry `data-*` state markers, but open/close, selection, focus, and keyboard
behavior should remain native runtime concerns.

### M5 - Authoring Tooling

Status: planned across the 31-90 day roadmap.

- `a3s gui check` for RSX syntax, bindings, action ids, component contracts,
  ACL capability policy, and platform requirements without opening a surface
- build-time `CompiledRsxNode` artifacts consumed by runtime-only releases
- a debug watcher that preserves model/widget identity and the last-good frame
- template discovery and source maps for multi-file `.rsx`
- source span and template/import provenance queries
- diagnostics that point at component contract violations
- static RSX diagnostics that help React developers migrate simple component
  shapes by hand without introducing a JavaScript runtime

## Non-Goals

- No JavaScript runtime.
- No JavaScript hook execution.
- No DOM or CSSOM compatibility layer.
- No arbitrary JS expressions inside RSX.
- No WebView requirement for native desktop surfaces.
