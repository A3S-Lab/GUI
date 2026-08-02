import {
  A3sClientHandshakeV1,
  type A3sClientHandshakeOptionsV1,
} from "./client-handshake.ts";
import { A3sClientSessionV1, type TsxWelcomeMessageV1 } from "./client-session.ts";
import {
  A3sFrameError,
  A3sJsonFrameDecoderV1,
  encodeA3sJsonFrameV1,
} from "./framing.ts";
import {
  TSX_PROTOCOL_NAME,
  TSX_PROTOCOL_VERSION_V1,
  type TsxClientMessageV1,
  type TsxHostMessageV1,
} from "./generated/protocol.ts";
import { snapshotA3sProtocolJsonV1 } from "./protocol-json.ts";

export interface A3sByteTransportV1 {
  readonly incoming: AsyncIterable<Uint8Array>;
  write(chunk: Uint8Array): void | PromiseLike<void>;
  close(): void | PromiseLike<void>;
}

export type A3sTransportErrorCodeV1 =
  | "concurrentRead"
  | "endOfStream"
  | "invalidHostMessage"
  | "invalidOptions"
  | "invalidState"
  | "processExited"
  | "processSpawnFailed"
  | "shutdownFailed"
  | "streamFailed"
  | "writeFailed";

export class A3sTransportError extends Error {
  readonly code: A3sTransportErrorCodeV1;

  constructor(code: A3sTransportErrorCodeV1, message: string, cause?: unknown) {
    super(message, cause === undefined ? undefined : { cause });
    this.name = "A3sTransportError";
    this.code = code;
  }
}

export type A3sFramedClientConnectionStatusV1 = "open" | "failed" | "closed";

export interface A3sFramedClientConnectionStateV1 {
  readonly status: A3sFramedClientConnectionStatusV1;
  readonly sessionId: string;
  readonly maximumFrameBytes: number;
  readonly bufferedHostMessages: number;
}

/** A negotiated, single-reader TSX message stream over arbitrary byte I/O. */
export class A3sFramedClientConnectionV1 {
  readonly #transport: A3sByteTransportV1;
  readonly #reader: FramedHostReaderV1;
  readonly #handshake: A3sClientHandshakeV1;
  readonly #session: A3sClientSessionV1;
  #status: A3sFramedClientConnectionStatusV1 = "open";
  #reading = false;
  #writeTail: Promise<void> = Promise.resolve();
  #closePromise: Promise<void> | null = null;

  constructor(
    transport: A3sByteTransportV1,
    reader: FramedHostReaderV1,
    handshake: A3sClientHandshakeV1,
    session: A3sClientSessionV1,
  ) {
    this.#transport = transport;
    this.#reader = reader;
    this.#handshake = handshake;
    this.#session = session;
  }

  get handshake(): A3sClientHandshakeV1 {
    return this.#handshake;
  }

  get session(): A3sClientSessionV1 {
    return this.#session;
  }

  get welcome(): Readonly<TsxWelcomeMessageV1> {
    return this.#session.welcome;
  }

  get state(): Readonly<A3sFramedClientConnectionStateV1> {
    return Object.freeze({
      status: this.#status,
      sessionId: this.#session.state.sessionId,
      maximumFrameBytes: this.#session.state.maximumFrameBytes,
      bufferedHostMessages: this.#reader.bufferedMessageCount,
    });
  }

