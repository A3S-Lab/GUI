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

The Node process session and actual local I/O, host supervision/replay, the
native host executable, and the stable full semantic component API remain later
delivery slices. This package is therefore not a published SDK or a runnable
native TSX application yet. The current `createApp` requires an explicit typed
`A3sApplicationHostV1`; the future zero-configuration `run()` API will supply
the supervised process host.

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
