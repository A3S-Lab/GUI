import {
  A3sActionRegistryError,
  RevisionActionRegistryV1,
  type A3sActionDispatchResultV1,
  type TsxCommittedMessageV1,
  type TsxEventMessageV1,
} from "./action-registry.ts";
import {
  TSX_PROTOCOL_NAME,
  TSX_PROTOCOL_VERSION_V1,
  TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
  TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES,
  TSX_PROTOCOL_V1_MAX_DIAGNOSTICS,
  TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES,
  TSX_PROTOCOL_V1_MAX_EVENT_ITEMS,
  TSX_PROTOCOL_V1_MAX_SAFE_INTEGER,
  TSX_PROTOCOL_V1_MAX_SESSION_ID_BYTES,
  TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
  type ProtocolUiFrameV1,
  type TsxClientMessageV1,
  type TsxHostMessageV1,
} from "./generated/protocol.ts";

export type TsxWelcomeMessageV1 = Extract<
  TsxHostMessageV1,
  { readonly type: "welcome" }
>;

export type TsxRenderMessageV1 = Extract<
  TsxClientMessageV1,
  { readonly type: "render" }
>;

export type A3sClientSessionStatusV1 = "negotiated" | "failed" | "closed";

export type A3sClientSessionErrorCodeV1 =
  | "frameTooLarge"
  | "invalidMessage"
  | "invalidMessageId"
  | "invalidRevision"
  | "invalidSession"
  | "invalidState"
  | "invalidWelcome"
  | "messageIdExhausted";

export class A3sClientSessionError extends Error {
  readonly code: A3sClientSessionErrorCodeV1;

  constructor(code: A3sClientSessionErrorCodeV1, message: string, cause?: unknown) {
    super(message, cause === undefined ? undefined : { cause });
    this.name = "A3sClientSessionError";
    this.code = code;
  }
}

export interface A3sClientSessionStateV1 {
  readonly status: A3sClientSessionStatusV1;
  readonly sessionId: string;
  readonly lastClientMessageId: number;
  readonly lastHostMessageId: number;
  readonly committedRenderRevision: number;
  readonly committedHostRevision: number | null;
  readonly pendingRenderRevision: number | null;
  readonly maximumFrameBytes: number;
}

interface PendingRenderV1 {
  readonly renderRevision: number;
  readonly frameId: string;
}

const textEncoder = new TextEncoder();

/** Owns the post-handshake protocol identity for one TypeScript application. */
export class A3sClientSessionV1 {
  readonly #welcome: Readonly<TsxWelcomeMessageV1>;
  readonly #sessionId: string;
  readonly #maximumFrameBytes: number;
  #status: A3sClientSessionStatusV1 = "negotiated";
  #lastClientMessageId = 1;
  #lastHostMessageId = 1;
  #committedRenderRevision = 0;
  #committedHostRevision: number | null = null;
  #pending: PendingRenderV1 | null = null;

