import assert from "node:assert/strict";
import test from "node:test";

import {
  A3sFrameError,
  A3sJsonFrameDecoderV1,
  encodeA3sJsonFrameV1,
} from "../src/index.ts";

test("framing emits a little-endian length prefix and decodes split consecutive messages", () => {
  const first = { type: "first", value: "hello" };
  const second = { type: "second", values: [1, true, null] };
  const firstFrame = encodeA3sJsonFrameV1(first, 1_024);
  const secondFrame = encodeA3sJsonFrameV1(second, 1_024);
  const firstPayloadBytes = new TextEncoder().encode(JSON.stringify(first)).byteLength;

  assert.equal(
    new DataView(firstFrame.buffer, firstFrame.byteOffset, 4).getUint32(0, true),
    firstPayloadBytes,
  );

  const stream = new Uint8Array(firstFrame.byteLength + secondFrame.byteLength);
  stream.set(firstFrame);
  stream.set(secondFrame, firstFrame.byteLength);
  const decoder = new A3sJsonFrameDecoderV1(1_024);
  assert.deepEqual(decoder.push(stream.subarray(0, 2)), []);
  assert.deepEqual(decoder.push(stream.subarray(2, firstFrame.byteLength + 3)), [first]);
  assert.deepEqual(decoder.push(stream.subarray(firstFrame.byteLength + 3)), [second]);
  decoder.finish();
});

test("framing rejects accessors and cycles without evaluating application code", () => {
  let getterCalls = 0;
  const accessor = {};
  Object.defineProperty(accessor, "value", {
    enumerable: true,
    get() {
      getterCalls += 1;
      return "unsafe";
    },
  });

  assert.throws(
    () => encodeA3sJsonFrameV1(accessor, 1_024),
    (error) => error instanceof A3sFrameError && error.code === "invalidValue",
  );
  assert.equal(getterCalls, 0);

  const accessorArray = [];
  Object.defineProperty(accessorArray, 0, {
    enumerable: true,
    get() {
      getterCalls += 1;
      return "unsafe";
    },
  });
  accessorArray.length = 1;
  assert.throws(
    () => encodeA3sJsonFrameV1(accessorArray, 1_024),
    (error) => error instanceof A3sFrameError && error.code === "invalidValue",
  );
  assert.equal(getterCalls, 0);

  const cyclic = {};
  cyclic.self = cyclic;
  assert.throws(
    () => encodeA3sJsonFrameV1(cyclic, 1_024),
    (error) => error instanceof A3sFrameError && error.code === "invalidValue",
  );
});

test("decoder poisons after invalid lengths or JSON and reports truncated streams", () => {
  const oversized = new Uint8Array(4);
  new DataView(oversized.buffer).setUint32(0, 1_025, true);
  const oversizedDecoder = new A3sJsonFrameDecoderV1(1_024);
  assert.throws(
    () => oversizedDecoder.push(oversized),
    (error) => error instanceof A3sFrameError && error.code === "frameTooLarge",
  );
  assert.equal(oversizedDecoder.poisoned, true);
  assert.throws(
    () => oversizedDecoder.push(new Uint8Array()),
    (error) => error instanceof A3sFrameError && error.code === "invalidState",
  );

  const invalidJson = new Uint8Array([1, 0, 0, 0, 0xff]);
  const jsonDecoder = new A3sJsonFrameDecoderV1(1_024);
  assert.throws(
    () => jsonDecoder.push(invalidJson),
    (error) => error instanceof A3sFrameError && error.code === "invalidJson",
  );

  const truncated = new A3sJsonFrameDecoderV1(1_024);
  truncated.push(new Uint8Array([5, 0, 0]));
  assert.throws(
    () => truncated.finish(),
    (error) => error instanceof A3sFrameError && error.code === "truncatedFrame",
  );
});

test("encoder fails before publishing a frame that exceeds its negotiated limit", () => {
  assert.throws(
    () => encodeA3sJsonFrameV1({ value: "x".repeat(2_048) }, 1_024),
    (error) => error instanceof A3sFrameError && error.code === "frameTooLarge",
  );
  assert.throws(
    () => new A3sJsonFrameDecoderV1(0),
    (error) => error instanceof A3sFrameError && error.code === "invalidLimit",
  );
});
