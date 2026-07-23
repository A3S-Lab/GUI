# A3S GUI Roadmap

Updated: 2026-07-23

This roadmap evolves A3S GUI from a native widget renderer into a deterministic,
automatable, policy-aware application runtime with a productive RSX authoring
loop and mature desktop interaction contracts. Production surfaces continue to
use real AppKit, GTK4, and WinUI widgets.

The plan is intentionally dependency-ordered. A capability does not advance
because its implementation exists in isolation; it advances only after its
acceptance gates pass on the required headless and native paths.

## Planning Model

- The core program is split into two approximately 90-day increments. Relative
  day ranges express sequencing, not fixed release dates.
- The schedule assumes two core Rust engineers plus part-time owners for the
  AppKit, GTK4, and WinUI validation lanes. Fewer owners should reduce scope,
  not skip gates.
- Every milestone lands through small, reviewable changes. There is no
  milestone-sized integration branch.
- P0 work establishes correctness and delivery infrastructure. P1 work builds
  product-facing capability on that foundation. P2 work is admitted only after
  diagnostics or product dogfood proves the need.

| Priority | Scope |
| --- | --- |
| P0 | Deterministic application profile, test harness, diagnostics, executable component Stories, AOT, policy checks, hot reload, and window-layer correctness |
| P1 | Typed authoring, resources and boundaries, assets, themes, localization, collections, DataTable, packaging, and real-platform truth |
| P2 | Fine-grained invalidation, Dock/workbench, undo/redo, advanced content surfaces, and experimental rendering or hotpatch work |

## Architecture Commitments

- Keep OS-native widgets as the default rendering path. A custom-drawn surface
  requires a separate RFC and must not replace native text input, IME,
  accessibility, or focus behavior.
- Offer a strict application profile with one state transition boundary:
  typed messages enter `update`, effects return typed messages, and `view`
  remains a pure projection of model state.
- Keep string action and route identifiers at authoring or protocol boundaries.
  Business reducers use typed messages and typed route values.
- Compile development and release inputs into the same `CompiledRsxNode` IR.
  Runtime-only release artifacts do not carry the SWC parser or built-in design
  registry unless explicitly requested.
- Treat accessibility trees, native IR, and ordered platform commands as the
  cross-platform semantic truth. Pixel snapshots remain optional and
  platform-specific.
- Keep product I/O, configuration parsing, storage, network access, and
  operating-system capabilities outside the renderer and GUI core.
- Use ACL (`.acl`) exclusively for new GUI product configuration and capability
  policy. Hosts validate ACL and pass typed, immutable values into GUI core.

External projects are design references, not runtime dependencies:

