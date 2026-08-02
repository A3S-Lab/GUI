const crashAfterCommit = process.argv.includes("--crash-after-commit");
const keyboardThenStale = process.argv.includes("--keyboard-then-stale");
const maximumFrameBytes = 16 * 1024 * 1024;
let buffered = Buffer.alloc(0);
let firstAction = null;
let hostMessageId = 0;
let hostRevision = 0;
let sessionId = null;

process.stdin.on("data", (chunk) => {
  buffered = Buffer.concat([buffered, chunk]);
  while (buffered.length >= 4) {
    const length = buffered.readUInt32LE(0);
    if (length === 0 || length > maximumFrameBytes || buffered.length < length + 4) {
      return;
    }
    const message = JSON.parse(buffered.subarray(4, length + 4).toString("utf8"));
    buffered = buffered.subarray(length + 4);
    handle(message);
  }
});

function handle(message) {
  if (sessionId === null) {
    sessionId = message.sessionId;
    write({
      type: "welcome",
      protocol: "a3s.gui.tsx",
      protocolVersion: 1,
      sessionId,
      messageId: ++hostMessageId,
      renderRevision: 0,
      payload: {
        selectedProtocolVersion: 1,
        hostVersion: "0.1.0",
        hostBuildId: "recovering-process-fixture",
        platform: "headless",
        renderer: "software",
        limits: { maximumFrameBytes, maximumInFlightRenders: 1 },
        capabilities: ["headlessRendering", "selfDrawnRendering"],
      },
    });
    return;
  }
  if (message.type === "render") {
    hostRevision += 1;
    firstAction ??= message.payload.actions[0]?.id ?? null;
    write({
      type: "committed",
      protocol: "a3s.gui.tsx",
      protocolVersion: 1,
      sessionId,
      messageId: ++hostMessageId,
      renderRevision: message.renderRevision,
      payload: {
        frameId: message.payload.frameId,
        hostRevision,
        rootId: "root",
        layoutFingerprint: "0000000000000000",
        sceneFingerprint: "0000000000000000",
      },
    }, () => afterCommit(message));
    return;
  }
  if (message.type === "close") {
    write({ ...message, messageId: ++hostMessageId }, () => process.exit(0));
    return;
  }
  process.exit(3);
}

function afterCommit(render) {
  if (crashAfterCommit) {
    process.stderr.write("injected post-commit crash\n", () => process.exit(17));
    return;
  }
  if (!keyboardThenStale) {
    return;
  }
  if (firstAction === null) {
    process.exit(4);
    return;
  }
  if (render.renderRevision === 1) {
    write(event(firstAction, 1, 1));
  } else if (render.renderRevision === 2) {
    write(event(firstAction, 1, 2));
  }
}

function event(action, renderRevision, eventSequence) {
  return {
    type: "event",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId,
    messageId: ++hostMessageId,
    renderRevision,
    payload: {
      hostRevision,
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

function write(message, callback) {
  const payload = Buffer.from(JSON.stringify(message), "utf8");
  const header = Buffer.allocUnsafe(4);
  header.writeUInt32LE(payload.length);
  process.stdout.write(Buffer.concat([header, payload]), callback);
}
