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
import type { TsxPostWelcomeHostMessageV1 } from "./client-host-sequence.ts";
import {
  spawnA3sNodeProcessTransportV1,
  type SpawnA3sNodeProcessOptionsV1,
} from "./node-process-transport.ts";
import {
  A3sFramedClientConnectionV1,
  connectA3sFramedClientV1,
} from "./transport.ts";

const MAXIMUM_PENDING_EVENT_TASKS = 1_024;
const DEFAULT_CONTROL_TIMEOUT_MS = 5_000;
const MAXIMUM_CONTROL_TIMEOUT_MS = 60_000;

export type A3sFramedHostStatusV1 = "open" | "closing" | "closed" | "failed";

export type A3sFramedHostErrorCodeV1 =
  | "endOfStream"
  | "eventHandlerFailed"
  | "eventHandlerMissing"
  | "hostClosed"
  | "hostFatal"
  | "controlTimedOut"
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
  readonly pendingPingNonce: number | null;
  readonly receivedHostPings: number;
  readonly lastHostPingNonce: number | null;
  readonly pendingEventTasks: number;
  readonly failure: A3sFramedHostError | null;
}

export type A3sHostEventHandlerV1 = (
  message: Readonly<TsxEventMessageV1>,
) => void | PromiseLike<void>;

export interface A3sFramedApplicationHostOptionsV1 {
  readonly onEvent?: A3sHostEventHandlerV1;
  readonly controlTimeoutMs?: number;
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

interface PendingControlV1 {
  readonly resolve: () => void;
  readonly reject: (error: A3sFramedHostError) => void;
  readonly timer: ReturnType<typeof setTimeout>;
}

interface PendingPingV1 extends PendingControlV1 {
  readonly nonce: number;
}

/** Ordered application-message pump over one negotiated framed connection. */
export class A3sFramedApplicationHostV1 implements A3sApplicationHostV1 {
  readonly #connection: A3sFramedClientConnectionV1;
  readonly #readerTask: Promise<void>;
  readonly #controlTimeoutMs: number;
  readonly #eventTasks = new Set<Promise<void>>();
  #status: A3sFramedHostStatusV1 = "open";
  #failure: A3sFramedHostError | null = null;
  #pending: PendingRenderV1 | null = null;
  #pendingPing: PendingPingV1 | null = null;
  #pendingClose: PendingControlV1 | null = null;
  #receivedHostPings = 0;
  #lastHostPingNonce: number | null = null;
  #eventHandler: A3sHostEventHandlerV1 | null;
  #closePromise: Promise<void> | null = null;

