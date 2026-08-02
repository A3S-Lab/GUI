import assert from "node:assert/strict";
import { randomUUID } from "node:crypto";
import { fileURLToPath } from "node:url";
import test from "node:test";

import {
  Button,
  Text,
  View,
  connectA3sNodeApplicationHostV1,
  createApp,
  useState,
} from "../src/index.ts";
import { jsx, jsxs } from "../src/jsx-runtime.ts";

test("a restarted process accepts keyboard input and rejects a stale revision", async () => {
  const fixture = fileURLToPath(
    new URL("./fixtures/recovering-process-host.mjs", import.meta.url),
  );
  const runtime = new ProcessRecoveryRuntime(fixture);
  let callbackCalls = 0;

  function Counter() {
    const [count, setCount] = useState(0);
    return jsxs(View, {
      children: [
        jsx(Text, { children: `count:${count}` }, "value"),
        jsx(Button, {
          onPress: () => {
            callbackCalls += 1;
            setCount((value) => value + 1);
          },
          children: "Increment",
        }, "increment"),
      ],
    });
  }

  const app = await createApp(Counter, { frameId: "real-process-input" }).run({
    runtime,
    recovery: { maximumRestarts: 2, restartDelayMs: 0 },
  });

  await waitFor(() => app.state.hostGeneration === 3);
  assert.equal(runtime.connectCount, 3);
  assert.equal(new Set(runtime.sessionIds).size, 3);
  assert.equal(callbackCalls, 1);
  assert.equal(app.state.committedRenders, 2);
  assert.equal(app.state.replayedRenders, 2);
  assert.equal(app.state.session.committedRenderRevision, 1);
  assert.equal(app.state.actions.active.renderRevision, 1);
  assert.equal(app.state.status, "running");

  await app.shutdown();
});

class ProcessRecoveryRuntime {
  connectCount = 0;
  sessionIds = [];
  #fixture;

  constructor(fixture) {
    this.#fixture = fixture;
  }

  async connect(onEvent) {
    this.connectCount += 1;
    const mode = this.connectCount === 1
      ? "--crash-after-commit"
      : this.connectCount === 2
        ? "--keyboard-then-stale"
        : null;
    const host = await connectA3sNodeApplicationHostV1({
      process: {
        command: process.execPath,
        args: [this.#fixture, ...(mode === null ? [] : [mode])],
      },
      handshake: {
        sdkVersion: "0.0.0-development",
        sessionId: randomUUID(),
        requestedRenderer: "software",
      },
      onEvent,
      controlTimeoutMs: 1_000,
    });
    this.sessionIds.push(host.welcome.sessionId);
    return host;
  }
}

async function waitFor(predicate) {
  for (let attempt = 0; attempt < 500; attempt += 1) {
    if (predicate()) {
      return;
    }
    await new Promise((resolve) => setTimeout(resolve, 10));
  }
  throw new Error("timed out waiting for process recovery");
}
