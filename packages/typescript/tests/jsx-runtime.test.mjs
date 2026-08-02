import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  Button,
  Text,
  View,
  Window,
  compileFrameV1,
  defineAction,
} from "../src/index.ts";
import { Fragment, jsx, jsxs } from "../src/jsx-runtime.ts";
import { jsxDEV } from "../src/jsx-dev-runtime.ts";
import { counterCompilation } from "./fixtures/counter.automatic.mjs";

const packageRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repositoryRoot = resolve(packageRoot, "..", "..");
const counterFixturePath = resolve(
  repositoryRoot,
  "tests",
  "fixtures",
  "tsx-protocol",
  "render-counter-v1.json",
);

test("automatic runtime creates immutable elements and normalizes keys", () => {
  const element = jsx(Button, { key: "props-key", children: "Save" }, "argument-key");

  assert.equal(element.key, "argument-key");
  assert.equal(element.type, "Button");
  assert.equal(element.props.key, undefined);
  assert.equal(element.props.children, "Save");
  assert.equal(element.staticChildren, false);
  assert.ok(Object.isFrozen(element));
  assert.ok(Object.isFrozen(element.props));
  assert.throws(
    () => jsx(Button, { ref: {} }),
    /refs are not part of protocol 1/u,
  );
});

test("static arrays and fragments flatten in source order with deterministic keys", () => {
  const root = jsxs(View, {
    children: [
      "A",
      null,
      false,
      jsxs(Fragment, { children: ["B", 3] }),
      jsx(Text, { children: "C" }),
    ],
  });
  const { frame } = compileFrameV1("normalization", root);

  assert.equal(frame.root.kind, "element");
  assert.deepEqual(
    frame.root.children.map((child) => [child.kind, child.key]),
    [
      ["text", "text-0"],
      ["text", "text-1"],
      ["text", "text-2"],
      ["element", "child-0"],
    ],
  );
  assert.deepEqual(
    frame.root.children.slice(0, 3).map((child) => child.value),
    ["A", "B", "3"],
  );
});

test("mutable arrays require explicit keys and preserve identity when reordered", () => {
  assert.throws(
    () => compileFrameV1("missing-keys", jsx(View, {
      children: [jsx(Text, { children: "A" })],
    })),
    /mutable JSX arrays require an explicit key/u,
  );

  const render = (items) => compileFrameV1(
    "keyed",
    jsx(View, {
      children: items.map((item) => jsx(Text, { children: item }, item)),
    }),
  ).frame;
  const first = render(["alpha", "beta"]);
  const second = render(["beta", "alpha"]);
  assert.equal(first.root.kind, "element");
  assert.equal(second.root.kind, "element");
  assert.deepEqual(first.root.children.map((child) => child.key), ["alpha", "beta"]);
  assert.deepEqual(second.root.children.map((child) => child.key), ["beta", "alpha"]);

  assert.throws(
    () => render(["same", "same"]),
    /duplicate key "same"/u,
  );

  function EmptyComponent() {
    return null;
  }
  assert.throws(
    () => compileFrameV1("empty-missing-key", jsx(View, {
      children: [jsxDEV(
        EmptyComponent,
        {},
        undefined,
        false,
        { fileName: "empty-list.tsx", lineNumber: 4, columnNumber: 9 },
      )],
    })),
    /empty-list\.tsx:4:9: mutable JSX arrays require an explicit key/u,
  );
});

test("function components resolve before transport and async components fail", () => {
  function LabelledButton({ label, onPress }) {
    return jsx(Button, { onPress, children: label });
  }

  const handler = () => undefined;
  const { frame, callbacks } = compileFrameV1(
    "component",
    jsx(LabelledButton, { label: "Save", onPress: handler }),
  );
  assert.equal(frame.root.kind, "element");
  assert.equal(frame.root.tag, "Button");
  assert.equal(frame.root.key, "root");
  const actionId = frame.root.props.events.onPress;
  assert.equal(callbacks.get(actionId), handler);

  async function AsyncComponent() {
    return jsx(Text, { children: "later" });
  }
  assert.throws(
    () => compileFrameV1("async", jsx(AsyncComponent, {})),
    /returned a promise/u,
  );
});

