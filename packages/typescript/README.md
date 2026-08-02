# @a3s/gui TypeScript development package

This private package contains the protocol-v1 JSX core and the first stateful
application scheduler. It provides the standard `jsx-runtime` and
`jsx-dev-runtime` entry points, immutable A3S element records, keyed
function-component instances, fragment/child/key/prop normalization, preview
`Window`/`View`/`Text`/`Button` tokens, and deterministic lowering to the
Rust-generated `ProtocolUiFrameV1` declarations.

Function event props become collision-safe action ids. Their functions are
retained in a read-only callback snapshot and never enter JSON.
`RevisionActionRegistryV1` promotes those snapshots only after a matching
protocol `committed` message. It keeps one pending, one active, and one
rollback scope; rejects stale render/host revisions and non-consecutive event
sequences before callback execution; preflights every action in an event
vector; and awaits callbacks in exact wire order. A callback error consumes
the sequence and stops later callbacks so a partially executed event is never
replayed.

`A3sClientSessionV1` owns the post-handshake envelope around that registry. It
validates and snapshots the host `welcome`, emits complete `render` messages,
tracks an independent client sequence plus distinct ordered host receipt and
semantic-application high-water marks, enforces a bounded reorder window and
the negotiated frame-byte limit, and rejects the wrong session before revision
or callback preflight. Protocol failures poison the session; an application
callback failure consumes the event and host message exactly once without
poisoning it. It also owns bidirectional ping/pong and close/ack state, so the
byte transport cannot bypass protocol ordering during liveness or shutdown.
`createApp` serializes overlapping host events and commit acknowledgements so
callback execution cannot race callback-scope promotion.

`A3sClientHandshakeV1` owns the preceding `hello`/`welcome` negotiation, while
`encodeA3sJsonFrameV1` and `A3sJsonFrameDecoderV1` implement the Rust-compatible
four-byte little-endian length prefix over strict UTF-8 JSON. The incremental
decoder accepts arbitrary stream chunking, validates lengths before allocation,
and becomes unusable after a boundary or JSON violation. Encoding snapshots
plain JSON data without invoking accessors. These APIs add no runtime package
dependency.

`A3sFramedClientConnectionV1` connects that negotiation to a single-reader byte
transport, serializes client writes, narrows partially received frames to the
negotiated limit, and snapshots host messages before exposing them.
`A3sNodeProcessTransportV1` supplies a real Node `child_process` adapter with an
explicit command, no shell, bounded stderr retention, abnormal-exit reporting,
and timeout-backed shutdown. Runtime code uses only Node built-ins;
`@types/node` is pinned as a development-only type-checking dependency.

`createApp` owns the transport-neutral root lifecycle. `useState`,
`useReducer`, `useMemo`, `useRef`, `useContext`, and post-commit `useEffect` use
deterministic component paths and hook slots. Nested providers are transparent
Node-only scopes and never enter the wire frame. `ErrorBoundary` rolls back a
failed descendant candidate before resolving its value or function fallback;
a failed fallback rejects the whole candidate and preserves the committed
frame, callbacks, and effects. State changes are batched across an entire
ordered event vector, and candidates remain isolated until the typed host
returns `committed`. Keyed component reorder preserves state; committed removal
and shutdown run bounded deterministic cleanup.

```tsx
const Theme = createContext<"light" | "dark">("light");

<Theme.Provider value="dark">
  <ErrorBoundary fallback={(error) => <Text>{error.message}</Text>}>
    <Workspace />
  </ErrorBoundary>
</Theme.Provider>;
```

```ts
const compiled = compileFrameV1("counter", <Counter />);
const callbacks = new RevisionActionRegistryV1();

callbacks.stage(1, compiled);       // active callbacks are unchanged
callbacks.commit(committedMessage); // matching revision + frame only
await callbacks.dispatch(eventMessage);
callbacks.clear();                  // release all retained callback scopes
```

The Rust `a3s-gui-tsx-host` process accepts strict framed
hello/render/ping/pong/close traffic, commits full frames through the software
self-drawn runtime, emits idle host pings, and terminates on a fixed unanswered
probe deadline. `A3sFramedApplicationHostV1` joins the negotiated connection to
`createApp` with one shared client session, one reader, bounded event tasks,
ordered commit/event delivery, bidirectional liveness, and protocol close/ack;
a real Node fixture drives two renders, both ping directions, and graceful
shutdown through the Rust process. Hostless `createApp` definitions now expose
`run()`: it validates the exact platform artifact, generates the handshake
session id, binds event dispatch before the initial render, and returns the
running application after its first commit. An explicit bounded recovery policy
observes Host termination, reconnects with a fresh session identity, replays
the retained committed frame at revision 1 without resetting component state,
and gates events until the replay commits. Exhausting the global restart budget
reports `A3sApplicationRecoveryError` and closes deterministically. Native OS
hosts, published platform artifacts, and the stable full semantic component API
remain later delivery slices. This package is therefore not yet a published
native SDK.

The zero-argument application path is:

```ts
const app = await createApp(App).run();
await app.shutdown();
```

Development supervision is opt-in and strictly bounded:

```ts
const app = await createApp(App).run({
  recovery: {
    maximumRestarts: 2, // 1..16 across the application lifetime
    restartDelayMs: 250, // 0..60_000
  },
});
```

Without `recovery`, restart policy remains application-owned. A custom policy
may call `app.beginRecovery()` and then `app.recover(freshHost)`; the Host must
use a different negotiated session identity. A successful replay increments
`app.state.hostGeneration` and `app.state.replayedRenders` but not logical
`committedRenders`.

Installed releases resolve an exact optional platform package and verify its
strict manifest plus executable SHA-256 before spawning. Repository development
and tests may set `A3S_GUI_TSX_HOST` to an absolute prebuilt Host path together
with `A3S_GUI_ALLOW_UNVERIFIED_HOST=1`. The override is explicit and does not act
as a production PATH fallback.

Typed Host injection remains available for transport and failure testing:

```ts
const host = await connectA3sNodeApplicationHostV1({
  process: { command: hostBinary },
  handshake: {
    sdkVersion: "0.0.0-development",
    sessionId: "app-session",
    requestedRenderer: "software",
    maximumFrameBytes: 1024 * 1024,
  },
});
const app = createApp(App, { host });
host.setEventHandler(async (message) => { await app.dispatch(message); });
await app.start();
await host.ping(1);
await app.shutdown();
```

Install the pinned development compiler without running dependency scripts:

```sh
npm ci --ignore-scripts
```

Regenerate the protocol module from the Rust DTO source of truth:

```sh
just generate-tsx-protocol
```

Check declaration drift, compile the real automatic-runtime TSX fixture, and
run the dependency-free Node runtime/golden tests:

```sh
just check-tsx-protocol
just test-typescript
```

The TypeScript package has no production dependencies or install scripts. The
pinned TypeScript compiler is a test-only development dependency.
