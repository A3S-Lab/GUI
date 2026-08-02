import type { A3sEventHandler } from "./action.ts";
import type { CompiledA3sFrameV1 } from "./frame.ts";
import {
  TSX_PROTOCOL_NAME,
  TSX_PROTOCOL_VERSION_V1,
  TSX_PROTOCOL_V1_MAX_EVENT_ITEMS,
  TSX_PROTOCOL_V1_MAX_SAFE_INTEGER,
  type TsxActionInvocationV1,
  type TsxHostMessageV1,
} from "./generated/protocol.ts";
import { isAcceptedActionIdV1 } from "./identity.ts";

export type TsxCommittedMessageV1 = Extract<
  TsxHostMessageV1,
  { readonly type: "committed" }
>;

export type TsxEventMessageV1 = Extract<
  TsxHostMessageV1,
  { readonly type: "event" }
>;

export type A3sActionRegistryErrorCodeV1 =
  | "busy"
  | "invalidCommit"
  | "invalidEvent"
  | "invalidFrame"
  | "invalidRevision"
  | "noActiveRevision"
  | "noPendingRevision"
  | "pendingRevisionExists"
  | "staleEvent"
  | "unknownAction"
  | "callbackFailed";

export class A3sActionRegistryError extends Error {
  readonly code: A3sActionRegistryErrorCodeV1;
  readonly invocationIndex: number | null;
  readonly completedCallbacks: number;

  constructor(
    code: A3sActionRegistryErrorCodeV1,
    message: string,
    options: {
      readonly cause?: unknown;
      readonly invocationIndex?: number;
      readonly completedCallbacks?: number;
    } = {},
  ) {
    super(message, { cause: options.cause });
    this.name = "A3sActionRegistryError";
    this.code = code;
    this.invocationIndex = options.invocationIndex ?? null;
    this.completedCallbacks = options.completedCallbacks ?? 0;
  }
}

export interface A3sActionScopeSummaryV1 {
  readonly renderRevision: number;
  readonly hostRevision: number | null;
  readonly frameId: string;
  readonly actionCount: number;
  readonly callbackCount: number;
}

export interface A3sActionRegistryStateV1 {
  readonly pending: Readonly<A3sActionScopeSummaryV1> | null;
  readonly active: Readonly<A3sActionScopeSummaryV1> | null;
  readonly rollback: Readonly<A3sActionScopeSummaryV1> | null;
  readonly lastEventSequence: number;
  readonly dispatching: boolean;
}

export interface A3sActionDispatchResultV1 {
  readonly renderRevision: number;
  readonly hostRevision: number;
  readonly eventSequence: number;
  readonly invocationCount: number;
  readonly callbackCount: number;
}

interface PreparedActionScopeV1 {
  readonly renderRevision: number;
  readonly frameId: string;
  readonly actions: ReadonlyMap<string, boolean>;
  readonly callbacks: ReadonlyMap<string, A3sEventHandler>;
}

interface CommittedActionScopeV1 extends PreparedActionScopeV1 {
  readonly hostRevision: number;
}

interface PreparedInvocationV1 {
  readonly invocation: Readonly<TsxActionInvocationV1>;
  readonly handler: A3sEventHandler | null;
}

/**
 * Owns the bounded callback scopes associated with protocol-1 render revisions.
 *
 * The registry deliberately does not own IPC identity or message-id ordering;
 * `A3sClientSessionV1` validates those before calling this class. The registry
 * enforces the render/host revision pair, global event sequence, complete
 * action-vector preflight, and sequential callback execution.
 */
export class RevisionActionRegistryV1 {
  #pending: PreparedActionScopeV1 | null = null;
  #active: CommittedActionScopeV1 | null = null;
  #rollback: CommittedActionScopeV1 | null = null;
  #lastEventSequence = 0;
  #dispatching = false;

