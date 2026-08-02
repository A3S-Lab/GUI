import assert from "node:assert/strict";
import test from "node:test";

import {
  A3sFramedApplicationHostV1,
  A3sFramedHostError,
  A3sJsonFrameDecoderV1,
  Button,
  Text,
  View,
  compileFrameV1,
  connectA3sNodeApplicationHostV1,
  connectA3sFramedClientV1,
  createApp,
  encodeA3sJsonFrameV1,
  useState,
} from "../src/index.ts";
import { jsx, jsxs } from "../src/jsx-runtime.ts";

test("framed application host drives createApp through one shared client session", async () => {
  const transport = new TestByteTransport();
  const connection = await transport.connect();
  const host = new A3sFramedApplicationHostV1(connection);
  const app = createApp(
    () => jsx(Text, { children: "connected" }),
    { frameId: "connected-app", host },
  );

  assert.equal(host.session, connection.session);
  await app.start();
  await app.rerender();

  assert.deepEqual(
    transport.writes.slice(1).map((message) => [message.messageId, message.renderRevision]),
    [[2, 1], [3, 2]],
  );
  assert.equal(app.state.committedRenders, 2);
  assert.equal(app.state.session.lastClientMessageId, 3);
  assert.equal(app.state.session.lastHostMessageId, 3);
  assert.equal(host.state.pendingRenderRevision, null);

  await app.shutdown();
  assert.equal(host.state.status, "closed");
  assert.equal(transport.closed, true);
  assert.deepEqual(
    transport.writes.slice(-1).map((message) => [message.type, message.messageId]),
    [["close", 4]],
  );
  assert.equal(app.state.session.lastHostMessageId, 4);
});

test("framed application host performs client liveness before graceful close", async () => {
  const transport = new TestByteTransport();
  const connection = await transport.connect();
  const host = new A3sFramedApplicationHostV1(connection);

  await host.ping(73);
  assert.deepEqual(transport.writes[1], {
    type: "ping",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "application-host-test",
    messageId: 2,
    renderRevision: 0,
    payload: { nonce: 73 },
  });
  assert.equal(host.state.pendingPingNonce, null);
  assert.equal(host.session.state.lastClientMessageId, 2);
  assert.equal(host.session.state.lastHostMessageId, 2);

  await host.close();
  assert.deepEqual(
    transport.writes.slice(1).map((message) => [message.type, message.messageId]),
    [["ping", 2], ["close", 3]],
  );
  assert.equal(host.session.state.status, "closed");
  assert.equal(host.session.state.lastClientMessageId, 3);
  assert.equal(host.session.state.lastHostMessageId, 3);
});

test("framed application host answers host ping across commit and event work", async () => {
  const transport = new TestByteTransport({ pingAfterCommit: 41 });
  const connection = await transport.connect();
  const host = new A3sFramedApplicationHostV1(connection);

  function Counter() {
    const [count, setCount] = useState(0);
    return jsxs(View, {
      children: [
        jsx(Text, { children: `count:${count}` }, "value"),
        jsx(Button, {
          onPress: () => setCount((value) => value + 1),
          children: "Increment",
        }, "increment"),
      ],
    });
  }

  const app = createApp(Counter, { frameId: "host-liveness", host });
  host.setEventHandler(async (message) => {
    await app.dispatch(message);
  });
  await app.start();
  assert.equal(host.state.receivedHostPings, 1);
  assert.equal(host.state.lastHostPingNonce, 41);
  assert.equal(transport.writes.some((message) =>
    message.type === "pong" && message.payload.nonce === 41
  ), true);

  const action = transport.writes.find((message) => message.type === "render")
    .payload.actions[0].id;
  transport.pushEvent(action);
  transport.pushHostPing(42);
  await waitFor(() => app.state.committedRenders === 2);
  await waitFor(() => host.state.receivedHostPings === 2);

  assert.equal(textContent(
    transport.writes.filter((message) => message.type === "render").at(-1).payload.root,
  ), "count:1Increment");
  assert.equal(transport.writes.some((message) =>
    message.type === "pong" && message.payload.nonce === 42
  ), true);
  assert.equal(app.state.session.lastHostMessageId, 6);
  assert.equal(app.state.session.lastReceivedHostMessageId, 6);
  await app.shutdown();
});

