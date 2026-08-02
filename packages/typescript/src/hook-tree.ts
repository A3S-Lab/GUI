import {
  A3sJsxError,
  describeElementType,
  type A3sFunctionComponent,
} from "./element.ts";
import { isA3sContext, type A3sContext } from "./context.ts";
import { ContextValueStack } from "./context-stack.ts";
import type {
  ComponentRenderCheckpoint,
  ComponentRenderRequest,
  ComponentRenderRuntime,
} from "./component-runtime.ts";
import {
  A3sHookError,
  withHookDispatcher,
  type A3sDispatch,
  type A3sEffect,
  type A3sEffectCleanup,
  type A3sMutableRef,
  type A3sReducer,
  type A3sStateSetter,
  type A3sStateUpdate,
  type HookDispatcher,
} from "./hooks.ts";
import { dependenciesEqual, snapshotDependencies } from "./hook-dependencies.ts";

type HookKind = HookSlot["kind"];

interface StateCell<State = unknown> {
  value: State;
  mounted: boolean;
  readonly set: A3sStateSetter<State>;
}

interface ReducerCell<State = unknown, Action = unknown> {
  value: State;
  mounted: boolean;
  reducer: A3sReducer<State, Action>;
  readonly dispatch: A3sDispatch<Action>;
}

interface StateHook {
  readonly kind: "state";
  readonly cell: StateCell;
}

interface ReducerHook {
  readonly kind: "reducer";
  readonly cell: ReducerCell;
  readonly reducer: A3sReducer<unknown, unknown>;
}

interface MemoHook {
  readonly kind: "memo";
  readonly value: unknown;
  readonly dependencies: readonly unknown[];
}

interface RefHook {
  readonly kind: "ref";
  readonly ref: A3sMutableRef<unknown>;
}

interface EffectHook {
  readonly kind: "effect";
  readonly effect: A3sEffect;
  readonly dependencies: readonly unknown[] | null;
  cleanup: A3sEffectCleanup | null;
  readonly changed: boolean;
}

interface ContextHook {
  readonly kind: "context";
}

type HookSlot = StateHook | ReducerHook | MemoHook | RefHook | EffectHook | ContextHook;

interface ComponentInstance {
  readonly identity: string;
  readonly component: A3sFunctionComponent;
  readonly name: string;
  readonly slots: HookSlot[];
}

interface CleanupTask {
  readonly component: string;
  readonly hook: number;
  readonly cleanup: A3sEffectCleanup;
}

interface SetupTask {
  readonly component: string;
  readonly hook: number;
  readonly slot: EffectHook;
}

export class ComponentHookTree implements ComponentRenderRuntime {
  readonly #scheduleUpdate: () => void;
  #active = new Map<string, ComponentInstance>();
  #candidate: Map<string, ComponentInstance> | null = null;
  readonly #contexts = new ContextValueStack();
  #renderDepth = 0;

  constructor(scheduleUpdate: () => void) {
    this.#scheduleUpdate = scheduleUpdate;
  }

  get activeComponentCount(): number {
    return this.#active.size;
  }

  beginCandidate(): void {
    if (this.#candidate !== null) {
      throw new A3sHookError("hookOrder", "a component render candidate is already active");
    }
    this.#candidate = new Map();
  }

  abortCandidate(): void {
    if (this.#candidate !== null) {
      for (const instance of this.#candidate.values()) {
        if (this.#active.get(instance.identity)?.component !== instance.component) {
          markInstanceUnmounted(instance);
        }
      }
    }
    this.#candidate = null;
  }

