# @a3s/gui TypeScript development package

This private package contains the first headless protocol-v1 JSX core. It now
provides the standard `jsx-runtime` and `jsx-dev-runtime` entry points,
immutable A3S element records, synchronous function-component expansion,
fragment/child/key/prop normalization, preview `Window`/`View`/`Text`/`Button`
tokens, and deterministic lowering to the Rust-generated
`ProtocolUiFrameV1` declarations.

Function event props become collision-safe action ids. Their functions are
retained in a read-only callback snapshot and never enter JSON. That snapshot
is not yet the committed/rollback revision registry: event-vector dispatch,
state/hooks, the Node process session, the native host executable, and the
stable full semantic component API remain later delivery slices. This package
is therefore not a published SDK or a runnable native TSX application yet.

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
