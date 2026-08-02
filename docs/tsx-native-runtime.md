# TSX to Native Runtime Architecture

Status: T1 complete; T2 stateful scheduling in progress. The private
development package contains Rust-generated wire declarations, standard
automatic JSX entry points, keyed function-component instances, strict
child/key/prop normalization, deterministic frame lowering, state/reducer/
memo/ref/context/effect hooks, transactional render error boundaries, batched
rerenders, revision-scoped callback scopes, ordered event dispatch, and
cross-language golden tests. Actual process I/O/session integration and the
executable native host are not implemented yet.

This document defines an optional TypeScript authoring path for A3S GUI. The
developer experience is a directly executable `.tsx` application:

```sh
nub app.tsx
```

The result is a native A3S window rendered through `NativeElement`, GUI layout,
and A3S Graphics. There is no browser, WebView, DOM, CSSOM, HTML renderer, or
React renderer in the path.

The design takes the runtime lessons from
[Nub](https://github.com/nubjs/nub/tree/9e2eb69a0bd164be54a9c1741bb3820bc224fc85)
without making A3S GUI a Nub fork or a Node-specific renderer. Nub augments
stock Node, handles `.tsx` through Node loader hooks, and performs its
TypeScript/JSX transform in a Rust/Oxc N-API addon. A3S consumes the emitted
standard automatic JSX-runtime calls; it does not duplicate Nub's loader or
transpiler.

## Goal and Meaning of Direct

"TSX renders native GUI directly" means:

- TSX creates A3S element records rather than HTML or DOM nodes.
- TypeScript components and hooks run in the application Node process.
- resolved element records cross a local, versioned session boundary.
- the Rust host validates and lowers those records through the existing
  `CompiledRsxNode -> NativeElement -> LayoutSnapshot -> Graphics Scene` path.
- operating-system hosts own only windows, input, IME, accessibility bridges,
  clipboard, system dialogs, surfaces, and presentation.
- native events return as ordered A3S action invocations and never as DOM
  events.

Those operating-system responsibilities follow the
[self-drawn platform host architecture](platform-hosts.md): macOS uses a
single custom Metal-backed surface, Linux uses Wayland/X11, and Windows uses
Win32 with Graphics presentation. No TSX element can request a platform
content control.

"Direct" does not mean that JavaScript receives a native widget pointer or a
`wgpu` handle. Those handles remain thread-affine and process-local.

## Decisions

| Question | Decision |
| --- | --- |
| TSX execution | Stock Node, with Nub as the preferred zero-build runner |
| JSX transform | Standard automatic runtime with `jsxImportSource: "@a3s/gui"` |
| JavaScript engine | None embedded in `a3s-gui`; the application owns Node |
| Native boundary | A supervised child host over framed local IPC, not N-API |
| UI transport | Fully resolved, versioned A3S frame records |
| Reconciliation | Component identity in TypeScript; semantic/layout/scene reconciliation in Rust |
| Event handlers | Stable action ids backed by a JavaScript callback registry |
| Rendering | A3S Graphics self-drawn content; no new legacy-widget visual path |
| React | Familiar function components and hooks, but no React dependency or compatibility promise |
| Nub | First-class runner, not a required production runtime or private API dependency |

## Lessons Adopted from Nub

Nub provides four useful constraints for this design.

1. **Augment a standard runtime.** Nub keeps stock Node as the JavaScript
   runtime and installs TypeScript/JSX support through Node's extension
   surfaces. A3S should consume standard JSX output instead of inventing an
   A3S-only TSX compiler.
2. **Keep the hot native boundary narrow.** Nub moved parsing, transform,
   resolution, and cache work into one Rust addon while keeping user code in
   Node. A3S similarly keeps layout, scene extraction, rendering, input, and
   platform services in Rust, while application state and callbacks stay in
   TypeScript.
3. **Version generated output with its native consumer.** Nub treats its
   transformer, runtime helpers, and transpile cache as one release contract.
   A3S must release the TypeScript protocol types, host protocol range, and
   native host binaries as one tested compatibility set.
4. **Do not require install scripts.** Nub's package manager blocks
   `postinstall` scripts by default. A3S platform binaries must therefore ship
   as selected optional packages and be resolved at runtime, without a
   `postinstall` downloader.

Nub's
[`transform-core.mjs`](https://github.com/nubjs/nub/blob/9e2eb69a0bd164be54a9c1741bb3820bc224fc85/runtime/transform-core.mjs)
already reads `jsxImportSource` and emits automatic-runtime imports for TSX.
The A3S package only needs to provide `@a3s/gui/jsx-runtime` and
`@a3s/gui/jsx-dev-runtime`.

## What Is Deliberately Not Copied

- A3S does not register another TypeScript loader when Nub is already running.
- A3S does not depend on Nub's internal preload modules, cache format, or N-API
  addon.
- A3S does not load the GUI runtime as a Node N-API `cdylib` by default.
  Platform application/window shells may require main-thread ownership,
  operating-system event loops are thread-affine, and a same-process host
  failure would terminate application state with the window host.
- A3S does not implement a DOM facade, synthetic browser layout, or a React
  custom renderer.
- A3S does not serialize closures, promises, class instances, native handles,
  or arbitrary JavaScript objects.

An embedded adapter may be evaluated later, but it must implement the same
session contract and cannot become the portable default.

## Target Developer Experience

`tsconfig.json` uses the standard automatic JSX runtime:

```json
{
  "compilerOptions": {
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "jsx": "react-jsx",
    "jsxImportSource": "@a3s/gui",
    "strict": true
  }
}
```

The TypeScript compiler documents that this form imports `jsx` and `jsxs` from
`${jsxImportSource}/jsx-runtime`. Nub honors the same `tsconfig.json` option in
its Oxc transform.

An application is ordinary TSX:

```tsx
import {
  Button,
  Text,
  View,
  Window,
  createApp,
  useState,
} from "@a3s/gui";

function Counter() {
  const [count, setCount] = useState(0);

  return (
    <Window title="Counter" width={360} height={220}>
      <View className="flex-col gap-4 p-6">
        <Text>Count: {count}</Text>
        <Button onPress={() => setCount((value) => value + 1)}>
          Increment
        </Button>
      </View>
    </Window>
  );
}

await createApp(Counter).run();
```

`Window` is session metadata, not an application-content widget. `View`,
`Text`, and `Button` resolve to existing A3S semantic tags. Custom capitalized
functions execute in Node and must eventually return A3S elements.

The same source should run through:

```sh
nub app.tsx
nub watch app.tsx
```

Production builds may use any TypeScript tool that emits the standard
automatic JSX runtime. Nub is the preferred development runner, not a wire
protocol requirement.

## End-to-End Architecture

```text
app.tsx
   |
   | Nub / Node loader + Oxc transform
   v
@a3s/gui/jsx-runtime
   |
   | immutable A3sElement records
   v
TypeScript component scheduler
   |  owns hooks, state, callbacks, effects, component identity
   |
   v
frame compiler + action registry
   |  fully resolved ProtocolUiFrameV1-compatible records
   |
   v
length-prefixed local session transport
   |
   v
a3s-gui-host (Rust process)
   |  validates version, limits, keys, props, and action ids
   v
CompiledRsxNode -> RsxCompilerBridge -> NativeElement
   |                         |                  |
   v                         v                  v
LayoutSnapshot         semantic/a11y      interaction/hit records
   |
   v
A3S Graphics Scene -> FramePlanner -> software / Metal / DX12 / Vulkan
   |
   v
thin native window host
   |
   | ordered normalized events and action invocations
   +--------------------------------------------------> TypeScript scheduler
```

There are two reconciliation domains and one owner in each domain:

- TypeScript reconciles function-component instances and hook slots.
- Rust reconciles resolved semantic elements, layout records, draw records,
  accessibility nodes, and native host state.

The TypeScript side sends a complete resolved frame for the first protocol.
It does not compute native insert/update/remove patches. Existing Rust stable
keys, transactions, rollback, layout diffs, and retained scene damage remain
the single native reconciliation implementation.

## Process and Thread Model

| Process or thread | Owns | Must not own |
| --- | --- | --- |
| Node application | component functions, hooks, application state, callback registry, effects, host supervision | native handles, layout truth, scene mutation, platform event loop |
| IPC reader/writer | framing, bounded queues, protocol validation, cancellation | component execution, renderer work |
| Rust GUI main thread | native event loop, window lifecycle, input/IME bridge, accessibility bridge, committed UI session | JavaScript execution, product I/O |
| Graphics runtime | scene preparation, resources, GPU submission, presentation recovery | component state, action callbacks, accessibility policy |

The Rust host starts as a child of the Node application. It receives inherited
anonymous pipes, reserves stdout for framed protocol data, and sends human
diagnostics to stderr. Platform packages may use a GUI-subsystem executable as
long as the inherited handles remain available.

The process boundary is part of correctness:

- the operating-system host can own the child process main thread.
- a Rust panic, GPU device loss, or malformed frame has a defined failure
  boundary.
- the Node event loop remains free for application I/O.
- the protocol can be replayed by headless tests without a Node ABI dependency.

## JSX Runtime Contract

The package exports these mandatory automatic-runtime entry points:

- `@a3s/gui/jsx-runtime`: `jsx`, `jsxs`, `Fragment`, and the `JSX` namespace
- `@a3s/gui/jsx-dev-runtime`: `jsxDEV`, `Fragment`, and the `JSX` namespace

The runtime follows these normalization rules:

- strings and finite numbers become text records
- `null`, `undefined`, and booleans render nothing
- arrays and fragments are flattened in source order
- promises, symbols, bigints, class instances, and plain object children fail
  with a source-aware diagnostic
- a root fragment must resolve to exactly one content element in protocol 1;
  multiple roots require an explicit `View`
- spreads are evaluated by JavaScript before transport
- `style` accepts only the typed portable style value domain
- unknown props are retained only when the schema classifies them as portable
  attributes or `data-*` metadata
- event props accept functions or explicit action objects; functions never
  cross the process boundary

Development elements retain file, line, and column provenance from `jsxDEV`.
Provenance is diagnostic metadata and is removed from release frames unless
debug inspection is enabled.

### Identity

Identity must be deterministic across component rerenders and process replay.

- explicit JSX `key` is required for mutable lists
- unkeyed static children receive a deterministic sibling-position key
- development mode diagnoses mutable arrays without explicit keys
- component paths and host-element paths are separate; custom components do
  not create native nodes
- native sibling keys remain unique and continue through the existing
  collision-safe length-prefixed layout path
- the target action identity derives from the stable component path, host key,
  and event prop; ids never depend on function source or object address

The current T1 compiler has no stateful component-instance tree yet, so its
automatic ids use the collision-safe host-key path plus event prop. The T2
component scheduler must add the stable component-instance prefix before that
identity contract becomes public. Callback generations already live in
revision scopes rather than in either id shape.

The implemented `RevisionActionRegistryV1` retains one pending scope, the
active committed scope, and one rollback scope. Staging validates and copies
the complete action/callback set before publishing the candidate. A matching
`committed` message atomically moves the old active scope to rollback, promotes
the candidate, and drops the older rollback generation; rejection removes only
the candidate. Late TSX or self-drawn-host revisions and skipped, duplicate, or
late event sequences are rejected before callback dispatch. The rollback scope
is retained for bounded recovery only and never accepts stale events.

The registry preflights the complete invocation vector before consuming its
sequence or running application code, then awaits callbacks one at a time in
wire order. Repeated action ids therefore run repeatedly and explicit actions
without a JavaScript handler remain valid no-ops. Unknown or disabled actions
reject the entire vector. Once the first callback can run, the sequence is
consumed even if a later callback throws: earlier JavaScript side effects
cannot be rolled back safely, and replaying the same vector would duplicate
them. The active and rollback scopes remain intact after that application
error, while later invocations in the failing vector do not run.

Callbacks that decide hit testing before an event is dispatched are a distinct
protocol concern. In particular, React Aria's `shouldAcceptItemDrop` and
`getDropOperation` require a synchronous answer while resolving the current
collection target. They must not be encoded as ordinary action ids, serialized
closures, or post-event reducers. The Rust boundary now defines
`SelfDrawnDropPolicyQuery`/`Response`, strict
`ProtocolDropPolicyQueryV1`/`ResponseV1` envelopes, and
`ProtocolDropPolicyResolverV1`. Every query identifies the committed frame,
event, query sequence, policy id, typed target, drag types, and allowed
operations; stale or malformed responses and source-disallowed operations fail
closed. The exchange trait makes timeout, unavailable transport, and handler
failure explicit and maps all three to `cancel`. Before these APIs are exposed
to TSX, the Node runtime must bind its revision-scoped callback registry to a
bounded implementation of that exchange. Node continues to own JavaScript
callbacks; the Rust host never executes JS.

### Components and Hooks

The initial runtime is intentionally smaller than React:

- function components
- `useState` and `useReducer`
- `useMemo`, `useRef`, and typed context
- `useEffect`, executed only after the native host reports a committed frame
- batched state updates with at most one pending render per microtask
- deterministic cleanup on dependency changes, unmount, host loss, and app
  shutdown
- transparent nested context providers that remain local to Node
- function-style `ErrorBoundary` fallbacks with failed-candidate rollback

An error boundary catches synchronous errors while resolving its descendant
component tree. It does not catch event callback, effect, host submission, or
native presentation failures. The failed descendant Hook candidates are
discarded before the fallback is resolved. Only a host-committed fallback may
unmount the previously active subtree and run its cleanup.

`useLayoutEffect`, Suspense, concurrent rendering, portals, server components,
and class components are not part of protocol 1. A cross-process
`useLayoutEffect` would either lie about its timing or block presentation; it
must not be exposed until the protocol has an explicit pre-present phase.

Effects may perform application I/O in Node. They cannot call into native
objects directly. Native operations such as focus, clipboard, dialog, and
window control use typed session commands with capability checks.

## TSX-to-Rust Mapping

| TSX value | Wire representation | Existing Rust consumer |
| --- | --- | --- |
| intrinsic string | `ProtocolCompiledNodeV1::Element.tag` | `RsxCompilerBridge` |
| `key` | element or text `key` | compiled-tree validation and stable native identity |
| string/number child | text node | `CompiledRsxNode::Text` |
| `className` | compiled `className` | portable style/Tailwind parser |
| style object | scalar style map | `CompiledStyleValue` conversion |
| portable props | resolved compiled props | semantic component and intrinsic mapping |
| `aria-*`, HTML, `data-*` | typed fields or attribute map | accessibility and metadata projection |
| event callback | action id in `events` | shared semantic action selection and the stable-id self-drawn interaction session |
| `Window` | frame `window` metadata | platform window session |
| custom function component | evaluated before transport | no native node |

The first TypeScript package generates its protocol declarations from the Rust
versioned DTOs. Checked-in canonical JSON fixtures are decoded by Rust,
re-encoded canonically, and compared in TypeScript CI. Hand-maintained duplicate
wire interfaces are not accepted as a long-term source of truth.

## Session Protocol

The existing `ProtocolCompiledNodeV1`, `ProtocolCompiledPropsV1`, and
`ProtocolUiFrameV1` are the starting input vocabulary. They already require
resolved bindings, reject unknown fields, carry stable keys, serialize action
ids, and lower into `UiFrame`.

The TSX host must not expose the current `ProtocolRenderResponseV1` as its
public process protocol. That response contains legacy planned-widget commands
for an external executor, and its event payload preserves only a compatibility
single invocation. A combined self-drawn host needs a session envelope around
the reusable input DTOs.

### Envelope

Every message includes:

```text
protocol:        "a3s.gui.tsx"
protocolVersion  u32
sessionId        opaque random string
messageId        monotonically increasing u64 per sender
renderRevision   active or proposed u64
type             tagged message kind
payload          versioned message payload
```

JSON is the protocol-1 payload encoding because it matches the current Serde
contract and is easy to inspect. Each message is framed as a little-endian
`u32` byte length followed by UTF-8 JSON. The receiver rejects zero-length,
oversized, truncated, invalid UTF-8, duplicate-field, unknown-kind, and
unsupported-version messages before state mutation. The initial maximum JSON
payload size is explicit in the handshake and bounded to 16 MiB or less.
Every numeric `u64` field is also bounded to `Number.MAX_SAFE_INTEGER` before
transport. Layout and scene fingerprints may use all 64 bits, so they cross the
JSON boundary as exactly sixteen lowercase hexadecimal digits instead of
lossy numbers.

Landed Rust foundation: `tsx_protocol` defines strict direction-specific
control messages with the fixed `a3s.gui.tsx` identifier, atomic
`TsxHostHandshakeV1` negotiation, exact per-sender `TsxMessageSequenceV1`, and
blocking plus incremental framed JSON codecs. The decoder validates a declared
length before allocating, becomes unusable after a framing/JSON violation, and
has a checked end-of-stream path for partial headers and payloads.
`TsxHostApplicationSessionV1` now accepts strict full-frame `render` messages,
promotes them only through a bounded `committed` response, emits complete
ordered `event` batches, permits exactly one render in flight, and leaves the
active revision unchanged on validation, host, or response-encoding failure.
Its feature-gated adapters project `SelfDrawnFrameSnapshot` and
`SelfDrawnInputDispatch` directly; no planned-widget response is involved.
The optional `typescript-schema` feature walks these Rust DTOs, writes one
deterministically ordered TypeScript module, and fingerprints its declaration
body. Canonical hello, counter render, committed, and event fixtures pin the
wire spelling in Rust and Node 24 tests. The private TypeScript peer now
provides standard automatic JSX entry points, keyed function-component
instances, strict normalization, deterministic frame lowering, a stateful
application scheduler, and a pinned TypeScript 5.9 `react-jsx` fixture. Command
messages, actual local process I/O, registry/session identity integration,
and host supervision remain pending.

### Messages

| Direction | Message | Purpose |
| --- | --- | --- |
| Node -> host | `hello` | SDK version, supported protocol range, requested renderer and debug capabilities |
| Host -> Node | `welcome` | selected version, host/build identity, platform, renderer, limits, capabilities |
| Node -> host | `render` | complete resolved frame and next render revision |
| Host -> Node | `committed` | frame accepted atomically; returns revision, root identity, layout/scene fingerprints, diagnostics |
| Host -> Node | `event` | ordered normalized event with all routed action invocations and interaction changes |
| Node -> host | `command` | typed focus, clipboard, dialog, window, inspector, or shutdown request |
| Host -> Node | `commandResult` | typed success, denial, cancellation, or error |
| Either | `ping` / `pong` | liveness without changing UI state |
| Either | `close` | graceful shutdown and reason |
| Host -> Node | `fatal` | unrecoverable protocol, renderer, or platform failure |

One render revision may be in flight. State changes that occur while it is in
flight are coalesced into the newest pending TypeScript tree. `useEffect`
callbacks run after `committed`, never after serialization alone.

Events carry both the revision against which hit testing occurred and a
strictly increasing event sequence. The TypeScript runtime dispatches the full
ordered invocation vector, applies batched state updates, and then schedules at
most one new frame. Stale, skipped, or duplicated sequences are explicit
session errors.

Protocol `renderRevision` identifies the TSX tree and callback scope.
`hostRevision` identifies the committed self-drawn snapshot used for layout,
hit testing, scene, and accessibility. They deliberately are not aliases: a
TSX rerender may replace callbacks while producing identical Native IR, so the
self-drawn host can retain its snapshot revision while the TSX render revision
advances. An event must match the active host revision and is then delivered
against the active TSX render revision.

### Commit and Recovery

The host performs each render as prepare, validate, commit, and present:

1. decode and validate without changing active state
2. lower to semantic/native IR and build layout, scene, hit, and accessibility
   products
3. reject error diagnostics and retain the previous committed revision
4. atomically commit the host action-id scope and render products
5. report `committed`; Node promotes the matching callback scope, and
   presentation telemetry may follow separately

The Node runtime retains the last committed full frame and action scope. An
opt-in development supervisor may restart a failed host, negotiate a new
session, and replay that frame. Production restart policy is application-owned.
The host never silently falls back from GPU to a visually different renderer.

## Repository and Package Shape

The implementation should remain in this repository so protocol changes and
SDK generation land together:

```text
packages/typescript/
|- package.json
|- src/jsx-runtime.ts
|- src/jsx-dev-runtime.ts
|- src/element.ts
|- src/component-runtime/
|- src/action-registry/
|- src/session/
|- src/generated/protocol.ts
`- tests/

src/tsx_protocol/
|- application.rs  strict render, commit, event, context, and diagnostic DTOs
|- message.rs      direction-specific messages and common envelope metadata
|- handshake.rs    atomic capability, renderer, and limit negotiation
|- framing.rs      limits plus blocking and incremental JSON framing
|- session.rs      one-in-flight transactional revision/event ordering
`- tests.rs

src/platform_host/       shared zero-widget host contract and OS shells
src/bin/a3s_gui_host.rs
tests/fixtures/tsx-protocol/
packaging/npm/
|- host-darwin-arm64/
|- host-linux-x64-gnu/
`- host-win32-x64-msvc/
```

Names are planning names, not pre-approved public package names. The important
boundaries are:

- protocol DTOs do not depend on SWC, Node, N-API, Graphics, or an OS toolkit
- the TypeScript package contains no native rendering implementation
- the platform host binary depends on the self-drawn Graphics path and one
  thin platform integration
- target hosts contain no platform content-widget feature or dependency
- generated platform packages contain prebuilt binaries and checksums, not
  install-time download code
- deleted content-widget renderers are not restored for TSX

## Packaging and Versioning

The portable package is `@a3s/gui`. Platform binaries are selected through
`optionalDependencies`, for example:

```text
@a3s/gui-host-darwin-arm64
@a3s/gui-host-linux-x64-gnu
@a3s/gui-host-win32-x64-msvc
```

The launcher resolves the exact package for `process.platform`,
`process.arch`, and libc where relevant. It validates the embedded host
manifest and checksum before spawn. It never downloads or executes a package
manager during application startup.

The release contract records:

- SDK version
- minimum and maximum TSX session protocol versions
- native host version and target triple
- GUI crate revision
- Graphics scene schema and pinned Graphics revision
- generated protocol declaration fingerprint

SDK and host versions may differ only when their advertised protocol ranges
overlap. CI tests the oldest supported SDK against the newest host and the
newest SDK against the oldest supported host.

## Security and Resource Limits

- reject unknown message kinds and unknown fields in versioned records
- cap frame bytes, depth, node count, text bytes, attribute count, action count,
  resource bytes, and queued messages
- validate every finite number before layout or scene mutation
- redact password and sensitive control values from diagnostics, event logs,
  accessibility snapshots, replay files, and crash reports
- keep native file, network, process, and environment access out of UI props
- expose clipboard, dialogs, URLs, files, and external processes only through
  typed capability-checked commands
- do not evaluate source text received from the host or transport
- terminate the session on framing desynchronization instead of guessing
  message boundaries

## Delivery Track

This track can begin during renderer M3, but the first supported visible TSX
application depends on host H1, one supported H2-H4 platform slice, and the
minimum M4 text/input slice.

### T0 - Contract and Architecture

Status: architecture accepted; the Rust-side strict handshake/framing,
render/commit/event session, counter parity fixtures, self-drawn adapters, and
drop-policy DTO/resolver adapter are implemented. Calculator and
commands, stateful JSX execution, registry/session integration, and Node-side
transport are pending. Rust-generated declarations, the headless automatic
JSX core, bounded revision callback scopes, ordered dispatch, and shared
Rust/Node counter fixtures are implemented.

- accept process, ownership, identity, protocol, and packaging decisions
- retain the landed shared Rust/Node golden frame/event gate and extend it to
  the calculator scenario
- define the first counter and calculator parity scenarios
- record unsupported React and browser behaviors explicitly

Gate: architecture review agrees that TSX is a peer authoring frontend and
cannot bypass Native IR, layout, Graphics, interaction, or accessibility.

### T1 - Headless Protocol and JSX Core

Status: Rust transport foundation in progress. `hello`/`welcome`, atomic limit
and renderer negotiation, exact message-id sequencing, 16 MiB-capped framing,
incremental decoding, strict `render`/`committed`/`event` DTOs, transactional
session ordering, self-drawn adapters, four canonical JSON fixtures, and the
static counter Native IR/accessibility parity test have landed. Safe-integer
validation, hexadecimal 64-bit fingerprints, deterministic Rust-generated
TypeScript declarations, a fixed schema fingerprint, the private package
skeleton, automatic `jsx-runtime`/`jsx-dev-runtime`, immutable element records,
synchronous function-component expansion, strict child/key/prop/style/window
normalization, deterministic callback-to-action ids, read-only callback
snapshots, one pending/active/rollback registry, strict ordered asynchronous
event dispatch, and Node 24 plus TypeScript 5.9 golden/type tests have also
landed.

- extend the landed application messages with command messages and actual
  local process I/O
- connect the landed callback registry to the process session and enforce
  session/message identity before its revision/action preflight
- connect the landed strict drop-policy query/response DTOs to that transport
  and the Node callback registry
- extend the landed static counter parity fixture to the calculator scenario

Gates:

- TypeScript golden frames decode and canonicalize byte-for-byte in Rust
- malformed, oversized, stale, duplicate, and unknown messages fail before
  state mutation
- the Rust RSX and TSX versions of the static counter have identical Native IR
  and accessibility fingerprints
- `cargo check --no-default-features --lib` remains free of Node and Graphics

### T2 - Stateful TypeScript Runtime

Status: in progress.

Delivered:

- keyed function-component candidate/active instance trees
- state, reducer, memo, ref, and post-commit effect hooks
- batched state updates around ordered event dispatch and rerender scheduling
- candidate hook promotion aligned with callback-scope commit/reject
- deterministic effect/component cleanup on dependency changes, committed
  unmounts, and graceful scheduler shutdown
- typed transport-neutral host submission and commit acknowledgement
- typed nested context scopes that never enter protocol frames
- render error boundaries with source-located fallback values/functions,
  partial-candidate rollback, and rejected-fallback last-frame preservation

Remaining:

- callback registry integration with session/message identity
- supervised headless Rust host process I/O, crash recovery, and replay
- complete keyboard, stale-event, host-crash, and replay gates

Gates:

- the TSX counter passes click, keyboard, keyed-rerender, stale-event, effect,
  cleanup, host-crash, and replay tests
- one event batch produces at most one next render
- failed frames preserve the last committed UI and callback scope
- hook order violations and missing mutable-list keys are source-located

### T3 - First Self-Drawn Native Window

Dependencies: host H1, one supported H2-H4 platform slice, and the minimum M4
text, pointer, keyboard, focus, and accessibility slice.

- launch the real host binary from `nub app.tsx`
- present the TSX counter and shared calculator through A3S Graphics
- return native input as ordered action invocations
- expose typed focus, clipboard, window-close, and inspector commands

Gates:

- no legacy content widget is created for a TSX application
- Rust RSX and TSX calculators pin the same Native IR, layout, scene, and final
  model fingerprints for the same scenarios
- software output is byte-identical and GPU output stays inside reviewed
  thresholds
- OS accessibility smoke exposes the same names, roles, values, states, focus,
  and actions

### T4 - Nub Watch and Tri-Platform Packages

- platform binary packages with no install scripts
- `nub watch` integration with transactional last-good-frame reload
- source maps from native diagnostics to TSX locations
- macOS, Linux, and Windows launch, signing, packaging, and recovery lanes

Gates:

- editing component code preserves the window and committed native identity
  where keys are unchanged
- syntax or contract errors keep the last good frame and report the TSX source
- clean installs resolve the correct signed host without network work at launch
- Metal, Vulkan, and DX12 lanes pass the same counter and calculator scenarios

### T5 - Production SDK

- stable public TypeScript API and semantic component declarations
- inspector, accessibility audit, performance telemetry, and replay tooling
- compatibility policy and release automation
- production examples and migration guide from browser React/TSX

Gate: TSX is documented as supported only after the full platform, renderer,
interaction, accessibility, packaging, recovery, and compatibility matrix is
green.

## First Reviewable Commit Sequence

1. Add session DTOs, framing limits, and protocol golden fixtures. Landed.
2. Add generated TypeScript wire declarations and drift CI. Landed.
3. Add the automatic JSX runtime with normalization and key tests. Landed.
4. Add action ids, callback scopes, and event-vector dispatch. Landed.
5. Add the headless host binary and a static TSX counter fixture.
6. Add state/reducer/effect scheduling and counter interaction tests.
7. Connect the host to the generic self-drawn window path.
8. Add Nub watch reload and platform binary packaging only after the native
   presentation gates pass.

Each commit must leave Rust-only builds and the existing Rust authoring path
green. No milestone-sized integration branch is required.

## Risks and Controls

| Risk | Control |
| --- | --- |
| Two state models diverge | TypeScript owns component state; Rust owns semantic/interaction/render state; wire records are the only boundary |
| React compatibility expands scope | Publish A3S-specific supported hooks and explicit non-goals; do not depend on React |
| Event closure leaks | Revision-scoped callback registry, rollback scope, bounded retention, deterministic cleanup |
| Stale input mutates new state | render revision plus strictly ordered event sequence before callback lookup |
| IPC adds latency | one local host, bounded binary framing, full-frame coalescing, measured phase telemetry |
| Full frames become expensive | retain Rust diffing first; add measured protocol patches only after representative budgets exist |
| Node blocks native UI | separate processes and independent event loops |
| Native host crash loses app state | Node retains state and last committed frame; supervised replay is opt-in and explicit |
| Package install scripts are blocked | prebuilt optional platform packages; no downloader or postinstall hook |
| SDK/host drift | negotiated protocol range, generated declarations, cross-version CI, manifest fingerprint |
| Legacy renderer becomes permanent | TSX visible-window gate requires the self-drawn path; legacy widgets receive no TSX features |

## Non-Goals for Protocol 1

- DOM, CSSOM, browser event, HTML layout, or WebView compatibility
- React package compatibility or use of React internals
- synchronous native object access from JavaScript
- same-process N-API GUI hosting as the portable default
- arbitrary custom drawing callbacks crossing IPC
- JavaScript layout, scene generation, or GPU command generation
- transport-level incremental element patches before full-frame budgets exist
- `useLayoutEffect`, Suspense, concurrent rendering, server components, class
  components, or hydration
- mobile and web targets

## References

- [Nub repository and runtime overview](https://github.com/nubjs/nub/tree/9e2eb69a0bd164be54a9c1741bb3820bc224fc85)
- [Nub shared transform core](https://github.com/nubjs/nub/blob/9e2eb69a0bd164be54a9c1741bb3820bc224fc85/runtime/transform-core.mjs)
- [Nub Rust/Oxc N-API transform](https://github.com/nubjs/nub/blob/9e2eb69a0bd164be54a9c1741bb3820bc224fc85/crates/nub-native/src/transform.rs)
- [TypeScript `jsxImportSource`](https://www.typescriptlang.org/tsconfig/jsxImportSource.html)
- [TypeScript JSX runtime entry points](https://www.typescriptlang.org/docs/handbook/jsx)
- [Node module customization hooks](https://nodejs.org/api/module.html#customization-hooks)
- [npm optional dependencies](https://docs.npmjs.com/cli/configuring-npm/package-json/#optionaldependencies)
- [A3S GUI architecture](architecture.md)
- [A3S GUI protocol and app shell](app-shell.md)
- [A3S GUI delivery roadmap](roadmap.md)
