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

assert.equal(app.state.committedRenders, 2);
assert.equal(app.state.session.lastClientMessageId, 3);
assert.equal(app.state.session.lastHostMessageId, 3);
assert.equal(app.state.session.committedRenderRevision, 2);
assert.equal(app.state.session.committedHostRevision, 1);
assert.equal(host.state.status, "open");

await app.shutdown();
assert.equal(host.state.status, "closed");
