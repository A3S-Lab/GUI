import assert from "node:assert/strict";
import test from "node:test";

import {
  Button,
  Text,
  View,
  createApp,
  useEffect,
  useMemo,
  useReducer,
  useRef,
  useState,
} from "../src/index.ts";
import { jsx, jsxs } from "../src/jsx-runtime.ts";
import { jsxDEV } from "../src/jsx-dev-runtime.ts";

test("createApp batches state and reducer updates into one committed revision", async () => {
  const host = new RecordingHost();
  let memoCalls = 0;

  function Counter() {
    const [count, setCount] = useState(0);
    const [reduced, dispatch] = useReducer((value, delta) => value + delta, 0);
    const renders = useRef(0);
    renders.current += 1;
    const total = useMemo(() => {
      memoCalls += 1;
      return count + reduced;
    }, [count, reduced]);

    return jsxs(View, {
      children: [
        jsx(Text, { children: `total:${total};renders:${renders.current}` }, "value"),
        jsx(Button, {
          onPress: async () => {
            setCount((value) => value + 1);
            await Promise.resolve();
            setCount((value) => value + 1);
            dispatch(1);
            dispatch(2);
          },
          children: "Increment",
        }, "increment"),
      ],
    });
  }

  const app = createApp(Counter, { frameId: "stateful-counter", host });
  await app.start();

  assert.equal(host.candidates.length, 1);
  assert.equal(host.last.type, "render");
  assert.equal(host.last.sessionId, "application-test");
  assert.equal(host.last.messageId, 2);
  assert.equal(textContent(host.last.payload.root), "total:0;renders:1Increment");
  assert.equal(memoCalls, 1);
  assert.equal(app.state.session.lastClientMessageId, 2);
  assert.equal(app.state.session.lastHostMessageId, 2);

  await app.dispatch(host.event(1));

  assert.equal(host.candidates.length, 2);
  assert.equal(host.last.renderRevision, 2);
  assert.equal(host.last.messageId, 3);
  assert.equal(textContent(host.last.payload.root), "total:5;renders:2Increment");
  assert.equal(memoCalls, 2);
  assert.equal(app.state.actions.active.renderRevision, 2);
  assert.equal(app.state.session.lastHostMessageId, 4);

  await app.rerender();
  assert.equal(host.candidates.length, 3);
  assert.equal(host.last.messageId, 4);
  assert.equal(textContent(host.last.payload.root), "total:5;renders:3Increment");
  assert.equal(memoCalls, 2);
  await app.shutdown();
  assert.equal(host.closeCount, 1);
  assert.equal(app.state.session.status, "closed");
});

test("keyed component instances retain state and clean effects after commit", async () => {
  const host = new RecordingHost();
  const rowSetters = new Map();
  const lifecycle = [];
  let setOrder;

  function Row({ id }) {
    const [value, setValue] = useState(0);
    rowSetters.set(id, setValue);
    useEffect(() => {
      lifecycle.push(`mount:${id}`);
      return () => lifecycle.push(`cleanup:${id}`);
    }, [id]);
    return jsx(Text, { children: `${id}:${value}` });
  }

  function Rows() {
    const [order, updateOrder] = useState(["a", "b"]);
    setOrder = updateOrder;
    return jsx(View, {
      children: order.map((id) => jsx(Row, { id }, id)),
    });
  }

  const app = createApp(Rows, { frameId: "keyed-rows", host });
  await app.start();
  assert.equal(textContent(host.last.payload.root), "a:0b:0");
  assert.deepEqual(lifecycle, ["mount:a", "mount:b"]);

  rowSetters.get("a")(7);
  await app.flush();
  setOrder(["b", "a"]);
  await app.flush();

  assert.equal(textContent(host.last.payload.root), "b:0a:7");
  assert.deepEqual(lifecycle, ["mount:a", "mount:b"]);

  setOrder(["b"]);
  await app.flush();
  assert.deepEqual(lifecycle, ["mount:a", "mount:b", "cleanup:a"]);

  const committedAfterRemoval = host.candidates.length;
  rowSetters.get("a")(99);
  await Promise.resolve();
  assert.equal(await app.flush(), false);
  assert.equal(host.candidates.length, committedAfterRemoval);

  await app.shutdown();
  assert.deepEqual(lifecycle, [
    "mount:a",
    "mount:b",
    "cleanup:a",
    "cleanup:b",
  ]);
});

