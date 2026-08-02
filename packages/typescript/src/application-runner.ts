import { randomUUID } from "node:crypto";
import { createRequire } from "node:module";

import {
  connectA3sNodeApplicationHostV1,
  type A3sHostEventHandlerV1,
} from "./application-host.ts";
import type {
  A3sApplicationHostV1,
  A3sApplicationHostTerminationV1,
  A3sApplicationV1,
  A3sObservableApplicationHostV1,
} from "./application.ts";
import { resolveA3sHostArtifactV1 } from "./host-artifact.ts";
import type { A3sJsxProps } from "./element.ts";

const require = createRequire(import.meta.url);
const A3S_TYPESCRIPT_SDK_VERSION = packageVersion();
const MAXIMUM_RESTARTS = 16;
const MAXIMUM_RESTART_DELAY_MS = 60_000;
const MAXIMUM_GATED_EVENTS = 1_024;

export type A3sApplicationRecoveryErrorCodeV1 =
  | "hostNotObservable"
  | "restartsExhausted";

export class A3sApplicationRecoveryError extends Error {
  readonly code: A3sApplicationRecoveryErrorCodeV1;
  readonly restartAttempts: number;

  constructor(
    code: A3sApplicationRecoveryErrorCodeV1,
    message: string,
    restartAttempts: number,
    cause?: unknown,
  ) {
    super(message, cause === undefined ? undefined : { cause });
    this.name = "A3sApplicationRecoveryError";
    this.code = code;
    this.restartAttempts = restartAttempts;
  }
}

/** Supplies one negotiated native host without owning application state. */
export interface A3sApplicationRuntimeV1 {
  connect(onEvent: A3sHostEventHandlerV1): Promise<A3sApplicationHostV1>;
}

export interface A3sApplicationRunOptionsV1 {
  readonly runtime?: A3sApplicationRuntimeV1;
  readonly recovery?: A3sApplicationRecoveryOptionsV1;
}

/** Explicit bounded restart policy. Omit it to keep recovery application-owned. */
export interface A3sApplicationRecoveryOptionsV1 {
  readonly maximumRestarts: number;
  readonly restartDelayMs?: number;
}

type ApplicationFactoryV1<Props extends A3sJsxProps> = (
  host: A3sApplicationHostV1,
) => A3sApplicationV1<Props>;

interface ValidatedRecoveryPolicyV1 {
  readonly maximumRestarts: number;
  readonly restartDelayMs: number;
}

interface ValidatedRunOptionsV1 {
  readonly runtime: A3sApplicationRuntimeV1;
  readonly recovery: ValidatedRecoveryPolicyV1 | null;
}

/** Default Node runtime that launches the validated platform host artifact. */
export class A3sNodeApplicationRuntimeV1 implements A3sApplicationRuntimeV1 {
  async connect(onEvent: A3sHostEventHandlerV1): Promise<A3sApplicationHostV1> {
    if (typeof onEvent !== "function") {
      throw new TypeError("A3S application runtime requires an event handler");
    }
    const artifact = await resolveA3sHostArtifactV1();
    return connectA3sNodeApplicationHostV1({
      process: { command: artifact.command },
      handshake: {
        sdkVersion: A3S_TYPESCRIPT_SDK_VERSION,
        sessionId: randomUUID(),
        requestedRenderer: "auto",
      },
      onEvent,
    });
  }
}

/** One-shot hostless application definition returned by `createApp`. */
export class A3sApplicationRunnerV1<Props extends A3sJsxProps = A3sJsxProps> {
  readonly #factory: ApplicationFactoryV1<Props>;
  #status: "created" | "starting" | "started" | "failed" = "created";

  constructor(factory: ApplicationFactoryV1<Props>) {
    if (typeof factory !== "function") {
      throw new TypeError("A3S application runner requires an application factory");
    }
    this.#factory = factory;
  }

