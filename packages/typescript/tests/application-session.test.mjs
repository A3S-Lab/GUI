import assert from "node:assert/strict";
import test from "node:test";

import {
  A3sClientSessionError,
  Button,
  Text,
  View,
  createApp,
  useState,
} from "../src/index.ts";
import { jsx, jsxs } from "../src/jsx-runtime.ts";

test("application rejects wrong-session events before invoking callbacks", async () => {
  const host = new ImmediateSessionHost();
  const calls = [];

  function App() {
    return jsx(Button, {
      onPress: () => calls.push("pressed"),
      children: "Press",
    });
  }

  const app = createApp(App, { frameId: "wrong-session", host });
  await app.start();
  const wrongSession = { ...host.event(host.last, 1), sessionId: "other-session" };
  await assert.rejects(
    app.dispatch(wrongSession),
    (error) => error instanceof A3sClientSessionError && error.code === "invalidSession",
  );
  assert.deepEqual(calls, []);
  assert.equal(app.state.actions.lastEventSequence, 0);
  assert.equal(app.state.session.lastHostMessageId, 2);
  assert.equal(app.state.session.status, "failed");
  await app.shutdown();
});

test("host messages serialize when an event overlaps an in-flight commit", async () => {
  const host = new DeferredSessionHost();
  let setCount;
  let releaseCallback;
  let markCallbackStarted;
  const callbackStarted = new Promise((resolveStarted) => {
    markCallbackStarted = resolveStarted;
  });
  const callbackBlocked = new Promise((resolveBlocked) => {
    releaseCallback = resolveBlocked;
  });

  function App() {
    const [count, updateCount] = useState(0);
    setCount = updateCount;
    return jsxs(View, {
      children: [
        jsx(Text, { children: `count:${count}` }, "value"),
        jsx(Button, {
          onPress: async () => {
            markCallbackStarted();
            await callbackBlocked;
          },
          children: "Wait",
        }, "wait"),
      ],
    });
  }

  const app = createApp(App, { frameId: "overlap", host });
  const starting = app.start();
  await waitFor(() => host.candidates.length === 1);
  host.commitNext();
  await starting;

  const activeRender = host.candidates[0];
  setCount(1);
  const rendering = app.flush();
  await waitFor(() => host.candidates.length === 2);

  const dispatching = app.dispatch(host.event(activeRender, 1));
  await callbackStarted;
  host.commitNext();
  releaseCallback();

  await Promise.all([dispatching, rendering]);
  assert.equal(app.state.session.status, "negotiated");
  assert.equal(app.state.session.lastHostMessageId, 4);
  assert.equal(app.state.actions.active.renderRevision, 2);
  assert.equal(textContent(host.last.payload.root), "count:1Wait");
  await app.shutdown();
});

class ImmediateSessionHost {
  welcome = welcome();
  candidates = [];
  hostRevision = 0;
  hostMessageId = 1;

  get last() {
    return this.candidates.at(-1);
  }

  async submitRender(candidate) {
    this.candidates.push(candidate);
    this.hostRevision += 1;
    this.hostMessageId += 1;
    return committed(candidate, this.hostMessageId, this.hostRevision);
  }

  event(candidate, eventSequence) {
    this.hostMessageId += 1;
    return eventFor(
      { ...candidate, hostRevision: this.hostRevision },
      this.hostMessageId,
      eventSequence,
    );
  }
}

class DeferredSessionHost {
  welcome = welcome();
  candidates = [];
  pending = [];
  hostRevision = 0;
  hostMessageId = 1;

  get last() {
    return this.candidates.at(-1);
  }

  submitRender(candidate) {
    this.candidates.push(candidate);
    return new Promise((resolve) => this.pending.push({ candidate, resolve }));
  }

  commitNext() {
    const pending = this.pending.shift();
    assert.ok(pending);
    this.hostRevision += 1;
    this.hostMessageId += 1;
    pending.resolve(committed(pending.candidate, this.hostMessageId, this.hostRevision));
  }

  event(candidate, eventSequence) {
    this.hostMessageId += 1;
    return eventFor(
      { ...candidate, hostRevision: this.hostRevision },
      this.hostMessageId,
      eventSequence,
    );
  }
}

function welcome() {
  return {
    type: "welcome",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "application-session-test",
    messageId: 1,
    renderRevision: 0,
    payload: {
      selectedProtocolVersion: 1,
      hostVersion: "0.1.0",
      hostBuildId: "application-session-test",
      platform: "headless",
      renderer: "software",
      limits: {
        maximumFrameBytes: 16 * 1024 * 1024,
        maximumInFlightRenders: 1,
      },
    },
  };
}

function committed(candidate, messageId, hostRevision) {
  return {
    type: "committed",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: candidate.sessionId,
    messageId,
    renderRevision: candidate.renderRevision,
    payload: {
      frameId: candidate.payload.frameId,
      hostRevision,
      rootId: "root",
      layoutFingerprint: "0000000000000000",
      sceneFingerprint: "0000000000000000",
    },
  };
}

function eventFor(candidate, messageId, eventSequence) {
  const action = candidate.payload.actions[0]?.id;
  assert.ok(action);
  return {
    type: "event",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: candidate.sessionId,
    messageId,
    renderRevision: candidate.renderRevision,
    payload: {
      hostRevision: candidate.hostRevision,
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
    await Promise.resolve();
  }
  assert.fail("condition did not become true within 100 microtasks");
}
