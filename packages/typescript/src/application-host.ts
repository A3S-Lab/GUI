import type {
  TsxCommittedMessageV1,
  TsxEventMessageV1,
} from "./action-registry.ts";
import type {
  A3sApplicationHostV1,
  A3sRenderCandidateV1,
} from "./application.ts";
import type { A3sClientHandshakeOptionsV1 } from "./client-handshake.ts";
import type { A3sClientSessionV1 } from "./client-session.ts";
import {
  spawnA3sNodeProcessTransportV1,
  type SpawnA3sNodeProcessOptionsV1,
} from "./node-process-transport.ts";
import {
  A3sFramedClientConnectionV1,
  connectA3sFramedClientV1,
} from "./transport.ts";
import type { TsxHostMessageV1 } from "./generated/protocol.ts";

const MAXIMUM_PENDING_EVENT_TASKS = 1_024;

export type A3sFramedHostStatusV1 = "open" | "closing" | "closed" | "failed";

export type A3sFramedHostErrorCodeV1 =
  | "endOfStream"
  | "eventHandlerFailed"
  | "eventHandlerMissing"
  | "hostClosed"
  | "hostFatal"
  | "invalidConnection"
  | "invalidHostMessage"
  | "invalidOptions"
  | "invalidState"
  | "renderInFlight"
  | "streamFailed";

export class A3sFramedHostError extends Error {
  readonly code: A3sFramedHostErrorCodeV1;

  constructor(code: A3sFramedHostErrorCodeV1, message: string, cause?: unknown) {
    super(message, cause === undefined ? undefined : { cause });
    this.name = "A3sFramedHostError";
    this.code = code;
  }
}

export interface A3sFramedApplicationHostStateV1 {
  readonly status: A3sFramedHostStatusV1;
  readonly pendingRenderRevision: number | null;
  readonly pendingEventTasks: number;
  readonly failure: A3sFramedHostError | null;
}

export type A3sHostEventHandlerV1 = (
  message: Readonly<TsxEventMessageV1>,
) => void | PromiseLike<void>;

export interface A3sFramedApplicationHostOptionsV1 {
  readonly onEvent?: A3sHostEventHandlerV1;
}

export interface ConnectA3sNodeApplicationHostOptionsV1
  extends A3sFramedApplicationHostOptionsV1 {
  readonly process: SpawnA3sNodeProcessOptionsV1;
  readonly handshake: A3sClientHandshakeOptionsV1;
}

interface PendingRenderV1 {
  readonly renderRevision: number;
  readonly resolve: (message: TsxCommittedMessageV1) => void;
  readonly reject: (error: A3sFramedHostError) => void;
}

/** Ordered application-message pump over one negotiated framed connection. */
export class A3sFramedApplicationHostV1 implements A3sApplicationHostV1 {
  readonly #connection: A3sFramedClientConnectionV1;
  readonly #readerTask: Promise<void>;
  readonly #eventTasks = new Set<Promise<void>>();
  #status: A3sFramedHostStatusV1 = "open";
  #failure: A3sFramedHostError | null = null;
  #pending: PendingRenderV1 | null = null;
  #eventHandler: A3sHostEventHandlerV1 | null;
  #closePromise: Promise<void> | null = null;

  constructor(
    connection: A3sFramedClientConnectionV1,
    options: A3sFramedApplicationHostOptionsV1 = {},
  ) {
    validateConnection(connection);
    const onEvent = validateHostOptions(options);
    this.#connection = connection;
    this.#eventHandler = onEvent;
    this.#readerTask = this.#readMessages();
  }

  get welcome() {
    return this.#connection.welcome;
  }

  get session(): A3sClientSessionV1 {
    return this.#connection.session;
  }

  get state(): Readonly<A3sFramedApplicationHostStateV1> {
    return Object.freeze({
      status: this.#status,
      pendingRenderRevision: this.#pending?.renderRevision ?? null,
      pendingEventTasks: this.#eventTasks.size,
      failure: this.#failure,
    });
  }

  setEventHandler(handler: A3sHostEventHandlerV1): void {
    this.#assertOpen("set an event handler");
    if (typeof handler !== "function") {
      throw hostError("invalidState", "framed host event handler must be a function");
    }
    if (this.#eventHandler !== null) {
      throw hostError("invalidState", "framed host event handler is already set");
    }
    this.#eventHandler = handler;
  }

