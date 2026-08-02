import assert from "node:assert/strict";
import test from "node:test";

import {
  A3sApplicationRecoveryError,
  Button,
  Text,
  View,
  createApp,
  useState,
} from "../src/index.ts";
import { jsx, jsxs } from "../src/jsx-runtime.ts";

test("application recovery replays the committed frame into a fresh session", async () => {
  const first = new RecoverableHost("replay-session-1");
  let setCount;

  function Counter() {
    const [count, updateCount] = useState(0);
    setCount = updateCount;
    return jsxs(View, {
      children: [
        jsx(Text, { children: `count:${count}` }, "value"),
        jsx(Button, {
          onPress: () => updateCount((value) => value + 1),
          children: "Increment",
        }, "increment"),
      ],
    });
  }

  const app = createApp(Counter, { frameId: "recoverable-counter", host: first });
  await app.start();
  setCount(4);
  await app.flush();
  assert.equal(textContent(first.last.payload.root), "count:4Increment");

  const second = new RecoverableHost("replay-session-2");
  await app.recover(second);

  assert.equal(app.host, second);
  assert.equal(app.state.hostGeneration, 2);
  assert.equal(app.state.replayedRenders, 1);
  assert.equal(app.state.committedRenders, 2);
  assert.equal(app.state.session.sessionId, "replay-session-2");
  assert.equal(app.state.session.committedRenderRevision, 1);
  assert.equal(app.state.actions.active.renderRevision, 1);
  assert.equal(second.candidates.length, 1);
  assert.equal(second.last.renderRevision, 1);
  assert.equal(textContent(second.last.payload.root), "count:4Increment");
  assert.equal(first.closeCount, 1);

  await app.dispatch(second.event());
  assert.equal(second.last.renderRevision, 2);
  assert.equal(textContent(second.last.payload.root), "count:5Increment");
  await app.shutdown();
});

test("failed replay preserves the committed application session for a later retry", async () => {
  const first = new RecoverableHost("retry-session-1");
  const app = createApp(
    () => jsx(Text, { children: "retained" }),
    { frameId: "retry-replay", host: first },
  );
  await app.start();

  const rejected = new RecoverableHost("retry-session-2", { rejectRender: true });
  await assert.rejects(app.recover(rejected), /injected replay rejection/u);
  assert.equal(app.host, first);
  assert.equal(app.state.session.sessionId, "retry-session-1");
  assert.equal(app.state.actions.active.renderRevision, 1);
  assert.equal(app.state.hostGeneration, 1);
  assert.equal(app.state.replayedRenders, 0);
  assert.equal(rejected.closeCount, 1);

  const accepted = new RecoverableHost("retry-session-3");
  await app.recover(accepted);
  assert.equal(app.host, accepted);
  assert.equal(app.state.session.sessionId, "retry-session-3");
  assert.equal(app.state.hostGeneration, 2);
  assert.equal(app.state.replayedRenders, 1);
  await app.shutdown();
});

test("application recovery rejects reuse of the previous session identity", async () => {
  const first = new RecoverableHost("duplicate-session");
  const app = createApp(
    () => jsx(Text, { children: "identity" }),
    { frameId: "fresh-recovery-identity", host: first },
  );
  await app.start();

  const duplicate = new RecoverableHost("duplicate-session");
  await assert.rejects(app.recover(duplicate), /fresh host session identity/u);
  assert.equal(duplicate.candidates.length, 0);
  assert.equal(duplicate.closeCount, 1);
  assert.equal(app.state.session.sessionId, "duplicate-session");

  const fresh = new RecoverableHost("fresh-session");
  await app.recover(fresh);
  assert.equal(app.state.session.sessionId, "fresh-session");
  await app.shutdown();
});

test("state updates during replay follow the retained frame in the fresh session", async () => {
  const first = new RecoverableHost("concurrent-session-1");
  let setCount;
  const app = createApp(() => {
    const [count, updateCount] = useState(0);
    setCount = updateCount;
    return jsx(Text, { children: `count:${count}` });
  }, { host: first });
  await app.start();

  const second = new RecoverableHost("concurrent-session-2", { deferRenders: true });
  const recovery = app.recover(second);
  await waitFor(() => second.candidates.length === 1);
  assert.equal(textContent(second.last.payload.root), "count:0");

  setCount(9);
  second.commitNext();
  await recovery;
  await waitFor(() => second.candidates.length === 2);
  assert.equal(second.last.renderRevision, 2);
  assert.equal(textContent(second.last.payload.root), "count:9");
  second.commitNext();
  await app.flush();
  await app.shutdown();
});

test("opt-in runner supervision restarts once and binds events after replay", async () => {
  const runtime = new RecoveringRuntime();

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

  const app = await createApp(Counter).run({
    runtime,
    recovery: { maximumRestarts: 1, restartDelayMs: 0 },
  });
  const originalSession = app.state.session.sessionId;
  runtime.hosts[0].crash(new Error("injected idle host crash"));

  await waitFor(() => app.state.hostGeneration === 2);
  assert.equal(runtime.connectCount, 2);
  assert.notEqual(app.state.session.sessionId, originalSession);
  assert.equal(app.state.replayedRenders, 1);
  assert.equal(textContent(runtime.hosts[1].last.payload.root), "count:0Increment");

  await runtime.hosts[1].press();
  assert.equal(textContent(runtime.hosts[1].last.payload.root), "count:1Increment");
  await app.shutdown();
});