  constructor(welcome: TsxWelcomeMessageV1) {
    const snapshot = snapshotProtocolValue(
      welcome,
      "welcome message",
      "invalidWelcome",
    ) as TsxWelcomeMessageV1;
    validateWelcome(snapshot);
    this.#welcome = snapshot;
    this.#sessionId = snapshot.sessionId;
    this.#maximumFrameBytes = snapshot.payload.limits.maximumFrameBytes;
    assertEncodedSize(snapshot, this.#maximumFrameBytes, "invalidWelcome");
  }

  get welcome(): Readonly<TsxWelcomeMessageV1> {
    return this.#welcome;
  }

  get state(): Readonly<A3sClientSessionStateV1> {
    return Object.freeze({
      status: this.#status,
      sessionId: this.#sessionId,
      lastClientMessageId: this.#lastClientMessageId,
      lastHostMessageId: this.#lastHostMessageId,
      committedRenderRevision: this.#committedRenderRevision,
      committedHostRevision: this.#committedHostRevision,
      pendingRenderRevision: this.#pending?.renderRevision ?? null,
      maximumFrameBytes: this.#maximumFrameBytes,
    });
  }

  createRender(
    renderRevision: number,
    frame: Readonly<ProtocolUiFrameV1>,
  ): Readonly<TsxRenderMessageV1> {
    this.#assertNegotiated("create a render message");
    if (this.#pending !== null) {
      throw sessionError(
        "invalidState",
        `cannot create render revision ${renderRevision}; revision ${this.#pending.renderRevision} is already pending`,
      );
    }
    const revision = requireSafeInteger(
      renderRevision,
      "render revision",
      1,
      "invalidRevision",
    );
    const expectedRevision = nextSafeInteger(
      this.#committedRenderRevision,
      "render revision",
    );
    if (revision !== expectedRevision) {
      throw sessionError(
        "invalidRevision",
        `render revision ${revision} is invalid; expected ${expectedRevision}`,
      );
    }

    const payload = snapshotProtocolValue(
      frame,
      "render payload",
      "invalidMessage",
    ) as ProtocolUiFrameV1;
    const payloadRecord = requireRecord(payload, "render payload", "invalidMessage");
    const frameId = requireBoundedText(
      payloadRecord.frameId,
      "render frame id",
      TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES,
      "invalidMessage",
    );
    const messageId = nextSafeInteger(this.#lastClientMessageId, "client message id");
    const message = Object.freeze({
      type: "render" as const,
      protocol: TSX_PROTOCOL_NAME,
      protocolVersion: TSX_PROTOCOL_VERSION_V1,
      sessionId: this.#sessionId,
      messageId,
      renderRevision: revision,
      payload,
    });
    assertEncodedSize(message, this.#maximumFrameBytes, "frameTooLarge");

    this.#lastClientMessageId = messageId;
    this.#pending = Object.freeze({ renderRevision: revision, frameId });
    return message;
  }

  rejectRender(renderRevision: number): void {
    if (this.#status === "closed") {
      throw sessionError("invalidState", "cannot reject a render after the session is closed");
    }
    const revision = requireSafeInteger(
      renderRevision,
      "render revision",
      1,
      "invalidRevision",
    );
    if (this.#pending === null) {
      throw sessionError("invalidState", "the client session has no pending render to reject");
    }
    if (this.#pending.renderRevision !== revision) {
      throw sessionError(
        "invalidRevision",
        `cannot reject render revision ${revision}; revision ${this.#pending.renderRevision} is pending`,
      );
    }
    this.#pending = null;
  }

  commitRender(
    message: TsxCommittedMessageV1,
    actions: RevisionActionRegistryV1,
  ): void {
    this.#assertNegotiated("commit a render message");
    let snapshot: TsxCommittedMessageV1;
    try {
      snapshot = this.#validateCommitted(message);
      actions.commit(snapshot);
    } catch (cause) {
      this.#status = "failed";
      throw cause;
    }

    this.#lastHostMessageId = snapshot.messageId;
    this.#committedRenderRevision = snapshot.renderRevision;
    this.#committedHostRevision = snapshot.payload.hostRevision;
    this.#pending = null;
  }

  async dispatchEvent(
    message: TsxEventMessageV1,
    actions: RevisionActionRegistryV1,
  ): Promise<Readonly<A3sActionDispatchResultV1>> {
    this.#assertNegotiated("dispatch an event message");
    let snapshot: TsxEventMessageV1;
    try {
      snapshot = this.#validateEvent(message);
    } catch (cause) {
      this.#status = "failed";
      throw cause;
    }

    try {
      const result = await actions.dispatch(snapshot);
      this.#lastHostMessageId = snapshot.messageId;
      return result;
    } catch (cause) {
      if (cause instanceof A3sActionRegistryError && cause.code === "callbackFailed") {
        this.#lastHostMessageId = snapshot.messageId;
      } else {
        this.#status = "failed";
      }
      throw cause;
    }
  }

  close(): void {
    this.#pending = null;
    this.#status = "closed";
  }

  #validateCommitted(message: TsxCommittedMessageV1): TsxCommittedMessageV1 {
    const snapshot = this.#snapshotHostMessage(message, "committed") as TsxCommittedMessageV1;
    const pending = this.#pending;
    if (pending === null) {
      throw sessionError("invalidState", "the client session has no pending render to commit");
    }
    if (snapshot.renderRevision !== pending.renderRevision) {
      throw sessionError(
        "invalidRevision",
        `committed render revision ${snapshot.renderRevision} is invalid; revision ${pending.renderRevision} is pending`,
      );
    }

    const payload = requireRecord(snapshot.payload, "committed payload", "invalidMessage");
    assertExactKeys(
      payload,
      ["frameId", "hostRevision", "rootId", "layoutFingerprint", "sceneFingerprint"],
      ["diagnostics"],
      "committed payload",
      "invalidMessage",
    );
    const frameId = requireBoundedText(
      payload.frameId,
      "committed frame id",
      TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES,
      "invalidMessage",
    );
    if (frameId !== pending.frameId) {
      throw sessionError(
        "invalidMessage",
        `committed frame ${JSON.stringify(frameId)} does not match pending frame ${JSON.stringify(pending.frameId)}`,
      );
    }
    const hostRevision = requireSafeInteger(
      payload.hostRevision,
      "committed host revision",
      1,
      "invalidRevision",
    );
    if (
      this.#committedHostRevision !== null &&
      hostRevision < this.#committedHostRevision
    ) {
      throw sessionError(
        "invalidRevision",
        `committed host revision ${hostRevision} is older than active host revision ${this.#committedHostRevision}`,
      );
    }
    requireBoundedText(
      payload.rootId,
      "committed root id",
      TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES,
      "invalidMessage",
    );
    requireFingerprint(payload.layoutFingerprint, "layout fingerprint");
    requireFingerprint(payload.sceneFingerprint, "scene fingerprint");
    validateDiagnostics(payload.diagnostics);
    return snapshot;
  }

  #validateEvent(message: TsxEventMessageV1): TsxEventMessageV1 {
    const snapshot = this.#snapshotHostMessage(message, "event") as TsxEventMessageV1;
    if (snapshot.renderRevision !== this.#committedRenderRevision) {
      throw sessionError(
        "invalidRevision",
        `event render revision ${snapshot.renderRevision} is stale; active revision is ${this.#committedRenderRevision}`,
      );
    }
    const payload = requireRecord(snapshot.payload, "event payload", "invalidMessage");
    assertExactKeys(
      payload,
      ["hostRevision", "eventSequence"],
      ["target", "invocations", "interactionChanges", "propagationStoppedAt"],
      "event payload",
      "invalidMessage",
    );
    requireSafeInteger(
      payload.hostRevision,
      "event host revision",
      1,
      "invalidRevision",
    );
    requireSafeInteger(
      payload.eventSequence,
      "event sequence",
      1,
      "invalidRevision",
    );
    validateOptionalElementId(payload.target, "event target");
    validateOptionalElementId(payload.propagationStoppedAt, "propagation stop target");
    validateBoundedArray(payload.invocations, "event invocations");
    validateBoundedArray(payload.interactionChanges, "event interaction changes");
    return snapshot;
  }

  #snapshotHostMessage(
    message: unknown,
    expectedType: "committed" | "event",
  ): TsxHostMessageV1 {
    const snapshot = snapshotProtocolValue(
      message,
      `${expectedType} message`,
      "invalidMessage",
    ) as TsxHostMessageV1;
    const record = requireRecord(snapshot, `${expectedType} message`, "invalidMessage");
    assertExactKeys(
      record,
      [
        "type",
        "protocol",
        "protocolVersion",
        "sessionId",
        "messageId",
        "renderRevision",
        "payload",
      ],
      [],
      `${expectedType} message`,
      "invalidMessage",
    );
    if (record.type !== expectedType) {
      throw sessionError("invalidMessage", `expected a ${expectedType} host message`);
    }
    if (
      record.protocol !== TSX_PROTOCOL_NAME ||
      record.protocolVersion !== TSX_PROTOCOL_VERSION_V1
    ) {
      throw sessionError(
        "invalidMessage",
        `expected ${TSX_PROTOCOL_NAME} v${TSX_PROTOCOL_VERSION_V1}`,
      );
    }
    if (record.sessionId !== this.#sessionId) {
      throw sessionError(
        "invalidSession",
        `host message session ${JSON.stringify(record.sessionId)} does not match negotiated session ${JSON.stringify(this.#sessionId)}`,
      );
    }
    const messageId = requireSafeInteger(
      record.messageId,
      "host message id",
      1,
      "invalidMessageId",
    );
    const expectedMessageId = nextSafeInteger(this.#lastHostMessageId, "host message id");
    if (messageId !== expectedMessageId) {
      throw sessionError(
        "invalidMessageId",
        `host message id ${messageId} is invalid; expected ${expectedMessageId}`,
      );
    }
    requireSafeInteger(
      record.renderRevision,
      `${expectedType} render revision`,
      1,
      "invalidRevision",
    );
    assertEncodedSize(snapshot, this.#maximumFrameBytes, "frameTooLarge");
    return snapshot;
  }

  #assertNegotiated(operation: string): void {
    if (this.#status !== "negotiated") {
      throw sessionError(
        "invalidState",
        `cannot ${operation} while the client session is ${this.#status}`,
      );
    }
  }
}

function validateWelcome(message: TsxWelcomeMessageV1): void {
  const record = requireRecord(message, "welcome message", "invalidWelcome");
  assertExactKeys(
    record,
    [
      "type",
      "protocol",
      "protocolVersion",
      "sessionId",
      "messageId",
      "renderRevision",
      "payload",
    ],
    [],
    "welcome message",
    "invalidWelcome",
  );
  if (
    record.type !== "welcome" ||
    record.protocol !== TSX_PROTOCOL_NAME ||
    record.protocolVersion !== TSX_PROTOCOL_VERSION_V1 ||
    record.messageId !== 1 ||
    record.renderRevision !== 0
  ) {
    throw sessionError(
      "invalidWelcome",
      `expected the first ${TSX_PROTOCOL_NAME} v${TSX_PROTOCOL_VERSION_V1} host message to be welcome`,
    );
  }
  requireBoundedText(
    record.sessionId,
    "welcome session id",
    TSX_PROTOCOL_V1_MAX_SESSION_ID_BYTES,
    "invalidWelcome",
  );

  const payload = requireRecord(record.payload, "welcome payload", "invalidWelcome");
  assertExactKeys(
    payload,
    [
      "selectedProtocolVersion",
      "hostVersion",
      "hostBuildId",
      "platform",
      "renderer",
      "limits",
    ],
    ["capabilities", "debugCapabilities"],
    "welcome payload",
    "invalidWelcome",
  );
  if (payload.selectedProtocolVersion !== TSX_PROTOCOL_VERSION_V1) {
    throw sessionError("invalidWelcome", "welcome selected an unsupported protocol version");
  }
  requireBoundedText(
    payload.hostVersion,
    "host version",
    TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
    "invalidWelcome",
  );
  requireBoundedText(
    payload.hostBuildId,
    "host build id",
    TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
    "invalidWelcome",
  );
  requireEnum(payload.platform, ["headless", "macos", "windows", "linux"], "host platform");
  requireEnum(payload.renderer, ["software", "gpu"], "host renderer");

  const limits = requireRecord(payload.limits, "welcome limits", "invalidWelcome");
  assertExactKeys(
    limits,
    ["maximumFrameBytes", "maximumInFlightRenders"],
    [],
    "welcome limits",
    "invalidWelcome",
  );
  requireSafeInteger(
    limits.maximumFrameBytes,
    "maximum frame bytes",
    1,
    "invalidWelcome",
    TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
  );
  if (limits.maximumInFlightRenders !== 1) {
    throw sessionError("invalidWelcome", "protocol v1 requires one in-flight render");
  }
  validateUniqueEnumArray(
    payload.capabilities,
    [
      "headlessRendering",
      "selfDrawnRendering",
      "dropPolicyQueries",
      "structuredDiagnostics",
    ],
    "host capabilities",
  );
  validateUniqueEnumArray(
    payload.debugCapabilities,
    ["protocolTrace", "structuredDiagnostics", "inspector"],
    "debug capabilities",
  );
}

function validateDiagnostics(value: unknown): void {
  if (value === undefined) {
    return;
  }
  if (!Array.isArray(value) || value.length > TSX_PROTOCOL_V1_MAX_DIAGNOSTICS) {
    throw sessionError(
      "invalidMessage",
      `committed diagnostics must contain at most ${TSX_PROTOCOL_V1_MAX_DIAGNOSTICS} items`,
    );
  }
  for (let index = 0; index < value.length; index += 1) {
    const diagnostic = requireRecord(
      value[index],
      `committed diagnostic ${index}`,
      "invalidMessage",
    );
    assertExactKeys(
      diagnostic,
      ["severity", "code", "message"],
      ["elementId"],
      `committed diagnostic ${index}`,
      "invalidMessage",
    );
    requireEnum(
      diagnostic.severity,
      ["information", "warning", "error"],
      `committed diagnostic ${index} severity`,
      "invalidMessage",
    );
    if (diagnostic.severity === "error") {
      throw sessionError("invalidMessage", "committed messages cannot contain error diagnostics");
    }
    requireBoundedText(
      diagnostic.code,
      `committed diagnostic ${index} code`,
      TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
      "invalidMessage",
    );
    requireBoundedText(
      diagnostic.message,
      `committed diagnostic ${index} message`,
      TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES,
      "invalidMessage",
    );
    validateOptionalElementId(
      diagnostic.elementId,
      `committed diagnostic ${index} element id`,
    );
  }
}

function validateBoundedArray(value: unknown, name: string): void {
  if (value === undefined) {
    return;
  }
  if (!Array.isArray(value) || value.length > TSX_PROTOCOL_V1_MAX_EVENT_ITEMS) {
    throw sessionError(
      "invalidMessage",
      `${name} must contain at most ${TSX_PROTOCOL_V1_MAX_EVENT_ITEMS} items`,
    );
  }
}

function validateOptionalElementId(value: unknown, name: string): void {
  if (value !== undefined && value !== null) {
    requireBoundedText(
      value,
      name,
      TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES,
      "invalidMessage",
    );
  }
}

function validateUniqueEnumArray(
  value: unknown,
  allowed: readonly string[],
  name: string,
): void {
  if (value === undefined) {
    return;
  }
  if (!Array.isArray(value)) {
    throw sessionError("invalidWelcome", `${name} must be an array`);
  }
  const seen = new Set<string>();
  for (const item of value) {
    const entry = requireEnum(item, allowed, name);
    if (seen.has(entry)) {
      throw sessionError("invalidWelcome", `${name} must not contain duplicates`);
    }
    seen.add(entry);
  }
}

function requireEnum(
  value: unknown,
  allowed: readonly string[],
  name: string,
  code: A3sClientSessionErrorCodeV1 = "invalidWelcome",
): string {
  if (typeof value !== "string" || !allowed.includes(value)) {
    throw sessionError(code, `${name} is invalid`);
  }
  return value;
}

function requireFingerprint(value: unknown, name: string): void {
  if (typeof value !== "string" || !/^[0-9a-f]{16}$/u.test(value)) {
    throw sessionError("invalidMessage", `${name} must be sixteen lowercase hexadecimal digits`);
  }
}

function requireBoundedText(
  value: unknown,
  name: string,
  maximumBytes: number,
  code: A3sClientSessionErrorCodeV1,
): string {
  if (
    typeof value !== "string" ||
    value.trim().length === 0 ||
    textEncoder.encode(value).byteLength > maximumBytes ||
    /[\u0000-\u001f\u007f-\u009f]/u.test(value)
  ) {
    throw sessionError(code, `${name} must be non-empty bounded text`);
  }
  return value;
}

function requireSafeInteger(
  value: unknown,
  name: string,
  minimum: number,
  code: A3sClientSessionErrorCodeV1,
  maximum: number = TSX_PROTOCOL_V1_MAX_SAFE_INTEGER,
): number {
  if (
    typeof value !== "number" ||
    !Number.isSafeInteger(value) ||
    value < minimum ||
    value > maximum
  ) {
    throw sessionError(code, `${name} must be an integer from ${minimum} through ${maximum}`);
  }
  return value;
}

function nextSafeInteger(value: number, name: string): number {
  if (value >= TSX_PROTOCOL_V1_MAX_SAFE_INTEGER) {
    throw sessionError("messageIdExhausted", `${name} exhausted the protocol-safe range`);
  }
  return value + 1;
}

function assertEncodedSize(
  message: unknown,
  maximumBytes: number,
  code: "frameTooLarge" | "invalidWelcome",
): void {
  const bytes = textEncoder.encode(JSON.stringify(message)).byteLength;
  if (bytes === 0 || bytes > maximumBytes) {
    throw sessionError(
      code,
      `protocol message contains ${bytes} bytes, exceeding the negotiated ${maximumBytes}-byte limit`,
    );
  }
}

function assertExactKeys(
  record: Readonly<Record<string, unknown>>,
  required: readonly string[],
  optional: readonly string[],
  name: string,
  code: A3sClientSessionErrorCodeV1,
): void {
  const allowed = new Set([...required, ...optional]);
  for (const key of required) {
    if (!Object.hasOwn(record, key)) {
      throw sessionError(code, `${name} is missing field ${JSON.stringify(key)}`);
    }
  }
  for (const key of Object.keys(record)) {
    if (!allowed.has(key)) {
      throw sessionError(code, `${name} contains unknown field ${JSON.stringify(key)}`);
    }
  }
}

function requireRecord(
  value: unknown,
  name: string,
  code: A3sClientSessionErrorCodeV1,
): Readonly<Record<string, unknown>> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw sessionError(code, `${name} must be an object`);
  }
  return value as Readonly<Record<string, unknown>>;
}