  constructor(
    connection: A3sFramedClientConnectionV1,
    options: A3sFramedApplicationHostOptionsV1 = {},
  ) {
    validateConnection(connection);
    const validated = validateHostOptions(options);
    this.#connection = connection;
    this.#controlTimeoutMs = validated.controlTimeoutMs;
    this.#eventHandler = validated.onEvent;
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
      pendingPingNonce: this.#pendingPing?.nonce ?? null,
      receivedHostPings: this.#receivedHostPings,
      lastHostPingNonce: this.#lastHostPingNonce,
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

  async ping(nonce: number): Promise<void> {
    this.#assertOpen("ping the TSX host");
    this.#assertControlIdle("ping the TSX host");
    const message = this.#connection.session.createPing(nonce);
    let resolve!: () => void;
    let reject!: (error: A3sFramedHostError) => void;
    const response = new Promise<void>((resolvePromise, rejectPromise) => {
      resolve = resolvePromise;
      reject = rejectPromise;
    });
    const timer = setTimeout(() => {
      if (this.#pendingPing?.nonce === message.payload.nonce) {
        this.#fail(
          hostError(
            "controlTimedOut",
            `TSX host did not answer liveness nonce ${message.payload.nonce} within ${this.#controlTimeoutMs}ms`,
          ),
        );
      }
    }, this.#controlTimeoutMs);
    this.#pendingPing = { nonce: message.payload.nonce, resolve, reject, timer };
    try {
      await this.#connection.writeClientMessage(message);
    } catch (cause) {
      this.#fail(hostError("streamFailed", "could not write a ping to the TSX host", cause));
    }
    await response;
  }

  close(): Promise<void> {
    if (this.#closePromise !== null) {
      return this.#closePromise;
    }
    const close = this.#close();
    this.#closePromise = close;
    void close.catch(() => {
      if (this.#status === "open" && this.#closePromise === close) {
        this.#closePromise = null;
      }
    });
    return close;
  }

  async #close(): Promise<void> {
    if (this.#status === "closed") {
      return;
    }
    if (this.#status === "failed") {
      await this.#connection.close();
      await this.#readerTask;
      await Promise.all(this.#eventTasks);
      this.#connection.session.close();
      this.#status = "closed";
      return;
    }
    this.#assertControlIdle("close the TSX host");
    const message = this.#connection.session.createClose();
    this.#status = "closing";
    let resolve!: () => void;
    let reject!: (error: A3sFramedHostError) => void;
    const acknowledgement = new Promise<void>((resolvePromise, rejectPromise) => {
      resolve = resolvePromise;
      reject = rejectPromise;
    });
    const timer = setTimeout(() => {
      if (this.#pendingClose !== null) {
        this.#fail(
          hostError(
            "controlTimedOut",
            `TSX host did not acknowledge close within ${this.#controlTimeoutMs}ms`,
          ),
        );
      }
    }, this.#controlTimeoutMs);
    this.#pendingClose = { resolve, reject, timer };
    try {
      await this.#connection.writeClientMessage(message);
      await acknowledgement;
      await this.#connection.close();
      await this.#readerTask;
      await Promise.all(this.#eventTasks);
      this.#status = "closed";
    } catch (cause) {
      const error = cause instanceof A3sFramedHostError
        ? cause
        : hostError("streamFailed", "could not close the framed TSX host", cause);
      this.#fail(error);
      try {
        await this.#connection.close();
      } catch {
        // Preserve the control or stream failure as the primary error.
      }
      throw error;
    }
  }

  async #readMessages(): Promise<void> {
    try {
      while (true) {
        const message = await this.#connection.readHostMessage();
        if (message === null) {
          if (this.#status === "closed") {
            return;
          }
          throw hostError("endOfStream", "TSX host stream ended without a close request");
        }
        if (message.type === "welcome") {
          throw hostError("invalidHostMessage", "TSX host emitted a second welcome");
        }
        let received: Readonly<TsxPostWelcomeHostMessageV1>;
        try {
          received = this.#connection.session.receiveHostMessage(message);
        } catch (cause) {
          throw hostError(
            "invalidHostMessage",
            `TSX host emitted an invalid ${message.type} message`,
            cause,
          );
        }
        if (await this.#acceptHostMessage(received)) {
          return;
        }
      }
    } catch (cause) {
      if (this.#status === "failed" || this.#status === "closed") {
        return;
      }
      this.#fail(
        cause instanceof A3sFramedHostError
          ? cause
          : hostError("streamFailed", "TSX host message pump failed", cause),
      );
    }
  }

