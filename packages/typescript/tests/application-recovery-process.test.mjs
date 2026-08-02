import assert from "node:assert/strict";
import { randomUUID } from "node:crypto";
import { fileURLToPath } from "node:url";
import test from "node:test";

import {
  Text,
  connectA3sNodeApplicationHostV1,
  createApp,
} from "../src/index.ts";
import { jsx } from "../src/jsx-runtime.ts";

test("the application runner replays after a real child host crashes", async () => {
  const fixture = fileURLToPath(
    new URL("./fixtures/recovering-process-host.mjs", import.meta.url),
  );
  const runtime = new ProcessRecoveryRuntime(fixture);
  const app = await createApp(
    () => jsx(Text, { children: "retained across process restart" }),
    { frameId: "real-process-replay" },
  ).run({
    runtime,
    recovery: { maximumRestarts: 1, restartDelayMs: 0 },
  });
  const originalSession = app.state.session.sessionId;

  await waitFor(() => app.state.hostGeneration === 2);
  assert.equal(runtime.connectCount, 2);
  assert.equal(app.state.replayedRenders, 1);
  assert.notEqual(app.state.session.sessionId, originalSession);
  assert.equal(app.state.session.committedRenderRevision, 1);
  assert.equal(app.state.status, "running");

  await app.shutdown();
});

class ProcessRecoveryRuntime {
  connectCount = 0;
  #fixture;

  constructor(fixture) {
    this.#fixture = fixture;
  }

  async connect(onEvent) {
    this.connectCount += 1;
    return connectA3sNodeApplicationHostV1({
      process: {
        command: process.execPath,
        args: [
          this.#fixture,
          ...(this.connectCount === 1 ? ["--crash-after-commit"] : []),
        ],
      },
      handshake: {
        sdkVersion: "0.0.0-development",
        sessionId: randomUUID(),
        requestedRenderer: "software",
      },
      onEvent,
      controlTimeoutMs: 1_000,
    });
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
