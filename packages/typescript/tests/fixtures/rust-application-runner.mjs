import assert from "node:assert/strict";

import {
  Text,
  createApp,
} from "../../src/index.ts";
import { jsx } from "../../src/jsx-runtime.ts";

const runner = createApp(
  () => jsx(Text, { children: "zero-configuration self-drawn host" }),
  { frameId: "automatic-rust-host" },
);
const app = await runner.run();

assert.equal(app.state.status, "running");
assert.equal(app.state.committedRenders, 1);
assert.match(
  app.state.session.sessionId,
  /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u,
);
assert.equal(app.state.session.lastClientMessageId, 2);
assert.equal(app.state.session.lastHostMessageId, 2);
assert.equal(app.host.state.status, "open");

await app.host.ping(91);
assert.equal(app.state.session.lastClientMessageId, 3);
assert.equal(app.state.session.lastHostMessageId, 3);

await app.shutdown();
assert.equal(app.state.status, "closed");
assert.equal(app.host.state.status, "closed");
assert.equal(app.state.session.lastClientMessageId, 4);
assert.equal(app.state.session.lastHostMessageId, 4);
