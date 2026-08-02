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

test("createApp run connects, binds events, and returns the running application", async () => {
  const runtime = new RecordingRuntime();

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

  const runner = createApp(Counter, { frameId: "automatic-runner" });
  const app = await runner.run({ runtime });

  assert.equal(runtime.connectCount, 1);
  assert.equal(app.state.status, "running");
  assert.equal(app.state.committedRenders, 1);
  assert.equal(app.host, runtime.host);
  assert.equal(textContent(runtime.host.last.payload.root), "count:0Increment");

  await runtime.press();
  assert.equal(app.state.committedRenders, 2);
  assert.equal(textContent(runtime.host.last.payload.root), "count:1Increment");

  await app.shutdown();
  assert.equal(runtime.host.closeCount, 1);
  assert.equal(app.state.status, "closed");
});

test("createApp run closes a connected host when the initial render fails", async () => {
  const runtime = new RecordingRuntime({ rejectRender: true });
  const runner = createApp(() => jsx(Text, { children: "failure" }));

  await assert.rejects(runner.run({ runtime }), /injected render failure/u);
  assert.equal(runtime.connectCount, 1);
  assert.equal(runtime.host.closeCount, 1);
  await assert.rejects(runner.run({ runtime }), /cannot run.*failed/u);
});

test("createApp run options reject accessors before connecting", async () => {
  const runtime = new RecordingRuntime();
  let getterCalls = 0;
  const options = {};
  Object.defineProperty(options, "runtime", {
    enumerable: true,
    get() {
      getterCalls += 1;
      return runtime;
    },
  });
  const runner = createApp(() => jsx(Text, { children: "strict" }));

  await assert.rejects(runner.run(options), /runtime cannot be an accessor/u);
  assert.equal(getterCalls, 0);
  assert.equal(runtime.connectCount, 0);
});

test("createApp recovery policy rejects accessors before connecting", async () => {
  const runtime = new RecordingRuntime();
  let getterCalls = 0;
  const recovery = {};
  Object.defineProperty(recovery, "maximumRestarts", {
    enumerable: true,
    get() {
      getterCalls += 1;
      return 1;
    },
  });
  const runner = createApp(() => jsx(Text, { children: "strict recovery" }));

  await assert.rejects(
    runner.run({ runtime, recovery }),
    /maximumRestarts cannot be an accessor/u,
  );
  assert.equal(getterCalls, 0);
  assert.equal(runtime.connectCount, 0);
});

test("createApp recovery policy enforces bounded restart values", async () => {
  const policies = [
    {},
    { maximumRestarts: 0 },
    { maximumRestarts: 17 },
    { maximumRestarts: 1, restartDelayMs: -1 },
    { maximumRestarts: 1, restartDelayMs: 60_001 },
    { maximumRestarts: 1, unknown: true },
  ];
  for (const recovery of policies) {
    const runtime = new RecordingRuntime();
    const runner = createApp(() => jsx(Text, { children: "bounded recovery" }));
    await assert.rejects(runner.run({ runtime, recovery }), TypeError);
    assert.equal(runtime.connectCount, 0);
  }
});

test("opt-in recovery requires an observable host lifecycle", async () => {
  const runtime = new RecordingRuntime();
  const runner = createApp(() => jsx(Text, { children: "observable host" }));

  await assert.rejects(
    runner.run({ runtime, recovery: { maximumRestarts: 1 } }),
    (error) =>
      error instanceof A3sApplicationRecoveryError &&
      error.code === "hostNotObservable" &&
      error.restartAttempts === 0,
  );
  assert.equal(runtime.connectCount, 1);
  assert.equal(runtime.host.closeCount, 1);
});

test("createApp definition options reject accessors without evaluating them", () => {
  let getterCalls = 0;
  const options = {};
  Object.defineProperty(options, "frameId", {
    enumerable: true,
    get() {
      getterCalls += 1;
      return "unsafe";
    },
  });

  assert.throws(
    () => createApp(() => jsx(Text, { children: "strict" }), options),
    /frameId cannot be an accessor/u,
  );
  assert.equal(getterCalls, 0);
});

class RecordingRuntime {
  connectCount = 0;
  host;
  #onEvent = null;

  constructor(options = {}) {
    this.host = new RecordingHost(options);
  }

  async connect(onEvent) {
    this.connectCount += 1;
    this.#onEvent = onEvent;
    return this.host;
  }

  async press() {
    assert.notEqual(this.#onEvent, null);
    await this.#onEvent(this.host.event());
  }
}

class RecordingHost {
  welcome = welcome();
  candidates = [];
  closeCount = 0;
  #hostMessageId = 1;
  #hostRevision = 0;
  #rejectRender;

  constructor(options) {
    this.#rejectRender = options.rejectRender ?? false;
  }

  get last() {
    const candidate = this.candidates.at(-1);
    assert.notEqual(candidate, undefined);
    return candidate;
  }

  async submitRender(candidate) {
    if (this.#rejectRender) {
      throw new Error("injected render failure");
    }
    this.candidates.push(candidate);
    this.#hostMessageId += 1;
    this.#hostRevision += 1;
    return committed(candidate, this.#hostMessageId, this.#hostRevision);
  }

  event() {
    const action = this.last.payload.actions[0]?.id;
    assert.notEqual(action, undefined);
    this.#hostMessageId += 1;
    return {
      type: "event",
      protocol: "a3s.gui.tsx",
      protocolVersion: 1,
      sessionId: "application-runner-test",
      messageId: this.#hostMessageId,
      renderRevision: this.last.renderRevision,
      payload: {
        hostRevision: this.#hostRevision,
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

  async close() {
    this.closeCount += 1;
  }
}

function welcome() {
  return {
    type: "welcome",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "application-runner-test",
    messageId: 1,
    renderRevision: 0,
    payload: {
      selectedProtocolVersion: 1,
      hostVersion: "0.1.0",
      hostBuildId: "application-runner-test",
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
    sessionId: "application-runner-test",
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
