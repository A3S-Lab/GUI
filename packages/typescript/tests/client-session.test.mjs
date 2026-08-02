import assert from "node:assert/strict";
import test from "node:test";

import {
  A3sClientSessionError,
  A3sClientSessionV1,
  A3sActionRegistryError,
  Button,
  RevisionActionRegistryV1,
  Text,
  compileFrameV1,
  defineAction,
} from "../src/index.ts";
import { jsx } from "../src/jsx-runtime.ts";

test("client session emits exact render envelopes and advances independent directions", async () => {
  const calls = [];
  const actions = new RevisionActionRegistryV1();
  const session = new A3sClientSessionV1(welcome());
  const compiled = compileActionFrame("counter", "increment", () => calls.push("increment"));

  actions.stage(1, compiled);
  const render = session.createRender(1, compiled.frame);
  assert.deepEqual(render, {
    type: "render",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "session-test",
    messageId: 2,
    renderRevision: 1,
    payload: compiled.frame,
  });
  assert.equal(Object.isFrozen(render), true);
  assert.equal(session.state.lastClientMessageId, 2);
  assert.equal(session.state.lastHostMessageId, 1);
  assert.equal(session.state.pendingRenderRevision, 1);

  session.commitRender(committed(render, 2, 7), actions);
  assert.equal(actions.state.active.renderRevision, 1);
  assert.deepEqual(session.state, {
    status: "negotiated",
    sessionId: "session-test",
    lastClientMessageId: 2,
    lastHostMessageId: 2,
    lastReceivedHostMessageId: 2,
    committedRenderRevision: 1,
    committedHostRevision: 7,
    pendingRenderRevision: null,
    pendingPingNonce: null,
    pendingCloseReason: null,
    maximumFrameBytes: 4096,
  });

  const result = await session.dispatchEvent(
    event(render, 3, 7, 1, "increment"),
    actions,
  );
  assert.deepEqual(calls, ["increment"]);
  assert.equal(result.eventSequence, 1);
  assert.equal(session.state.lastHostMessageId, 3);
  assert.equal(session.state.lastClientMessageId, 2);
});

test("host ping can overtake reserved commit and event application", async () => {
  const calls = [];
  const actions = new RevisionActionRegistryV1();
  const session = new A3sClientSessionV1(welcome());
  const compiled = compileActionFrame("interleaved", "press", () => calls.push("press"));
  actions.stage(1, compiled);
  const render = session.createRender(1, compiled.frame);

  const reservedCommit = session.receiveHostMessage(committed(render, 2, 1));
  const reservedCommitPing = session.receiveHostMessage(control("ping", 3, { nonce: 31 }, 1));
  const firstPong = session.acceptPing(reservedCommitPing);
  assert.deepEqual(firstPong, {
    type: "pong",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "session-test",
    messageId: 3,
    renderRevision: 1,
    payload: { nonce: 31 },
  });
  assert.equal(session.state.lastHostMessageId, 1);
  assert.equal(session.state.lastReceivedHostMessageId, 3);

  session.commitRender(reservedCommit, actions);
  assert.equal(session.state.lastHostMessageId, 3);
  assert.equal(session.state.committedRenderRevision, 1);

  const reservedEvent = session.receiveHostMessage(event(render, 4, 1, 1, "press"));
  const reservedEventPing = session.receiveHostMessage(control("ping", 5, { nonce: 32 }, 1));
  const secondPong = session.acceptPing(reservedEventPing);
  assert.equal(secondPong.messageId, 4);
  assert.equal(secondPong.renderRevision, 1);
  assert.equal(session.state.lastHostMessageId, 3);
  assert.equal(session.state.lastReceivedHostMessageId, 5);

  await session.dispatchEvent(reservedEvent, actions);
  assert.deepEqual(calls, ["press"]);
  assert.equal(session.state.lastHostMessageId, 5);
  assert.equal(session.state.lastReceivedHostMessageId, 5);
});