  async run(
    options: A3sApplicationRunOptionsV1 = {},
  ): Promise<A3sApplicationV1<Props>> {
    if (this.#status !== "created") {
      throw new Error(`cannot run an A3S application runner in ${this.#status} state`);
    }
    const validated = validateRunOptions(options);
    const runtime = validated.runtime;
    this.#status = "starting";

    let application: A3sApplicationV1<Props> | null = null;
    let host: A3sApplicationHostV1 | null = null;
    const eventGate = new ApplicationEventGateV1();
    try {
      host = await runtime.connect((message) => eventGate.dispatch(message));
      validateConnectedHost(host);
      if (validated.recovery !== null) {
        requireObservableHost(host);
      }
      application = this.#factory(host);
      await application.start();
      eventGate.bind((message) => application!.dispatch(message).then(() => undefined));
      if (validated.recovery !== null) {
        void superviseApplicationV1(
          application,
          runtime,
          requireObservableHost(host),
          eventGate,
          validated.recovery,
        );
      }
      this.#status = "started";
      return application;
    } catch (cause) {
      this.#status = "failed";
      eventGate.close(cause);
      try {
        if (application !== null) {
          await application.shutdown();
        } else {
          await host?.close?.();
        }
      } catch (cleanupError) {
        const aggregate = new AggregateError(
          [cause, cleanupError],
          "A3S application startup failed and its native host could not close cleanly",
        );
        throw aggregate;
      }
      throw cause;
    }
  }
}

function packageVersion(): string {
  const metadata = require("../package.json") as unknown;
  if (typeof metadata !== "object" || metadata === null) {
    throw new TypeError("@a3s/gui package metadata must be an object");
  }
  const version = (metadata as Record<string, unknown>).version;
  if (
    typeof version !== "string" ||
    version.trim().length === 0 ||
    version.length > 1_024 ||
    /[\u0000-\u001f\u007f-\u009f]/u.test(version)
  ) {
    throw new TypeError("@a3s/gui package version is invalid");
  }
  return version;
}

function validateRunOptions(options: A3sApplicationRunOptionsV1): ValidatedRunOptionsV1 {
  if (!isPlainRecord(options)) {
    throw new TypeError("A3S application run options must be a plain object");
  }
  const descriptors = Object.getOwnPropertyDescriptors(options);
  for (const key of Reflect.ownKeys(descriptors)) {
    if (key !== "runtime" && key !== "recovery") {
      throw new TypeError(`A3S application run options contain unknown field ${String(key)}`);
    }
    const descriptor = descriptors[key];
    if (descriptor === undefined || !("value" in descriptor)) {
      throw new TypeError(`A3S application run option ${String(key)} cannot be an accessor`);
    }
  }
  const runtime = descriptors.runtime === undefined
    ? new A3sNodeApplicationRuntimeV1()
    : descriptors.runtime.value;
  if (
    typeof runtime !== "object" ||
    runtime === null ||
    typeof (runtime as A3sApplicationRuntimeV1).connect !== "function"
  ) {
    throw new TypeError("A3S application run runtime must implement connect");
  }
  return Object.freeze({
    runtime: runtime as A3sApplicationRuntimeV1,
    recovery: validateRecoveryPolicy(descriptors.recovery?.value),
  });
}

