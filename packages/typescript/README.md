# @a3s/gui TypeScript development package

This private package currently contains the generated protocol-v1 declarations
and cross-language golden tests only. It is not a published SDK and does not yet
provide `jsx-runtime`, `jsx-dev-runtime`, a Node process host, components, hooks,
or a callback registry.

Regenerate the protocol module from the Rust DTO source of truth:

```sh
just generate-tsx-protocol
```

Check drift and run the dependency-free Node fixture tests:

```sh
just check-tsx-protocol
just test-typescript
```