test("framed application host bounds unanswered liveness", async () => {
  const transport = new TestByteTransport({ autoControl: false });
  const connection = await transport.connect();
  const host = new A3sFramedApplicationHostV1(connection, { controlTimeoutMs: 20 });

  await assert.rejects(
    host.ping(11),
    (error) => error instanceof A3sFramedHostError && error.code === "controlTimedOut",
  );
  assert.equal(host.state.status, "failed");
  assert.equal(host.state.pendingPingNonce, null);
  await waitFor(() => transport.closed);
  await host.close();
  assert.equal(host.state.status, "closed");
});

test("framed application host exposes unexpected and graceful termination", async () => {
  const failedTransport = new TestByteTransport();
  const failedConnection = await failedTransport.connect();
  const failedHost = new A3sFramedApplicationHostV1(failedConnection);

  await failedTransport.close();
  const failed = await failedHost.termination;
  assert.equal(failed.status, "failed");
  assert.equal(failed.failure.code, "endOfStream");

  const gracefulTransport = new TestByteTransport();
  const gracefulConnection = await gracefulTransport.connect();
  const gracefulHost = new A3sFramedApplicationHostV1(gracefulConnection);

  await gracefulHost.close();
  assert.deepEqual(await gracefulHost.termination, {
    status: "closed",
    failure: null,
  });
});

test("framed application host rejects a second in-flight render and host fatal", async () => {
  const transport = new TestByteTransport({ autoCommit: false });
  const connection = await transport.connect();
  const host = new A3sFramedApplicationHostV1(connection);
  const compiled = compileFrameV1("pending", jsx(Text, { children: "pending" }));
  const render = host.session.createRender(1, compiled.frame);
  const pending = host.submitRender(render);
  await waitFor(() => transport.writes.length === 2);

  await assert.rejects(
    host.submitRender(render),
    (error) => error instanceof A3sFramedHostError && error.code === "renderInFlight",
  );
  transport.pushHostMessage({
    type: "fatal",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "application-host-test",
    messageId: 2,
    renderRevision: 0,
    payload: { code: "injected", message: "injected fatal" },
  });
  await assert.rejects(
    pending,
    (error) => error instanceof A3sFramedHostError && error.code === "hostFatal",
  );
  assert.equal(host.state.status, "failed");
  await host.close();
});

test("framed application host pumps an event before its resulting commit", async () => {
  const transport = new TestByteTransport();
  const connection = await transport.connect();
  const host = new A3sFramedApplicationHostV1(connection);

  function Counter() {
    const [count, setCount] = useState(0);
    return jsxs(View, {
      children: [
        jsx(Text, { children: `count:${count}` }, "value"),
        jsx(Button, {
          onPress: () => setCount((value) => value + 1),
          children: "Increment",
        }, "increment"),
      ],
    });
  }

  const app = createApp(Counter, { frameId: "event-pump", host });
  host.setEventHandler(async (message) => {
    await app.dispatch(message);
  });
  await app.start();
  const action = transport.writes[1].payload.actions[0].id;
  transport.pushEvent(action);
  await waitFor(() => app.state.committedRenders === 2);

  assert.equal(textContent(transport.writes[2].payload.root), "count:1Increment");
  assert.equal(app.state.session.lastClientMessageId, 3);
  assert.equal(app.state.session.lastHostMessageId, 4);
  assert.equal(host.state.pendingEventTasks, 0);
  await app.shutdown();
});

test("Node application host options reject accessors before spawning", async () => {
  let getterCalls = 0;
  const options = {
    process: { command: process.execPath },
    handshake: handshakeOptions(),
  };
  Object.defineProperty(options, "onEvent", {
    enumerable: true,
    get() {
      getterCalls += 1;
      return () => {};
    },
  });

  await assert.rejects(
    connectA3sNodeApplicationHostV1(options),
    (error) => error instanceof A3sFramedHostError && error.code === "invalidOptions",
  );
  assert.equal(getterCalls, 0);
});

class TestByteTransport {
  writes = [];
  closed = false;
  #autoCommit;
  #autoControl;
  #pingAfterCommit;
  #hostMessageId = 1;
  #chunks = [];
  #waiters = [];

  constructor(options = {}) {
    this.#autoCommit = options.autoCommit ?? true;
    this.#autoControl = options.autoControl ?? true;
    this.#pingAfterCommit = options.pingAfterCommit ?? null;
  }

  incoming = {
    [Symbol.asyncIterator]: () => this,
  };

