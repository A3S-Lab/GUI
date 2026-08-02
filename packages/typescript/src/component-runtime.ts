import type {
  A3sFunctionComponent,
  A3sJsxProps,
  A3sSourceLocation,
} from "./element.ts";

export interface ComponentRenderRequest {
  readonly identity: string;
  readonly component: A3sFunctionComponent;
  readonly props: Readonly<A3sJsxProps>;
  readonly source: A3sSourceLocation | null;
}

export interface ComponentRenderRuntime {
  renderComponent(
    request: Readonly<ComponentRenderRequest>,
    invoke: () => unknown,
  ): unknown;
}