  writeClientMessage(message: TsxClientMessageV1): Promise<void> {
    this.#assertOpen("write a client message");
    const snapshot = snapshotA3sProtocolJsonV1(
      message,
      "client message",
      (detail) => transportError("invalidState", detail),
    ) as TsxClientMessageV1;
    const record = requireRecord(snapshot, "client message", "invalidState");
    if (
      record.type === "hello" ||
      record.protocol !== TSX_PROTOCOL_NAME ||
      record.protocolVersion !== TSX_PROTOCOL_VERSION_V1 ||
      record.sessionId !== this.#session.state.sessionId
    ) {
      throw transportError(
        "invalidState",
        "negotiated transport received a client message with invalid protocol identity",
      );
    }
    const frame = encodeA3sJsonFrameV1(
      snapshot,
      this.#session.state.maximumFrameBytes,
    );
    const write = this.#writeTail.then(async () => {
      this.#assertOpen("write a client message");
      try {
        await this.#transport.write(frame);
      } catch (cause) {
        this.#status = "failed";
        if (cause instanceof A3sTransportError) {
          throw cause;
        }
        throw transportError("writeFailed", "could not write TSX client message", cause);
      }
    });
    this.#writeTail = write.then(
      () => undefined,
      () => undefined,
    );
    return write;
  }

  async readHostMessage(): Promise<Readonly<TsxHostMessageV1> | null> {
    this.#assertOpen("read a host message");
    if (this.#reading) {
      throw transportError(
        "concurrentRead",
        "TSX framed connections permit only one pending host read",
      );
    }
    this.#reading = true;
    try {
      const value = await this.#reader.read();
      if (value === null) {
        this.#handshake.close();
        this.#status = "closed";
        return null;
      }
      const message = snapshotHostMessage(value, this.#session.state.sessionId);
      if (message.type === "welcome") {
        throw transportError(
          "invalidHostMessage",
          "TSX host emitted a second welcome after negotiation",
        );
      }
      return message;
    } catch (cause) {
      this.#status = "failed";
      if (cause instanceof A3sTransportError) {
        throw cause;
      }
      throw transportError("streamFailed", "could not read TSX host message", cause);
    } finally {
      this.#reading = false;
    }
  }

  close(): Promise<void> {
    if (this.#closePromise !== null) {
      return this.#closePromise;
    }
    this.#closePromise = this.#close();
    return this.#closePromise;
  }

  async #close(): Promise<void> {
    try {
      await this.#writeTail;
      await this.#transport.close();
    } catch (cause) {
      this.#status = "failed";
      if (cause instanceof A3sTransportError) {
        throw cause;
      }
      throw transportError("shutdownFailed", "could not close TSX byte transport", cause);
    } finally {
      this.#handshake.close();
      this.#status = "closed";
    }
  }

  #assertOpen(operation: string): void {
    if (this.#status !== "open") {
      throw transportError(
        "invalidState",
        `cannot ${operation} while the framed connection is ${this.#status}`,
      );
    }
  }
}

export async function connectA3sFramedClientV1(
  transport: A3sByteTransportV1,
  options: A3sClientHandshakeOptionsV1,
): Promise<A3sFramedClientConnectionV1> {
  validateByteTransport(transport);
  const handshake = new A3sClientHandshakeV1(options);
  const decoder = new A3sJsonFrameDecoderV1(
    handshake.state.requestedMaximumFrameBytes,
  );
  const reader = new FramedHostReaderV1(transport.incoming, decoder);

  try {
    try {
      await transport.write(
        encodeA3sJsonFrameV1(
          handshake.hello,
          handshake.state.requestedMaximumFrameBytes,
        ),
      );
    } catch (cause) {
      if (cause instanceof A3sTransportError) {
        throw cause;
      }
      throw transportError("writeFailed", "could not write TSX client hello", cause);
    }
    const first = await reader.read();
    if (first === null) {
      throw transportError(
        "endOfStream",
        "TSX host stream ended before welcome",
      );
    }
    const record = requireRecord(first, "first host message", "invalidHostMessage");
    if (record.type !== "welcome") {
      throw transportError(
        "invalidHostMessage",
        "the first TSX host message must be welcome",
      );
    }
    if (reader.bufferedMessageCount !== 0) {
      throw transportError(
        "invalidHostMessage",
        "TSX host emitted application messages before welcome was accepted",
      );
    }
    const session = handshake.acceptWelcome(first as TsxWelcomeMessageV1);
    decoder.narrowMaximumPayloadBytes(session.state.maximumFrameBytes);
    return new A3sFramedClientConnectionV1(transport, reader, handshake, session);
  } catch (cause) {
    handshake.close();
    try {
      await transport.close();
    } catch {
      // Preserve the negotiation failure as the primary error.
    }
    if (cause instanceof A3sTransportError) {
      throw cause;
    }
    if (cause instanceof A3sFrameError) {
      throw transportError("streamFailed", "TSX negotiation frame failed", cause);
    }
    throw transportError("invalidHostMessage", "TSX host negotiation failed", cause);
  }
}

