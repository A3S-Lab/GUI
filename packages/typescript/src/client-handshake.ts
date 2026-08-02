import { A3sClientSessionV1, type TsxWelcomeMessageV1 } from "./client-session.ts";
import { A3sFrameError, encodeA3sJsonFrameV1 } from "./framing.ts";
import {
  TSX_PROTOCOL_NAME,
  TSX_PROTOCOL_VERSION_V1,
  TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
  TSX_PROTOCOL_V1_MAX_SESSION_ID_BYTES,
  TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
  type TsxClientMessageV1,
  type TsxDebugCapabilityV1,
  type TsxRendererV1,
} from "./generated/protocol.ts";
import { snapshotA3sProtocolJsonV1 } from "./protocol-json.ts";

export type TsxHelloMessageV1 = Extract<
  TsxClientMessageV1,
  { readonly type: "hello" }
>;

export type A3sClientHandshakeStatusV1 =
  | "awaitingWelcome"
  | "negotiated"
  | "failed"
  | "closed";

export type A3sClientHandshakeErrorCodeV1 =
  | "frameTooLarge"
  | "invalidOptions"
  | "invalidState"
  | "invalidWelcome";

export class A3sClientHandshakeError extends Error {
  readonly code: A3sClientHandshakeErrorCodeV1;

  constructor(
    code: A3sClientHandshakeErrorCodeV1,
    message: string,
    cause?: unknown,
  ) {
    super(message, cause === undefined ? undefined : { cause });
    this.name = "A3sClientHandshakeError";
    this.code = code;
  }
}

export interface A3sClientHandshakeOptionsV1 {
  readonly sdkVersion: string;
  readonly sessionId: string;
  readonly requestedRenderer?: TsxRendererV1;
  readonly maximumFrameBytes?: number;
  readonly debugCapabilities?: readonly TsxDebugCapabilityV1[];
}

export interface A3sClientHandshakeStateV1 {
  readonly status: A3sClientHandshakeStatusV1;
  readonly sessionId: string;
  readonly requestedRenderer: TsxRendererV1;
  readonly requestedMaximumFrameBytes: number;
  readonly negotiatedMaximumFrameBytes: number | null;
}

/** Owns the client half of the one-message TSX protocol negotiation. */
export class A3sClientHandshakeV1 {
  readonly #hello: Readonly<TsxHelloMessageV1>;
  readonly #sessionId: string;
  readonly #requestedRenderer: TsxRendererV1;
  readonly #requestedMaximumFrameBytes: number;
  readonly #requestedDebugCapabilities: ReadonlySet<TsxDebugCapabilityV1>;
  #status: A3sClientHandshakeStatusV1 = "awaitingWelcome";
  #session: A3sClientSessionV1 | null = null;

  constructor(options: A3sClientHandshakeOptionsV1) {
    const snapshot = snapshotA3sProtocolJsonV1(
      options,
      "client handshake options",
      (message) => handshakeError("invalidOptions", message),
    );
    const record = requireRecord(snapshot, "client handshake options", "invalidOptions");
    assertExactKeys(
      record,
      ["sdkVersion", "sessionId"],
      ["requestedRenderer", "maximumFrameBytes", "debugCapabilities"],
      "client handshake options",
      "invalidOptions",
    );

    const sdkVersion = requireBoundedText(
      record.sdkVersion,
      "SDK version",
      TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
      "invalidOptions",
    );
    const sessionId = requireBoundedText(
      record.sessionId,
      "session id",
      TSX_PROTOCOL_V1_MAX_SESSION_ID_BYTES,
      "invalidOptions",
    );
    const requestedRenderer = requireEnum(
      record.requestedRenderer ?? "auto",
      ["auto", "software", "gpu"],
      "requested renderer",
      "invalidOptions",
    ) as TsxRendererV1;
    const maximumFrameBytes = requireSafeInteger(
      record.maximumFrameBytes ?? TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
      "maximum frame bytes",
      1,
      TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
      "invalidOptions",
    );
    const debugCapabilities = validateDebugCapabilities(record.debugCapabilities);

    const frozenDebugCapabilities = Object.freeze([...debugCapabilities]) as unknown as
      TsxDebugCapabilityV1[];
    const payload: TsxHelloMessageV1["payload"] = {
      sdkVersion,
      minimumProtocolVersion: TSX_PROTOCOL_VERSION_V1,
      maximumProtocolVersion: TSX_PROTOCOL_VERSION_V1,
      requestedRenderer,
      maximumFrameBytes,
      ...(frozenDebugCapabilities.length === 0
        ? {}
        : { debugCapabilities: frozenDebugCapabilities }),
    };
    this.#hello = Object.freeze({
      type: "hello",
      protocol: TSX_PROTOCOL_NAME,
      protocolVersion: TSX_PROTOCOL_VERSION_V1,
      sessionId,
      messageId: 1,
      renderRevision: 0,
      payload: Object.freeze(payload),
    });
    try {
      encodeA3sJsonFrameV1(this.#hello, maximumFrameBytes);
    } catch (cause) {
      if (cause instanceof A3sFrameError && cause.code === "frameTooLarge") {
        throw handshakeError(
          "frameTooLarge",
          "client hello does not fit its requested frame-byte limit",
          cause,
        );
      }
      throw cause;
    }

    this.#sessionId = sessionId;
    this.#requestedRenderer = requestedRenderer;
    this.#requestedMaximumFrameBytes = maximumFrameBytes;
    this.#requestedDebugCapabilities = new Set(debugCapabilities);
  }

  get hello(): Readonly<TsxHelloMessageV1> {
    return this.#hello;
  }

  get session(): A3sClientSessionV1 | null {
    return this.#session;
  }

  get state(): Readonly<A3sClientHandshakeStateV1> {
    return Object.freeze({
      status: this.#status,
      sessionId: this.#sessionId,
      requestedRenderer: this.#requestedRenderer,
      requestedMaximumFrameBytes: this.#requestedMaximumFrameBytes,
      negotiatedMaximumFrameBytes: this.#session?.state.maximumFrameBytes ?? null,
    });
  }

  acceptWelcome(welcome: TsxWelcomeMessageV1): A3sClientSessionV1 {
    if (this.#status !== "awaitingWelcome") {
      throw handshakeError(
        "invalidState",
        `cannot accept welcome while the client handshake is ${this.#status}`,
      );
    }

    try {
      const session = new A3sClientSessionV1(welcome);
      if (session.state.sessionId !== this.#sessionId) {
        throw handshakeError(
          "invalidWelcome",
          `welcome session ${JSON.stringify(session.state.sessionId)} does not match hello session ${JSON.stringify(this.#sessionId)}`,
        );
      }
      if (session.state.maximumFrameBytes > this.#requestedMaximumFrameBytes) {
        throw handshakeError(
          "invalidWelcome",
          "welcome expanded the client maximum frame-byte limit",
        );
      }
      if (
        this.#requestedRenderer !== "auto" &&
        session.welcome.payload.renderer !== this.#requestedRenderer
      ) {
        throw handshakeError(
          "invalidWelcome",
          `welcome selected renderer ${JSON.stringify(session.welcome.payload.renderer)} instead of requested ${JSON.stringify(this.#requestedRenderer)}`,
        );
      }
      for (const capability of session.welcome.payload.debugCapabilities ?? []) {
        if (!this.#requestedDebugCapabilities.has(capability)) {
          throw handshakeError(
            "invalidWelcome",
            `welcome selected unrequested debug capability ${JSON.stringify(capability)}`,
          );
        }
      }

      this.#session = session;
      this.#status = "negotiated";
      return session;
    } catch (cause) {
      this.#status = "failed";
      if (cause instanceof A3sClientHandshakeError) {
        throw cause;
      }
      throw handshakeError("invalidWelcome", "host welcome was rejected", cause);
    }
  }

  close(): void {
    this.#session?.close();
    this.#status = "closed";
  }
}