test("client session bounds host messages ahead of semantic application", () => {
  const actions = new RevisionActionRegistryV1();
  const session = new A3sClientSessionV1(welcome());
  const compiled = compileActionFrame("bounded", "press", () => undefined);
  actions.stage(1, compiled);
  const render = session.createRender(1, compiled.frame);

  session.receiveHostMessage(committed(render, 2, 1));
  for (let messageId = 3; messageId <= 1_025; messageId += 1) {
    const ping = session.receiveHostMessage(
      control("ping", messageId, { nonce: messageId }, 1),
    );
    session.acceptPing(ping);
  }

  assert.throws(
    () => session.receiveHostMessage(control("ping", 1_026, { nonce: 1_026 }, 1)),
    (error) => error instanceof A3sClientSessionError && error.code === "invalidState",
  );
  assert.equal(session.state.status, "failed");
  assert.equal(session.state.lastHostMessageId, 1);
  assert.equal(session.state.lastReceivedHostMessageId, 1_025);
});

test("client session sequences liveness and graceful close atomically", () => {
  const session = new A3sClientSessionV1(welcome());

  const ping = session.createPing(42);
  assert.deepEqual(ping, {
    type: "ping",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "session-test",
    messageId: 2,
    renderRevision: 0,
    payload: { nonce: 42 },
  });
  assert.equal(Object.isFrozen(ping), true);
  assert.equal(session.state.pendingPingNonce, 42);
  assert.equal(session.state.lastClientMessageId, 2);
  assert.equal(session.state.lastHostMessageId, 1);

  session.acceptPong(control("pong", 2, { nonce: 42 }));
  assert.equal(session.state.pendingPingNonce, null);
  assert.equal(session.state.lastHostMessageId, 2);

  const close = session.createClose("requested", "test complete");
  assert.deepEqual(close, {
    type: "close",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "session-test",
    messageId: 3,
    renderRevision: 0,
    payload: { reason: "requested", message: "test complete" },
  });
  assert.equal(session.state.status, "closing");
  assert.equal(session.state.pendingCloseReason, "requested");

  session.acceptClose(control("close", 3, close.payload));
  assert.equal(session.state.status, "closed");
  assert.equal(session.state.pendingCloseReason, null);
  assert.equal(session.state.lastClientMessageId, 3);
  assert.equal(session.state.lastHostMessageId, 3);
});

test("wrong liveness nonce fails without consuming the host sequence", () => {
  const session = new A3sClientSessionV1(welcome());
  session.createPing(7);

  assert.throws(
    () => session.acceptPong(control("pong", 2, { nonce: 8 })),
    (error) => error instanceof A3sClientSessionError && error.code === "invalidMessage",
  );
  assert.equal(session.state.status, "failed");
  assert.equal(session.state.lastHostMessageId, 1);
  assert.equal(session.state.lastReceivedHostMessageId, 2);
  assert.equal(session.state.pendingPingNonce, 7);
});

test("render rejection retries the same revision with the next client message id", () => {
  const actions = new RevisionActionRegistryV1();
  const session = new A3sClientSessionV1(welcome());
  const first = compileActionFrame("first", "first", () => undefined);

  actions.stage(1, first);
  const firstRender = session.createRender(1, first.frame);
  assert.equal(firstRender.messageId, 2);
  actions.reject(1);
  session.rejectRender(1);

  const retry = compileActionFrame("retry", "retry", () => undefined);
  actions.stage(1, retry);
  const retryRender = session.createRender(1, retry.frame);
  assert.equal(retryRender.messageId, 3);
  assert.equal(retryRender.renderRevision, 1);
  assert.equal(session.state.committedRenderRevision, 0);
});

test("wrong session and skipped host ids fail before callback-scope mutation", () => {
  const actions = new RevisionActionRegistryV1();
  const session = new A3sClientSessionV1(welcome());
  const compiled = compileActionFrame("identity", "action", () => undefined);
  actions.stage(1, compiled);
  const render = session.createRender(1, compiled.frame);

  assert.throws(
    () => session.commitRender(
      { ...committed(render, 2, 1), sessionId: "other-session" },
      actions,
    ),
    (error) => error instanceof A3sClientSessionError && error.code === "invalidSession",
  );
  assert.equal(actions.state.active, null);
  assert.equal(actions.state.pending.renderRevision, 1);
  assert.equal(session.state.lastHostMessageId, 1);
  assert.equal(session.state.committedRenderRevision, 0);
  assert.equal(session.state.status, "failed");

  const orderedActions = new RevisionActionRegistryV1();
  const orderedSession = new A3sClientSessionV1(welcome());
  orderedActions.stage(1, compiled);
  const orderedRender = orderedSession.createRender(1, compiled.frame);
  assert.throws(
    () => orderedSession.commitRender(committed(orderedRender, 3, 1), orderedActions),
    (error) => error instanceof A3sClientSessionError && error.code === "invalidMessageId",
  );
  assert.equal(orderedActions.state.pending.renderRevision, 1);
  assert.equal(orderedSession.state.lastHostMessageId, 1);
});

