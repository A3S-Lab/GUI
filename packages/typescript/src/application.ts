import type {
  A3sActionDispatchResultV1,
  A3sActionRegistryStateV1,
  TsxCommittedMessageV1,
  TsxEventMessageV1,
} from "./action-registry.ts";
import { RevisionActionRegistryV1 } from "./action-registry.ts";
import {
  A3sClientSessionV1,
  type A3sClientSessionStateV1,
  type TsxRenderMessageV1,
  type TsxWelcomeMessageV1,
} from "./client-session.ts";
import {
  createA3sElement,
  type A3sFunctionComponent,
  type A3sJsxProps,
} from "./element.ts";
import {
  compileFrameWithRuntimeV1,
  type CompiledA3sFrameV1,
  type CompileFrameOptions,
} from "./frame.ts";
import { ComponentHookTree } from "./hook-tree.ts";
import { A3sApplicationRunnerV1 } from "./application-runner.ts";
import { prepareApplicationReplayV1 } from "./application-replay.ts";

export type A3sRenderCandidateV1 = TsxRenderMessageV1;

export interface A3sApplicationHostV1 {
  readonly welcome: TsxWelcomeMessageV1;
  /** When supplied, `welcome` must be this exact session's validated snapshot. */
  readonly session?: A3sClientSessionV1;
  submitRender(
    candidate: Readonly<A3sRenderCandidateV1>,
  ): TsxCommittedMessageV1 | Promise<TsxCommittedMessageV1>;
  close?(): void | Promise<void>;
}

export interface A3sApplicationHostTerminationV1 {
  readonly status: "closed" | "failed";
  readonly failure: unknown | null;
}

export interface A3sObservableApplicationHostV1 extends A3sApplicationHostV1 {
  readonly termination: Promise<Readonly<A3sApplicationHostTerminationV1>>;
}

export type A3sApplicationStatus =
  | "created"
  | "running"
  | "recovering"
  | "closing"
  | "closed";

export interface A3sApplicationStateV1 {
  readonly status: A3sApplicationStatus;
  readonly dirty: boolean;
  readonly renderInFlight: boolean;
  readonly committedRenders: number;
  readonly hostGeneration: number;
  readonly replayedRenders: number;
  readonly activeComponents: number;
  readonly lastError: unknown | null;
  readonly session: Readonly<A3sClientSessionStateV1>;
  readonly actions: Readonly<A3sActionRegistryStateV1>;
}

interface CreateAppBaseOptions {
  readonly frameId?: string;
  readonly compile?: CompileFrameOptions;
  readonly onError?: (error: unknown) => void;
}

type CreateAppPropsOptions<Props extends A3sJsxProps> =
  ({} extends Props
    ? { readonly props?: Readonly<Props> }
    : { readonly props: Readonly<Props> });

export type CreateAppOptions<Props extends A3sJsxProps> = CreateAppBaseOptions &
  CreateAppPropsOptions<Props> & {
    readonly host: A3sApplicationHostV1;
  };

export type CreateRunnableAppOptionsV1<Props extends A3sJsxProps> =
  CreateAppBaseOptions & CreateAppPropsOptions<Props>;

interface ValidatedApplicationOptionsV1<Props extends A3sJsxProps> {
  readonly host: A3sApplicationHostV1 | null;
  readonly frameId: string;
  readonly compile: Readonly<CompileFrameOptions>;
  readonly onError: ((error: unknown) => void) | null;
  readonly props: Readonly<Props>;
}

