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
  TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES,
  TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES,
  type ProtocolUiFrameV1,
  type TsxClientMessageV1,
  type TsxClosePayloadV1,
  type TsxCloseReasonV1,
  type TsxHostMessageV1,
} from "./generated/protocol.ts";
import { clientSessionError as sessionError } from "./client-session-error.ts";
import {
  assertEncodedSize,
  assertExactKeys,
  nextSafeInteger,
  requireBoundedText,
  requireEnum,
  requireFingerprint,
  requireRecord,
  requireSafeInteger,
  snapshotProtocolValue,
  validateBoundedArray,
  validateDiagnostics,
  validateOptionalCloseMessage,
  validateOptionalElementId,
  validateWelcome,
} from "./client-session-validation.ts";

export { A3sClientSessionError } from "./client-session-error.ts";
export type { A3sClientSessionErrorCodeV1 } from "./client-session-error.ts";

export type TsxWelcomeMessageV1 = Extract<
  TsxHostMessageV1,
  { readonly type: "welcome" }
>;

export type TsxRenderMessageV1 = Extract<
  TsxClientMessageV1,
  { readonly type: "render" }
>;

export type TsxClientPingMessageV1 = Extract<
  TsxClientMessageV1,
  { readonly type: "ping" }
>;

export type TsxHostPongMessageV1 = Extract<
  TsxHostMessageV1,
  { readonly type: "pong" }
>;

export type TsxClientCloseMessageV1 = Extract<
  TsxClientMessageV1,
  { readonly type: "close" }
>;

export type TsxHostCloseMessageV1 = Extract<
  TsxHostMessageV1,
  { readonly type: "close" }
>;

export type A3sClientSessionStatusV1 = "negotiated" | "closing" | "failed" | "closed";

export interface A3sClientSessionStateV1 {
  readonly status: A3sClientSessionStatusV1;
  readonly sessionId: string;
  readonly lastClientMessageId: number;
  readonly lastHostMessageId: number;
  readonly committedRenderRevision: number;
  readonly committedHostRevision: number | null;
  readonly pendingRenderRevision: number | null;
  readonly pendingPingNonce: number | null;
  readonly pendingCloseReason: TsxCloseReasonV1 | null;
  readonly maximumFrameBytes: number;
}