  async submitRender(
    candidate: Readonly<A3sRenderCandidateV1>,
  ): Promise<TsxCommittedMessageV1> {
    this.#assertOpen("submit a render");
    if (this.#pending !== null) {
      throw hostError(
        "renderInFlight",
        `cannot submit render revision ${candidate.renderRevision}; revision ${this.#pending.renderRevision} is already in flight`,
      );
    }
    if (candidate.type !== "render") {
      throw hostError("invalidState", "framed application host accepts only render candidates");
    }

    let resolve!: (message: TsxCommittedMessageV1) => void;
    let reject!: (error: A3sFramedHostError) => void;
    const response = new Promise<TsxCommittedMessageV1>((resolvePromise, rejectPromise) => {
      resolve = resolvePromise;
      reject = rejectPromise;
    });
    this.#pending = Object.freeze({
      renderRevision: candidate.renderRevision,
      resolve,
      reject,
    });

    try {
      await this.#connection.writeClientMessage(candidate);
    } catch (cause) {
      this.#fail(hostError("streamFailed", "could not write a render to the TSX host", cause));
    }
    return response;
  }

  close(): Promise<void> {
    if (this.#closePromise !== null) {
      return this.#closePromise;
    }
    this.#closePromise = this.#close();
    return this.#closePromise;
  }

  async #close(): Promise<void> {
    if (this.#status === "closed") {
      return;
    }
    if (this.#status === "open") {
      this.#status = "closing";
    }
    if (this.#pending !== null) {
      const error = hostError("invalidState", "framed host closed with a render in flight");
      const pending = this.#pending;
      this.#pending = null;
      pending.reject(error);
    }
    try {
      await this.#connection.close();
      await this.#readerTask;
      await Promise.all(this.#eventTasks);
      this.#status = "closed";
    } catch (cause) {
      const error = hostError("streamFailed", "could not close the framed TSX host", cause);
      this.#fail(error);
      throw error;
    }
  }

  async #readMessages(): Promise<void> {
    try {
      while (true) {
        const message = await this.#connection.readHostMessage();
        if (message === null) {
          if (this.#status === "closing" || this.#status === "closed") {
            return;
          }
          throw hostError("endOfStream", "TSX host stream ended without a close request");
        }
        this.#acceptHostMessage(message);
      }
    } catch (cause) {
      if (this.#status === "closing" || this.#status === "closed") {
        return;
      }
      this.#fail(
        cause instanceof A3sFramedHostError
          ? cause
          : hostError("streamFailed", "TSX host message pump failed", cause),
      );
    }
  }

  #acceptHostMessage(message: Readonly<TsxHostMessageV1>): void {
    switch (message.type) {
      case "committed": {
        const pending = this.#pending;
        if (pending === null) {
          throw hostError("invalidHostMessage", "TSX host committed without a pending render");
        }
        if (message.renderRevision !== pending.renderRevision) {
          throw hostError(
            "invalidHostMessage",
            `TSX host committed render revision ${message.renderRevision}; revision ${pending.renderRevision} is pending`,
          );
        }
        this.#pending = null;
        pending.resolve(message);
        return;
      }
      case "event":
        this.#dispatchEvent(message);
        return;
      case "fatal":
        throw hostError(
          "hostFatal",
          `TSX host reported ${JSON.stringify(message.payload.code)}: ${message.payload.message}`,
        );
      case "close":
        throw hostError(
          "hostClosed",
          `TSX host requested close with reason ${JSON.stringify(message.payload.reason)}`,
        );
      case "ping":
      case "pong":
        throw hostError(
          "invalidHostMessage",
          `TSX application pump does not yet support host ${message.type}`,
        );
      case "welcome":
        throw hostError("invalidHostMessage", "TSX host emitted a second welcome");
    }
  }

  #dispatchEvent(message: Readonly<TsxEventMessageV1>): void {
    const handler = this.#eventHandler;
    if (handler === null) {
      throw hostError("eventHandlerMissing", "TSX host emitted an event before a handler was set");
    }
    if (this.#eventTasks.size >= MAXIMUM_PENDING_EVENT_TASKS) {
      throw hostError(
        "invalidHostMessage",
        `TSX host exceeded the ${MAXIMUM_PENDING_EVENT_TASKS}-event task limit`,
      );
    }

    let result: void | PromiseLike<void>;
    try {
      result = handler(message);
    } catch (cause) {
      throw hostError("eventHandlerFailed", "TSX application event handler failed", cause);
    }
    let task!: Promise<void>;
    task = Promise.resolve(result).then(
      () => {
        this.#eventTasks.delete(task);
      },
      (cause) => {
        this.#eventTasks.delete(task);
        this.#fail(hostError("eventHandlerFailed", "TSX application event handler failed", cause));
      },
    );
    this.#eventTasks.add(task);
  }

  #fail(error: A3sFramedHostError): void {
    if (this.#status === "failed" || this.#status === "closed") {
      return;
    }
    this.#status = "failed";
    this.#failure = error;
    const pending = this.#pending;
    this.#pending = null;
    pending?.reject(error);
    void this.#connection.close().catch(() => {
      // Preserve the protocol/application failure as the primary error.
    });
  }

  #assertOpen(operation: string): void {
    if (this.#status !== "open") {
      throw this.#failure ?? hostError(
        "invalidState",
        `cannot ${operation} while the framed host is ${this.#status}`,
      );
    }
  }
}

