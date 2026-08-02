const maximumFrameBytes = 4_096;
const crashAfterWelcome = process.argv.includes("--crash-after-welcome");
let buffered = Buffer.alloc(0);
let welcomed = false;

process.stdin.on("data", (chunk) => {
  buffered = Buffer.concat([buffered, chunk]);
  while (buffered.length >= 4) {
    const length = buffered.readUInt32LE(0);
    if (length === 0 || length > maximumFrameBytes || buffered.length < 4 + length) {
      return;
    }
    const message = JSON.parse(buffered.subarray(4, 4 + length).toString("utf8"));
    buffered = buffered.subarray(4 + length);
    handle(message);
  }
});

process.stdin.on("end", () => {
  if (buffered.length !== 0) {
    process.exitCode = 2;
  }
});

function handle(message) {
  if (!welcomed) {
    if (message.type !== "hello" || message.messageId !== 1) {
      process.exit(3);
    }
    welcomed = true;
    write({
      type: "welcome",
      protocol: "a3s.gui.tsx",
      protocolVersion: 1,
      sessionId: message.sessionId,
      messageId: 1,
      renderRevision: 0,
      payload: {
        selectedProtocolVersion: 1,
        hostVersion: "0.1.0",
        hostBuildId: "process-fixture",
        platform: "headless",
        renderer: "software",
        limits: { maximumFrameBytes, maximumInFlightRenders: 1 },
        capabilities: ["headlessRendering", "selfDrawnRendering"],
      },
    }, () => {
      if (crashAfterWelcome) {
        process.stderr.write("injected host crash\n", () => process.exit(17));
      }
    });
    return;
  }

  if (message.type !== "render") {
    process.exit(4);
  }
  write({
    type: "committed",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: message.sessionId,
    messageId: 2,
    renderRevision: message.renderRevision,
    payload: {
      frameId: message.payload.frameId,
      hostRevision: 1,
      rootId: "root",
      layoutFingerprint: "0000000000000000",
      sceneFingerprint: "0000000000000000",
    },
  });
}

function write(message, callback) {
  const payload = Buffer.from(JSON.stringify(message), "utf8");
  const header = Buffer.allocUnsafe(4);
  header.writeUInt32LE(payload.length);
  process.stdout.write(Buffer.concat([header, payload]), callback);
}