  async #acceptHostMessage(
    message: Readonly<TsxPostWelcomeHostMessageV1>,
  ): Promise<boolean> {
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
        return false;
      }
      case "event":
        this.#dispatchEvent(message);
        return false;
      case "fatal":
        throw hostError(
          "hostFatal",
          `TSX host reported ${JSON.stringify(message.payload.code)}: ${message.payload.message}`,
        );
      case "close": {
        const pending = this.#pendingClose;
        if (this.#status !== "closing" || pending === null) {
          throw hostError(
            "hostClosed",
            `TSX host requested close with reason ${JSON.stringify(message.payload.reason)}`,
          );
        }
        try {
          this.#connection.session.acceptClose(message);
        } catch (cause) {
          throw hostError("invalidHostMessage", "TSX host emitted an invalid close reply", cause);
        }
        clearTimeout(pending.timer);
        this.#pendingClose = null;
        pending.resolve();
        return true;
      }
      case "ping": {
        let pong;
        try {
          pong = this.#connection.session.acceptPing(message);
        } catch (cause) {
          throw hostError("invalidHostMessage", "TSX host emitted an invalid ping", cause);
        }
        this.#receivedHostPings += 1;
        this.#lastHostPingNonce = message.payload.nonce;
        if (pong !== null) {
          try {
            await this.#connection.writeClientMessage(pong);
          } catch (cause) {
            throw hostError("streamFailed", "could not write a pong to the TSX host", cause);
          }
        }
        return false;
      }
      case "pong": {
        const pending = this.#pendingPing;
        if (pending === null) {
          throw hostError("invalidHostMessage", "TSX host emitted pong without a pending ping");
        }
        try {
          this.#connection.session.acceptPong(message);
        } catch (cause) {
          throw hostError("invalidHostMessage", "TSX host emitted an invalid pong", cause);
        }
        clearTimeout(pending.timer);
        this.#pendingPing = null;
        pending.resolve();
        return false;
      }
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
    const ping = this.#pendingPing;
    this.#pendingPing = null;
    if (ping !== null) {
      clearTimeout(ping.timer);
      ping.reject(error);
    }
    const close = this.#pendingClose;
    this.#pendingClose = null;
    if (close !== null) {
      clearTimeout(close.timer);
      close.reject(error);
    }
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

  #assertControlIdle(operation: string): void {
    if (this.#pending !== null) {
      throw hostError(
        "invalidState",
        `cannot ${operation} while render revision ${this.#pending.renderRevision} is in flight`,
      );
    }
    if (this.#eventTasks.size !== 0) {
      throw hostError(
        "invalidState",
        `cannot ${operation} while ${this.#eventTasks.size} event task(s) are pending`,
      );
    }
    if (this.#pendingPing !== null || this.#pendingClose !== null) {
      throw hostError("invalidState", `cannot ${operation} while a control message is pending`);
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
    const hostOptions: A3sFramedApplicationHostOptionsV1 = validated.onEvent === null
      ? { controlTimeoutMs: validated.controlTimeoutMs }
      : { onEvent: validated.onEvent, controlTimeoutMs: validated.controlTimeoutMs };
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

function validateHostOptions(options: A3sFramedApplicationHostOptionsV1): {
  onEvent: A3sHostEventHandlerV1 | null;
  controlTimeoutMs: number;
} {
  const values = plainOptionValues(
    options,
    "framed application host options",
    ["onEvent", "controlTimeoutMs"],
    [],
  );
  const onEvent = values.onEvent;
  if (onEvent !== undefined && typeof onEvent !== "function") {
    throw hostError("invalidOptions", "framed host onEvent must be a function");
  }
  return {
    onEvent: (onEvent ?? null) as A3sHostEventHandlerV1 | null,
    controlTimeoutMs: validateControlTimeout(values.controlTimeoutMs),
  };
}

function validateConnectOptions(
  options: ConnectA3sNodeApplicationHostOptionsV1,
): {
  process: SpawnA3sNodeProcessOptionsV1;
  handshake: A3sClientHandshakeOptionsV1;
  onEvent: A3sHostEventHandlerV1 | null;
  controlTimeoutMs: number;
} {
  const values = plainOptionValues(
    options,
    "Node application host options",
    ["process", "handshake", "onEvent", "controlTimeoutMs"],
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
    controlTimeoutMs: validateControlTimeout(values.controlTimeoutMs),
  };
}

function validateControlTimeout(value: unknown): number {
  if (value === undefined) {
    return DEFAULT_CONTROL_TIMEOUT_MS;
  }
  if (
    typeof value !== "number" ||
    !Number.isSafeInteger(value) ||
    value < 1 ||
    value > MAXIMUM_CONTROL_TIMEOUT_MS
  ) {
    throw hostError(
      "invalidOptions",
      `controlTimeoutMs must be an integer from 1 through ${MAXIMUM_CONTROL_TIMEOUT_MS}`,
    );
  }
  return value;
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
