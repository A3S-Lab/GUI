export type A3sStateUpdate<State> = State | ((previous: State) => State);
export type A3sStateSetter<State> = (update: A3sStateUpdate<State>) => void;
export type A3sReducer<State, Action> = (state: State, action: Action) => State;
export type A3sDispatch<Action> = (action: Action) => void;
export type A3sEffectCleanup = () => void;
export type A3sEffect = () => void | A3sEffectCleanup;

export interface A3sMutableRef<Value> {
  current: Value;
}

export type A3sHookErrorCode =
  | "effectFailed"
  | "hookOrder"
  | "hookOutsideComponent"
  | "invalidDependencies"
  | "renderPhaseUpdate";

export class A3sHookError extends Error {
  readonly code: A3sHookErrorCode;

  constructor(code: A3sHookErrorCode, message: string, cause?: unknown) {
    super(message, cause === undefined ? undefined : { cause });
    this.name = "A3sHookError";
    this.code = code;
  }
}

export interface HookDispatcher {
  useState<State>(
    initial: State | (() => State),
  ): readonly [State, A3sStateSetter<State>];
  useReducer<State, Action>(
    reducer: A3sReducer<State, Action>,
    initial: State,
  ): readonly [State, A3sDispatch<Action>];
  useMemo<Value>(factory: () => Value, dependencies: readonly unknown[]): Value;
  useRef<Value>(initial: Value): A3sMutableRef<Value>;
  useEffect(effect: A3sEffect, dependencies: readonly unknown[] | undefined): void;
}

let currentDispatcher: HookDispatcher | null = null;

export function useState<State>(
  initial: State | (() => State),
): readonly [State, A3sStateSetter<State>] {
  return requireDispatcher("useState").useState(initial);
}

export function useReducer<State, Action>(
  reducer: A3sReducer<State, Action>,
  initial: State,
): readonly [State, A3sDispatch<Action>] {
  return requireDispatcher("useReducer").useReducer(reducer, initial);
}

export function useMemo<Value>(
  factory: () => Value,
  dependencies: readonly unknown[],
): Value {
  return requireDispatcher("useMemo").useMemo(factory, dependencies);
}

export function useRef<Value>(initial: Value): A3sMutableRef<Value> {
  return requireDispatcher("useRef").useRef(initial);
}

export function useEffect(
  effect: A3sEffect,
  dependencies?: readonly unknown[],
): void {
  requireDispatcher("useEffect").useEffect(effect, dependencies);
}

export function withHookDispatcher<Value>(
  dispatcher: HookDispatcher,
  callback: () => Value,
): Value {
  const previous = currentDispatcher;
  currentDispatcher = dispatcher;
  try {
    return callback();
  } finally {
    currentDispatcher = previous;
  }
}

function requireDispatcher(name: string): HookDispatcher {
  if (currentDispatcher === null) {
    throw new A3sHookError(
      "hookOutsideComponent",
      `${name} can only run while an A3S function component is rendering`,
    );
  }
  return currentDispatcher;
}