  async connect() {
    this.pushHostMessage(welcome());
    return connectA3sFramedClientV1(this, handshakeOptions());
  }

  async write(chunk) {
    const message = decodeSingle(chunk);
    this.writes.push(message);
    if (this.#autoCommit && message.type === "render") {
      this.pushHostMessage(committed(message, ++this.#hostMessageId));
      if (this.#pingAfterCommit !== null) {
        this.pushHostPing(this.#pingAfterCommit);
        this.#pingAfterCommit = null;
      }
    } else if (this.#autoControl && message.type === "ping") {
      this.pushHostMessage(control("pong", ++this.#hostMessageId, message));
    } else if (this.#autoControl && message.type === "close") {
      this.pushHostMessage(control("close", ++this.#hostMessageId, message));
    }
  }

  next() {
    const chunk = this.#chunks.shift();
    if (chunk !== undefined) {
      return Promise.resolve({ done: false, value: chunk });
    }
    if (this.closed) {
      return Promise.resolve({ done: true, value: undefined });
    }
    return new Promise((resolve) => this.#waiters.push(resolve));
  }

  async close() {
    this.closed = true;
    for (const waiter of this.#waiters.splice(0)) {
      waiter({ done: true, value: undefined });
    }
  }

  pushHostMessage(message) {
    const chunk = encodeA3sJsonFrameV1(message, 16_384);
    const waiter = this.#waiters.shift();
    if (waiter === undefined) {
      this.#chunks.push(chunk);
    } else {
      waiter({ done: false, value: chunk });
    }
  }

  pushEvent(action) {
    this.pushHostMessage(event(++this.#hostMessageId, action));
  }

  pushHostPing(nonce) {
    this.pushHostMessage({
      type: "ping",
      protocol: "a3s.gui.tsx",
      protocolVersion: 1,
      sessionId: "application-host-test",
      messageId: ++this.#hostMessageId,
      renderRevision: 1,
      payload: { nonce },
    });
  }
}

function handshakeOptions() {
  return {
    sdkVersion: "0.0.0-development",
    sessionId: "application-host-test",
    requestedRenderer: "software",
    maximumFrameBytes: 16_384,
  };
}

function decodeSingle(frame) {
  const decoder = new A3sJsonFrameDecoderV1(16_384);
  const messages = decoder.push(frame);
  decoder.finish();
  assert.equal(messages.length, 1);
  return messages[0];
}

function welcome() {
  return {
    type: "welcome",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "application-host-test",
    messageId: 1,
    renderRevision: 0,
    payload: {
      selectedProtocolVersion: 1,
      hostVersion: "0.1.0",
      hostBuildId: "application-host-test",
      platform: "headless",
      renderer: "software",
      limits: { maximumFrameBytes: 16_384, maximumInFlightRenders: 1 },
      capabilities: ["headlessRendering", "selfDrawnRendering"],
    },
  };
}

function committed(render, messageId) {
  return {
    type: "committed",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: render.sessionId,
    messageId,
    renderRevision: render.renderRevision,
    payload: {
      frameId: render.payload.frameId,
      hostRevision: 1,
      rootId: "9:root",
      layoutFingerprint: "0000000000000001",
      sceneFingerprint: "0000000000000001",
    },
  };
}

function control(type, messageId, clientMessage) {
  return {
    type,
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: clientMessage.sessionId,
    messageId,
    renderRevision: clientMessage.renderRevision,
    payload: clientMessage.payload,
  };
}

function event(messageId, action) {
  return {
    type: "event",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "application-host-test",
    messageId,
    renderRevision: 1,
    payload: {
      hostRevision: 1,
      eventSequence: 1,
      target: "9:increment",
      invocations: [{
        node: "9:increment",
        action,
        event: "press",
        context: {
          device: 1,
          modality: "keyboard",
          modifiers: { alt: false, control: false, meta: false, shift: false },
          repeat: false,
          clickCount: 0,
          handledActivation: true,
          timestampMicros: 1,
        },
      }],
    },
  };
}

function textContent(node) {
  if (node.kind === "text") {
    return node.value;
  }
  return node.children.map(textContent).join("");
}

async function waitFor(predicate) {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (predicate()) {
      return;
    }
    await new Promise((resolve) => setImmediate(resolve));
  }
  throw new Error("timed out waiting for test condition");
}