  renderComponent(
    request: Readonly<ComponentRenderRequest>,
    invoke: () => unknown,
  ): unknown {
    const candidate = this.#candidate;
    if (candidate === null) {
      throw new A3sHookError("hookOrder", "component hooks require an active render candidate");
    }
    if (candidate.has(request.identity)) {
      throw new A3sJsxError(
        `component identity ${JSON.stringify(request.identity)} is duplicated`,
        request.source,
      );
    }

    const name = describeElementType(request.component);
    const previous = this.#active.get(request.identity);
    const committed = previous?.component === request.component ? previous : null;
    const dispatcher = new InstanceHookDispatcher(this, name, committed);
    this.#renderDepth += 1;
    try {
      const output = withHookDispatcher(dispatcher, invoke);
      dispatcher.finish();
      candidate.set(request.identity, {
        identity: request.identity,
        component: request.component,
        name,
        slots: dispatcher.slots,
      });
      return output;
    } catch (error) {
      if (error instanceof A3sHookError) {
        throw new A3sJsxError(error.message, request.source, error);
      }
      throw error;
    } finally {
      this.#renderDepth -= 1;
    }
  }

  withContextValue<Value, Result>(
    context: A3sContext<Value>,
    value: Value,
    callback: () => Result,
  ): Result {
    return this.#contexts.withValue(context, value, callback);
  }

  readContext<Value>(context: A3sContext<Value>): Value {
    if (!isA3sContext(context)) {
      throw new TypeError("useContext requires a context created by createContext");
    }
    return this.#contexts.read(context);
  }

  createCheckpoint(): ComponentRenderCheckpoint {
    const candidate = this.#requireCandidate();
    return Object.freeze({
      candidateIdentities: new Set(candidate.keys()),
    });
  }

  rollbackToCheckpoint(checkpoint: ComponentRenderCheckpoint): void {
    const candidate = this.#requireCandidate();
    for (const [identity, instance] of candidate) {
      if (checkpoint.candidateIdentities.has(identity)) {
        continue;
      }
      candidate.delete(identity);
      if (this.#active.get(identity)?.component !== instance.component) {
        markInstanceUnmounted(instance);
      }
    }
  }

  commitCandidate(): readonly A3sHookError[] {
    const candidate = this.#candidate;
    if (candidate === null) {
      throw new A3sHookError("hookOrder", "no component render candidate is ready to commit");
    }
    this.#candidate = null;

    const cleanups: CleanupTask[] = [];
    const setups: SetupTask[] = [];
    const previousInstances = [...this.#active.values()];
    for (let index = previousInstances.length - 1; index >= 0; index -= 1) {
      const previous = previousInstances[index];
      const next = candidate.get(previous.identity);
      if (next?.component !== previous.component) {
        collectAllCleanups(previous, cleanups);
        markInstanceUnmounted(previous);
        continue;
      }
      collectChangedCleanups(previous, next, cleanups);
    }

    for (const instance of candidate.values()) {
      for (let hook = 0; hook < instance.slots.length; hook += 1) {
        const slot = instance.slots[hook];
        if (slot.kind === "state") {
          slot.cell.mounted = true;
        } else if (slot.kind === "reducer") {
          slot.cell.mounted = true;
          slot.cell.reducer = slot.reducer;
        } else if (slot.kind === "effect" && slot.changed) {
          slot.cleanup = null;
          setups.push({ component: instance.name, hook, slot });
        }
      }
    }

    this.#active = candidate;
    const errors = runCleanups(cleanups);
    errors.push(...runSetups(setups));
    return Object.freeze(errors);
  }

  dispose(): readonly A3sHookError[] {
    this.abortCandidate();
    const cleanups: CleanupTask[] = [];
    const instances = [...this.#active.values()];
    for (let index = instances.length - 1; index >= 0; index -= 1) {
      collectAllCleanups(instances[index], cleanups);
      markInstanceUnmounted(instances[index]);
    }
    this.#active.clear();
    return Object.freeze(runCleanups(cleanups));
  }

  createStateCell<State>(initial: State): StateCell<State> {
    const cell = {
      value: initial,
      mounted: false,
      set: null as unknown as A3sStateSetter<State>,
    };
    cell.set = (update) => this.#updateState(cell, update);
    return cell;
  }

  createReducerCell<State, Action>(
    reducer: A3sReducer<State, Action>,
    initial: State,
  ): ReducerCell<State, Action> {
    const cell = {
      value: initial,
      mounted: false,
      reducer,
      dispatch: null as unknown as A3sDispatch<Action>,
    };
    cell.dispatch = (action) => this.#dispatchReducer(cell, action);
    return cell;
  }

  #updateState<State>(cell: StateCell<State>, update: A3sStateUpdate<State>): void {
    this.#assertCanUpdate();
    if (!cell.mounted) {
      return;
    }
    const next = typeof update === "function"
      ? (update as (previous: State) => State)(cell.value)
      : update;
    if (!Object.is(cell.value, next)) {
      cell.value = next;
      this.#scheduleUpdate();
    }
  }

  #dispatchReducer<State, Action>(cell: ReducerCell<State, Action>, action: Action): void {
    this.#assertCanUpdate();
    if (!cell.mounted) {
      return;
    }
    const next = cell.reducer(cell.value, action);
    if (!Object.is(cell.value, next)) {
      cell.value = next;
      this.#scheduleUpdate();
    }
  }