export async function connectA3sNodeApplicationHostV1(
  options: ConnectA3sNodeApplicationHostOptionsV1,
): Promise<A3sFramedApplicationHostV1> {
  const validated = validateConnectOptions(options);
  const transport = spawnA3sNodeProcessTransportV1(validated.process);
  try {
    const connection = await connectA3sFramedClientV1(transport, validated.handshake);
    const hostOptions = validated.onEvent === null ? {} : { onEvent: validated.onEvent };
    return new A3sFramedApplicationHostV1(connection, hostOptions);
  } catch (cause) {
    try {
      await transport.close();
    } catch {
      // Preserve the validation, spawn, or negotiation failure.
    }
    throw cause;
  }
}

function validateConnection(connection: A3sFramedClientConnectionV1): void {
  if (!(connection instanceof A3sFramedClientConnectionV1)) {
    throw hostError("invalidConnection", "framed application host requires a negotiated connection");
  }
}

function validateHostOptions(options: A3sFramedApplicationHostOptionsV1): A3sHostEventHandlerV1 | null {
  const values = plainOptionValues(options, "framed application host options", ["onEvent"], []);
  const onEvent = values.onEvent;
  if (onEvent === undefined) {
    return null;
  }
  if (typeof onEvent !== "function") {
    throw hostError("invalidOptions", "framed host onEvent must be a function");
  }
  return onEvent as A3sHostEventHandlerV1;
}

function validateConnectOptions(
  options: ConnectA3sNodeApplicationHostOptionsV1,
): {
  process: SpawnA3sNodeProcessOptionsV1;
  handshake: A3sClientHandshakeOptionsV1;
  onEvent: A3sHostEventHandlerV1 | null;
} {
  const values = plainOptionValues(
    options,
    "Node application host options",
    ["process", "handshake", "onEvent"],
    ["process", "handshake"],
  );
  const onEvent = values.onEvent;
  if (onEvent !== undefined && typeof onEvent !== "function") {
    throw hostError("invalidOptions", "Node application host onEvent must be a function");
  }
  return {
    process: values.process as SpawnA3sNodeProcessOptionsV1,
    handshake: values.handshake as A3sClientHandshakeOptionsV1,
    onEvent: (onEvent ?? null) as A3sHostEventHandlerV1 | null,
  };
}

function plainOptionValues(
  value: unknown,
  name: string,
  allowed: readonly string[],
  required: readonly string[],
): Record<string, unknown> {
  if (
    typeof value !== "object" ||
    value === null ||
    (Object.getPrototypeOf(value) !== Object.prototype && Object.getPrototypeOf(value) !== null)
  ) {
    throw hostError("invalidOptions", `${name} must be a plain object`);
  }
  const allowedKeys = new Set(allowed);
  const descriptors = Object.getOwnPropertyDescriptors(value);
  const output: Record<string, unknown> = {};
  for (const key of Reflect.ownKeys(descriptors)) {
    if (typeof key !== "string" || !allowedKeys.has(key)) {
      throw hostError("invalidOptions", `${name} contains unknown field ${JSON.stringify(key)}`);
    }
    const descriptor = descriptors[key];
    if (descriptor === undefined || !("value" in descriptor)) {
      throw hostError("invalidOptions", `${name}.${key} cannot be an accessor`);
    }
    output[key] = descriptor.value;
  }
  for (const key of required) {
    if (!(key in output)) {
      throw hostError("invalidOptions", `${name} is missing field ${JSON.stringify(key)}`);
    }
  }
  return output;
}

function hostError(
  code: A3sFramedHostErrorCodeV1,
  message: string,
  cause?: unknown,
): A3sFramedHostError {
  return new A3sFramedHostError(code, message, cause);
}