| Reference | Adopt selectively | Keep outside A3S GUI |
| --- | --- | --- |
| [Dioxus](https://github.com/dioxuslabs/dioxus) | Typed authoring, props/routes, AOT diagnostics, transactional developer tooling, and resource/error boundaries | DOM/WebView, hydration/server-function, and web deployment assumptions |
| [`gpui-component`](https://github.com/longbridge/gpui-component) | Stories, window layers, themes/assets/localization, collections/DataTable, Inspector, and optional Dock patterns | GPUI entity ownership, custom rendering, and a second reactive state model |

## Current Baseline

The following foundations are implemented and are not roadmap placeholders:

- authoring, design-system, and runtime-only Cargo boundaries; Rust `ComponentCx`
  functions, `.rsx` templates, hooks, routing, contracts, and semantic components
- shared compiled RSX/native IR, keyed reconciliation, and ordered setter batches
- frame checkpoint, prepare, commit, exact ACK validation, degraded state, and
  fresh-executor replay under strict protocol v1 ordering and retransmission
- bounded, joinable effects with cancellation, completion queues, and an
  injectable executor seam
- bounded diagnostic histories with password, credential, and CSP nonce
  redaction
- headless planning, recording, and accessibility projections
- a component playground that checks registry coverage
- normalized portable style, Tailwind-compatible variant metadata, and
  transactional runtime projection for interaction and typed state variants
- inherited BCP 47 locale/direction, thread-safe ICU4X collation and filtering,
  localized decimal/percent parsing and partial-input validation, NumberField
  model/display normalization, grouped native stepper controls,
  minimum-anchored button, Arrow/Page/Home/End, and focus-gated vertical-wheel
  stepping, cancellable cross-platform pointer-hold repetition, platform-default
  suppression, horizontal/zoom rejection, a 34-locale NumberField accessibility
  catalog, assertive focused-value announcements on AppKit/GTK4/WinUI, and
  decimal/percent/date/time formatting
- an activation-ordered mounted overlay stack with topmost Escape/outside
  dismissal, modal background inertness, portaled foreground layers, and
  shared focus containment, autofocus, and restoration
- typed Popover/Tooltip anchor resolution with 22 logical/physical placements,
  versioned `positionOverlay` commands, recovery replay, and AppKit, GTK4,
  WinUI, and headless projections
- AppKit, GTK4, and WinUI build/test matrices with real native dogfood surfaces
- versioned native-input requirement, run, and report artifacts derived from
  role capabilities; a strict verifier rejects adapter/headless traces as
  native evidence, and CI publishes each backend's outstanding matrix
- a Windows OS-input smoke harness that drives real XAML Button-backed
  `Button`, `DisclosureSummary`, `Link`, `ImageMapArea`, and `MenuItem` roles
  plus list-backed `ListBoxItem` and `TreeItem` roles through `SendInput`,
  synthetic pen/touch injection, and UI Automation, covering the complete
  98-case WinUI manifest

## Delivery Dependency

```text
diagnostics + deterministic tests + component Stories
    -> typed application and effect boundary
    -> AOT, static checks, and source provenance
    -> transactional hot reload and window-layer semantics
    -> typed routes, assets, themes, and localization
    -> resources, collection, and DataTable engines
    -> platform truth, performance budgets, and packaging
    -> measured fine-grained invalidation and optional workbench features
```

## Core Program: First Increment

### M0 - Measurement And Decision Records

Target: days 0-14. Status: planned.

#### Deliverables

- Define `FrameDiagnosticsV1` with timings for event validation, `update`,
  scope projection, contract validation, RSX rendering, native lowering,
  reconciliation, planning, prepare, commit, accessibility projection, and ACK.
- Record visited and rebuilt components, native create/update/move/remove
  commands, setter counts, remounts, queue depths, effect completions,
  degradation, replay, memory high-water marks, and ACK latency.
- Establish calculator, component playground, Box, large-form, 1,000-row, and
  overlay-stack fixtures. Store reference-machine and CI-runner metadata beside
  results.
- Define `ComponentStoryV1` (a Story) as the executable component example
  contract. A Story describes its component identifier, fixture model, states,
  variants, semantic automation scenarios, expected native/accessibility
  properties, and platform capability expectations.
- Record focused decisions for the typed application boundary, compiled artifact
  format, window-layer ownership, collection identity, and theme/asset loading.
- Document which metrics are stable enough for future CI budgets. Do not create
  budgets from a single run.

#### Acceptance Gates

- Under a deterministic clock and effect executor, repeated fixture runs emit
  the same stage order, counts, and canonical selected fingerprints. Wall-clock
  timings and environment-dependent memory samples are excluded.
- Each timing stage has an explicit start/end definition and excludes unrelated
  setup work.
- Sensitive values are absent from diagnostics and benchmark artifacts.
- No runtime optimization or signal/store redesign is approved without evidence
  from these fixtures.

### M1 - Deterministic Application And Test Profile

Target: days 15-45. Status: planned after M0.

#### Deliverables

- Add an additive `NativeApplication` profile with associated `Model` and
  `Message` types. `update` is the normal state mutation boundary, `view` reads
  model state, and effects complete only with another `Message`.
- In this profile, component selectors read `Model` and component actions emit
  `Message`. Mutating lifecycle hooks and direct `state_mut` or `runtime_mut`
  access remain documented low-level `NativeRuntimeApp` escape hatches; they
  are not silently admitted into the strict profile.
- Map string action identifiers to typed messages at the application/protocol
  boundary. Internal reducers do not route business behavior through strings.
- Publish a `GuiTestHarness` with a fake effect executor, `TestClock`, semantic
  queries, event dispatch, completion injection, deterministic rebuilds, and
  failure injection seams.
- Define `AutomationSnapshotV1` plus `press`, `type`, `key`, `focus`, and
  semantic `assert` operations over stable node identifiers and accessibility
  roles.
- Add a transport-neutral automation journal for input messages, effects,
  clock reads, revisions, ACKs, recovery, and selected fingerprints. Expose it
  through strict protocol v1 and a CLI adapter; file-based automation may be an
  adapter, but is not the core protocol.
- Generate the initial Story registry from the existing component registry.
  Do not maintain a second handwritten list of built-in components.
- Define `PlatformCapability` and
  `Supported | Caveat(reason) | Unsupported(reason)` declarations for each
  backend.
- Define the ACL schema for application capability grants. GUI core receives
  typed grants and does not depend on an ACL AST.

#### Acceptance Gates

- Repeated model/message inputs produce identical native IR, accessibility, and
  selected journal fingerprints under a deterministic clock and executor.
- Headless record/replay produces the same selected fingerprints.
- Strict-profile code has no direct mutable access; low-level APIs remain source-compatible.
- Every registered semantic component has at least one generated or explicit
  Story with a semantic assertion.
- An absent capability grant is rejected explicitly and is never silently
  downgraded.
- New configuration fixtures and examples use `.acl` exclusively.

### M2 - AOT, Typed Contracts, Policy, And CLI Foundation

Target: days 46-70. Status: planned after M1.

#### Deliverables

- Split immutable validation from render-time validation. Template structure,
  static actions, component contracts, default props, and class variant maps are
  validated or compiled once and reused.
- Generate a versioned build-time `CompiledRsxNode` artifact with a content
  hash, compatibility version, optional debug provenance, and explicit feature
  requirements.
- Carry source spans, template/import provenance, and component contract paths
  through authoring diagnostics. Strip or isolate provenance in release builds.
- Add `a3s gui check` for RSX syntax, bindings, actions, component contracts,
  ACL capability policy, and platform requirements without opening a native
  surface. M4 extends the same diagnostic model to typed routes, assets, theme
  tokens, and localization keys.
- Add initial `fmt`, `test`, `build`, and `doctor` commands. Early commands may
  orchestrate existing Cargo and `just` operations, but they expose one stable
  diagnostic model.
- Pilot generated typed props on representative simple, input, overlay,
  collection, and virtualized components. Generated props compile into the
  existing `RsxComponentContract`; they do not create a second component model.
- Add an application-layer `CapabilityBroker` for clipboard, file and system
  dialogs, opening URLs, credentials, and other approved OS interactions.
  Requests, results, support declarations, and errors use typed enums.
- Migrate one complete A3S Box workflow to the strict typed-message profile.
- Bind effective capability grants to the application or protocol session and
  default to no grants.

#### Acceptance Gates

- Debug and AOT inputs produce equivalent native IR and accessibility
  fingerprints for the same source.
- Runtime-only release dependency graphs contain no SWC parser or built-in
  design registry unless deliberately enabled.
- Static validation no longer repeats in the measured render hot path.
- Invalid ACL, contracts, and platform requirements fail before a window or
  protocol session starts.
- Unauthorized and unsupported capability requests return stable, testable
  errors.
- Credentials never enter Stories, snapshots, journals, recording backends,
  protocol diagnostics, `Debug`, or benchmark data.

### M3 - Transactional Developer Loop And Window Semantics

Target: days 71-90. Status: planned after M2.

#### Deliverables

- Add `a3s gui dev` and a development watcher that classifies RSX/template,
  style, asset, and Rust-code changes.
- Apply compatible template-only changes transactionally, preserve the
  application model and stable widget identity, and keep the last-good frame
  when parsing, validation, or compilation fails.
- Fall back to a controlled UI remount or normal Rust rebuild when hook order,
  component shape, or dynamic expression identity cannot be preserved safely.
  Do not use unsafe binary hotpatching in the production development path.
- Restrict the development protocol to loopback, development builds, and a
  random session token. It must not expose an unauthenticated control channel.
- Add a typed `WindowLayerCoordinator`, scoped to a stable `WindowSessionId`,
  for dialog stacks, sheets, notifications, tooltips, popovers, and menus.
  The application/host orchestrates top-level sessions; application state or
  the deterministic interaction runtime owns semantic layer state; native
  backends own thread-affine widget and focus handles.
- Define layer ordering, dismissal, Escape, outside press, background inertness,
  focus trapping, focus restoration, and background text-selection behavior.
- Add a read-only Inspector MVP for RSX provenance, resolved props/styles,
  Native IR, accessibility, native command diffs, and frame diagnostics.
- Add headless Story scenarios for nested dialogs, rapid close/reopen, focus
  restoration, transient-layer nesting, and background interaction blocking.

#### Acceptance Gates

- Invalid development input produces no native commit and does not replace the
  last-good frame or application model.
- Compatible edits retain every unchanged keyed widget identity.
- The provisional template-only hot-reload target is p90 at or below 500 ms and
  at or below 20% of the fixture's cold rebuild time. M0 data may tighten or
  replace this target explicitly.
- Window-layer and two-window isolation scenarios pass headlessly and on at
  least one real native backend before the contract is declared usable.
- Development-only protocol and Inspector dependencies are absent from
  runtime-only release graphs.

## Core Program: Second Increment

### M4 - Typed Product Foundation

Target: days 91-120. Status: planned after M3.

#### Deliverables

- Generate typed props for the public built-in registry from its existing
  contracts. Runtime validation, Story metadata, and documentation consume the
  same source; migrate the Box vertical slice first.
- Add typed route enums or derives with typed parameters, nested routes,
  layouts, and reverse generation. Typed routes compile into the existing
  router and protocol boundary.
- Add typed `AssetId` values and an application-supplied `AssetProvider` or
  `IconPack`. AOT manifests validate and include statically referenced plus
  explicitly declared dynamic icons, images, fonts, and platform resources.
- Define `ThemeSnapshotV1` with semantic, component, typography, density,
  motion, and optional syntax-highlight tokens. Hosts load validated `.acl`,
  resolve light/dark/system selection, and pass immutable snapshots to GUI
  core. Theme watching belongs to `a3s gui dev` or the host.
- Add a typed `MessageCatalog` boundary with component defaults, application
  namespace overrides, locale fallback, and RTL direction. Existing locale and
  direction propagation remains the rendering input.
- Expand `ComponentStoryV1` to cover applicable size, variant, disabled,
  selected, loading, error, keyboard, and focus states. Generate component
  documentation and support summaries from the registry.

#### Acceptance Gates

- Theme changes update supported setters without remounting unchanged widgets.
- Missing assets, invalid tokens, unknown routes, and missing required
  localization keys fail `a3s gui check` with source context.
- GUI core performs no direct theme, locale, asset, file, or network I/O and
  has no ACL parser dependency.
- Runtime-only builds consume compiled assets and theme values without carrying
  authoring parsers.
- Generated types and existing runtime contracts produce the same compiled IR;
  there is no parallel component or router runtime.
- One Box workflow uses the typed path end to end; raw route, action, and asset
  identifiers do not enter business reducers.

### M5 - Async Boundaries, Collections, And DataTable Core

Target: days 121-150. Status: planned after M4.

#### Deliverables

- Define resource lifecycle states and dependency keys. Resource effects cancel
  or supersede stale work, complete through typed messages, and cannot let an
  older generation overwrite a newer result.
- Add nearest-subtree loading and error boundaries with explicit reset
  behavior. Boundaries do not bypass transaction or reducer semantics.
- Define typed `CollectionQuery` and `CollectionPage` values plus an
  application-supplied `CollectionDataSource`. The contract covers stable item
  identity, sections, search/filter/sort/page inputs, visible-range/load-more
  hints, generation tokens, disabled items, and cancellation.
- Keep data sources read-oriented. User interaction and async completion emit
  typed messages; data sources do not mutate application state behind the
  reducer boundary.
- Build `DataTableState` on the collection contract with row virtualization,
  stable row/section identity, single/multiple/range selection, keyboard
  navigation, and typed sort/filter/page requests.

#### Acceptance Gates

- Resource cancellation and out-of-order completion tests prove that stale
  results cannot overwrite current state.
- Sorting, filtering, pagination, and incremental loading preserve selection by
  stable item identifier rather than row index.
- The required CI fixture covers at least 1,000 rows, with materialized rows
  proportional to the visible range plus bounded overscan.
- Table keyboard and accessibility semantics pass headlessly and on at least
  one real native backend.
- Rapid search and load-more tests prove that cancelled or stale results do not
  appear.

### M6 - Platform Truth, Performance, And Distribution

Target: days 151-180. Status: planned after M5.

#### Deliverables

- Extend `DataTableState` with column virtualization, row/column/cell selection,
  fixed and movable columns, resizing, grouped headers, context menus, and typed
  export requests.
- Persist user column preferences through an application-supplied session
  store. Product defaults use ACL; mutable session state is not product
  configuration.
- Run shared semantic Story scenarios against real AppKit, GTK4, and WinUI
  surfaces for menus, dialogs, keyboard input, focus, text input, collection
  selection, table navigation, close, and recovery flows.
- Generate or validate the documented platform support matrix from backend
  declarations and real automation results. Feed each platform run through the
  generated `NativeInputConformanceManifestV1`; placeholder, adapter-kernel, and
  portable-runtime behavior does not count as supported.
- Replace thread park/unpark fallbacks with real AppKit, GLib, and WinUI event
  loop wake messages for externally completed work.
- Add native failure injection for partial commit, teardown, fresh-executor
  replay, transient-layer dismissal, and idle shutdown.
- Establish stable baselines, then enforce named CI budgets for p90 frame
  stages, ACK latency, setter churn, remount ratio, materialized collection
  items, memory high-water marks, and bounded queue depth.
- Complete `dev`, `check`, `fmt`, `test`, `build`, `package`, and `doctor`
  workflows under the A3S GUI CLI. `package` produces developer bundles and
  supports host/CI-owned signing and notarization without exposing credentials
  to GUI artifacts or diagnostics.

#### Acceptance Gates

- Report the maturity fixture for a logical 10,000-row by 100-column data set;
  materialized cells remain proportional to visible ranges plus bounded
  overscan. Make it blocking only after the budget-baseline policy is met.
- Table keyboard navigation and accessibility expose correct row/column counts,
  selection, focus, and active item semantics on each supported backend.
- The shared semantic smoke suite passes on real macOS, Linux, and Windows
  runners.
- Performance regressions fail with named stage and budget evidence rather than
  a generic wall-clock timeout.
- `doctor` rejects unsupported capabilities and missing native prerequisites;
  each developer bundle launches in its native smoke lane, and a configured
  release lane either completes signing/notarization or emits no artifact.

## Post-Core Gated Initiatives

### Fine-Grained Invalidation

Status: gated by M0 and M6 evidence.

Use the least invasive optimization that addresses the measured stage:

1. reuse immutable validation, contracts, defaults, variants, and AOT artifacts
2. cache component input fingerprints and compiled subtrees
3. skip unchanged component projections
4. add read-only selector dependency tracking
5. rebuild only dirty component subtrees

Business state still changes only through typed messages and `update`; selector
subscriptions cannot create another mutable store or hidden write path. Proceed
only when view/projection work is a measured bottleneck and a representative
local-update fixture meets both targets without semantic drift:

- at least 80% fewer component/subtree rebuilds
- at least 30% lower p90 input-to-commit latency

Exact native IR, accessibility, journal, recovery, and replay semantics remain
gates; otherwise keep whole-view projection plus keyed native reconciliation.

### Dock And Workbench

Status: optional RFC after M6 product dogfood.

- Implement Dock as an optional module or crate, never a default core dependency.
- Define `DockLayoutV1` for split stacks, tabs, side/bottom docks, optional
  freeform tiles, active/zoomed panels, and typed panel identifiers.
- Keep persistence and migration in an application-supplied session store.
- Dogfood the first implementation in Box or the Inspector workbench.
- Require deterministic round trips, focus/keyboard and accessibility scenarios,
  and explicit handling of incompatible layout versions before stabilization.

### Undo, Advanced Content, And Experimental Rendering

Status: optional extensions.

- Model undo/redo as grouped typed messages, with sensitive state excludable
  from history and diagnostics.
- Keep editors, Markdown/HTML, charts, Tree-sitter, LSP, and similar heavy
  features in optional extension crates or features.
- Require a separate RFC for custom rendering, mobile, or Rust hotpatching.

## Success Metrics And Budget Policy

| Dimension | Required evidence |
| --- | --- |
| Determinism | Identical selected IR, accessibility, and journal fingerprints for identical model/message/time/effect inputs |
| Developer loop | Transactional last-good behavior, no model loss, compatible identity retention, and a measured p90 hot-reload budget |
| Build boundary | Runtime-only release graph excludes authoring, Story, Inspector, parser, and built-in registry dependencies unless explicitly enabled |
| Component quality | Every registered component has Story coverage, semantic assertions, and platform capability evidence for supported behavior |
| Window semantics | Nested modal, dismissal, focus trap/restore, background inertness, and transient-layer tests pass on required backends |
| Collections | Stable identity through data changes, bounded visible materialization, cancellation safety, keyboard navigation, and accessibility correctness |
| Runtime performance | Named p90 stage, setter churn, remount, queue, memory, and ACK budgets derived from representative baselines |
| Security and privacy | Default-deny capabilities and no sensitive values in snapshots, Stories, journals, diagnostics, recordings, or debug output |

Each platform/fixture budget records its runner, sample count, warmup, percentile,
variance, and owner; changes require workload or environment evidence.

## Ownership Boundaries

| Layer | Owns | Must not own |
| --- | --- | --- |
| GUI core | Native IR, reconciliation, effect semantics, protocol records, capability types, portable layer/theme/collection records | Product clients, ACL parsing, file/network I/O, persistence, OS permission prompts |
| Application/host | Typed model/messages, effect orchestration, `CapabilityBroker`, data sources, session storage, ACL loading, theme/catalog/asset provisioning | Native widget reconciliation or backend handle mutation |
| Authoring/tooling | RSX parsing, AOT compilation, provenance, Story generation, `dev`, `check`, `doctor`, and packaging | Runtime product state or credentials |
| Design system | Semantic components, typed props/contracts, variants, Story metadata, default localization keys | Product workflows, storage, or platform handles |
| Native backend | Widget lifetime, thread affinity, event-loop wake, focus, IME, accessibility integration, and supported native setters | Business reducers, product SDK calls, ACL parsing, or mutable application data sources |

Public application, provider, and data-source contracts should be `Send + Sync`
where applicable; thread-affine native handles remain backend-private.

## Compatibility And Rollout

- Add strict profiles and generated types additively; keep low-level APIs until
  one full Box workflow and all native smoke lanes pass the replacement path.
- Version tool-facing records: snapshots, compiled artifacts, themes, Stories,
  and Dock layouts.
- Keep debug/AOT and headless/native semantic equivalence as continuous gates.
- Generate registries, docs, support matrices, and manifests from source contracts.
- Feature-gate authoring, Inspector, Story, and advanced content; verify the
  no-default-features core graph in CI.
- Dogfood one complete product path before broadening an abstraction.

## Risk Controls

| Risk | Control |
| --- | --- |
| Scope expands into a general browser or IDE framework | Keep native desktop application workflows as the acceptance fixtures; move advanced surfaces to optional extensions |
| Typed and legacy APIs diverge | Compile both into the same IR and compare fingerprints until legacy deprecation is justified |
| Hot reload corrupts identity or state | Transactional validation, last-good retention, compatible identity checks, and controlled fallback remount/rebuild |
| Fine-grained reactivity introduces hidden mutation | Read-only subscriptions only; typed `update` remains the sole business-state mutation boundary |
| Platform behavior is inferred from placeholders | Capability declarations plus real native Story automation and generated support evidence |
| Tooling or content dependencies inflate production | Feature boundaries, runtime-only graph checks, AOT artifacts, and optional extension crates |
| Development control channels escape local use | Loopback-only listener, development feature gate, random session token, and no production listener |
| Sensitive values leak through observability | Central sensitivity metadata, bounded redacted records, and dedicated negative tests for every new artifact |

## Definition Of Done For Roadmap Features

- behavior has focused headless and relevant native integration coverage
- modified public behavior has a Component Story or product dogfood scenario
- accessibility, capability, sensitivity, and platform support are explicit
- diagnostics expose failures and cost; runtime-only boundaries, documentation,
  examples, formatting, and relevant crate-local checks pass

## Explicit Non-Goals

- No build-system migration or second `.native` language alongside RSX.
- No DOM, CSSOM, JavaScript, WebView, SSR, hydration, server functions, or browser bundle splitting in GUI core.
- No custom-drawn replacement for the default AppKit, GTK4, or WinUI backends.
- No production binary hotpatching or hidden mutable store beside `update`.
- No default core dependency on editors, Markdown/HTML, charts, Tree-sitter, LSP,
  or Dock.
- No product SDK, credential store, ACL parser, file watcher, or persistence in core.
- No byte-identical cross-platform screenshots or implicitly granted capabilities.