  get state(): Readonly<A3sActionRegistryStateV1> {
    return Object.freeze({
      pending: summarizeScope(this.#pending),
      active: summarizeScope(this.#active),
      rollback: summarizeScope(this.#rollback),
      lastEventSequence: this.#lastEventSequence,
      dispatching: this.#dispatching,
    });
  }

  /** Prepares exactly one next revision without changing the active callbacks. */
  stage(renderRevision: number, compiled: CompiledA3sFrameV1): void {
    this.#assertIdle("stage a render");
    const revision = requirePositiveSafeInteger(renderRevision, "render revision");
    if (this.#pending !== null) {
      throw registryError(
        "pendingRevisionExists",
        `cannot stage render revision ${revision}; revision ${this.#pending.renderRevision} is already pending`,
      );
    }
    const expected = nextProtocolInteger(
      this.#active?.renderRevision ?? 0,
      "render revision sequence",
    );
    if (revision !== expected) {
      throw registryError(
        "invalidRevision",
        `render revision ${revision} is invalid; expected ${expected}`,
      );
    }

    // Construct and validate the complete scope before publishing it.
    const pending = prepareScope(revision, compiled);
    this.#pending = pending;
  }

  /** Promotes only the matching host acknowledgement as one atomic mutation. */
  commit(message: TsxCommittedMessageV1): void {
    this.#assertIdle("commit a render");
    assertProtocolMessage(message, "committed");
    const renderRevision = requirePositiveSafeInteger(
      message.renderRevision,
      "committed render revision",
    );
    const hostRevision = requirePositiveSafeInteger(
      message.payload.hostRevision,
      "committed host revision",
    );
    const pending = this.#pending;
    if (pending === null) {
      throw registryError(
        "noPendingRevision",
        `cannot commit render revision ${renderRevision}; no render is pending`,
      );
    }
    if (pending.renderRevision !== renderRevision) {
      throw registryError(
        "invalidCommit",
        `cannot commit render revision ${renderRevision}; revision ${pending.renderRevision} is pending`,
      );
    }
    if (message.payload.frameId !== pending.frameId) {
      throw registryError(
        "invalidCommit",
        `committed frame ${JSON.stringify(message.payload.frameId)} does not match pending frame ${JSON.stringify(pending.frameId)}`,
      );
    }
    if (
      this.#active !== null &&
      hostRevision < this.#active.hostRevision
    ) {
      throw registryError(
        "invalidCommit",
        `committed host revision ${hostRevision} is older than active host revision ${this.#active.hostRevision}`,
      );
    }

    const next: CommittedActionScopeV1 = Object.freeze({
      ...pending,
      hostRevision,
    });
    this.#rollback = this.#active;
    this.#active = next;
    this.#pending = null;
  }

  /** Discards only the matching candidate and preserves both committed scopes. */
  reject(renderRevision: number): void {
    this.#assertIdle("reject a render");
    const revision = requirePositiveSafeInteger(renderRevision, "render revision");
    const pending = this.#pending;
    if (pending === null) {
      throw registryError(
        "noPendingRevision",
        `cannot reject render revision ${revision}; no render is pending`,
      );
    }
    if (pending.renderRevision !== revision) {
      throw registryError(
        "invalidRevision",
        `cannot reject render revision ${revision}; revision ${pending.renderRevision} is pending`,
      );
    }
    this.#pending = null;
  }

  /**
   * Dispatches the complete host vector in wire order.
   *
   * The whole vector is checked before the sequence is consumed or any user
   * callback runs. Once execution starts, the sequence is consumed even when a
   * callback throws so a partially observed event can never be replayed.
   */
  async dispatch(message: TsxEventMessageV1): Promise<Readonly<A3sActionDispatchResultV1>> {
    if (this.#dispatching) {
      throw registryError("busy", "an A3S event vector is already being dispatched");
    }
    assertProtocolMessage(message, "event");
    const active = this.#active;
    if (active === null) {
      throw registryError("noActiveRevision", "cannot dispatch an event without an active render");
    }
    const renderRevision = requirePositiveSafeInteger(
      message.renderRevision,
      "event render revision",
    );
    const hostRevision = requirePositiveSafeInteger(
      message.payload.hostRevision,
      "event host revision",
    );
    const eventSequence = requirePositiveSafeInteger(
      message.payload.eventSequence,
      "event sequence",
    );
    if (renderRevision !== active.renderRevision) {
      throw registryError(
        "staleEvent",
        `event render revision ${renderRevision} is stale; active revision is ${active.renderRevision}`,
      );
    }
    if (hostRevision < active.hostRevision) {
      throw registryError(
        "staleEvent",
        `event host revision ${hostRevision} is stale; active host revision is ${active.hostRevision}`,
      );
    }
    const expectedSequence = nextProtocolInteger(
      this.#lastEventSequence,
      "event sequence",
    );
    if (eventSequence !== expectedSequence) {
      throw registryError(
        "staleEvent",
        `event sequence ${eventSequence} is invalid; expected ${expectedSequence}`,
      );
    }

    const invocations = message.payload.invocations ?? [];
    if (!Array.isArray(invocations) || invocations.length > TSX_PROTOCOL_V1_MAX_EVENT_ITEMS) {
      throw registryError(
        "invalidEvent",
        `event invocations must be an array of at most ${TSX_PROTOCOL_V1_MAX_EVENT_ITEMS} items`,
      );
    }
    const prepared = invocations.map((invocation, index) =>
      prepareInvocation(invocation, index, active)
    );

    this.#dispatching = true;
    this.#lastEventSequence = eventSequence;
    if (hostRevision > active.hostRevision) {
      this.#active = Object.freeze({ ...active, hostRevision });
    }
    let callbackCount = 0;
    try {
      for (let index = 0; index < prepared.length; index += 1) {
        const item = prepared[index];
        if (item.handler === null) {
          continue;
        }
        try {
          await item.handler(item.invocation);
          callbackCount += 1;
        } catch (cause) {
          throw new A3sActionRegistryError(
            "callbackFailed",
            `action ${JSON.stringify(item.invocation.action)} callback failed at invocation ${index}`,
            { cause, invocationIndex: index, completedCallbacks: callbackCount },
          );
        }
      }
      return Object.freeze({
        renderRevision,
        hostRevision,
        eventSequence,
        invocationCount: prepared.length,
        callbackCount,
      });
    } finally {
      this.#dispatching = false;
    }
  }

  /** Releases pending, active, and rollback callback references at shutdown. */
  clear(): void {
    this.#assertIdle("clear callback scopes");
    this.#pending = null;
    this.#active = null;
    this.#rollback = null;
    this.#lastEventSequence = 0;
  }

  #assertIdle(operation: string): void {
    if (this.#dispatching) {
      throw registryError("busy", `cannot ${operation} while an event vector is dispatching`);
    }
  }
}

function prepareScope(
  renderRevision: number,
  compiled: CompiledA3sFrameV1,
): PreparedActionScopeV1 {
  if (
    typeof compiled !== "object" ||
    compiled === null ||
    typeof compiled.frame !== "object" ||
    compiled.frame === null ||
    typeof compiled.frame.frameId !== "string" ||
    compiled.frame.frameId.length === 0 ||
    !Array.isArray(compiled.frame.actions)
  ) {
    throw registryError("invalidFrame", "compiled frame callback scope is malformed");
  }
  const actions = new Map<string, boolean>();
  for (const action of compiled.frame.actions) {
    if (
      typeof action !== "object" ||
      action === null ||
      typeof action.disabled !== "boolean"
    ) {
      throw registryError("invalidFrame", "compiled frame contains a malformed action");
    }
    if (!isAcceptedActionIdV1(action.id)) {
      throw registryError("invalidFrame", "compiled frame contains an invalid action id");
    }
    if (actions.has(action.id)) {
      throw registryError(
        "invalidFrame",
        `compiled frame contains duplicate action id ${JSON.stringify(action.id)}`,
      );
    }
    actions.set(action.id, action.disabled);
  }

  const callbacks = new Map<string, A3sEventHandler>();
  for (const [id, handler] of compiled.callbacks) {
    if (!actions.has(id) || typeof handler !== "function") {
      throw registryError(
        "invalidFrame",
        `callback ${JSON.stringify(id)} does not match a compiled frame action`,
      );
    }
    callbacks.set(id, handler);
  }
  return Object.freeze({
    renderRevision,
    frameId: compiled.frame.frameId,
    actions,
    callbacks,
  });
}

function prepareInvocation(
  invocation: TsxActionInvocationV1,
  index: number,
  active: CommittedActionScopeV1,
): PreparedInvocationV1 {
  if (
    typeof invocation !== "object" ||
    invocation === null ||
    typeof invocation.action !== "string" ||
    invocation.action.length === 0
  ) {
    throw registryError("invalidEvent", `event invocation ${index} has an invalid action id`);
  }
  if (!active.actions.has(invocation.action)) {
    throw registryError(
      "unknownAction",
      `event invocation ${index} references unknown action ${JSON.stringify(invocation.action)}`,
    );
  }
  if (active.actions.get(invocation.action) === true) {
    throw registryError(
      "invalidEvent",
      `event invocation ${index} references disabled action ${JSON.stringify(invocation.action)}`,
    );
  }
  return Object.freeze({
    invocation: deepFreezeClone(invocation),
    handler: active.callbacks.get(invocation.action) ?? null,
  });
}

function assertProtocolMessage(
  message: TsxHostMessageV1,
  expectedType: "committed" | "event",
): void {
  if (
    typeof message !== "object" ||
    message === null ||
    message.type !== expectedType ||
    message.protocol !== TSX_PROTOCOL_NAME ||
    message.protocolVersion !== TSX_PROTOCOL_VERSION_V1 ||
    typeof message.payload !== "object" ||
    message.payload === null
  ) {
    throw registryError(
      expectedType === "committed" ? "invalidCommit" : "invalidEvent",
      `expected a valid ${expectedType} message for ${TSX_PROTOCOL_NAME} v${TSX_PROTOCOL_VERSION_V1}`,
    );
  }
}

function requirePositiveSafeInteger(value: unknown, name: string): number {
  if (
    typeof value !== "number" ||
    !Number.isSafeInteger(value) ||
    value < 1 ||
    value > TSX_PROTOCOL_V1_MAX_SAFE_INTEGER
  ) {
    throw registryError(
      "invalidRevision",
      `${name} must be a positive protocol-safe integer`,
    );
  }
  return value;
}

function nextProtocolInteger(value: number, name: string): number {
  if (value >= TSX_PROTOCOL_V1_MAX_SAFE_INTEGER) {
    throw registryError(
      "invalidRevision",
      `${name} exhausted the protocol-safe integer range`,
    );
  }
  return value + 1;
}

function summarizeScope(
  scope: PreparedActionScopeV1 | CommittedActionScopeV1 | null,
): Readonly<A3sActionScopeSummaryV1> | null {
  if (scope === null) {
    return null;
  }
  return Object.freeze({
    renderRevision: scope.renderRevision,
    hostRevision: "hostRevision" in scope ? scope.hostRevision : null,
    frameId: scope.frameId,
    actionCount: scope.actions.size,
    callbackCount: scope.callbacks.size,
  });
}

function deepFreezeClone<Value>(value: Value): Readonly<Value> {
  if (Array.isArray(value)) {
    return Object.freeze(
      value.map((item) => deepFreezeClone(item)),
    ) as unknown as Readonly<Value>;
  }
  if (typeof value === "object" && value !== null) {
    const clone: Record<string, unknown> = {};
    for (const [name, item] of Object.entries(value)) {
      Object.defineProperty(clone, name, {
        configurable: false,
        enumerable: true,
        value: deepFreezeClone(item),
        writable: false,
      });
    }
    return Object.freeze(clone) as Readonly<Value>;
  }
  return value as Readonly<Value>;
}

function registryError(
  code: A3sActionRegistryErrorCodeV1,
  message: string,
): A3sActionRegistryError {
  return new A3sActionRegistryError(code, message);
}