export class A3sApplicationV1<Props extends A3sJsxProps = A3sJsxProps> {
  readonly #root: A3sFunctionComponent<Props>;
  #host: A3sApplicationHostV1;
  readonly #frameId: string;
  readonly #compileOptions: CompileFrameOptions;
  readonly #onError: ((error: unknown) => void) | null;
  #actions = new RevisionActionRegistryV1();
  #session: A3sClientSessionV1;
  readonly #hooks: ComponentHookTree;
  #props: Readonly<Props>;
  #status: A3sApplicationStatus = "created";
  #dirty = false;
  #scheduled = false;
  #batchDepth = 0;
  #committedRenders = 0;
  #hostGeneration = 1;
  #replayedRenders = 0;
  #lastCommittedFrame: CompiledA3sFrameV1 | null = null;
  #lastError: unknown | null = null;
  #renderStarting = false;
  #inFlight: Promise<boolean> | null = null;
  #recoveryInFlight: Promise<void> | null = null;
  #hostMessageTail: Promise<void> = Promise.resolve();

  constructor(
    root: A3sFunctionComponent<Props>,
    options: CreateAppOptions<Props>,
  ) {
    const validated = validateApplicationOptions(root, options, true);
    const host = validated.host;
    if (host === null) {
      throw new TypeError("createApp requires a typed host with submitRender");
    }

    this.#root = root;
    this.#host = host;
    this.#frameId = validated.frameId;
    this.#props = validated.props;
    this.#compileOptions = validated.compile;
    this.#onError = validated.onError;
    this.#hooks = new ComponentHookTree(() => this.#requestRender());
    const sharedSession = host.session;
    this.#session = sharedSession ?? new A3sClientSessionV1(host.welcome);
    if (sharedSession !== undefined && sharedSession.welcome !== host.welcome) {
      throw new TypeError("createApp host session does not match its welcome message");
    }
  }

  get host(): A3sApplicationHostV1 {
    return this.#host;
  }