function validateDebugCapabilities(value: unknown): TsxDebugCapabilityV1[] {
  if (value === undefined) {
    return [];
  }
  if (!Array.isArray(value)) {
    throw handshakeError("invalidOptions", "debug capabilities must be an array");
  }
  const allowed = ["protocolTrace", "structuredDiagnostics", "inspector"] as const;
  const seen = new Set<TsxDebugCapabilityV1>();
  const capabilities: TsxDebugCapabilityV1[] = [];
  for (const entry of value) {
    const capability = requireEnum(
      entry,
      allowed,
      "debug capability",
      "invalidOptions",
    ) as TsxDebugCapabilityV1;
    if (seen.has(capability)) {
      throw handshakeError("invalidOptions", "debug capabilities must be unique");
    }
    seen.add(capability);
    capabilities.push(capability);
  }
  return capabilities;
}

function requireBoundedText(
  value: unknown,
  name: string,
  maximumBytes: number,
  code: A3sClientHandshakeErrorCodeV1,
): string {
  if (
    typeof value !== "string" ||
    value.trim().length === 0 ||
    new TextEncoder().encode(value).byteLength > maximumBytes ||
    /[\u0000-\u001f\u007f-\u009f]/u.test(value)
  ) {
    throw handshakeError(code, `${name} must be non-empty bounded text`);
  }
  return value;
}

function requireEnum(
  value: unknown,
  allowed: readonly string[],
  name: string,
  code: A3sClientHandshakeErrorCodeV1,
): string {
  if (typeof value !== "string" || !allowed.includes(value)) {
    throw handshakeError(code, `${name} is invalid`);
  }
  return value;
}

function requireSafeInteger(
  value: unknown,
  name: string,
  minimum: number,
  maximum: number,
  code: A3sClientHandshakeErrorCodeV1,
): number {
  if (
    typeof value !== "number" ||
    !Number.isSafeInteger(value) ||
    value < minimum ||
    value > maximum
  ) {
    throw handshakeError(code, `${name} must be an integer from ${minimum} through ${maximum}`);
  }
  return value;
}

function assertExactKeys(
  record: Readonly<Record<string, unknown>>,
  required: readonly string[],
  optional: readonly string[],
  name: string,
  code: A3sClientHandshakeErrorCodeV1,
): void {
  const allowed = new Set([...required, ...optional]);
  for (const key of required) {
    if (!Object.hasOwn(record, key)) {
      throw handshakeError(code, `${name} is missing field ${JSON.stringify(key)}`);
    }
  }
  for (const key of Object.keys(record)) {
    if (!allowed.has(key)) {
      throw handshakeError(code, `${name} contains unknown field ${JSON.stringify(key)}`);
    }
  }
}

function requireRecord(
  value: unknown,
  name: string,
  code: A3sClientHandshakeErrorCodeV1,
): Readonly<Record<string, unknown>> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw handshakeError(code, `${name} must be an object`);
  }
  return value as Readonly<Record<string, unknown>>;
}

function handshakeError(
  code: A3sClientHandshakeErrorCodeV1,
  message: string,
  cause?: unknown,
): A3sClientHandshakeError {
  return new A3sClientHandshakeError(code, message, cause);
}