  #assertCanUpdate(): void {
    if (this.#renderDepth > 0) {
      throw new A3sHookError(
        "renderPhaseUpdate",
        "A3S state cannot be updated while a function component is rendering",
      );
    }
  }

  #requireCandidate(): Map<string, ComponentInstance> {
    if (this.#candidate === null) {
      throw new A3sHookError("hookOrder", "component hooks require an active render candidate");
    }
    return this.#candidate;
  }
}

class InstanceHookDispatcher implements HookDispatcher {
  readonly slots: HookSlot[] = [];
  readonly #tree: ComponentHookTree;
  readonly #name: string;
  readonly #committed: ComponentInstance | null;
  #index = 0;

  constructor(
    tree: ComponentHookTree,
    name: string,
    committed: ComponentInstance | null,
  ) {
    this.#tree = tree;
    this.#name = name;
    this.#committed = committed;
  }

  useState<State>(initial: State | (() => State)): readonly [State, A3sStateSetter<State>] {
    const previous = this.#next("state") as StateHook | null;
    const cell = previous === null
      ? this.#tree.createStateCell(
        typeof initial === "function" ? (initial as () => State)() : initial,
      )
      : previous.cell as StateCell<State>;
    this.slots.push({ kind: "state", cell: cell as StateCell });
    return Object.freeze([cell.value, cell.set] as const);
  }

  useReducer<State, Action>(
    reducer: A3sReducer<State, Action>,
    initial: State,
  ): readonly [State, A3sDispatch<Action>] {
    if (typeof reducer !== "function") {
      throw new TypeError("useReducer requires a reducer function");
    }
    const previous = this.#next("reducer") as ReducerHook | null;
    const cell = previous === null
      ? this.#tree.createReducerCell(reducer, initial)
      : previous.cell as ReducerCell<State, Action>;
    this.slots.push({
      kind: "reducer",
      cell: cell as ReducerCell,
      reducer: reducer as A3sReducer<unknown, unknown>,
    });
    return Object.freeze([cell.value, cell.dispatch] as const);
  }

  useMemo<Value>(factory: () => Value, dependencies: readonly unknown[]): Value {
    if (typeof factory !== "function") {
      throw new TypeError("useMemo requires a factory function");
    }
    const snapshot = snapshotDependencies(dependencies, "useMemo");
    const previous = this.#next("memo") as MemoHook | null;
    const value = previous !== null && dependenciesEqual(previous.dependencies, snapshot)
      ? previous.value as Value
      : factory();
    this.slots.push({ kind: "memo", value, dependencies: snapshot });
    return value;
  }

  useRef<Value>(initial: Value): A3sMutableRef<Value> {
    const previous = this.#next("ref") as RefHook | null;
    const ref = previous === null ? { current: initial } : previous.ref;
    this.slots.push({ kind: "ref", ref: ref as A3sMutableRef<unknown> });
    return ref as A3sMutableRef<Value>;
  }

  useEffect(effect: A3sEffect, dependencies: readonly unknown[] | undefined): void {
    if (typeof effect !== "function") {
      throw new TypeError("useEffect requires an effect function");
    }
    const snapshot = dependencies === undefined
      ? null
      : snapshotDependencies(dependencies, "useEffect");
    const previous = this.#next("effect") as EffectHook | null;
    const changed = previous === null ||
      snapshot === null ||
      previous.dependencies === null ||
      !dependenciesEqual(previous.dependencies, snapshot);
    this.slots.push({
      kind: "effect",
      effect,
      dependencies: snapshot,
      cleanup: previous?.cleanup ?? null,
      changed,
    });
  }

  useContext<Value>(context: A3sContext<Value>): Value {
    this.#next("context");
    const value = this.#tree.readContext(context);
    this.slots.push({ kind: "context" });
    return value;
  }

  finish(): void {
    const expected = this.#committed?.slots.length ?? this.#index;
    if (this.#committed !== null && this.#index < expected) {
      throw new A3sHookError(
        "hookOrder",
        `${this.#name} rendered fewer hooks than its committed instance`,
      );
    }
  }

  #next(kind: HookKind): HookSlot | null {
    const index = this.#index;
    this.#index += 1;
    if (this.#committed === null) {
      return null;
    }
    const previous = this.#committed.slots[index];
    if (previous === undefined) {
      throw new A3sHookError(
        "hookOrder",
        `${this.#name} rendered more hooks than its committed instance`,
      );
    }
    if (previous.kind !== kind) {
      throw new A3sHookError(
        "hookOrder",
        `${this.#name} changed hook ${index + 1} from ${previous.kind} to ${kind}`,
      );
    }
    return previous;
  }
}