class FramedHostReaderV1 {
  readonly #iterator: AsyncIterator<Uint8Array>;
  readonly #decoder: A3sJsonFrameDecoderV1;
  readonly #buffer: unknown[] = [];
  #ended = false;

  constructor(incoming: AsyncIterable<Uint8Array>, decoder: A3sJsonFrameDecoderV1) {
    this.#iterator = incoming[Symbol.asyncIterator]();
    this.#decoder = decoder;
  }

  get bufferedMessageCount(): number {
    return this.#buffer.length;
  }

  async read(): Promise<unknown | null> {
    const buffered = this.#buffer.shift();
    if (buffered !== undefined) {
      return buffered;
    }
    if (this.#ended) {
      return null;
    }

    while (true) {
      let result: IteratorResult<Uint8Array>;
      try {
        result = await this.#iterator.next();
      } catch (cause) {
        if (cause instanceof A3sTransportError) {
          throw cause;
        }
        throw transportError("streamFailed", "TSX byte stream read failed", cause);
      }
      if (result.done) {
        this.#ended = true;
        try {
          this.#decoder.finish();
        } catch (cause) {
          throw transportError("streamFailed", "TSX byte stream ended mid-frame", cause);
        }
        return null;
      }
      if (!(result.value instanceof Uint8Array)) {
        throw transportError(
          "streamFailed",
          "TSX byte transport yielded a non-Uint8Array chunk",
        );
      }
      try {
        this.#buffer.push(...this.#decoder.push(result.value));
      } catch (cause) {
        throw transportError("streamFailed", "TSX byte stream frame failed", cause);
      }
      const message = this.#buffer.shift();
      if (message !== undefined) {
        return message;
      }
    }
  }
}

function snapshotHostMessage(value: unknown, sessionId: string): TsxHostMessageV1 {
  const snapshot = snapshotA3sProtocolJsonV1(
    value,
    "host message",
    (message) => transportError("invalidHostMessage", message),
  );
  const record = requireRecord(snapshot, "host message", "invalidHostMessage");
  if (
    !["welcome", "committed", "event", "ping", "pong", "close", "fatal"].includes(
      String(record.type),
    ) ||
    record.protocol !== TSX_PROTOCOL_NAME ||
    record.protocolVersion !== TSX_PROTOCOL_VERSION_V1 ||
    record.sessionId !== sessionId
  ) {
    throw transportError(
      "invalidHostMessage",
      "TSX host message has invalid type or protocol identity",
    );
  }
  return snapshot as TsxHostMessageV1;
}

function validateByteTransport(value: unknown): asserts value is A3sByteTransportV1 {
  const record = requireRecord(value, "byte transport", "invalidOptions");
  if (
    typeof record.write !== "function" ||
    typeof record.close !== "function" ||
    typeof record.incoming !== "object" ||
    record.incoming === null ||
    typeof (record.incoming as AsyncIterable<Uint8Array>)[Symbol.asyncIterator] !== "function"
  ) {
    throw transportError(
      "invalidOptions",
      "byte transport must expose incoming, write, and close boundaries",
    );
  }
}

function requireRecord(
  value: unknown,
  name: string,
  code: A3sTransportErrorCodeV1,
): Readonly<Record<string | symbol, unknown>> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw transportError(code, `${name} must be an object`);
  }
  return value as Readonly<Record<string | symbol, unknown>>;
}

export function transportError(
  code: A3sTransportErrorCodeV1,
  message: string,
  cause?: unknown,
): A3sTransportError {
  return new A3sTransportError(code, message, cause);
}