  get state(): Readonly<A3sApplicationStateV1> {
    return Object.freeze({
      status: this.#status,
      dirty: this.#dirty,
      renderInFlight: this.#renderStarting || this.#inFlight !== null,
      committedRenders: this.#committedRenders,
      hostGeneration: this.#hostGeneration,
      replayedRenders: this.#replayedRenders,
      activeComponents: this.#hooks.activeComponentCount,
      lastError: this.#lastError,
      session: this.#session.state,
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
    if (this.#status === "running" || this.#status === "recovering") {
      this.#requestRender();
    }
  }

  async rerender(): Promise<void> {
    this.#assertRunning("rerender");
    this.#dirty = true;
    await this.flush();
  }

  /**
   * Replaces a failed host and transactionally replays the last committed frame.
   *
   * Component instances and state remain owned by this application. The fresh
   * protocol session starts at render revision 1 with a fresh callback scope.
   */
  recover(host: A3sApplicationHostV1): Promise<void> {
    if (this.#status !== "running" && this.#status !== "recovering") {
      return Promise.reject(
        new Error(`cannot recover an A3S application in ${this.#status} state`),
      );
    }
    if (this.#recoveryInFlight !== null) {
      return Promise.reject(new Error("an A3S application recovery is already in flight"));
    }
    if (
      typeof host !== "object" ||
      host === null ||
      typeof host.submitRender !== "function"
    ) {
      return Promise.reject(
        new TypeError("application recovery requires a typed host with submitRender"),
      );
    }
    const recovery = this.#recover(host);
    this.#recoveryInFlight = recovery;
    void recovery.finally(() => {
      if (this.#recoveryInFlight === recovery) {
        this.#recoveryInFlight = null;
      }
    }).catch(() => {
      // The caller observes the original recovery promise.
    });
    return recovery;
  }

  /** Suspends rendering while an application-owned policy connects a new host. */
  beginRecovery(): void {
    if (this.#status === "recovering") {
      return;
    }
    if (this.#status !== "running") {
      throw new Error(`cannot begin recovery in ${this.#status} state`);
    }
    this.#status = "recovering";
  }

  /** Records a terminal host-policy failure and deterministically closes. */
  async abort(error: unknown): Promise<void> {
    if (this.#status === "closed") {
      this.#lastError = error;
      return;
    }
    this.#notifyError(error);
    await this.#hostMessageTail;
    try {
      await this.shutdown();
    } finally {
      this.#lastError = error;
    }
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
      result = await this.#enqueueHostMessage(() =>
        this.#session.dispatchEvent(message, this.#actions)
      );
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

    const recovery = this.#recoveryInFlight;
    if (recovery !== null) {
      try {
        await recovery;
      } catch {
        // Recovery already preserved the prior committed state.
      }
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
      this.#session.close();
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

    const renderRevision = this.#session.state.committedRenderRevision + 1;
    let candidate: Readonly<A3sRenderCandidateV1>;
    try {
      this.#actions.stage(renderRevision, compiled);
      candidate = this.#session.createRender(renderRevision, compiled.frame);
    } catch (error) {
      if (this.#actions.state.pending?.renderRevision === renderRevision) {
        this.#actions.reject(renderRevision);
      }
      this.#hooks.abortCandidate();
      throw error;
    }

    try {
      const committed = await this.#host.submitRender(candidate);
      const effectErrors = await this.#enqueueHostMessage(() => {
        this.#session.commitRender(committed, this.#actions);
        this.#batchDepth += 1;
        try {
          return this.#hooks.commitCandidate();
        } finally {
          this.#batchDepth -= 1;
        }
      });
      this.#lastCommittedFrame = compiled;
      this.#committedRenders += 1;
      for (const error of effectErrors) {
        this.#notifyError(error);
      }
      return true;
    } catch (error) {
      try {
        await this.#enqueueHostMessage(() => {
          if (this.#actions.state.pending?.renderRevision === renderRevision) {
            this.#actions.reject(renderRevision);
          }
          if (this.#session.state.pendingRenderRevision === renderRevision) {
            this.#session.rejectRender(renderRevision);
          }
          this.#hooks.abortCandidate();
        });
      } catch (rollbackError) {
        throw new AggregateError(
          [error, rollbackError],
          `render revision ${renderRevision} failed and could not be rolled back`,
        );
      }
      throw error;
    }
  }

  async #recover(host: A3sApplicationHostV1): Promise<void> {
    this.beginRecovery();
    const pending = this.#inFlight;
    if (pending !== null) {
      try {
        await pending;
      } catch {
        // The render path rolls its candidate back before recovery continues.
      }
    }
    await this.#hostMessageTail;
    const retained = this.#lastCommittedFrame;
    if (retained === null) {
      throw new Error("cannot recover before the application has committed its first frame");
    }

    let prepared;
    try {
      prepared = await prepareApplicationReplayV1(
        host,
        retained,
        this.#session.state.sessionId,
      );
    } catch (cause) {
      try {
        await host.close?.();
      } catch {
        // Preserve the replay failure.
      }
      this.#lastError = cause;
      throw cause;
    }

    const previousHost = this.#host;
    const previousSession = this.#session;
    this.#host = host;
    this.#session = prepared.session;
    this.#actions = prepared.actions;
    this.#hostGeneration += 1;
    this.#replayedRenders += 1;
    try {
      await previousHost.close?.();
    } catch (error) {
      this.#notifyError(error);
    } finally {
      previousSession.close();
    }
    this.#status = "running";
    if (this.#dirty) {
      this.#scheduleRender();
    }
  }

  #enqueueHostMessage<Result>(
    operation: () => Result | PromiseLike<Result>,
  ): Promise<Result> {
    const result = this.#hostMessageTail.then(operation);
    this.#hostMessageTail = result.then(
      () => undefined,
      () => undefined,
    );
    return result;
  }

  #requestRender(): void {
    if (this.#status !== "running" && this.#status !== "recovering") {
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
): A3sApplicationV1<Props>;
export function createApp<Props extends A3sJsxProps>(
  root: A3sFunctionComponent<Props>,
  ...options: {} extends Props
    ? [options?: CreateRunnableAppOptionsV1<Props>]
    : [options: CreateRunnableAppOptionsV1<Props>]
): A3sApplicationRunnerV1<Props>;
export function createApp<Props extends A3sJsxProps>(
  root: A3sFunctionComponent<Props>,
  options?: CreateAppOptions<Props> | CreateRunnableAppOptionsV1<Props>,
): A3sApplicationV1<Props> | A3sApplicationRunnerV1<Props> {
  if (isPlainRecord(options) && Object.hasOwn(options, "host")) {
    return new A3sApplicationV1(root, options as CreateAppOptions<Props>);
  }
  const validated = validateApplicationOptions(root, options ?? {}, false);
  const definition = {
    frameId: validated.frameId,
    compile: validated.compile,
    props: validated.props,
    ...(validated.onError === null ? {} : { onError: validated.onError }),
  };
  return new A3sApplicationRunnerV1((host) =>
    new A3sApplicationV1(root, {
      ...definition,
      host,
    } as CreateAppOptions<Props>)
  );
}

