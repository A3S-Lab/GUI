import {
  TSX_PROTOCOL_NAME,
  TSX_PROTOCOL_VERSION_V1,
  type TsxHostMessageV1,
} from "./generated/protocol.ts";
import { clientSessionError } from "./client-session-error.ts";
import {
  assertEncodedSize,
  assertExactKeys,
  nextSafeInteger,
  requireRecord,
  requireSafeInteger,
  snapshotProtocolValue,
} from "./client-session-validation.ts";

export type TsxPostWelcomeHostMessageV1 = Exclude<
  TsxHostMessageV1,
  { readonly type: "welcome" }
>;

const MAXIMUM_PENDING_HOST_MESSAGES = 1_024;

/** Separates ordered wire receipt from potentially asynchronous semantic application. */
export class A3sClientHostSequenceV1 {
  readonly #sessionId: string;
  readonly #maximumFrameBytes: number;
  readonly #reserved = new Map<number, Readonly<TsxPostWelcomeHostMessageV1>>();
  readonly #receivedMessageIds = new WeakMap<object, number>();
  readonly #applied = new Set<number>();
  #lastReceivedMessageId = 1;
  #lastAppliedMessageId = 1;
  #receivedRenderRevision = 0;

  constructor(sessionId: string, maximumFrameBytes: number) {
    this.#sessionId = sessionId;
    this.#maximumFrameBytes = maximumFrameBytes;
  }

  get lastReceivedMessageId(): number {
    return this.#lastReceivedMessageId;
  }

  get lastAppliedMessageId(): number {
    return this.#lastAppliedMessageId;
  }

  receive(
    message: TsxPostWelcomeHostMessageV1,
  ): Readonly<TsxPostWelcomeHostMessageV1> {
    if (this.#reserved.size + this.#applied.size >= MAXIMUM_PENDING_HOST_MESSAGES) {
      throw clientSessionError(
        "invalidState",
        `client session cannot hold more than ${MAXIMUM_PENDING_HOST_MESSAGES} host messages ahead of semantic application`,
      );
    }
    const snapshot = snapshotProtocolValue(
      message,
      "host message",
      "invalidMessage",
    ) as TsxHostMessageV1;
    const record = requireRecord(snapshot, "host message", "invalidMessage");
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
      "host message",
      "invalidMessage",
    );
    if (
      !["committed", "event", "ping", "pong", "close", "fatal"].includes(
        String(record.type),
      )
    ) {
      throw clientSessionError("invalidMessage", "expected a post-welcome host message");
    }
    if (
      record.protocol !== TSX_PROTOCOL_NAME ||
      record.protocolVersion !== TSX_PROTOCOL_VERSION_V1
    ) {
      throw clientSessionError(
        "invalidMessage",
        `expected ${TSX_PROTOCOL_NAME} v${TSX_PROTOCOL_VERSION_V1}`,
      );
    }
    if (record.sessionId !== this.#sessionId) {
      throw clientSessionError(
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
    const expectedMessageId = nextSafeInteger(
      this.#lastReceivedMessageId,
      "host message id",
    );
    if (messageId !== expectedMessageId) {
      throw clientSessionError(
        "invalidMessageId",
        `host message id ${messageId} is invalid; expected ${expectedMessageId}`,
      );
    }
    const minimumRevision = record.type === "committed" || record.type === "event" ? 1 : 0;
    const renderRevision = requireSafeInteger(
      record.renderRevision,
      `${String(record.type)} render revision`,
      minimumRevision,
      "invalidRevision",
    );
    const nextReceivedRevision = record.type === "committed"
      ? nextSafeInteger(this.#receivedRenderRevision, "received render revision")
      : this.#receivedRenderRevision;
    if (renderRevision !== nextReceivedRevision) {
      throw clientSessionError(
        "invalidRevision",
        `${String(record.type)} render revision ${renderRevision} does not match received revision ${nextReceivedRevision}`,
      );
    }
    assertEncodedSize(snapshot, this.#maximumFrameBytes, "frameTooLarge");

    const received = snapshot as TsxPostWelcomeHostMessageV1;
    this.#reserved.set(messageId, received);
    this.#receivedMessageIds.set(received, messageId);
    this.#lastReceivedMessageId = messageId;
    this.#receivedRenderRevision = nextReceivedRevision;
    return received;
  }

  take<Type extends TsxPostWelcomeHostMessageV1["type"]>(
    message: Extract<TsxPostWelcomeHostMessageV1, { readonly type: Type }>,
    expectedType: Type,
  ): Readonly<Extract<TsxPostWelcomeHostMessageV1, { readonly type: Type }>> {
    const receivedMessageId = this.#receivedMessageIds.get(message);
    let received = receivedMessageId === undefined
      ? this.receive(message)
      : this.#reserved.get(receivedMessageId);
    if (received === undefined) {
      throw clientSessionError(
        "invalidMessageId",
        `host message ${receivedMessageId} was already applied`,
      );
    }
    if (received.type !== expectedType) {
      throw clientSessionError("invalidMessage", `expected a ${expectedType} host message`);
    }
    return received as Readonly<Extract<
      TsxPostWelcomeHostMessageV1,
      { readonly type: Type }
    >>;
  }

  apply(messageId: number): void {
    if (messageId <= this.#lastAppliedMessageId || this.#applied.has(messageId)) {
      throw clientSessionError("invalidMessageId", `host message ${messageId} was already applied`);
    }
    if (!this.#reserved.has(messageId)) {
      throw clientSessionError(
        "invalidState",
        `host message ${messageId} is not reserved for semantic application`,
      );
    }
    this.#reserved.delete(messageId);
    this.#applied.add(messageId);
    while (this.#applied.delete(this.#lastAppliedMessageId + 1)) {
      this.#lastAppliedMessageId += 1;
    }
  }
}
