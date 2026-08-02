import type {
  A3sFunctionComponent,
  A3sJsxChild,
  A3sJsxError,
  A3sJsxProps,
} from "./element.ts";

const ERROR_BOUNDARY_MARKER = Symbol.for("@a3s/gui.error-boundary.v1");

export type A3sErrorFallback =
  | A3sJsxChild
  | ((error: A3sJsxError) => A3sJsxChild);

export interface A3sErrorBoundaryProps extends A3sJsxProps {
  readonly fallback: A3sErrorFallback;
  readonly children?: A3sJsxChild;
}

export interface A3sErrorBoundaryComponent
  extends A3sFunctionComponent<A3sErrorBoundaryProps> {
  readonly $$typeof: typeof ERROR_BOUNDARY_MARKER;
}

const errorBoundary = function ErrorBoundary(
  props: Readonly<A3sErrorBoundaryProps>,
): A3sJsxChild {
  return props.children ?? null;
} as A3sErrorBoundaryComponent;

Object.defineProperty(errorBoundary, "$$typeof", {
  value: ERROR_BOUNDARY_MARKER,
});

export const ErrorBoundary: A3sErrorBoundaryComponent = Object.freeze(errorBoundary);

export function isA3sErrorBoundary(
  value: unknown,
): value is A3sErrorBoundaryComponent {
  return (
    typeof value === "function" &&
    Object.isFrozen(value) &&
    (value as Partial<A3sErrorBoundaryComponent>).$$typeof === ERROR_BOUNDARY_MARKER
  );
}
