# A3S GUI Roadmap

Updated: 2026-07-24

## Product Direction

A3S GUI is a Rust-native, cross-platform implementation of the interaction,
accessibility, and component model represented by
[React Aria](https://react-aria.adobe.com/getting-started). It does not embed a
browser and does not expose DOM or CSSOM as its runtime.

[GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) is the only
desktop content renderer. AppKit, GTK4, and WinUI control backends are being
removed. Native means a native executable with native windows, input, IME,
clipboard, accessibility, and operating-system integration; it no longer means
that each component is an operating-system widget.

The immediate program is therefore a renderer migration, not another round of
platform-specific widget tuning. Broader application tooling and advanced
components resume only after the GPUI path satisfies the cutover gates in this
document.

## Planning Rules

- Milestones are dependency ordered. A later milestone cannot compensate for a
  failed earlier gate.
- Status is based on executable evidence, not the presence of an API or a
  platform-specific prototype.
- Changes land as small main-branch commits. There is no milestone-sized
  integration branch.
- The legacy backends are frozen. They receive no new features or visual fixes.
- A temporary legacy path may remain only until the GPUI calculator runs on all
  three desktop operating systems. It is not a supported dual-renderer product
  mode.
- Every visual implementation must lower the shared A3S tree. A calculator-only
  GPUI view or a second component model does not count as progress.

| Priority | Scope |
| --- | --- |
| P0 | GPUI foundation, deterministic scene contract, calculator vertical slice, input/IME/accessibility, visual regression infrastructure, and legacy backend removal |
| P1 | React Aria component projection, overlays, collections, date/color controls, tables, virtualization, themes, assets, and localization |
| P2 | Typed application profile, AOT/tooling maturity, fine-grained invalidation, packaging optimization, Dock/workbench, and advanced content surfaces |

## Non-Negotiable Architecture

```text
Rust component + RSX
        |
        v
CompiledRsxNode
        |
        v
semantic NativeElement tree
        |
        +------> interaction, selection, focus, i18n, actions
        |
        +------> AccessKit semantics
        |
        v
versioned GPUI scene
        |
        v
GPUI layout + paint + platform window
        |
        +------> Metal on macOS
        +------> GPUI Windows renderer
        +------> GPUI Wayland/X11 renderer
```

- A3S owns component contracts, RSX compilation, semantic state, actions,
  interaction policy, portable style, localization, and the versioned scene
  input.
- GPUI owns the desktop application loop, windows, layout execution, painting,
  text services, pointer/keyboard delivery, IME integration, clipboard bridges,
  and AccessKit connection.
- Product model state remains in A3S. GPUI `Entity` values are backend-private
  view state and must not become a second public application state model.
- GPUI types do not leak into the renderer-independent protocol, component,
  semantic, or headless APIs.
- Headless execution remains for deterministic semantic testing and protocol
  consumers. It is not a second visual renderer.
- Operating-system-owned surfaces such as file pickers, permission prompts,
  notifications, and window chrome may remain platform native. They are outside
  the application-content pixel contract.
- Unsupported style or accessibility projection is reported explicitly. The
  GPUI backend must never silently discard a property that a built-in component
  relies on.

## Cross-Platform Consistency Contract

GPUI supplies one layout and paint implementation, but GPU drivers and platform
text rasterizers can still produce different image bytes. A3S therefore defines
consistency through an exact scene contract plus a strict normalized image
gate. It does not claim that arbitrary screenshots from different machines
have identical PNG hashes.

### Deterministic Reference Mode

Reference rendering fixes all environment-dependent inputs:

- viewport size and logical scale
- sRGB color space and transparent-background policy
- light/dark theme and density
- locale, direction, time zone, clock, and random seed
- animation time, reduced-motion state, focus visibility, and input modality
- embedded font files, font fallback order, icons, and raster assets
- GPUI revision, shader revision, and snapshot schema version

System UI fonts are forbidden in reference fixtures. The project will vendor a
redistributable font with its license, register it through GPUI, and use its
exact bytes on macOS, Linux, Windows, and CI.

### Required Evidence

| Layer | Gate |
| --- | --- |
| Semantic tree | Canonical Native IR and accessibility fingerprints are identical for the same inputs |
| Scene | Ordered paint primitives, text runs, assets, colors, and interaction regions have an identical `GpuiSceneV1` fingerprint |
| Geometry | Quantized logical bounds, baselines, clipping, and z-order are identical; text may not wrap or reflow differently |
| Image | Normalized screenshots meet the checked-in perceptual and per-pixel thresholds on all three desktop lanes |
| Interaction | The same semantic scenario produces the same actions, state transitions, focus path, and final scene fingerprint |
| Accessibility | The same semantic tree and supported actions reach AccessKit; platform smoke evidence verifies the OS bridge |

M0 establishes separate text and non-text image masks and records provisional
thresholds. A threshold may be relaxed only with a reviewed reference update
that demonstrates no user-visible regression. Layout drift, different line
breaks, missing glyphs, different assets, or component-size drift always fail,
regardless of aggregate image score.

The first required visual fixture is the calculator at 410 by 620 logical
pixels. Component Stories add fixed reference viewports as coverage expands.

## Current Baseline

The repository already provides:

- Rust `ComponentCx` functions, `.rsx` templates, hooks, routing, contracts, and
  a broad semantic component registry
- shared compiled RSX and Native IR, keyed reconciliation, ordered frame
  transactions, rollback, ACK validation, and recovery replay
- portable style parsing with Tailwind-compatible classes and interaction/state
  variants
- action routing, press/hover/focus behavior, selection, collections,
  localization, NumberField behavior, overlays, and live-region semantics
- accessibility names, descriptions, relationships, states, structure, and
  capability reporting
- headless semantic, protocol, conformance, and deterministic runtime tests
- shared calculator state, reducer, RSX component tree, and platform entrypoints
- three platform control implementations and their CI/package lanes

The existing `Renderer` is a semantic tree reconciler; it does not draw pixels.
The three platform control implementations separately translate style and
layout into AppKit, GTK4, and WinUI behavior. That translation boundary is the
source of the unacceptable visual divergence and is the part being replaced.

## GPUI Dependency Policy

GPUI is pre-1.0 and its main branch changes frequently. Integration follows
these rules:

- Pin `gpui` and `gpui_platform` to one full Zed commit. Never use a wildcard,
  branch-only dependency, or an unreviewed floating revision.
- Keep the pinned revision compatible with the repository Rust toolchain.
- Record the Apache-2.0 license and required notices in distribution metadata.
- Update GPUI only in a dedicated pull request that runs the full scene,
  screenshot, interaction, accessibility, performance, and packaging matrix.
- Prefer official published crates once the required platform and AccessKit
  functionality is available together. Do not use an unofficial fork merely to
  avoid a temporary publication gap.
- Cache source and build artifacts in CI, but verify the locked source checksum
  before reuse.
- Isolate GPUI API churn inside `gpui_native`; semantic components and product
  applications must not change during a routine GPUI revision update.

## P0 Renderer Migration

### M0 - Decision Record And Measurable Contract

Status: planned first.

#### Deliverables

- Add an architecture decision record that selects GPUI as the only desktop
  renderer and records the final pinned Zed revision.
- Define `GpuiSceneV1`, including stable node identity, ordered primitives,
  layout values, text runs, clipping, transforms, opacity, z-order, hit regions,
  and accessibility references.
- Define the deterministic reference environment and the screenshot comparison
  algorithm, including text/non-text masks and artifact metadata.
- Select, license, and checksum the embedded reference font and icon set.
- Capture current calculator screenshots only as migration evidence. They do
  not become the new visual truth.
- Inventory every `PortableStyle` field used by built-in components and classify
  it as required for the first slice, required before cutover, deferred with an
  explicit capability, or unsupported.
- Inventory every `NativeRole`, event, focus operation, text-input operation,
  overlay operation, and accessibility field that needs GPUI projection.
- Record cold-fetch and clean-build costs for the pinned GPUI dependency and set
  the CI caching strategy.

#### Acceptance Gates

- The dependency resolves from a clean checkout on Rust 1.95 for macOS, Linux,
  and Windows.
- The scene schema and screenshot metadata are versioned and canonical.
- The font and asset license review passes.
- Every legacy backend responsibility has a named GPUI replacement or an
  explicit system-surface exception.
- No implementation milestone starts with an unresolved choice between a
  bespoke calculator view and generic Native IR lowering; generic lowering is
  mandatory.

### M1 - GPUI Application And Scene Foundation

Status: planned after M0.

#### Deliverables

- Add the `gpui-native` feature and a `gpui_native` module as the only new
  desktop backend.
- Implement a GPUI application/window entrypoint and UI-thread ownership model.
- Implement a retained A3S scene host that consumes normal renderer
  create/update/insert/remove/root operations and requests GPUI redraws.
- Lower the initial generic roles for windows, containers, labels, buttons, and
  toolbars without platform-specific branches.
- Lower the foundational style subset: display, flex direction, alignment,
  sizing, min/max sizing, gap, padding, margin, background, border, radius,
  text color, font family/size/weight, line height, text alignment, visibility,
  opacity, clipping, and cursor.
- Register the embedded font and fixed asset provider before the first window
  opens.
- Add canonical scene serialization and a test-only capture API.
- Keep GPUI errors contextual and recoverable; do not introduce production
  `unwrap`, `expect`, or event-loop panics.

#### Acceptance Gates

- One generic A3S tree opens, redraws, resizes, and closes cleanly on macOS,
  Linux, and Windows.
- All three operating systems produce the same scene fingerprint.
- The GPUI module is the only module importing GPUI types.
- Repeated renders with no semantic change produce no scene mutation.
- Missing fonts, assets, styles, or unsupported roles fail with a stable
  diagnostic instead of falling back silently.

### M2 - Calculator Reference Vertical Slice

Status: planned after M1.

#### Deliverables

- Run the existing `shared_calculator_component`, state model, reducer, and RSX
  tree through the generic GPUI host.
- Replace the platform-selecting calculator recipe with one
  `just calculator` command and one `gpui_calculator` entrypoint.
- Project button hit regions, press/hover/focus-visible state, keyboard
  activation, disabled state, labels, and action values.
- Make display history, error state, typography, spacing, border, and color
  output part of the scene contract.
- Add fixed reference screenshots and tri-platform comparison artifacts.
- Add a debug scene inspector that can identify the first divergent node,
  property, primitive, or pixel region.

#### Acceptance Gates

- Calculator arithmetic and all existing reducer tests remain unchanged.
- The example contains no calculator-specific GPUI element tree.
- Mouse and keyboard can complete representative digit, operator, clear,
  backspace, sign, decimal, and equals scenarios.
- macOS, Linux, and Windows produce identical scene/geometry fingerprints and
  pass the normalized screenshot gate.
- `just calculator` uses no shell-specific shebang or platform-specific example
  selection.
- A screenshot failure uploads the three images, diff heatmap, scene snapshots,
  font checksums, scale, GPU metadata, and first divergent scene path.

### M3 - Input, Text, Focus, Overlay, And AccessKit Parity

Status: planned after M2.

#### Deliverables

- Map GPUI pointer, keyboard, scroll, drag, and focus events into the existing
  `NativeEvent` vocabulary without changing component reducers.
- Implement focus scopes, tab order, focus-visible modality, restoration, and
  programmatic focus.
- Implement editable text, selection, clipboard, composition, IME, dead keys,
  password redaction, multiline input, search input, and NumberField behavior.
- Implement shared overlay geometry, clipping, z-order, modal inertness,
  dismissal, focus containment, popovers, tooltips, dialogs, and menus.
- Project the complete supported A3S accessibility tree through GPUI AccessKit,
  including role, name, description, value, state, structure, relationships,
  actions, and announcements.
- Replace per-toolkit input manifests with GPUI semantic requirements plus
  macOS, Linux, and Windows OS-bridge evidence.
- Add real-platform fixtures for Latin, CJK, RTL, dead-key, and assistive
  technology flows.

#### Acceptance Gates

- The shared input conformance suite passes on all three operating systems.
- IME composition is not reduced to synthetic key events and does not corrupt
  selection or committed text.
- Focus and overlay scenarios end with identical semantic and scene state on
  all platforms.
- AccessKit tree snapshots match headless semantics for supported fields.
- Real screen-reader smoke tests can discover, focus, invoke, and read the
  required reference controls.
- Sensitive values remain absent from logs, snapshots, diffs, accessibility
  output where prohibited, and failure artifacts.

### M4 - Legacy Backend Removal And Default Cutover

Status: planned after M3; required before new component rendering work.

#### Deliverables

- Make `gpui-native` the sole desktop feature and default desktop path.
- Delete the AppKit, GTK4, and WinUI surface, adapter, driver, widget-name,
  event-loop, automation, example, smoke-binary, and backend packaging code.
- Remove `appkit`, `appkit-native`, `gtk4`, `gtk4-native`, `winui`, and
  `winui-native` Cargo features and dependencies.
- Replace `NativeBackendKind` desktop variants with `Gpui`; retain `Headless`
  only for nonvisual tests and protocol use.
- Remove platform-prefixed calculator, controls, playground, counter, and
  dogfood entrypoints. Shared examples use one GPUI entrypoint.
- Replace CI and bundle matrices with GPUI macOS, Linux, and Windows lanes.
- Rewrite architecture, app-shell, packaging, React Aria, README, and support
  documentation around the single backend.
- Remove compatibility shims rather than preserving deprecated public wrappers.

#### Acceptance Gates

- Cargo metadata, public exports, examples, `just` recipes, CI, and packaging
  contain no legacy backend feature or dependency.
- Legacy backend source directories are deleted.
- The no-default-features semantic core still builds without GPUI.
- The default desktop build opens the GPUI calculator on all three operating
  systems.
- All existing semantic, protocol, accessibility, and calculator tests pass.
- GPUI bundle smoke artifacts launch and exit cleanly on all three platforms.

## P1 React Aria Projection

P1 does not create new platform renderers. Each component extends the same
semantic tree, GPUI lowering, AccessKit mapping, Story fixtures, and visual
contract.

### M5 - Foundations, Interaction, And Forms

Status: planned after M4.

#### Scope

- Button, Link, ToggleButton, ToggleButtonGroup
- Checkbox, CheckboxGroup, Switch, Radio, RadioGroup
- TextField, SearchField, NumberField, TextArea
- Label, Description, FieldError, FieldSet, Form
- Focusable, FocusRing, FocusScope, FocusWithin, VisuallyHidden
- Press, Hover, Keyboard, LongPress, Move, clipboard, and file trigger
- ProgressBar, Meter, Separator, Toolbar, disclosure, and basic feedback

#### Acceptance Gates

- Every component has default, hover, pressed, focused, focus-visible,
  disabled, read-only, invalid, loading, and selected Stories where applicable.
- Pointer, keyboard, touch/pen where available, and assistive actions reach the
  same reducer transitions.
- Form values, validation, composition, and labels are semantically correct.
- Scene fingerprints are identical and reference screenshots pass on all three
  platforms.

### M6 - Overlays, Selection, And Collections

Status: planned after M5.

#### Scope

- Dialog, Modal, Popover, Tooltip, Toast
- Menu, SubmenuTrigger, ComboBox, Select, ListBox
- Tabs, Breadcrumbs, TagGroup, GridList, Tree
- drag/drop, DropZone, DropIndicator, LoadMoreItem
- collection identity, typeahead, range selection, and virtualization

#### Acceptance Gates

- Dismissal, layer order, focus trap/restore, background inertness, nested
  overlays, and two-window isolation pass shared scenarios.
- Selection survives filtering, sorting, paging, and incremental loading by
  stable item identity.
- A 1,000-row fixture materializes only the visible range plus bounded overscan.
- Keyboard navigation and AccessKit collection semantics pass on every desktop
  lane.

### M7 - Date, Time, Color, Range, And Data Tables

Status: planned after M6.

#### Scope

- Calendar, RangeCalendar, DateField, DatePicker, DateRangePicker, TimeField
- ColorArea, ColorField, ColorPicker, ColorSlider, ColorSwatch, ColorWheel
- Slider and range controls
- Table, resizable columns, grouped headers, row/column/cell selection,
  virtualization, sort/filter/page requests, and typed export requests
- locale calendars, time zones, numbering systems, RTL, and high-contrast
  themes

#### Acceptance Gates

- Locale and time-zone fixtures produce deterministic semantic and scene
  output.
- Pointer geometry and keyboard increments agree for every range-like control.
- Table row/column counts, headers, selection, focus, and active-item semantics
  are correct in AccessKit.
- A logical 10,000-row by 100-column maturity fixture keeps materialized cells
  proportional to visible ranges plus bounded overscan.

## P2 Runtime And Tooling Maturity

These initiatives resume after the renderer cutover and the relevant P1
component gates:

1. typed `NativeApplication<Model, Message>` update/effect/view boundary
2. deterministic test clock, effect executor, automation journal, and failure
   injection
3. AOT `CompiledRsxNode` artifacts and runtime-only dependency boundaries
4. `a3s gui check`, `fmt`, `test`, `dev`, `build`, `package`, and `doctor`
5. typed assets, themes, localization catalogs, routes, capabilities, and ACL
   grants
6. transactional template hot reload and last-good rendering
7. measured fine-grained invalidation
8. optional Dock/workbench and advanced content extensions

Fine-grained invalidation proceeds only after diagnostics show that projection
or scene construction is a bottleneck. It must preserve exact scene,
accessibility, action, recovery, and replay semantics.

## Continuous Integration Matrix

| Lane | Required work |
| --- | --- |
| Semantic | Format, lint, no-default-features core, unit/integration tests, protocol compatibility, deterministic IR/accessibility fingerprints |
| Linux GPUI | Build and test Wayland/X11 support, run reference Stories in a controlled display/GPU environment, upload screenshots and scene artifacts |
| macOS GPUI | Build, launch, input/accessibility smoke, reference Stories, bundle validation |
| Windows GPUI | Build, launch, input/IME/UI Automation bridge smoke, reference Stories, bundle validation |
| Cross-platform compare | Compare canonical scene fingerprints and normalized screenshots from the three platform lanes |
| Dependency | Verify pinned GPUI source, licenses, notices, advisories, and runtime-only dependency boundaries |

A visual gate cannot be replaced by a single-platform golden image. A semantic
gate cannot be replaced by a screenshot. Both are required.

## Performance And Reliability Budgets

M0 records baselines before blocking budgets are enabled. Required budget
dimensions are:

- event-to-scene and event-to-present p50/p90/p99
- scene nodes and paint primitives visited, rebuilt, inserted, moved, and
  removed
- text shaping and glyph upload work
- GPU frame time, dropped frames, and texture memory
- input, effect, and redraw queue depth
- overlay and collection materialization counts
- cold dependency fetch, clean build, incremental build, and bundle size
- shutdown completion, leaked windows/entities, and background task joins

Each budget records the fixture, GPUI revision, runner, operating system, GPU,
driver, scale, sample count, warmup, percentile, variance, and owner.

## Ownership Boundaries

| Layer | Owns | Must not own |
| --- | --- | --- |
| GUI core | semantic Native IR, reconciliation, actions, interaction, focus policy, selection, i18n, protocol records, capabilities | GPUI entities, windows, GPU resources, product I/O, ACL parsing |
| GPUI backend | scene lowering, window/event loop, redraw scheduling, text/IME bridge, clipboard, AccessKit bridge, GPU resources, system-surface adapters | business state, product SDK calls, application persistence |
| Application/host | typed model/messages, effects, capability broker, data sources, session storage, ACL loading, theme/catalog/asset provisioning | scene mutation, GPUI handles, renderer-specific branching |
| Authoring/tooling | RSX parsing, contracts, AOT, provenance, Stories, visual tools, CLI, packaging | runtime product state or credentials |
| Design system | semantic components, typed props, variants, tokens, Stories, default messages | platform checks, GPUI entities, product workflows |

Public application, provider, and data-source contracts remain `Send + Sync`
where applicable. GPUI window and renderer values remain UI-thread-private.

## Risk Controls

| Risk | Control |
| --- | --- |
| GPUI pre-1.0 churn | Exact revision pin, backend isolation, dedicated upgrade PR, full tri-platform gate |
| Main-branch dependency is slow or unavailable | Locked source verification, CI source cache, documented recovery; move to official published crates when feature-equivalent |
| Platform text looks different | Embedded font bytes, fixed fallback, identical shaping inputs, baseline/wrap gate, text-specific visual diff |
| GPU/driver output differs | Canonical scene fingerprint, fixed sRGB reference mode, normalized capture, strict perceptual threshold, runner metadata |
| Style support silently degrades | Generated coverage inventory, explicit unsupported diagnostics, component Story gate |
| GPUI state leaks into application APIs | Backend-private entities and compile-time boundary tests |
| Input or IME is simplified during migration | Real OS composition/dead-key/CJK/RTL fixtures; synthetic key tests are insufficient |
| Accessibility regresses after removing widgets | AccessKit tree parity plus real platform screen-reader/automation smoke |
| Dual backends become permanent | Legacy freeze and mandatory M4 deletion before P1 visual expansion |
| Visual snapshots are casually re-blessed | Reviewed reference change with scene diff, image diff, reason, and owner |
| Sensitive values leak through diagnostics | Central sensitivity metadata and negative tests for every scene/snapshot artifact |

## Definition Of Done

A roadmap item is complete only when:

- it uses the shared semantic and GPUI path without platform-specific component
  branches
- behavior has focused semantic tests and relevant real-platform integration
  coverage
- scene, geometry, screenshot, interaction, and accessibility gates pass where
  applicable
- unsupported behavior is explicit in capabilities and diagnostics
- sensitive data is excluded from retained artifacts
- documentation, examples, formatting, and crate-local checks pass
- the change is committed, pushed, and required CI is green

## Explicit Non-Goals

- No AppKit, GTK4, or WinUI control renderer or compatibility shim after M4.
- No DOM, CSSOM, JavaScript runtime, WebView, SSR, hydration, or browser bundle.
- No platform-specific visual component implementations.
- No calculator-specific GPUI tree that bypasses shared RSX and Native IR.
- No second public reactive store based on GPUI entities.
- No promise of byte-identical output across arbitrary GPUs and drivers;
  consistency is enforced by the exact scene/geometry contract and normalized
  visual gates.
- No operating-system window chrome or permission dialog in the content pixel
  contract.
- No production binary hotpatching.
- No default core dependency on editors, Markdown/HTML, charts, Tree-sitter,
  LSP, Dock, product SDKs, credential stores, ACL parsers, file watchers, or
  persistence.

## Immediate Commit Sequence

1. M0 ADR, GPUI source pin experiment, license record, scene schema, font choice,
   and visual-test specification.
2. M1 application/window shell, scene host, foundational generic lowering, and
   three-platform static fixture.
3. M2 shared calculator, one cross-platform recipe, scene snapshots, and image
   comparison artifacts.
4. M3 pointer/keyboard/focus/text/IME/overlay/AccessKit parity.
5. M4 deletion of all three legacy control backends and default GPUI cutover.
6. M5-M7 component-category migration and conformance in dependency order.
