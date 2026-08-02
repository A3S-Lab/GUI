import assert from "node:assert/strict";
import test from "node:test";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  A3sTransportError,
  RevisionActionRegistryV1,
  Text,
  compileFrameV1,
  connectA3sFramedClientV1,
  spawnA3sNodeProcessTransportV1,
} from "../src/index.ts";
import { jsx } from "../src/jsx-runtime.ts";

const fixture = resolve(
  dirname(fileURLToPath(import.meta.url)),
  "fixtures",
  "framed-process-host.mjs",
);

test("Node process transport performs a real framed handshake and render round trip", async () => {
  const transport = spawnA3sNodeProcessTransportV1({
    command: process.execPath,
    args: [fixture],
    maximumStderrBytes: 4_096,
    shutdownTimeoutMs: 2_000,
  });
  const connection = await connectA3sFramedClientV1(transport, handshakeOptions());
  const compiled = compileFrameV1("process", jsx(Text, { children: "process" }));
  const actions = new RevisionActionRegistryV1();
  actions.stage(1, compiled);
  const render = connection.session.createRender(1, compiled.frame);

  await connection.writeClientMessage(render);
  const committed = await connection.readHostMessage();
  assert.equal(committed.type, "committed");
  assert.equal(committed.sessionId, "process-session");
  assert.equal(committed.messageId, 2);
  assert.equal(committed.payload.frameId, "process");
  connection.session.commitRender(committed, actions);
  assert.equal(connection.session.state.committedRenderRevision, 1);

  await connection.close();
  assert.equal(transport.state.status, "closed");
  assert.equal(transport.state.exitCode, 0);
  assert.equal(transport.state.stderr, "");
});

test("Node process transport surfaces abnormal exit with bounded stderr", async () => {
  const transport = spawnA3sNodeProcessTransportV1({
    command: process.execPath,
    args: [fixture, "--crash-after-welcome"],
    maximumStderrBytes: 12,
    shutdownTimeoutMs: 2_000,
  });
  const connection = await connectA3sFramedClientV1(transport, handshakeOptions());

  await assert.rejects(
    connection.readHostMessage(),
    (error) =>
      error instanceof A3sTransportError &&
      error.code === "processExited" &&
      error.message.includes("code 17"),
  );
  assert.equal(transport.state.stderr, "injected hos");
  assert.equal(transport.state.stderrTruncated, true);
  await connection.close();
});

test("Node process transport rejects unsafe or unbounded spawn options", () => {
  assert.throws(
    () => spawnA3sNodeProcessTransportV1({ command: "" }),
    (error) => error instanceof A3sTransportError && error.code === "invalidOptions",
  );
  assert.throws(
    () => spawnA3sNodeProcessTransportV1({
      command: process.execPath,
      maximumStderrBytes: 1_048_577,
    }),
    (error) => error instanceof A3sTransportError && error.code === "invalidOptions",
  );
  assert.throws(
    () => spawnA3sNodeProcessTransportV1({
      command: process.execPath,
      shell: true,
    }),
    (error) => error instanceof A3sTransportError && error.code === "invalidOptions",
  );
});

function handshakeOptions() {
  return {
    sdkVersion: "0.0.0-development",
    sessionId: "process-session",
    requestedRenderer: "software",
    maximumFrameBytes: 4_096,
  };
}