function collectAllCleanups(instance: ComponentInstance, tasks: CleanupTask[]): void {
  for (let hook = instance.slots.length - 1; hook >= 0; hook -= 1) {
    const slot = instance.slots[hook];
    if (slot.kind === "effect" && slot.cleanup !== null) {
      tasks.push({ component: instance.name, hook, cleanup: slot.cleanup });
    }
  }
}

function collectChangedCleanups(
  previous: ComponentInstance,
  next: ComponentInstance,
  tasks: CleanupTask[],
): void {
  for (let hook = previous.slots.length - 1; hook >= 0; hook -= 1) {
    const oldSlot = previous.slots[hook];
    const newSlot = next.slots[hook];
    if (
      oldSlot.kind === "effect" &&
      newSlot?.kind === "effect" &&
      newSlot.changed &&
      oldSlot.cleanup !== null
    ) {
      tasks.push({ component: previous.name, hook, cleanup: oldSlot.cleanup });
    }
  }
}

function markInstanceUnmounted(instance: ComponentInstance): void {
  for (const slot of instance.slots) {
    if (slot.kind === "state" || slot.kind === "reducer") {
      slot.cell.mounted = false;
    }
  }
}

function runCleanups(tasks: readonly CleanupTask[]): A3sHookError[] {
  const errors: A3sHookError[] = [];
  for (const task of tasks) {
    try {
      task.cleanup();
    } catch (cause) {
      errors.push(new A3sHookError(
        "effectFailed",
        `${task.component} effect ${task.hook + 1} cleanup failed`,
        cause,
      ));
    }
  }
  return errors;
}

function runSetups(tasks: readonly SetupTask[]): A3sHookError[] {
  const errors: A3sHookError[] = [];
  for (const task of tasks) {
    try {
      const cleanup = task.slot.effect();
      if (isThenable(cleanup)) {
        throw new TypeError("effects cannot return promises");
      }
      if (cleanup !== undefined && typeof cleanup !== "function") {
        throw new TypeError("effects must return a cleanup function or undefined");
      }
      task.slot.cleanup = cleanup ?? null;
    } catch (cause) {
      errors.push(new A3sHookError(
        "effectFailed",
        `${task.component} effect ${task.hook + 1} failed`,
        cause,
      ));
    }
  }
  return errors;
}

function isThenable(value: unknown): value is PromiseLike<unknown> {
  return (
    (typeof value === "object" && value !== null) || typeof value === "function"
  ) && typeof (value as { then?: unknown }).then === "function";
}
