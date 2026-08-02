import { TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES } from "./generated/protocol.ts";
import { encodeA3sProtocolJsonPayloadV1 } from "./protocol-json.ts";

export type A3sFrameErrorCodeV1 =
  | "emptyFrame"
  | "frameTooLarge"
  | "invalidChunk"
  | "invalidJson"
  | "invalidLimit"
  | "invalidState"
  | "invalidValue"
  | "truncatedFrame";

export class A3sFrameError extends Error {
  readonly code: A3sFrameErrorCodeV1;

  constructor(code: A3sFrameErrorCodeV1, message: string, cause?: unknown) {
    super(message, cause === undefined ? undefined : { cause });
    this.name = "A3sFrameError";
    this.code = code;
  }
}

/** Encodes one strict JSON value behind a four-byte little-endian length. */
export function encodeA3sJsonFrameV1(
  value: unknown,
  maximumPayloadBytes: number = TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
): Uint8Array {
  const limit = validateFrameLimitV1(maximumPayloadBytes);
  const payload = encodeA3sProtocolJsonPayloadV1(
    value,
    "protocol message",
    (message) => frameError("invalidValue", message),
  );
  validatePayloadLengthV1(payload.byteLength, limit);

  const frame = new Uint8Array(4 + payload.byteLength);
  new DataView(frame.buffer).setUint32(0, payload.byteLength, true);
  frame.set(payload, 4);
  return frame;
}

/** Incrementally decodes consecutive TSX protocol frames from arbitrary chunks. */
export class A3sJsonFrameDecoderV1 {
  readonly #maximumPayloadBytes: number;
  readonly #header = new Uint8Array(4);
  #headerBytes = 0;
  #payload: Uint8Array | null = null;
  #payloadBytes = 0;
  #poisoned = false;

  constructor(maximumPayloadBytes: number = TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES) {
    this.#maximumPayloadBytes = validateFrameLimitV1(maximumPayloadBytes);
  }

  get maximumPayloadBytes(): number {
    return this.#maximumPayloadBytes;
  }

  get poisoned(): boolean {
    return this.#poisoned;
  }

  push(chunk: Uint8Array): readonly unknown[] {
    if (this.#poisoned) {
      throw frameError(
        "invalidState",
        "TSX protocol frame decoder is poisoned after an earlier failure",
      );
    }
    if (!(chunk instanceof Uint8Array)) {
      return this.#fail("invalidChunk", "TSX protocol chunks must be Uint8Array values");
    }

    const messages: unknown[] = [];
    let offset = 0;
    while (offset < chunk.byteLength) {
      if (this.#payload === null) {
        const count = Math.min(4 - this.#headerBytes, chunk.byteLength - offset);
        this.#header.set(chunk.subarray(offset, offset + count), this.#headerBytes);
        this.#headerBytes += count;
        offset += count;
        if (this.#headerBytes !== 4) {
          continue;
        }

        const length = new DataView(
          this.#header.buffer,
          this.#header.byteOffset,
          4,
        ).getUint32(0, true);
        try {
          validatePayloadLengthV1(length, this.#maximumPayloadBytes);
        } catch (error) {
          this.#poisoned = true;
          throw error;
        }
        this.#payload = new Uint8Array(length);
        this.#payloadBytes = 0;
      }

      const count = Math.min(
        this.#payload.byteLength - this.#payloadBytes,
        chunk.byteLength - offset,
      );
      this.#payload.set(chunk.subarray(offset, offset + count), this.#payloadBytes);
      this.#payloadBytes += count;
      offset += count;
      if (this.#payloadBytes !== this.#payload.byteLength) {
        continue;
      }

      const payload = this.#payload;
      this.#payload = null;
      this.#payloadBytes = 0;
      this.#headerBytes = 0;
      try {
        const json = new TextDecoder("utf-8", { fatal: true }).decode(payload);
        messages.push(JSON.parse(json));
      } catch (cause) {
        return this.#fail(
          "invalidJson",
          `TSX protocol payload is not valid UTF-8 JSON: ${String(cause)}`,
          cause,
        );
      }
    }

    return Object.freeze(messages);
  }

  finish(): void {
    if (this.#poisoned) {
      throw frameError(
        "invalidState",
        "TSX protocol frame decoder is poisoned after an earlier failure",
      );
    }
    if (this.#payload !== null) {
      throw frameError(
        "truncatedFrame",
        `TSX protocol stream ended after ${this.#payloadBytes} of ${this.#payload.byteLength} payload bytes`,
      );
    }
    if (this.#headerBytes !== 0) {
      throw frameError(
        "truncatedFrame",
        `TSX protocol stream ended after ${this.#headerBytes} of 4 length-prefix bytes`,
      );
    }
  }

  #fail(
    code: A3sFrameErrorCodeV1,
    message: string,
    cause?: unknown,
  ): never {
    this.#poisoned = true;
    throw frameError(code, message, cause);
  }
}

function validateFrameLimitV1(value: number): number {
  if (
    !Number.isSafeInteger(value) ||
    value < 1 ||
    value > TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES
  ) {
    throw frameError(
      "invalidLimit",
      `TSX protocol frame limit must be an integer from 1 through ${TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES}`,
    );
  }
  return value;
}

function validatePayloadLengthV1(length: number, maximumPayloadBytes: number): void {
  if (length === 0) {
    throw frameError("emptyFrame", "TSX protocol frames cannot have an empty payload");
  }
  if (length > maximumPayloadBytes) {
    throw frameError(
      "frameTooLarge",
      `TSX protocol frame declares ${length} payload bytes, exceeding the negotiated ${maximumPayloadBytes}-byte limit`,
    );
  }
}

function frameError(
  code: A3sFrameErrorCodeV1,
  message: string,
  cause?: unknown,
): A3sFrameError {
  return new A3sFrameError(code, message, cause);
}
