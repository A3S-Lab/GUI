import assert from "node:assert/strict";
import test from "node:test";

import {
  A3sJsonFrameDecoderV1,
  A3sTransportError,
  Text,
  compileFrameV1,
  connectA3sFramedClientV1,
  encodeA3sJsonFrameV1,
} from "../src/index.ts";
import { jsx } from "../src/jsx-runtime.ts";

test("framed connection negotiates once and exchanges bounded ordered messages", async () => {
  const transport = new TestByteTransport();
  transport.push(encodeA3sJsonFrameV1(welcome(), 4_096));

  const connection = await connectA3sFramedClientV1(transport, handshakeOptions());
  assert.equal(connection.state.status, "open");
  assert.equal(connection.session.state.maximumFrameBytes, 2_048);
  assert.deepEqual(decodeSingle(transport.writes[0]), connection.handshake.hello);

  const compiled = compileFrameV1("connection", jsx(Text, { children: "hello" }));
  const render = connection.session.createRender(1, compiled.frame);
  await connection.writeClientMessage(render);
  assert.deepEqual(decodeSingle(transport.writes[1]), render);

  const commit = committed(render);
  const framedCommit = encodeA3sJsonFrameV1(commit, 2_048);
  transport.push(framedCommit.subarray(0, 3));
  transport.push(framedCommit.subarray(3));
  const received = await connection.readHostMessage();
  assert.deepEqual(received, commit);
  assert.equal(Object.isFrozen(received), true);

  await connection.close();
  assert.equal(connection.state.status, "closed");
  assert.equal(transport.closed, true);
});

test("framed connection rejects a non-welcome first message and closes transport", async () => {
  const transport = new TestByteTransport();
  transport.push(encodeA3sJsonFrameV1({ ...welcome(), type: "committed" }, 4_096));

  await assert.rejects(
    connectA3sFramedClientV1(transport, handshakeOptions()),
    (error) => error instanceof A3sTransportError && error.code === "invalidHostMessage",
  );
  assert.equal(transport.closed, true);
});

test("framed connection rejects concurrent reads without consuming the pending read", async () => {
  const transport = new TestByteTransport();
  transport.push(encodeA3sJsonFrameV1(welcome(), 4_096));
  const connection = await connectA3sFramedClientV1(transport, handshakeOptions());

  const pending = connection.readHostMessage();
  await assert.rejects(
    connection.readHostMessage(),
    (error) => error instanceof A3sTransportError && error.code === "concurrentRead",
  );
  const close = {
    type: "close",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "stream-session",
    messageId: 2,
    renderRevision: 0,
    payload: { reason: "hostShutdown" },
  };
  transport.push(encodeA3sJsonFrameV1(close, 2_048));
  assert.deepEqual(await pending, close);
  await connection.close();
});

test("framed connection narrows a partially received post-welcome frame", async () => {
  const transport = new TestByteTransport();
  const welcomeFrame = encodeA3sJsonFrameV1(welcome(), 4_096);
  const oversizedHeader = new Uint8Array(4);
  new DataView(oversizedHeader.buffer).setUint32(0, 3_000, true);
  const chunk = new Uint8Array(welcomeFrame.byteLength + oversizedHeader.byteLength);
  chunk.set(welcomeFrame);
  chunk.set(oversizedHeader, welcomeFrame.byteLength);
  transport.push(chunk);

  await assert.rejects(
    connectA3sFramedClientV1(transport, handshakeOptions()),
    (error) => error instanceof A3sTransportError && error.code === "streamFailed",
  );
  assert.equal(transport.closed, true);
});

function handshakeOptions() {
  return {
    sdkVersion: "0.0.0-development",
    sessionId: "stream-session",
    requestedRenderer: "software",
    maximumFrameBytes: 4_096,
  };
}

function welcome() {
  return {
    type: "welcome",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "stream-session",
    messageId: 1,
    renderRevision: 0,
    payload: {
      selectedProtocolVersion: 1,
      hostVersion: "0.1.0",
      hostBuildId: "stream-test",
      platform: "headless",
      renderer: "software",
      limits: { maximumFrameBytes: 2_048, maximumInFlightRenders: 1 },
      capabilities: ["headlessRendering", "selfDrawnRendering"],
    },
  };
}

function committed(render) {
  return {
    type: "committed",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: render.sessionId,
    messageId: 2,
    renderRevision: render.renderRevision,
    payload: {
      frameId: render.payload.frameId,
      hostRevision: 1,
      rootId: "root",
      layoutFingerprint: "0000000000000000",
      sceneFingerprint: "0000000000000000",
    },
  };
}

function decodeSingle(frame) {
  const decoder = new A3sJsonFrameDecoderV1(4_096);
  const messages = decoder.push(frame);
  decoder.finish();
  assert.equal(messages.length, 1);
  return messages[0];
}

class TestByteTransport {
  writes = [];
  closed = false;
  #chunks = [];
  #waiters = [];
  #ended = false;

  incoming = {
    [Symbol.asyncIterator]: () => this,
  };

  async write(chunk) {
    this.writes.push(Uint8Array.from(chunk));
  }

  async close() {
    this.closed = true;
    this.finish();
  }

  push(chunk) {
    if (this.#ended) {
      throw new Error("test transport already ended");
    }
    const result = { done: false, value: Uint8Array.from(chunk) };
    const waiter = this.#waiters.shift();
    if (waiter === undefined) {
      this.#chunks.push(result);
    } else {
      waiter(result);
    }
  }

  finish() {
    this.#ended = true;
    for (const waiter of this.#waiters.splice(0)) {
      waiter({ done: true, value: undefined });
    }
  }

  next() {
    const result = this.#chunks.shift();
    if (result !== undefined) {
      return Promise.resolve(result);
    }
    if (this.#ended) {
      return Promise.resolve({ done: true, value: undefined });
    }
    return new Promise((resolve) => this.#waiters.push(resolve));
  }
}