function snapshotProtocolValue(
  value: unknown,
  path: string,
  code: A3sClientSessionErrorCodeV1,
  active = new Set<object>(),
): unknown {
  if (value === null || typeof value === "string" || typeof value === "boolean") {
    return value;
  }
  if (typeof value === "number") {
    if (!Number.isFinite(value)) {
      throw sessionError(code, `${path} contains a non-finite number`);
    }
    return value;
  }
  if (typeof value !== "object") {
    throw sessionError(code, `${path} contains a non-JSON value`);
  }
  if (active.has(value)) {
    throw sessionError(code, `${path} contains a cycle`);
  }
  active.add(value);
  try {
    if (Array.isArray(value)) {
      const clone: unknown[] = [];
      for (let index = 0; index < value.length; index += 1) {
        if (!Object.hasOwn(value, index)) {
          throw sessionError(code, `${path} contains a sparse array`);
        }
        clone.push(snapshotProtocolValue(value[index], `${path}[${index}]`, code, active));
      }
      return Object.freeze(clone);
    }
    const prototype = Object.getPrototypeOf(value);
    if (prototype !== Object.prototype && prototype !== null) {
      throw sessionError(code, `${path} must contain only plain objects`);
    }
    const descriptors = Object.getOwnPropertyDescriptors(value);
    const enumerableSymbols = Object.getOwnPropertySymbols(value).filter(
      (symbol) => Object.getOwnPropertyDescriptor(value, symbol)?.enumerable,
    );
    if (enumerableSymbols.length > 0) {
      throw sessionError(code, `${path} cannot contain symbol fields`);
    }
    const clone: Record<string, unknown> = {};
    for (const [name, descriptor] of Object.entries(descriptors)) {
      if (!descriptor.enumerable) {
        continue;
      }
      if (!("value" in descriptor)) {
        throw sessionError(code, `${path}.${name} cannot be an accessor`);
      }
      Object.defineProperty(clone, name, {
        configurable: false,
        enumerable: true,
        value: snapshotProtocolValue(descriptor.value, `${path}.${name}`, code, active),
        writable: false,
      });
    }
    return Object.freeze(clone);
  } finally {
    active.delete(value);
  }
}

function sessionError(
  code: A3sClientSessionErrorCodeV1,
  message: string,
  cause?: unknown,
): A3sClientSessionError {
  return new A3sClientSessionError(code, message, cause);
}