function validateRecoveryPolicy(value: unknown): ValidatedRecoveryPolicyV1 | null {
  if (value === undefined) {
    return null;
  }
  if (!isPlainRecord(value)) {
    throw new TypeError("A3S application recovery policy must be a plain object");
  }
  const descriptors = Object.getOwnPropertyDescriptors(value);
  for (const key of Reflect.ownKeys(descriptors)) {
    if (key !== "maximumRestarts" && key !== "restartDelayMs") {
      throw new TypeError(
        `A3S application recovery policy contains unknown field ${String(key)}`,
      );
    }
    const descriptor = descriptors[key];
    if (descriptor === undefined || !("value" in descriptor)) {
      throw new TypeError(
        `A3S application recovery option ${String(key)} cannot be an accessor`,
      );
    }
  }
  const maximumRestarts = descriptors.maximumRestarts?.value;
  if (
    typeof maximumRestarts !== "number" ||
    !Number.isSafeInteger(maximumRestarts) ||
    maximumRestarts < 1 ||
    maximumRestarts > MAXIMUM_RESTARTS
  ) {
    throw new TypeError(
      `A3S application maximumRestarts must be an integer from 1 through ${MAXIMUM_RESTARTS}`,
    );
  }
  const restartDelayMs = descriptors.restartDelayMs?.value ?? 0;
  if (
    typeof restartDelayMs !== "number" ||
    !Number.isSafeInteger(restartDelayMs) ||
    restartDelayMs < 0 ||
    restartDelayMs > MAXIMUM_RESTART_DELAY_MS
  ) {
    throw new TypeError(
      `A3S application restartDelayMs must be an integer from 0 through ${MAXIMUM_RESTART_DELAY_MS}`,
    );
  }
  return Object.freeze({ maximumRestarts, restartDelayMs });
}

function validateConnectedHost(host: A3sApplicationHostV1): void {
  if (
    typeof host !== "object" ||
    host === null ||
    typeof host.submitRender !== "function"
  ) {
    throw new TypeError("A3S application runtime returned an invalid host");
  }
}

function requireObservableHost(
  host: A3sApplicationHostV1,
): A3sObservableApplicationHostV1 {
  const termination = (host as Partial<A3sObservableApplicationHostV1>).termination;
  if (!(termination instanceof Promise)) {
    throw new A3sApplicationRecoveryError(
      "hostNotObservable",
      "A3S application recovery requires a host termination promise",
      0,
    );
  }
  return host as A3sObservableApplicationHostV1;
}

async function superviseApplicationV1<Props extends A3sJsxProps>(
  application: A3sApplicationV1<Props>,
  runtime: A3sApplicationRuntimeV1,
  initialHost: A3sObservableApplicationHostV1,
  initialGate: ApplicationEventGateV1,
  policy: ValidatedRecoveryPolicyV1,
): Promise<void> {
  let host = initialHost;
  let gate = initialGate;
  let restartAttempts = 0;
  const failures: unknown[] = [];

  while (!isApplicationStopping(application)) {
    let termination: Readonly<A3sApplicationHostTerminationV1>;
    try {
      termination = validateHostTermination(await host.termination);
    } catch (cause) {
      termination = Object.freeze({ status: "failed", failure: cause });
    }
    if (isApplicationStopping(application)) {
      return;
    }
    const hostFailure = termination.failure ?? new Error(
      "native TSX host closed without an application shutdown request",
    );
    failures.push(hostFailure);
    gate.close(hostFailure);
    application.beginRecovery();

    let recovered = false;
    while (restartAttempts < policy.maximumRestarts) {
      restartAttempts += 1;
      if (policy.restartDelayMs > 0) {
        await delay(policy.restartDelayMs);
      }
      if (isApplicationStopping(application)) {
        return;
      }

      const nextGate = new ApplicationEventGateV1();
      let nextHost: A3sApplicationHostV1 | null = null;
      try {
        nextHost = await runtime.connect((message) => nextGate.dispatch(message));
        validateConnectedHost(nextHost);
        const observable = requireObservableHost(nextHost);
        await application.recover(nextHost);
        nextGate.bind((message) => application.dispatch(message).then(() => undefined));
        host = observable;
        gate = nextGate;
        recovered = true;
        break;
      } catch (cause) {
        failures.push(cause);
        nextGate.close(cause);
        try {
          await nextHost?.close?.();
        } catch {
          // Preserve the connection or replay failure.
        }
      }
    }
    if (recovered) {
      continue;
    }

    const cause = new AggregateError(
      failures,
      `native TSX host recovery exhausted ${restartAttempts} restart attempt(s)`,
    );
    const error = new A3sApplicationRecoveryError(
      "restartsExhausted",
      `native TSX host recovery exhausted ${restartAttempts} restart attempt(s)`,
      restartAttempts,
      cause,
    );
    try {
      await application.abort(error);
    } catch {
      // `abort` leaves the application closed even when host cleanup fails.
    }
    return;
  }
}