test("callback failures consume both event and host message sequences exactly once", async () => {
  const calls = [];
  const actions = new RevisionActionRegistryV1();
  const session = new A3sClientSessionV1(welcome());
  const compiled = compileActionFrame("failure", "action", () => {
    calls.push("failed");
    throw new Error("injected callback failure");
  });
  actions.stage(1, compiled);
  const render = session.createRender(1, compiled.frame);
  session.commitRender(committed(render, 2, 1), actions);

  await assert.rejects(
    session.dispatchEvent(event(render, 3, 1, 1, "action"), actions),
    (error) => error instanceof A3sActionRegistryError && error.code === "callbackFailed",
  );
  assert.deepEqual(calls, ["failed"]);
  assert.equal(actions.state.lastEventSequence, 1);
  assert.equal(session.state.lastHostMessageId, 3);
  assert.equal(session.state.status, "negotiated");

  await assert.rejects(
    session.dispatchEvent(event(render, 3, 1, 1, "action"), actions),
    (error) => error instanceof A3sClientSessionError && error.code === "invalidMessageId",
  );
  assert.deepEqual(calls, ["failed"]);
});

test("invalid welcome and oversized render frames fail atomically", () => {
  assert.throws(
    () => new A3sClientSessionV1({ ...welcome(), messageId: 2 }),
    (error) => error instanceof A3sClientSessionError && error.code === "invalidWelcome",
  );

  const session = new A3sClientSessionV1(welcome());
  const oversized = compileFrameV1(
    "oversized",
    jsx(Text, { children: "x".repeat(8_192) }),
  );
  assert.throws(
    () => session.createRender(1, oversized.frame),
    (error) => error instanceof A3sClientSessionError && error.code === "frameTooLarge",
  );
  assert.equal(session.state.lastClientMessageId, 1);
  assert.equal(session.state.pendingRenderRevision, null);
});

function compileActionFrame(frameId, actionId, handler) {
  return compileFrameV1(
    frameId,
    jsx(Button, {
      onPress: defineAction(actionId, handler),
      children: actionId,
    }),
  );
}

function welcome() {
  return {
    type: "welcome",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "session-test",
    messageId: 1,
    renderRevision: 0,
    payload: {
      selectedProtocolVersion: 1,
      hostVersion: "0.1.0",
      hostBuildId: "test-build",
      platform: "headless",
      renderer: "software",
      limits: {
        maximumFrameBytes: 4096,
        maximumInFlightRenders: 1,
      },
      capabilities: ["headlessRendering", "structuredDiagnostics"],
      debugCapabilities: ["structuredDiagnostics"],
    },
  };
}

function committed(render, messageId, hostRevision) {
  return {
    type: "committed",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: render.sessionId,
    messageId,
    renderRevision: render.renderRevision,
    payload: {
      frameId: render.payload.frameId,
      hostRevision,
      rootId: "root",
      layoutFingerprint: "0000000000000000",
      sceneFingerprint: "0000000000000000",
    },
  };
}

function control(type, messageId, payload, renderRevision = 0) {
  return {
    type,
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "session-test",
    messageId,
    renderRevision,
    payload,
  };
}

function event(render, messageId, hostRevision, eventSequence, action) {
  return {
    type: "event",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: render.sessionId,
    messageId,
    renderRevision: render.renderRevision,
    payload: {
      hostRevision,
      eventSequence,
      target: "root",
      invocations: [{
        node: "root",
        action,
        event: "press",
        context: {
          device: 1,
          modality: "keyboard",
          modifiers: { alt: false, control: false, meta: false, shift: false },
          repeat: false,
          clickCount: 1,
          handledActivation: true,
          timestampMicros: eventSequence,
        },
      }],
    },
  };
}
