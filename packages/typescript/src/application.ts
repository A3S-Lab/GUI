import type {
  A3sActionDispatchResultV1,
  A3sActionRegistryStateV1,
  TsxCommittedMessageV1,
  TsxEventMessageV1,
} from "./action-registry.ts";
import { RevisionActionRegistryV1 } from "./action-registry.ts";
import {
  createA3sElement,
  type A3sFunctionComponent,
  type A3sJsxProps,
} from "./element.ts";
import {
  compileFrameWithRuntimeV1,
  type CompileFrameOptions,
} from "./frame.ts";
import type { ProtocolUiFrameV1 } from "./generated/protocol.ts";
import { ComponentHookTree } from "./hook-tree.ts";

export interface A3sRenderCandidateV1 {
  readonly renderRevision: number;
  readonly frame: Readonly<ProtocolUiFrameV1>;
}

export interface A3sApplicationHostV1 {
  submitRender(
    candidate: Readonly<A3sRenderCandidateV1>,
  ): TsxCommittedMessageV1 | Promise<TsxCommittedMessageV1>;
  close?(): void | Promise<void>;
}

export type A3sApplicationStatus = "created" | "running" | "closing" | "closed";

export interface A3sApplicationStateV1 {
  readonly status: A3sApplicationStatus;
  readonly dirty: boolean;
  readonly renderInFlight: boolean;
  readonly committedRenders: number;
  readonly activeComponents: number;
  readonly lastError: unknown | null;
  readonly actions: Readonly<A3sActionRegistryStateV1>;
}

interface CreateAppBaseOptions {
  readonly host: A3sApplicationHostV1;
  readonly frameId?: string;
  readonly compile?: CompileFrameOptions;
  readonly onError?: (error: unknown) => void;
}

export type CreateAppOptions<Props extends A3sJsxProps> = CreateAppBaseOptions &
  ({} extends Props
    ? { readonly props?: Readonly<Props> }
    : { readonly props: Readonly<Props> });

export class A3sApplicationV1<Props extends A3sJsxProps = A3sJsxProps> {
  readonly #root: A3sFunctionComponent<Props>;
  readonly #host: A3sApplicationHostV1;
  readonly #frameId: string;
  readonly #compileOptions: CompileFrameOptions;
  readonly #onError: ((error: unknown) => void) | null;
  readonly #actions = new RevisionActionRegistryV1();
  readonly #hooks: ComponentHookTree;
  #props: Readonly<Props>;
  #status: A3sApplicationStatus = "created";
  #dirty = false;
  #scheduled = false;
  #batchDepth = 0;
  #committedRenders = 0;
  #lastError: unknown | null = null;
  #renderStarting = false;
  #inFlight: Promise<boolean> | null = null;

  constructor(
    root: A3sFunctionComponent<Props>,
    options: CreateAppOptions<Props>,
  ) {
    if (typeof root !== "function") {
      throw new TypeError("createApp requires a synchronous function component");
    }
    if (!isPlainRecord(options)) {
      throw new TypeError("createApp options must be a plain object");
    }
    if (
      typeof options.host !== "object" ||
      options.host === null ||
      typeof options.host.submitRender !== "function"
    ) {
      throw new TypeError("createApp requires a typed host with submitRender");
    }
    if (options.onError !== undefined && typeof options.onError !== "function") {
      throw new TypeError("createApp onError must be a function");
    }

    this.#root = root;
    this.#host = options.host;
    const frameId = options.frameId ?? "app";
    if (typeof frameId !== "string" || frameId.length === 0) {
      throw new TypeError("createApp frameId must be a non-empty string");
    }
    this.#frameId = frameId;
    const initialProps = options.props === undefined ? {} as Props : options.props;
    this.#props = snapshotProps<Props>(initialProps);
    this.#compileOptions = Object.freeze({ ...(options.compile ?? {}) });
    this.#onError = options.onError ?? null;
    this.#hooks = new ComponentHookTree(() => this.#requestRender());
  }