test("replacement Host events wait until the replay callback scope is active", async () => {
  const runtime = new RecoveringRuntime({ emitEventAfterCommitOn: 2 });

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

  const app = await createApp(Counter).run({
    runtime,
    recovery: { maximumRestarts: 1 },
  });
  runtime.hosts[0].crash(new Error("injected crash before gated event"));

  await waitFor(() => app.state.committedRenders === 2);
  assert.equal(app.state.hostGeneration, 2);
  assert.equal(runtime.hosts[1].candidates[0].renderRevision, 1);
  assert.equal(runtime.hosts[1].candidates[1].renderRevision, 2);
  assert.equal(textContent(runtime.hosts[1].last.payload.root), "count:1Increment");
  await app.shutdown();
});

test("runner supervision stops after its bounded restart budget is exhausted", async () => {
  const observed = [];
  const runtime = new RecoveringRuntime({ failConnectionsAfter: 1 });
  const app = await createApp(() => jsx(Text, { children: "bounded" }), {
    onError: (error) => observed.push(error),
  }).run({
    runtime,
    recovery: { maximumRestarts: 2, restartDelayMs: 0 },
  });

  runtime.hosts[0].crash(new Error("injected permanent host crash"));
  await waitFor(() => app.state.status === "closed");

  assert.equal(runtime.connectCount, 3);
  assert.equal(observed.length, 1);
  assert.equal(observed[0] instanceof A3sApplicationRecoveryError, true);
  assert.equal(observed[0].code, "restartsExhausted");
  assert.equal(app.state.lastError, observed[0]);
});

class RecoveringRuntime {
  connectCount = 0;
  hosts = [];
  #emitEventAfterCommitOn;
  #failConnectionsAfter;

  constructor(options = {}) {
    this.#failConnectionsAfter = options.failConnectionsAfter ?? Number.POSITIVE_INFINITY;
    this.#emitEventAfterCommitOn = options.emitEventAfterCommitOn ?? null;
  }

  async connect(onEvent) {
    this.connectCount += 1;
    if (this.connectCount > this.#failConnectionsAfter) {
      throw new Error(`injected connection failure ${this.connectCount}`);
    }
    const host = new RecoverableHost(`supervised-session-${this.connectCount}`, {
      onEvent,
      emitEventAfterCommit: this.connectCount === this.#emitEventAfterCommitOn,
    });
    this.hosts.push(host);
    return host;
  }
}

class RecoverableHost {
  candidates = [];
  closeCount = 0;
  hostMessageId = 1;
  hostRevision = 0;
  welcome;
  termination;
  #onEvent;
  #deferRenders;
  #emitEventAfterCommit;
  #emittedEvent = false;
  #pending = [];
  #rejectRender;
  #resolveTermination;
  #terminated = false;

  constructor(sessionId, options = {}) {
    this.welcome = welcome(sessionId);
    this.#onEvent = options.onEvent ?? null;
    this.#deferRenders = options.deferRenders ?? false;
    this.#emitEventAfterCommit = options.emitEventAfterCommit ?? false;
    this.#rejectRender = options.rejectRender ?? false;
    this.termination = new Promise((resolve) => {
      this.#resolveTermination = resolve;
    });
  }

  get last() {
    const candidate = this.candidates.at(-1);
    assert.notEqual(candidate, undefined);
    return candidate;
  }

  async submitRender(candidate) {
    if (this.#rejectRender) {
      throw new Error("injected replay rejection");
    }
    this.candidates.push(candidate);
    if (this.#deferRenders) {
      return new Promise((resolve) => this.#pending.push({ candidate, resolve }));
    }
    const result = this.#commit(candidate);
    if (
      this.#emitEventAfterCommit &&
      !this.#emittedEvent &&
      this.#onEvent !== null
    ) {
      this.#emittedEvent = true;
      void Promise.resolve(this.#onEvent(this.event())).catch(() => {
        // The runner test observes application/Host failure through state.
      });
    }
    return result;
  }

  commitNext() {
    const pending = this.#pending.shift();
    assert.notEqual(pending, undefined);
    pending.resolve(this.#commit(pending.candidate));
  }

  #commit(candidate) {
    this.hostMessageId += 1;
    this.hostRevision += 1;
    return committed(candidate, this.hostMessageId, this.hostRevision);
  }

  event() {
    const action = this.last.payload.actions[0]?.id;
    assert.notEqual(action, undefined);
    this.hostMessageId += 1;
    return {
      type: "event",
      protocol: "a3s.gui.tsx",
      protocolVersion: 1,
      sessionId: this.welcome.sessionId,
      messageId: this.hostMessageId,
      renderRevision: this.last.renderRevision,
      payload: {
        hostRevision: this.hostRevision,
        eventSequence: 1,
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
            timestampMicros: 1,
          },
        }],
      },
    };
  }

  async press() {
    assert.notEqual(this.#onEvent, null);
    await this.#onEvent(this.event());
  }

  crash(failure) {
    if (this.#terminated) {
      return;
    }
    this.#terminated = true;
    this.#resolveTermination({ status: "failed", failure });
  }

  async close() {
    this.closeCount += 1;
    if (!this.#terminated) {
      this.#terminated = true;
      this.#resolveTermination({ status: "closed", failure: null });
    }
  }
}

function welcome(sessionId) {
  return {
    type: "welcome",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId,
    messageId: 1,
    renderRevision: 0,
    payload: {
      selectedProtocolVersion: 1,
      hostVersion: "0.1.0",
      hostBuildId: sessionId,
      platform: "headless",
      renderer: "software",
      limits: { maximumFrameBytes: 16 * 1024 * 1024, maximumInFlightRenders: 1 },
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

function textContent(node) {
  if (node.kind === "text") {
    return node.value;
  }
  return node.children.map(textContent).join("");
}

async function waitFor(predicate) {
  for (let attempt = 0; attempt < 200; attempt += 1) {
    if (predicate()) {
      return;
    }
    await new Promise((resolve) => setImmediate(resolve));
  }
  throw new Error("timed out waiting for recovery state");
}
