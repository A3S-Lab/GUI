import type {
  A3sFunctionComponent,
  A3sJsxProps,
  A3sSourceLocation,
} from "./element.ts";
import type { A3sContext } from "./context.ts";
import type { A3sComponentIdentityV1 } from "./identity.ts";

export interface ComponentRenderRequest {
  readonly identity: A3sComponentIdentityV1;
  readonly component: A3sFunctionComponent;
  readonly props: Readonly<A3sJsxProps>;
  readonly source: A3sSourceLocation | null;
}

export interface ComponentRenderCheckpoint {
  readonly candidateIdentities: ReadonlySet<A3sComponentIdentityV1>;
}

export interface ComponentRenderRuntime {
  renderComponent(
    request: Readonly<ComponentRenderRequest>,
    invoke: () => unknown,
  ): unknown;
  withContextValue<Value, Result>(
    context: A3sContext<Value>,
    value: Value,
    callback: () => Result,
  ): Result;
  createCheckpoint(): ComponentRenderCheckpoint;
  rollbackToCheckpoint(checkpoint: ComponentRenderCheckpoint): void;
}