  get state(): Readonly<A3sApplicationStateV1> {
    return Object.freeze({
      status: this.#status,
      dirty: this.#dirty,
      renderInFlight: this.#renderStarting || this.#inFlight !== null,
      committedRenders: this.#committedRenders,
      activeComponents: this.#hooks.activeComponentCount,
      lastError: this.#lastError,
      actions: this.#actions.state,
    });
  }

  async start(): Promise<void> {
    if (this.#status !== "created") {
      throw new Error(`cannot start an A3S application in ${this.#status} state`);
    }
    this.#status = "running";
    this.#dirty = true;
    await this.flush();
  }

  updateProps(props: Readonly<Props>): void {
    this.#assertMutable("update root props");
    this.#props = snapshotProps(props);
    if (this.#status === "running") {
      this.#requestRender();
    }
  }

  async rerender(): Promise<void> {
    this.#assertRunning("rerender");
    this.#dirty = true;
    await this.flush();
  }

  async flush(): Promise<boolean> {
    this.#assertRunning("flush renders");
    this.#scheduled = false;
    if (this.#inFlight !== null) {
      await this.#inFlight;
      return this.#dirty ? this.flush() : false;
    }
    if (this.#renderStarting) {
      throw new Error("cannot flush an A3S application reentrantly from its host");
    }
    if (!this.#dirty) {
      return false;
    }

    this.#dirty = false;
    this.#renderStarting = true;
    let work: Promise<boolean>;
    try {
      work = this.#renderOnce();
    } finally {
      this.#renderStarting = false;
    }
    this.#inFlight = work;
    try {
      return await work;
    } catch (error) {
      this.#lastError = error;
      throw error;
    } finally {
      if (this.#inFlight === work) {
        this.#inFlight = null;
      }
      if (this.#dirty) {
        this.#scheduleRender();
      }
    }
  }

  async dispatch(
    message: TsxEventMessageV1,
  ): Promise<Readonly<A3sActionDispatchResultV1>> {
    this.#assertRunning("dispatch events");
    this.#batchDepth += 1;
    let result: Readonly<A3sActionDispatchResultV1> | null = null;
    let dispatchFailed = false;
    let dispatchError: unknown;
    try {
      result = await this.#actions.dispatch(message);
    } catch (error) {
      dispatchFailed = true;
      dispatchError = error;
    } finally {
      this.#batchDepth -= 1;
    }
    let renderFailed = false;
    let renderError: unknown;
    if (this.#dirty) {
      try {
        await this.flush();
      } catch (error) {
        renderFailed = true;
        renderError = error;
      }
    }
    if (dispatchFailed) {
      this.#lastError = dispatchError;
      if (renderFailed) {
        const aggregate = new AggregateError(
          [dispatchError, renderError],
          "A3S event dispatch and its state render both failed",
        );
        this.#lastError = aggregate;
        throw aggregate;
      }
      throw dispatchError;
    }
    if (renderFailed) {
      this.#lastError = renderError;
      throw renderError;
    }
    if (result === null) {
      throw new Error("A3S event dispatch completed without a result");
    }
    return result;
  }

  async shutdown(): Promise<void> {
    if (this.#status === "closed") {
      return;
    }
    if (this.#actions.state.dispatching) {
      throw new Error("cannot shut down an A3S application during event dispatch");
    }

    this.#status = "closing";
    this.#dirty = false;
    this.#scheduled = false;
    const pending = this.#inFlight;
    if (pending !== null) {
      try {
        await pending;
      } catch (error) {
        this.#notifyError(error);
      }
    }

    for (const error of this.#hooks.dispose()) {
      this.#notifyError(error);
    }
    this.#actions.clear();
    try {
      await this.#host.close?.();
    } finally {
      this.#status = "closed";
    }
  }

