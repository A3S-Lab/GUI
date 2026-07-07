# RSX Framework Plan

RSX is a Rust-owned UI language with `ComponentCx` function components and
separate `.rsx` view templates. It should feel familiar to React authors without
inheriting the JavaScript runtime, DOM, CSSOM, or browser routing model.

## Direction

The framework should keep these boundaries stable:

- RSX syntax is declarative and statically analyzable.
- Page logic lives in Rust `ComponentCx` hooks: selectors, reducers, effects,
  resources, and router hooks.
- Components are authored as Rust functions that receive
  `&mut ComponentCx<State>` and return `RSX`.
- `.rsx` files are view templates. They consume `state.*`, `props.*`, derived
  values, context, resources, and action ids produced by Rust hooks.
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
- `cx.use_state`, `cx.use_reducer`, `cx.use_press`, `cx.use_button`,
  `cx.use_link`,
  `cx.use_breadcrumbs`, `cx.use_checkbox_group`, `cx.use_collection`,
  `cx.use_collection_section`, `cx.use_radio_group`, `cx.use_tab_list`,
  `cx.use_landmark`, `cx.use_group`, `cx.use_disclosure_group`,
  `cx.use_virtualizer`, `cx.use_clipboard`, `cx.use_drag`, `cx.use_drop`,
  `cx.use_form`, `cx.use_effect`,
  `cx.use_memo`, `cx.use_context`, and
  `cx.use_resource` provide the hook vocabulary in explicit Rust context form.
- State, props, derived, context, resource, and memo hooks build the render
  scope consumed by `.rsx` view-template props and bindings.
- Reducer/action hooks mutate Rust state from native events.
- Effect and transition hooks run after reducers.
- Mount and unmount hooks initialize and clean up page-local state.
- `RsxRouter` selects active pages, provides route context, runs route
  lifecycle hooks, and supports a persistent layout outlet.

### Design System

The first design-system layer is `rsx_ui`, a React-inspired Rust RSX component set
backed by the Uber-style tokens in the repository root `DESIGN.md`.

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
- `UiRow`
- `UiTableColumn`
- `UiColumn`
- `UiTableCell`
- `UiCell`
- `UiTableFooter`
- `UiTableCaption`
- `UiTabs`
- `UiTabsList`
- `UiTabsTrigger`
- `UiTabsContent`
- `UiTabList`
- `UiTabPanels`
- `UiTab`
- `UiTabPanel`
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
Static base classes and Rust-side `ComponentClassVariants` merge with caller
`className`, matching the important parts of shadcn's `cn(...)` and variant
behavior without JavaScript.

`rsx_ui` sits above the semantic mapper. It should not own accessibility or
platform-control behavior directly; components such as `UiTabs` expand to
semantic `Tabs`, `TabList`, `Tab`, and `TabPanel` nodes, which are then lowered
through the native semantic layer.

### Style

The style layer parses Tailwind/shadcn-compatible class strings into
`PortableStyle`. It should keep expanding toward the subset used by real
component contracts:

- semantic colors and opacity modifiers
- spacing, sizing, radius, border, ring, shadow, typography, and layout tokens
- state variants such as `hover`, `focus-visible`, `disabled`, and
  `aria-invalid`
- data variants used by Radix-style components
- structural variants used by shadcn SVG and slot patterns

### Native Runtime

The runtime remains native-first. It registers actions, routes native events,
maintains interaction state, exposes accessibility trees, and sends platform
commands to AppKit, GTK, WinUI, or test backends.

## Milestones

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

Later.

- template discovery and source maps for multi-file `.rsx`
- diagnostics that point at component contract violations
- static RSX diagnostics that help React developers migrate simple component
  shapes by hand without introducing a JavaScript runtime

## Non-Goals

- No JavaScript runtime.
- No JavaScript hook execution.
- No DOM or CSSOM compatibility layer.
- No arbitrary JS expressions inside RSX.
- No WebView requirement for native desktop surfaces.
