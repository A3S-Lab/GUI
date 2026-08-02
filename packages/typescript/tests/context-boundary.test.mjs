import assert from "node:assert/strict";
import test from "node:test";

import {
  A3sJsxError,
  ErrorBoundary,
  Text,
  View,
  createApp,
  createContext,
  useContext,
  useEffect,
  useState,
} from "../src/index.ts";
import { jsx, jsxs } from "../src/jsx-runtime.ts";
import { jsxDEV } from "../src/jsx-dev-runtime.ts";

test("context defaults and nested providers remain outside the wire tree", async () => {
  const Theme = createContext("default");
  const host = new ImmediateHost();
  let setTheme;

  function Reader({ name }) {
    const theme = useContext(Theme);
    return jsx(Text, { children: `${name}:${theme}` });
  }

  function ContextApp() {
    const [theme, updateTheme] = useState("outer");
    setTheme = updateTheme;
    return jsxs(View, {
      children: [
        jsx(Reader, { name: "default" }, "default"),
        jsxs(Theme.Provider, {
          value: theme,
          children: [
            jsx(Reader, { name: "outer" }, "outer"),
            jsx(Theme.Provider, {
              value: "inner",
              children: jsx(Reader, { name: "inner" }),
            }, "inner-provider"),
          ],
        }, "outer-provider"),
      ],
    });
  }

  const app = createApp(ContextApp, { frameId: "context", host });
  await app.start();
  assert.equal(
    textContent(host.last.payload.root),
    "default:defaultouter:outerinner:inner",
  );
  assert.deepEqual(elementTags(host.last.payload.root), ["View", "Text", "Text", "Text"]);

  setTheme("updated");
  await app.flush();
  assert.equal(
    textContent(host.last.payload.root),
    "default:defaultouter:updatedinner:inner",
  );
  assert.equal(app.state.activeComponents, 4);
  await app.shutdown();
});

test("error boundaries roll back partial candidates before committing fallback", async () => {
  const host = new ImmediateHost();
  const lifecycle = [];
  let setFail;
  let capturedError;

  function Probe() {
    useEffect(() => {
      lifecycle.push("mount:probe");
      return () => lifecycle.push("cleanup:probe");
    }, []);
    return jsx(Text, { children: "probe" });
  }

  function Faulty({ fail }) {
    useEffect(() => {
      lifecycle.push("mount:faulty");
      return () => lifecycle.push("cleanup:faulty");
    }, []);
    if (fail) {
      throw new Error("injected render failure");
    }
    return jsx(Text, { children: "healthy" });
  }

  function Fallback({ error }) {
    useEffect(() => {
      lifecycle.push("mount:fallback");
      return () => lifecycle.push("cleanup:fallback");
    }, []);
    return jsx(Text, { children: `fallback:${error.source.fileName}` });
  }

  function BoundaryApp() {
    const [fail, updateFail] = useState(false);
    setFail = updateFail;
    return jsx(View, {
      children: jsxs(ErrorBoundary, {
        fallback: (error) => {
          capturedError = error;
          return jsx(Fallback, { error });
        },
        children: [
          jsx(Probe, {}, "probe"),
          jsxDEV(
            Faulty,
            { fail },
            "faulty",
            false,
            { fileName: "faulty.tsx", lineNumber: 9, columnNumber: 5 },
          ),
        ],
      }, "content"),
    });
  }

  const app = createApp(BoundaryApp, { frameId: "boundary", host });
  await app.start();
  assert.equal(textContent(host.last.payload.root), "probehealthy");
  assert.deepEqual(lifecycle, ["mount:probe", "mount:faulty"]);

  setFail(true);
  await app.flush();
  assert.equal(textContent(host.last.payload.root), "fallback:faulty.tsx");
  assert.ok(capturedError instanceof A3sJsxError);
  assert.equal(capturedError.source.fileName, "faulty.tsx");
  assert.deepEqual(lifecycle, [
    "mount:probe",
    "mount:faulty",
    "cleanup:faulty",
    "cleanup:probe",
    "mount:fallback",
  ]);

  setFail(false);
  await app.flush();
  assert.equal(textContent(host.last.payload.root), "probehealthy");
  assert.deepEqual(lifecycle, [
    "mount:probe",
    "mount:faulty",
    "cleanup:faulty",
    "cleanup:probe",
    "mount:fallback",
    "cleanup:fallback",
    "mount:probe",
    "mount:faulty",
  ]);
  await app.shutdown();
});

