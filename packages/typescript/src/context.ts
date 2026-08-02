import type {
  A3sFunctionComponent,
  A3sJsxChild,
  A3sJsxProps,
} from "./element.ts";

const CONTEXT_MARKER = Symbol.for("@a3s/gui.context.v1");
const CONTEXT_PROVIDER_MARKER = Symbol.for("@a3s/gui.context-provider.v1");

export interface A3sContextProviderProps<Value> extends A3sJsxProps {
  readonly value: Value;
  readonly children?: A3sJsxChild;
}

export interface A3sContextProvider<Value>
  extends A3sFunctionComponent<A3sContextProviderProps<Value>> {
  readonly $$typeof: typeof CONTEXT_PROVIDER_MARKER;
  readonly context: A3sContext<Value>;
}

export interface A3sContext<Value> {
  readonly $$typeof: typeof CONTEXT_MARKER;
  readonly defaultValue: Value;
  readonly Provider: A3sContextProvider<Value>;
}

export function createContext<Value>(defaultValue: Value): A3sContext<Value> {
  const context = {} as A3sContext<Value>;
  const provider = function A3sContextProvider(
    props: Readonly<A3sContextProviderProps<Value>>,
  ): A3sJsxChild {
    return props.children ?? null;
  } as A3sContextProvider<Value>;

  Object.defineProperties(provider, {
    $$typeof: { value: CONTEXT_PROVIDER_MARKER },
    context: { value: context },
  });
  Object.defineProperties(context, {
    $$typeof: { value: CONTEXT_MARKER },
    defaultValue: { value: defaultValue },
    Provider: { value: Object.freeze(provider) },
  });
  return Object.freeze(context);
}

export function isA3sContext(value: unknown): value is A3sContext<unknown> {
  return (
    typeof value === "object" &&
    value !== null &&
    Object.isFrozen(value) &&
    (value as Partial<A3sContext<unknown>>).$$typeof === CONTEXT_MARKER
  );
}

export function isA3sContextProvider(
  value: unknown,
): value is A3sContextProvider<unknown> {
  return (
    typeof value === "function" &&
    Object.isFrozen(value) &&
    (value as Partial<A3sContextProvider<unknown>>).$$typeof ===
      CONTEXT_PROVIDER_MARKER &&
    isA3sContext((value as Partial<A3sContextProvider<unknown>>).context)
  );
}