interface PendingRenderV1 {
  readonly renderRevision: number;
  readonly frameId: string;
}

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
  #pendingPingNonce: number | null = null;
  #pendingClose: Readonly<TsxClosePayloadV1> | null = null;

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
      pendingPingNonce: this.#pendingPingNonce,
      pendingCloseReason: this.#pendingClose?.reason ?? null,
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

  createPing(nonce: number): Readonly<TsxClientPingMessageV1> {
    this.#assertNegotiated("create a ping message");
    if (this.#pendingPingNonce !== null) {
      throw sessionError(
        "invalidState",
        `cannot create ping ${nonce}; nonce ${this.#pendingPingNonce} is already pending`,
      );
    }
    const validatedNonce = requireSafeInteger(
      nonce,
      "liveness nonce",
      0,
      "invalidMessage",
    );
    const messageId = nextSafeInteger(this.#lastClientMessageId, "client message id");
    const message = Object.freeze({
      type: "ping" as const,
      protocol: TSX_PROTOCOL_NAME,
      protocolVersion: TSX_PROTOCOL_VERSION_V1,
      sessionId: this.#sessionId,
      messageId,
      renderRevision: this.#committedRenderRevision,
      payload: Object.freeze({ nonce: validatedNonce }),
    });
    assertEncodedSize(message, this.#maximumFrameBytes, "frameTooLarge");

    this.#lastClientMessageId = messageId;
    this.#pendingPingNonce = validatedNonce;
    return message;
  }

  acceptPong(message: TsxHostPongMessageV1): void {
    this.#assertNegotiated("accept a pong message");
    try {
      const snapshot = this.#validateControlMessage(message, "pong");
      const payload = requireRecord(snapshot.payload, "pong payload", "invalidMessage");
      assertExactKeys(payload, ["nonce"], [], "pong payload", "invalidMessage");
      const nonce = requireSafeInteger(
        payload.nonce,
        "pong nonce",
        0,
        "invalidMessage",
      );
      if (this.#pendingPingNonce === null) {
        throw sessionError("invalidState", "the client session has no pending ping");
      }
      if (nonce !== this.#pendingPingNonce) {
        throw sessionError(
          "invalidMessage",
          `pong nonce ${nonce} does not match pending nonce ${this.#pendingPingNonce}`,
        );
      }

      this.#lastHostMessageId = snapshot.messageId;
      this.#pendingPingNonce = null;
    } catch (cause) {
      this.#status = "failed";
      throw cause;
    }
  }

  createClose(
    reason: TsxCloseReasonV1 = "normal",
    message?: string,
  ): Readonly<TsxClientCloseMessageV1> {
    this.#assertNegotiated("create a close message");
    if (this.#pending !== null) {
      throw sessionError(
        "invalidState",
        `cannot close while render revision ${this.#pending.renderRevision} is pending`,
      );
    }
    if (this.#pendingPingNonce !== null) {
      throw sessionError(
        "invalidState",
        `cannot close while liveness nonce ${this.#pendingPingNonce} is pending`,
      );
    }
    const validatedReason = requireEnum(
      reason,
      ["normal", "requested", "protocolError", "hostShutdown"],
      "close reason",
      "invalidMessage",
    ) as TsxCloseReasonV1;
    const payload = message === undefined
      ? Object.freeze({ reason: validatedReason })
      : Object.freeze({
        reason: validatedReason,
        message: requireBoundedText(
          message,
          "close message",
          TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES,
          "invalidMessage",
        ),
      });
    const messageId = nextSafeInteger(this.#lastClientMessageId, "client message id");
    const close = Object.freeze({
      type: "close" as const,
      protocol: TSX_PROTOCOL_NAME,
      protocolVersion: TSX_PROTOCOL_VERSION_V1,
      sessionId: this.#sessionId,
      messageId,
      renderRevision: this.#committedRenderRevision,
      payload,
    });
    assertEncodedSize(close, this.#maximumFrameBytes, "frameTooLarge");

    this.#lastClientMessageId = messageId;
    this.#pendingClose = payload;
    this.#status = "closing";
    return close;
  }

  acceptClose(message: TsxHostCloseMessageV1): void {
    if (this.#status !== "closing" || this.#pendingClose === null) {
      throw sessionError("invalidState", "the client session has no pending close request");
    }
    try {
      const snapshot = this.#validateControlMessage(message, "close");
      const payload = requireRecord(snapshot.payload, "close payload", "invalidMessage");
      assertExactKeys(payload, ["reason"], ["message"], "close payload", "invalidMessage");
      const reason = requireEnum(
        payload.reason,
        ["normal", "requested", "protocolError", "hostShutdown"],
        "close reason",
        "invalidMessage",
      ) as TsxCloseReasonV1;
      const closeMessage = validateOptionalCloseMessage(payload.message);
      const pendingMessage = this.#pendingClose.message ?? null;
      if (reason !== this.#pendingClose.reason || closeMessage !== pendingMessage) {
        throw sessionError("invalidMessage", "host close acknowledgement does not match the request");
      }

      this.#lastHostMessageId = snapshot.messageId;
      this.#pendingClose = null;
      this.#status = "closed";
    } catch (cause) {
      this.#status = "failed";
      throw cause;
    }
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
    this.#pendingPingNonce = null;
    this.#pendingClose = null;
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
    expectedType: "committed" | "event" | "pong" | "close",
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
    const renderRevision = requireSafeInteger(
      record.renderRevision,
      `${expectedType} render revision`,
      expectedType === "committed" || expectedType === "event" ? 1 : 0,
      "invalidRevision",
    );
    if (
      (expectedType === "pong" || expectedType === "close") &&
      renderRevision !== this.#committedRenderRevision
    ) {
      throw sessionError(
        "invalidRevision",
        `${expectedType} render revision ${renderRevision} does not match committed revision ${this.#committedRenderRevision}`,
      );
    }
    assertEncodedSize(snapshot, this.#maximumFrameBytes, "frameTooLarge");
    return snapshot;
  }

  #validateControlMessage(
    message: unknown,
    expectedType: "pong" | "close",
  ): Extract<TsxHostMessageV1, { readonly type: typeof expectedType }> {
    return this.#snapshotHostMessage(message, expectedType) as Extract<
      TsxHostMessageV1,
      { readonly type: typeof expectedType }
    >;
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