test("a failing boundary fallback preserves the last committed subtree", async () => {
  const host = new ImmediateHost();
  const lifecycle = [];
  let setFail;
  let incrementStable;

  function Stable() {
    const [count, setCount] = useState(0);
    incrementStable = () => setCount((value) => value + 1);
    useEffect(() => {
      lifecycle.push("mount:stable");
      return () => lifecycle.push("cleanup:stable");
    }, []);
    return jsx(Text, { children: `stable:${count}` });
  }

  function Child({ fail }) {
    useEffect(() => {
      lifecycle.push("mount:child");
      return () => lifecycle.push("cleanup:child");
    }, []);
    if (fail) {
      throw new Error("child failed");
    }
    return jsx(Text, { children: "committed-child" });
  }

  function App() {
    const [fail, updateFail] = useState(false);
    setFail = updateFail;
    return jsx(View, {
      children: jsxDEV(
        ErrorBoundary,
        {
          fallback: () => {
            throw new Error("fallback failed");
          },
          children: [
            jsx(Stable, {}, "stable"),
            jsx(Child, { fail }, "child"),
          ],
        },
        undefined,
        false,
        { fileName: "boundary.tsx", lineNumber: 3, columnNumber: 3 },
      ),
    });
  }

  const app = createApp(App, { frameId: "failing-boundary", host });
  await app.start();
  assert.deepEqual(lifecycle, ["mount:stable", "mount:child"]);

  setFail(true);
  await assert.rejects(
    app.flush(),
    /boundary\.tsx:3:3: error boundary fallback threw while rendering/u,
  );
  assert.equal(host.candidates.length, 1);
  assert.equal(app.state.actions.active.renderRevision, 1);
  assert.equal(textContent(host.last.payload.root), "stable:0committed-child");
  assert.deepEqual(lifecycle, ["mount:stable", "mount:child"]);

  setFail(false);
  incrementStable();
  await app.flush();
  assert.equal(host.last.renderRevision, 2);
  assert.equal(textContent(host.last.payload.root), "stable:1committed-child");
  assert.deepEqual(lifecycle, ["mount:stable", "mount:child"]);
  await app.shutdown();
  assert.deepEqual(lifecycle, [
    "mount:stable",
    "mount:child",
    "cleanup:child",
    "cleanup:stable",
  ]);
});

class ImmediateHost {
  welcome = welcome("context-boundary-test");
  candidates = [];
  hostRevision = 0;
  hostMessageId = 1;

  get last() {
    return this.candidates.at(-1);
  }

  async submitRender(candidate) {
    this.hostRevision += 1;
    this.hostMessageId += 1;
    const recorded = { ...candidate, hostRevision: this.hostRevision };
    this.candidates.push(recorded);
    return {
      type: "committed",
      protocol: "a3s.gui.tsx",
      protocolVersion: 1,
      sessionId: "context-boundary-test",
      messageId: this.hostMessageId,
      renderRevision: candidate.renderRevision,
      payload: {
        frameId: candidate.payload.frameId,
        hostRevision: this.hostRevision,
        rootId: "root",
        layoutFingerprint: "0000000000000000",
        sceneFingerprint: "0000000000000000",
      },
    };
  }
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
      hostBuildId: "context-boundary-test",
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

function elementTags(node) {
  if (node.kind === "text") {
    return [];
  }
  return [node.tag, ...node.children.flatMap(elementTags)];
}
