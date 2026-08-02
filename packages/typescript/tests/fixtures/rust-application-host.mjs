import assert from "node:assert/strict";

import {
  Text,
  connectA3sNodeApplicationHostV1,
  createApp,
} from "../../src/index.ts";
import { jsx } from "../../src/jsx-runtime.ts";

const hostBinary = process.argv[2];
assert.equal(typeof hostBinary, "string");

const host = await connectA3sNodeApplicationHostV1({
  process: {
    command: hostBinary,
    args: [
      "--liveness-interval-ms",
      "20",
      "--liveness-timeout-ms",
      "1000",
    ],
    maximumStderrBytes: 16_384,
    shutdownTimeoutMs: 5_000,
  },
  handshake: {
    sdkVersion: "0.0.0-development",
    sessionId: "node-rust-application",
    requestedRenderer: "software",
    maximumFrameBytes: 1_048_576,
  },
});

const app = createApp(
  () => jsx(Text, { children: "Node to Rust self-drawn frame" }),
  { frameId: "node-rust-application", host },
);
host.setEventHandler(async (message) => {
  await app.dispatch(message);
});

await app.start();
await app.rerender();
await waitFor(() => host.state.receivedHostPings >= 1);
await host.ping(73);

assert.equal(app.state.committedRenders, 2);
assert.equal(host.state.receivedHostPings >= 1, true);
assert.notEqual(host.state.lastHostPingNonce, null);
assert.equal(app.state.session.lastClientMessageId >= 5, true);
assert.equal(
  app.state.session.lastClientMessageId,
  app.state.session.lastHostMessageId,
);
assert.equal(
  app.state.session.lastReceivedHostMessageId,
  app.state.session.lastHostMessageId,
);
assert.equal(app.state.session.committedRenderRevision, 2);
assert.equal(app.state.session.committedHostRevision, 1);
assert.equal(host.state.status, "open");

const messageIdBeforeClose = app.state.session.lastClientMessageId;
await app.shutdown();
assert.equal(host.state.status, "closed");
assert.equal(app.state.session.status, "closed");
assert.equal(app.state.session.lastClientMessageId >= messageIdBeforeClose + 1, true);
assert.equal(
  app.state.session.lastClientMessageId,
  app.state.session.lastHostMessageId,
);

async function waitFor(predicate) {
  for (let attempt = 0; attempt < 200; attempt += 1) {
    if (predicate()) {
      return;
    }
    await new Promise((resolve) => setTimeout(resolve, 5));
  }
  throw new Error("timed out waiting for host-initiated liveness");
}