  async #renderOnce(): Promise<boolean> {
    this.#hooks.beginCandidate();
    let compiled;
    try {
      const root = createA3sElement(
        this.#root,
        this.#props,
        null,
        { staticChildren: false },
      );
      compiled = compileFrameWithRuntimeV1(
        this.#frameId,
        root,
        this.#hooks,
        this.#compileOptions,
      );
    } catch (error) {
      this.#hooks.abortCandidate();
      throw error;
    }

    const renderRevision = (this.#actions.state.active?.renderRevision ?? 0) + 1;
    try {
      this.#actions.stage(renderRevision, compiled);
    } catch (error) {
      this.#hooks.abortCandidate();
      throw error;
    }
    const candidate = Object.freeze({
      renderRevision,
      frame: compiled.frame,
    });

    try {
      const committed = await this.#host.submitRender(candidate);
      this.#actions.commit(committed);
      this.#batchDepth += 1;
      let effectErrors;
      try {
        effectErrors = this.#hooks.commitCandidate();
      } finally {
        this.#batchDepth -= 1;
      }
      this.#committedRenders += 1;
      for (const error of effectErrors) {
        this.#notifyError(error);
      }
      return true;
    } catch (error) {
      if (this.#actions.state.pending?.renderRevision === renderRevision) {
        this.#actions.reject(renderRevision);
      }
      this.#hooks.abortCandidate();
      throw error;
    }
  }

  #requestRender(): void {
    if (this.#status !== "running") {
      return;
    }
    this.#dirty = true;
    if (
      this.#batchDepth === 0 &&
      !this.#renderStarting &&
      this.#inFlight === null
    ) {
      this.#scheduleRender();
    }
  }

  #scheduleRender(): void {
    if (this.#scheduled || this.#status !== "running" || !this.#dirty) {
      return;
    }
    this.#scheduled = true;
    queueMicrotask(() => {
      this.#scheduled = false;
      if (this.#status !== "running" || !this.#dirty || this.#batchDepth > 0) {
        return;
      }
      void this.flush().catch((error: unknown) => this.#notifyError(error));
    });
  }

  #notifyError(error: unknown): void {
    this.#lastError = error;
    if (this.#onError === null) {
      return;
    }
    try {
      this.#onError(error);
    } catch {
      // Error observers cannot mutate scheduler correctness.
    }
  }

  #assertMutable(operation: string): void {
    if (this.#status === "closing" || this.#status === "closed") {
      throw new Error(`cannot ${operation} in ${this.#status} state`);
    }
  }

  #assertRunning(operation: string): void {
    if (this.#status !== "running") {
      throw new Error(`cannot ${operation} in ${this.#status} state`);
    }
  }
}

export function createApp<Props extends A3sJsxProps>(
  root: A3sFunctionComponent<Props>,
  options: CreateAppOptions<Props>,
): A3sApplicationV1<Props> {
  return new A3sApplicationV1(root, options);
}

function snapshotProps<Props extends A3sJsxProps>(props: Readonly<Props>): Readonly<Props> {
  if (!isPlainRecord(props)) {
    throw new TypeError("A3S root props must be a plain object");
  }
  const descriptors = Object.getOwnPropertyDescriptors(props);
  const enumerableSymbols = Object.getOwnPropertySymbols(props).filter(
    (symbol) => Object.getOwnPropertyDescriptor(props, symbol)?.enumerable,
  );
  if (enumerableSymbols.length > 0) {
    throw new TypeError("A3S root props cannot contain enumerable symbol keys");
  }
  const snapshot: Record<string, unknown> = {};
  for (const [name, descriptor] of Object.entries(descriptors)) {
    if (!descriptor.enumerable) {
      continue;
    }
    if (!("value" in descriptor)) {
      throw new TypeError(`A3S root prop ${JSON.stringify(name)} cannot be an accessor`);
    }
    Object.defineProperty(snapshot, name, {
      configurable: false,
      enumerable: true,
      value: descriptor.value,
      writable: false,
    });
  }
  return Object.freeze(snapshot) as Readonly<Props>;
}

function isPlainRecord(value: unknown): value is Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}