test("host rejection preserves the committed frame, callbacks, and effects", async () => {
  const host = new RecordingHost();
  const lifecycle = [];

  function Counter() {
    const [count, setCount] = useState(0);
    useEffect(() => {
      lifecycle.push(`effect:${count}`);
      return () => lifecycle.push(`cleanup:${count}`);
    }, [count]);
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

  const app = createApp(Counter, { frameId: "rejected-counter", host });
  await app.start();
  assert.deepEqual(lifecycle, ["effect:0"]);

  host.rejectNext = true;
  await assert.rejects(app.dispatch(host.event(1)), /injected host rejection/u);

  assert.equal(app.state.actions.active.renderRevision, 1);
  assert.equal(app.state.actions.pending, null);
  assert.equal(app.state.session.lastClientMessageId, 3);
  assert.equal(app.state.session.lastHostMessageId, 3);
  assert.equal(app.state.session.pendingRenderRevision, null);
  assert.match(app.state.lastError.message, /injected host rejection/u);
  assert.equal(textContent(host.committed.at(-1).payload.root), "count:0Increment");
  assert.deepEqual(lifecycle, ["effect:0"]);

  await app.rerender();
  assert.equal(host.last.renderRevision, 2);
  assert.equal(host.last.messageId, 4);
  assert.equal(textContent(host.last.payload.root), "count:1Increment");
  assert.deepEqual(lifecycle, ["effect:0", "cleanup:0", "effect:1"]);
  await app.shutdown();
  assert.deepEqual(lifecycle, [
    "effect:0",
    "cleanup:0",
    "effect:1",
    "cleanup:1",
  ]);
});

test("hook order failures are source-located and leave the active revision intact", async () => {
  const host = new RecordingHost();
  let setExpanded;

  function Conditional({ expanded }) {
    useState("first");
    if (expanded) {
      useState("second");
    }
    return jsx(Text, { children: expanded ? "expanded" : "collapsed" });
  }

  function Root() {
    const [expanded, updateExpanded] = useState(false);
    setExpanded = updateExpanded;
    return jsxDEV(
      Conditional,
      { expanded },
      undefined,
      false,
      { fileName: "conditional.tsx", lineNumber: 12, columnNumber: 7 },
    );
  }

  const app = createApp(Root, { frameId: "hook-order", host });
  await app.start();
  assert.equal(textContent(host.last.payload.root), "collapsed");

  setExpanded(true);
  await assert.rejects(
    app.flush(),
    /conditional\.tsx:12:7: Conditional rendered more hooks than its committed instance/u,
  );
  assert.equal(app.state.actions.active.renderRevision, 1);
  assert.equal(host.candidates.length, 1);

  setExpanded(false);
  await app.flush();
  assert.equal(host.last.renderRevision, 2);
  assert.equal(textContent(host.last.payload.root), "collapsed");
  await app.shutdown();
});

test("callback failures still commit earlier state side effects once", async () => {
  const host = new RecordingHost();

  function FailingCounter() {
    const [count, setCount] = useState(0);
    return jsxs(View, {
      children: [
        jsx(Text, { children: `count:${count}` }, "value"),
        jsx(Button, {
          onPress: () => {
            setCount((value) => value + 1);
            throw new Error("injected callback failure");
          },
          children: "Fail",
        }, "fail"),
      ],
    });
  }

  const app = createApp(FailingCounter, { frameId: "failing-counter", host });
  await app.start();

  await assert.rejects(
    app.dispatch(host.event(1)),
    /callback failed at invocation 0/u,
  );
  assert.equal(host.candidates.length, 2);
  assert.equal(app.state.actions.active.renderRevision, 2);
  assert.equal(app.state.actions.lastEventSequence, 1);
  assert.equal(app.state.session.lastClientMessageId, 3);
  assert.equal(app.state.session.lastHostMessageId, 4);
  assert.equal(textContent(host.last.payload.root), "count:1Fail");
  await app.shutdown();
});

test("updates while a render is in flight coalesce into one following revision", async () => {
  const host = new DeferredHost();
  let setCount;

  function Counter() {
    const [count, updateCount] = useState(0);
    setCount = updateCount;
    return jsx(Text, { children: `count:${count}` });
  }

  const app = createApp(Counter, { frameId: "deferred-counter", host });
  const starting = app.start();
  await waitFor(() => host.candidates.length === 1);
  host.commitNext();
  await starting;

  setCount(1);
  await waitFor(() => host.candidates.length === 2);
  assert.equal(textContent(host.last.payload.root), "count:1");

  setCount(2);
  setCount(3);
  await Promise.resolve();
  assert.equal(host.candidates.length, 2);

  host.commitNext();
  await waitFor(() => host.candidates.length === 3);
  assert.equal(host.last.renderRevision, 3);
  assert.equal(textContent(host.last.payload.root), "count:3");

  host.commitNext();
  await app.flush();
  assert.equal(app.state.committedRenders, 3);
  await app.shutdown();
});

class RecordingHost {
  welcome = welcome("application-test");
  candidates = [];
  committed = [];
  closeCount = 0;
  hostRevision = 0;
  hostMessageId = 1;
  rejectNext = false;

  get last() {
    return this.candidates.at(-1);
  }

  async submitRender(candidate) {
    const recorded = {
      ...candidate,
      hostRevision: this.hostRevision + (this.rejectNext ? 0 : 1),
    };
    this.candidates.push(recorded);
    if (this.rejectNext) {
      this.rejectNext = false;
      throw new Error("injected host rejection");
    }
    this.hostRevision += 1;
    this.hostMessageId += 1;
    this.committed.push(recorded);
    return committed(candidate, this.hostMessageId, this.hostRevision);
  }

  event(eventSequence) {
    this.hostMessageId += 1;
    return eventFor(this.last, this.hostMessageId, eventSequence);
  }

  async close() {
    this.closeCount += 1;
  }
}

class DeferredHost {
  welcome = welcome("application-test");
  candidates = [];
  pending = [];
  hostRevision = 0;
  hostMessageId = 1;

  get last() {
    return this.candidates.at(-1);
  }

  submitRender(candidate) {
    this.candidates.push(candidate);
    return new Promise((resolve) => this.pending.push({ candidate, resolve }));
  }

  commitNext() {
    const pending = this.pending.shift();
    assert.ok(pending);
    this.hostRevision += 1;
    this.hostMessageId += 1;
    pending.resolve(committed(pending.candidate, this.hostMessageId, this.hostRevision));
  }

}

function committed(candidate, messageId, hostRevision) {
  return {
    type: "committed",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "application-test",
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

function eventFor(candidate, messageId, eventSequence) {
  const action = candidate.payload.actions[0]?.id;
  assert.ok(action);
  return {
    type: "event",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "application-test",
    messageId,
    renderRevision: candidate.renderRevision,
    payload: {
      hostRevision: candidate.hostRevision,
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
      hostBuildId: "application-test",
      platform: "headless",
      renderer: "software",
      limits: {
        maximumFrameBytes: 16 * 1024 * 1024,
        maximumInFlightRenders: 1,
      },
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
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (predicate()) {
      return;
    }
    await Promise.resolve();
  }
  assert.fail("condition did not become true within 100 microtasks");
}
