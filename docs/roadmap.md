# A3S GUI Roadmap

Updated: 2026-07-10

This roadmap evolves A3S GUI from a native widget renderer into a deterministic,
automatable, and policy-aware application runtime. It preserves the project's
core distinction: production surfaces use real AppKit, GTK4, and WinUI widgets.

## Architecture Direction

- Keep OS-native widgets as the default rendering path. A custom-drawn surface
  requires a separate RFC and must not replace native text input, IME,
  accessibility, or focus behavior.
- Offer a strict application profile with one state transition boundary:
  typed messages enter `update`, effects return typed messages, and `view`
  remains a pure projection of model state.
- Compile development and release inputs into the same `CompiledRsxNode` IR.
  Release artifacts must not carry the SWC parser or design-system authoring
  registry unless explicitly requested.
- Treat accessibility trees, native IR, and ordered platform commands as the
  cross-platform semantic truth. Pixel snapshots remain platform-specific.
- Keep product I/O and operating-system capabilities outside the renderer and
  GUI core.
- Use ACL (`.acl`) exclusively for all new GUI product configuration and
  capability policy.

## Current Baseline

The following foundations are implemented and are not roadmap placeholders:

- authoring, design-system, and runtime-only Cargo feature boundaries
- shared compiled RSX and typed native widget IR
- keyed reconciliation and ordered `NativeWidgetSetterBatch` updates
- frame checkpoint, prepare, commit, exact ACK validation, degraded state, and
  full replay through a fresh executor
- bounded, joinable effects with cancellation, completion queues, and an
  injectable executor seam
- strict protocol v1 revisions, event ordering, retransmission, and ACK rules
- bounded diagnostic histories with password, credential, and CSP nonce
  redaction
- headless planning, recording, and accessibility projections
- AppKit, GTK4, and WinUI build/test matrices

## 0-30 Days: Deterministic And Testable

Status: planned.

### Deliverables

- Add an additive `NativeApplication` profile with associated `Model` and
  `Message` types. `update` is the normal state mutation boundary, `view` reads
  model state, and effects can only complete with another `Message`.
- Keep `NativeRuntimeApp`, `state_mut`, and `runtime_mut` as documented low-level
  escape hatches rather than removing existing APIs.
- Map string action identifiers to typed messages at the application/protocol
  boundary. Internal reducers must not route business behavior through strings.
- Publish a `GuiTestHarness` with a fake effect executor, `TestClock`, semantic
  queries, event dispatch, completion injection, and deterministic rebuilds.
- Define `AutomationSnapshotV1` plus `press`, `type`, `key`, `focus`, and
  semantic `assert` operations over stable node identifiers and accessibility
  roles.
- Add a `FrameDiagnostics` schema for stage timings, command/setter counts,
  remounts, queue depths, ACK latency, degradation, and recovery. Collect data
  before enforcing budgets.
- Define `PlatformCapability` and
  `Supported | Caveat(reason) | Unsupported(reason)` declarations for every
  backend.
- Define the ACL schema for application capability grants. The application or
  host parses and validates `.acl`; GUI core receives typed, immutable grants
  and does not depend on an ACL AST.
- Establish calculator, component playground, Box, large-form, and 1,000-row
  benchmark fixtures.

### Acceptance Gates

- Repeated model/message inputs produce identical native IR and accessibility
  fingerprints under a deterministic clock and effect executor.
- Existing public APIs remain source-compatible.
- An absent capability grant is rejected explicitly and never silently
  downgraded.
- New configuration fixtures and examples use `.acl` exclusively.

## 31-60 Days: Automation, AOT, And Capabilities

Status: planned after the deterministic test foundation.

### Deliverables

- Migrate one complete A3S Box workflow to the strict typed-message profile as
  the first product dogfood path.
- Add an automation journal for input messages, effect requests/completions,
  clock reads, revisions, ACKs, and optional model/native/accessibility
  fingerprints.
- Expose the transport-neutral automation contract through strict protocol v1
  and a CLI adapter. File-based automation may be an adapter, not the core
  protocol.
- Add `a3s gui check` for RSX syntax, bindings, actions, component contracts,
  ACL capability policy, and platform capability requirements without opening
  a native surface.
- Generate a build-time `CompiledRsxNode` artifact. Runtime-only release builds
  must consume that artifact without linking authoring dependencies.
- Add an application-layer `CapabilityBroker` for clipboard, dialogs, opening
  URLs, and credentials. Requests, results, support declarations, and errors
  use typed enums.
- Parse capability grants from validated ACL and default to no grants. Bind the
  effective grant set to the application or protocol session.
- Ensure credentials never enter automation snapshots, journals, recording
  backends, protocol diagnostics, or `Debug` output.

### Acceptance Gates

- Headless record/replay produces the same selected fingerprints.
- The release runtime graph contains no SWC parser or built-in design-system
  registry unless its feature is enabled deliberately.
- Invalid ACL fails before a window or protocol session starts.
- Unauthorized and unsupported capability requests return stable, testable
  errors.

## 61-90 Days: Developer Experience And Platform Truth

Status: planned after strict-loop dogfood.

### Deliverables

- Add a development RSX watcher that preserves model state and stable widget
  identity, and keeps the last-good frame when compilation fails.
- Carry source spans, template/import provenance, and component contract paths
  through authoring diagnostics. Strip or isolate provenance in release builds.
- Run the same semantic automation scenarios against real AppKit, GTK4, and
  WinUI surfaces for menus, dialogs, keyboard input, focus, text input, close,
  and recovery flows.
- Generate or validate the documented platform support matrix from backend
  declarations and automation results. Placeholder behavior cannot count as
  supported.
- Replace thread park/unpark fallbacks with real AppKit, GLib, and WinUI event
  loop wake messages for externally completed work.
- Add native failure injection for partial commit, teardown, fresh-executor
  replay, and idle shutdown.
- Establish baselines, then enforce CI budgets for p90 frame stages, ACK
  latency, setter churn, remount ratio, and bounded queue depth.
- Unify `dev`, `check`, `test`, `build`, `package`, and `doctor` workflows under
  the A3S GUI CLI.

### Acceptance Gates

- The shared semantic smoke suite passes on real macOS, Linux, and Windows
  runners.
- Debug and AOT paths produce equivalent native IR for the same source.
- Performance regressions fail with named budget evidence rather than a generic
  wall-clock timeout.
- Product packaging reports unsupported capabilities and missing native
  prerequisites through `doctor` before build or launch.

## Ownership Boundaries

| Layer | Owns | Must not own |
| --- | --- | --- |
| GUI core | Native IR, reconciliation, effect semantics, protocol records, capability types | Product clients, ACL parsing, file/network I/O, OS permission prompts |
| Application/host | Typed messages, effect orchestration, `CapabilityBroker`, ACL loading and validation | Native widget reconciliation |
| Authoring/tooling | RSX parsing, AOT compilation, provenance, `check`, `doctor`, packaging | Runtime product state |
| Native backend | Widget lifetime, thread affinity, event-loop wake, focus, IME, accessibility integration | Business reducers or product SDK calls |

## Explicit Non-Goals

- No migration to Zig or another build system.
- No second `.native` markup language alongside RSX.
- No custom-drawn replacement for the default native widget backends.
- No byte-identical screenshot requirement across operating systems.
- No product SDK, credential store, or ACL parser dependency in GUI core.
- No capability granted implicitly because a platform happens to support it.