function validateApplicationOptions<Props extends A3sJsxProps>(
  root: A3sFunctionComponent<Props>,
  options: unknown,
  requireHost: boolean,
): ValidatedApplicationOptionsV1<Props> {
  if (typeof root !== "function") {
    throw new TypeError("createApp requires a synchronous function component");
  }
  const values = plainApplicationOptionValues(options, requireHost);
  const host = values.host ?? null;
  if (
    requireHost &&
    (
      typeof host !== "object" ||
      host === null ||
      typeof (host as A3sApplicationHostV1).submitRender !== "function"
    )
  ) {
    throw new TypeError("createApp requires a typed host with submitRender");
  }
  const frameId = values.frameId ?? "app";
  if (typeof frameId !== "string" || frameId.length === 0) {
    throw new TypeError("createApp frameId must be a non-empty string");
  }
  const onError = values.onError === undefined ? null : values.onError;
  if (onError !== null && typeof onError !== "function") {
    throw new TypeError("createApp onError must be a function");
  }
  const initialProps = values.props === undefined ? {} as Props : values.props as Props;
  return {
    host: host as A3sApplicationHostV1 | null,
    frameId,
    compile: snapshotCompileOptions(values.compile),
    onError: onError as ((error: unknown) => void) | null,
    props: snapshotProps<Props>(initialProps),
  };
}

function plainApplicationOptionValues(
  options: unknown,
  requireHost: boolean,
): Record<string, unknown> {
  if (!isPlainRecord(options)) {
    throw new TypeError("createApp options must be a plain object");
  }
  const allowed = new Set(["frameId", "compile", "onError", "props"]);
  if (requireHost) {
    allowed.add("host");
  }
  const descriptors = Object.getOwnPropertyDescriptors(options);
  const values: Record<string, unknown> = {};
  for (const key of Reflect.ownKeys(descriptors)) {
    if (typeof key !== "string" || !allowed.has(key)) {
      throw new TypeError(`createApp options contain unknown field ${String(key)}`);
    }
    const descriptor = descriptors[key];
    if (descriptor === undefined || !("value" in descriptor)) {
      throw new TypeError(`createApp option ${key} cannot be an accessor`);
    }
    values[key] = descriptor.value;
  }
  if (requireHost && !Object.hasOwn(values, "host")) {
    throw new TypeError("createApp requires a typed host with submitRender");
  }
  return values;
}

function snapshotCompileOptions(value: unknown): Readonly<CompileFrameOptions> {
  if (value === undefined) {
    return Object.freeze({});
  }
  if (!isPlainRecord(value)) {
    throw new TypeError("createApp compile options must be a plain object");
  }
  const descriptors = Object.getOwnPropertyDescriptors(value);
  const snapshot: Record<string, unknown> = {};
  for (const key of Reflect.ownKeys(descriptors)) {
    if (key !== "maximumDepth" && key !== "maximumNodes") {
      throw new TypeError(`createApp compile options contain unknown field ${String(key)}`);
    }
    const descriptor = descriptors[key];
    if (descriptor === undefined || !("value" in descriptor)) {
      throw new TypeError(`createApp compile option ${String(key)} cannot be an accessor`);
    }
    snapshot[key] = descriptor.value;
  }
  return Object.freeze(snapshot) as Readonly<CompileFrameOptions>;
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