test("props become canonical protocol fields and callbacks stay out of the wire frame", () => {
  const handler = () => "handled";
  const { frame, callbacks } = compileFrameV1(
    "props",
    jsx(Button, {
      className: "rounded p-2",
      style: { zIndex: 2, color: "red", enabled: true },
      "data-z": 2,
      "data-a": true,
      "aria-disabled": true,
      onPress: handler,
      children: "Save",
    }, "save"),
  );
  assert.equal(frame.root.kind, "element");
  const props = frame.root.props;
  assert.equal(props.className, "rounded p-2");
  assert.equal(props.isDisabled, true);
  assert.deepEqual(props.style, { color: "red", enabled: true, zIndex: 2 });
  assert.deepEqual(props.attributes, {
    "aria-disabled": "true",
    "data-a": "true",
    "data-z": "2",
  });
  assert.deepEqual(props.explicitProps, [
    "className",
    "data-a",
    "data-z",
    "isDisabled",
    "onPress",
    "style",
  ]);

  const actionId = "a3s:a1:4:save7:onPress";
  assert.equal(props.events.onPress, actionId);
  assert.deepEqual(frame.actions, [{ id: actionId, disabled: true }]);
  assert.equal(callbacks.get(actionId), handler);
  assert.equal(callbacks.set, undefined);
  assert.doesNotMatch(JSON.stringify(frame), /handled|function/u);

  assert.throws(
    () => compileFrameV1("bad-style", jsx(View, { style: "color: red" })),
    /style must be a plain object/u,
  );
  assert.throws(
    () => compileFrameV1("bad-prop", jsx(View, { collection: { unsafe: true } })),
    /collection must be a portable/u,
  );
  const prototypeProp = {};
  Object.defineProperty(prototypeProp, "__proto__", {
    enumerable: true,
    value: "unsafe",
  });
  assert.throws(
    () => compileFrameV1("bad-name", jsx(View, prototypeProp)),
    /"__proto__" is not a portable attribute name/u,
  );
});

test("explicit actions produce the shared Rust-canonical counter fixture", () => {
  const fixtureSource = readFileSync(counterFixturePath, "utf8").trim();
  const fixture = JSON.parse(fixtureSource);
  assert.deepEqual(counterCompilation.frame, fixture.payload);
  assert.equal(counterCompilation.callbacks.size, 0);

  const message = {
    type: "render",
    protocol: "a3s.gui.tsx",
    protocolVersion: 1,
    sessionId: "tsx-fixture",
    messageId: 2,
    renderRevision: 1,
    payload: counterCompilation.frame,
  };
  assert.equal(JSON.stringify(message), fixtureSource);
});

test("Window lowers to session metadata and never becomes a content node", () => {
  const close = () => undefined;
  const { frame, callbacks } = compileFrameV1(
    "window",
    jsx(Window, {
      title: "Counter",
      width: 360,
      height: 220,
      minWidth: 300,
      resizable: false,
      onClose: close,
      children: jsxs(View, { children: ["Count 0", jsx(Button, { children: "Increment" })] }),
    }),
  );

  assert.equal(frame.root.kind, "element");
  assert.equal(frame.root.tag, "View");
  assert.equal(frame.root.key, "root");
  assert.deepEqual(frame.window, {
    title: "Counter",
    onClose: "a3s:a1:7:$window7:onClose",
    width: 360,
    height: 220,
    minWidth: 300,
    resizable: false,
  });
  assert.equal(callbacks.get(frame.window.onClose), close);
  assert.ok(frame.actions.some((action) => action.id === frame.window.onClose));

  assert.throws(
    () => compileFrameV1("multiple", jsxs(Fragment, {
      children: [jsx(View, {}), jsx(View, {})],
    })),
    /root fragment must resolve to exactly one/u,
  );
});

test("development diagnostics retain source provenance", () => {
  const source = { fileName: "counter.tsx", lineNumber: 8, columnNumber: 11 };
  assert.throws(
    () => jsxDEV(View, { children: Promise.resolve("later") }, undefined, false, source),
    /counter\.tsx:8:11: promise and thenable children/u,
  );
  assert.throws(
    () => jsxDEV(View, { children: 1n }, undefined, false, source),
    /counter\.tsx:8:11: bigint values/u,
  );
  assert.throws(
    () => jsxDEV(View, { children: Symbol("later") }, undefined, false, source),
    /counter\.tsx:8:11: symbol values/u,
  );
  assert.throws(
    () => jsxDEV(View, { children: {} }, undefined, false, source),
    /counter\.tsx:8:11: plain objects/u,
  );
  assert.throws(
    () => jsxDEV(View, { children: new Date() }, undefined, false, source),
    /counter\.tsx:8:11: Date instances/u,
  );
});

test("action ids are collision-safe and conflicting explicit actions fail", () => {
  const first = () => 1;
  const second = () => 2;
  const shared = defineAction("shared", first, { label: "First" });
  const frame = compileFrameV1(
    "scopes",
    jsxs(View, {
      children: [
        jsx(Button, { onPress: () => 1 }, "a/b"),
        jsx(Button, { onPress: () => 2 }, "a"),
      ],
    }, "root"),
  ).frame;
  assert.equal(frame.root.kind, "element");
  assert.notEqual(
    frame.root.children[0].props.events.onPress,
    frame.root.children[1].props.events.onPress,
  );

  assert.throws(
    () => compileFrameV1(
      "conflict",
      jsxs(View, {
        children: [
          jsx(Button, { onPress: shared }, "first"),
          jsx(Button, {
            onPress: defineAction("shared", second, { label: "Second" }),
          }, "second"),
        ],
      }),
    ),
    /conflicting callback metadata/u,
  );
  assert.throws(
    () => defineAction("10"),
    /cannot be canonical JavaScript array-index names/u,
  );
});
