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
    committedRenderRevision: 1,
    committedHostRevision: 7,
    pendingRenderRevision: null,
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
