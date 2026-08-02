# Self-Drawn Application Shell

## Status

The repository does not currently ship an executable macOS, Windows, or Linux
window shell. The former platform content-control shells and their examples
were deleted with their dependencies.

What exists is the shared shell contract:

- `PlatformHost` for zero-widget OS transactions and events;
- `SelfDrawnWindowRuntime` for atomic semantic/layout/scene/presentation
  frames;
- recording and reference implementations for deterministic tests;
- `NativeRuntimeApp` and protocol application loops for headless semantic
  reducer tests.

A concrete OS shell must be built on `PlatformHost`; it must not revive a
content-widget backend.

## Shared lifecycle

A production shell follows this lifecycle:

1. create the OS application/event-loop context;
2. create a top-level window and Graphics surface;
3. construct the semantic application and `SelfDrawnWindowRuntime`;
4. render a candidate semantic/layout/scene frame;
5. prepare and commit one host transaction;
6. present the matching prepared frame;
7. drain normalized host events;
8. route actions to Rust reducers or TypeScript callbacks;
9. rerender when state changes;
10. recover surface/host state or close cleanly.

Semantic state advances only after the host and presentation acknowledgements
match the candidate revision.

## Event batches

Host events are drained in bounded batches. Each event is validated before
routing. An event may:

- update interaction/focus/selection state without invoking an application
  callback;
- invoke one or more ordered callbacks;
- request a rerender;
- request a system service;
- close the application.

A reducer failure does not commit a partial rerender. Stale host, frame, and
element identities are rejected.

## Window contract

`PlatformWindowSpec` and `PlatformWindowCommand` carry title, logical size,
constraints, visibility, and lifecycle requests. `PlatformWindowEvent`
reports resize, scale, focus, close, and surface state.

Window geometry is not component layout. The host supplies the viewport and
scale; A3S layout computes every content box.

## Input, text, and accessibility

The shell converts OS input to `PlatformInputEvent`. Hit testing and semantic
routing stay in the shared runtime.

Text input uses explicit `PlatformTextInputCommand` and
`PlatformTextInputEvent` sessions. No hidden platform content control may own
the editor state.

Accessibility uses `PlatformAccessibilitySnapshot` and
`PlatformAccessibilityAction`. The host publishes the snapshot through the
operating system and returns actions to the semantic runtime.

## Development workflow

Portable shell work is verified with:

```sh
just check-platform-host
just test-platform-host
just check-platform-runtime
just test-platform-runtime
```

The maintained self-drawn reference example is:

```sh
cargo run --locked --no-default-features \
  --features authoring,platform-runtime,software-reference \
  --example self_drawn_calculator
```

This exercises semantic compilation, layout, scene extraction, reference
presentation, and H1 frame semantics without pretending to be a real OS
window.

## Concrete-host acceptance

A platform shell is not complete until it demonstrates:

- create/show/resize/focus/close lifecycle;
- scale and surface-loss recovery;
- real Graphics presentation;
- pointer, keyboard, wheel, text/IME, and accessibility actions;
- clipboard and declared system services;
- bounded queues and stale-revision rejection;
- deterministic story parity with the software reference path;
- no application-content widget or toolkit-layout dependency.

Packaging resumes only after a concrete self-drawn shell satisfies this gate.