class ApplicationEventGateV1 {
  readonly #pending: Array<{
    readonly message: Parameters<A3sHostEventHandlerV1>[0];
    readonly resolve: () => void;
    readonly reject: (cause: unknown) => void;
  }> = [];
  #handler: A3sHostEventHandlerV1 | null = null;
  #closed: unknown | null = null;
  #tail: Promise<void> = Promise.resolve();

  dispatch(message: Parameters<A3sHostEventHandlerV1>[0]): Promise<void> {
    if (this.#closed !== null) {
      return Promise.reject(this.#closed);
    }
    if (this.#handler !== null) {
      return this.#enqueue(message, this.#handler);
    }
    if (this.#pending.length >= MAXIMUM_GATED_EVENTS) {
      return Promise.reject(
        new Error(`native TSX host exceeded ${MAXIMUM_GATED_EVENTS} gated events`),
      );
    }
    return new Promise<void>((resolve, reject) => {
      this.#pending.push({ message, resolve, reject });
    });
  }

  bind(handler: A3sHostEventHandlerV1): void {
    if (this.#handler !== null || this.#closed !== null || typeof handler !== "function") {
      throw new Error("A3S host event gate cannot be rebound");
    }
    this.#handler = handler;
    for (const pending of this.#pending.splice(0)) {
      void this.#enqueue(pending.message, handler).then(pending.resolve, pending.reject);
    }
  }

  close(cause: unknown): void {
    if (this.#closed !== null) {
      return;
    }
    this.#closed = cause ?? new Error("native TSX host event gate closed");
    this.#handler = null;
    for (const pending of this.#pending.splice(0)) {
      pending.reject(this.#closed);
    }
  }

  #enqueue(
    message: Parameters<A3sHostEventHandlerV1>[0],
    handler: A3sHostEventHandlerV1,
  ): Promise<void> {
    const task = this.#tail.then(() => handler(message)).then(() => undefined);
    this.#tail = task.catch(() => undefined);
    return task;
  }
}

function delay(milliseconds: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

function validateHostTermination(
  value: unknown,
): Readonly<A3sApplicationHostTerminationV1> {
  if (!isPlainRecord(value)) {
    throw new TypeError("A3S Host termination must be a plain object");
  }
  const descriptors = Object.getOwnPropertyDescriptors(value);
  const keys = Reflect.ownKeys(descriptors);
  if (
    keys.length !== 2 ||
    !Object.hasOwn(descriptors, "status") ||
    !Object.hasOwn(descriptors, "failure")
  ) {
    throw new TypeError("A3S Host termination must contain exact status and failure fields");
  }
  for (const key of keys) {
    if (key !== "status" && key !== "failure") {
      throw new TypeError(`A3S Host termination contains unknown field ${String(key)}`);
    }
    const descriptor = descriptors[key];
    if (descriptor === undefined || !("value" in descriptor)) {
      throw new TypeError(`A3S Host termination field ${String(key)} cannot be an accessor`);
    }
  }
  const status = descriptors.status!.value;
  const failure = descriptors.failure!.value;
  if (status !== "closed" && status !== "failed") {
    throw new TypeError("A3S Host termination status must be closed or failed");
  }
  if (
    (status === "closed" && failure !== null) ||
    (status === "failed" && failure === null)
  ) {
    throw new TypeError("A3S Host termination failure does not match its status");
  }
  return Object.freeze({ status, failure });
}

function isApplicationStopping<Props extends A3sJsxProps>(
  application: A3sApplicationV1<Props>,
): boolean {
  const status = application.state.status;
  return status === "closing" || status === "closed";
}

function isPlainRecord(value: unknown): value is Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}
