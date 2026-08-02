import assert from "node:assert/strict";
import test from "node:test";

import {
  A3sClientHandshakeError,
  A3sClientHandshakeV1,
  A3sClientSessionV1,
} from "../src/index.ts";

test("client handshake emits canonical hello and accepts one matching welcome", () => {
  const handshake = new A3sClientHandshakeV1({
    sdkVersion: "0.0.0-development",
    sessionId: "handshake-session",
    requestedRenderer: "software",
    maximumFrameBytes: 4_096,
    debugCapabilities: ["protocolTrace", "structuredDiagnostics"],
  });

  assert.deepEqual(handshake.hello, {
    type: "hello",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "handshake-session",
    messageId: 1,
    renderRevision: 0,
    payload: {
      sdkVersion: "0.0.0-development",
      minimumProtocolVersion: 1,
      maximumProtocolVersion: 1,
      requestedRenderer: "software",
      maximumFrameBytes: 4_096,
      debugCapabilities: ["protocolTrace", "structuredDiagnostics"],
    },
  });
  assert.equal(Object.isFrozen(handshake.hello), true);
  assert.equal(Object.isFrozen(handshake.hello.payload.debugCapabilities), true);
  assert.equal(handshake.state.status, "awaitingWelcome");

  const session = handshake.acceptWelcome(welcome());
  assert.equal(session instanceof A3sClientSessionV1, true);
  assert.equal(session.state.sessionId, "handshake-session");
  assert.equal(session.state.maximumFrameBytes, 2_048);
  assert.deepEqual(handshake.state, {
    status: "negotiated",
    sessionId: "handshake-session",
    requestedRenderer: "software",
    requestedMaximumFrameBytes: 4_096,
    negotiatedMaximumFrameBytes: 2_048,
  });
  assert.equal(handshake.session, session);
  assert.throws(
    () => handshake.acceptWelcome(welcome()),
    (error) => error instanceof A3sClientHandshakeError && error.code === "invalidState",
  );
});

test("client handshake rejects identity and negotiation downgrades atomically", () => {
  const wrongSession = createHandshake();
  assert.throws(
    () => wrongSession.acceptWelcome({ ...welcome(), sessionId: "other-session" }),
    (error) => error instanceof A3sClientHandshakeError && error.code === "invalidWelcome",
  );
  assert.equal(wrongSession.state.status, "failed");
  assert.equal(wrongSession.session, null);

  const wrongRenderer = createHandshake();
  assert.throws(
    () => wrongRenderer.acceptWelcome({
      ...welcome(),
      payload: { ...welcome().payload, renderer: "gpu" },
    }),
    (error) => error instanceof A3sClientHandshakeError && error.code === "invalidWelcome",
  );

  const expandedLimit = createHandshake();
  assert.throws(
    () => expandedLimit.acceptWelcome({
      ...welcome(),
      payload: {
        ...welcome().payload,
        limits: { maximumFrameBytes: 8_192, maximumInFlightRenders: 1 },
      },
    }),
    (error) => error instanceof A3sClientHandshakeError && error.code === "invalidWelcome",
  );

  const unrequestedDebug = createHandshake();
  assert.throws(
    () => unrequestedDebug.acceptWelcome({
      ...welcome(),
      payload: { ...welcome().payload, debugCapabilities: ["inspector"] },
    }),
    (error) => error instanceof A3sClientHandshakeError && error.code === "invalidWelcome",
  );
});

test("client handshake validates bounded unique options before creating hello", () => {
  assert.throws(
    () => new A3sClientHandshakeV1({
      sdkVersion: "0.0.0-development",
      sessionId: "handshake-session",
      debugCapabilities: ["protocolTrace", "protocolTrace"],
    }),
    (error) => error instanceof A3sClientHandshakeError && error.code === "invalidOptions",
  );
  assert.throws(
    () => new A3sClientHandshakeV1({
      sdkVersion: "0.0.0-development",
      sessionId: "handshake-session",
      maximumFrameBytes: 1,
    }),
    (error) => error instanceof A3sClientHandshakeError && error.code === "frameTooLarge",
  );
});

function createHandshake() {
  return new A3sClientHandshakeV1({
    sdkVersion: "0.0.0-development",
    sessionId: "handshake-session",
    requestedRenderer: "software",
    maximumFrameBytes: 4_096,
    debugCapabilities: ["protocolTrace"],
  });
}

function welcome() {
  return {
    type: "welcome",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "handshake-session",
    messageId: 1,
    renderRevision: 0,
    payload: {
      selectedProtocolVersion: 1,
      hostVersion: "0.1.0",
      hostBuildId: "test-build",
      platform: "headless",
      renderer: "software",
      limits: {
        maximumFrameBytes: 2_048,
        maximumInFlightRenders: 1,
      },
      capabilities: ["headlessRendering", "selfDrawnRendering"],
      debugCapabilities: ["protocolTrace"],
    },
  };
}
