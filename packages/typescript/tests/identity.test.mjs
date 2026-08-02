import assert from "node:assert/strict";
import test from "node:test";

import {
  A3S_AUTOMATIC_ACTION_ID_PREFIX_V1,
  A3S_COMPONENT_IDENTITY_PREFIX_V1,
  A3S_GENERATED_ID_NAMESPACE_V1,
  Button,
  compileFrameV1,
  defineAction,
  isA3sAutomaticActionIdV1,
  isA3sComponentIdentityV1,
} from "../src/index.ts";
import { createComponentIdentityV1 } from "../src/identity.ts";
import { jsx } from "../src/jsx-runtime.ts";
import { jsxDEV } from "../src/jsx-dev-runtime.ts";

test("versioned generated identities use canonical UTF-8 length prefixes", () => {
  assert.equal(A3S_GENERATED_ID_NAMESPACE_V1, "a3s:");
  assert.equal(A3S_COMPONENT_IDENTITY_PREFIX_V1, "a3s:c1:");
  assert.equal(A3S_AUTOMATIC_ACTION_ID_PREFIX_V1, "a3s:a1:");

  const component = createComponentIdentityV1(["root", "key:行"]);
  assert.equal(component, "a3s:c1:4:root7:key:行");
  assert.equal(isA3sComponentIdentityV1(component), true);
  assert.equal(isA3sComponentIdentityV1("a3s:c1:4:root6:key:行"), false);
  assert.equal(isA3sComponentIdentityV1("a3s:c1:04:root7:key:行"), false);

  assert.equal(
    isA3sAutomaticActionIdV1("a3s:a1:4:save7:onPress"),
    true,
  );
  assert.equal(
    isA3sAutomaticActionIdV1("a3s:a1:4:save8:onPress"),
    false,
  );
  assert.equal(isA3sAutomaticActionIdV1("a3s:a1:4:save"), false);
});

test("automatic action identity follows the native host path, not component wrappers", () => {
  const handler = () => undefined;

  function SaveButton() {
    return jsx(Button, { onPress: handler, children: "Save" }, "save");
  }

  function Transparent({ children }) {
    return children;
  }

  const direct = compileFrameV1("direct", jsx(SaveButton, {}));
  const wrapped = compileFrameV1(
    "wrapped",
    jsx(Transparent, { children: jsx(SaveButton, {}) }),
  );
  const directAction = direct.frame.actions[0]?.id;
  const wrappedAction = wrapped.frame.actions[0]?.id;

  assert.equal(directAction, "a3s:a1:4:save7:onPress");
  assert.equal(wrappedAction, directAction);
  assert.equal(isA3sAutomaticActionIdV1(directAction), true);
});

test("explicit action ids cannot occupy generated identity space or violate wire text limits", () => {
  assert.throws(
    () => defineAction("a3s:manual"),
    /reserved for framework-generated identities/u,
  );
  assert.throws(
    () => defineAction(" \t "),
    /must contain non-whitespace text/u,
  );
  assert.throws(
    () => defineAction("save\u0000now"),
    /cannot contain control characters/u,
  );
  assert.throws(
    () => defineAction("界".repeat(342)),
    /exceeds the 1024-byte protocol limit/u,
  );
  assert.throws(
    () => defineAction("safe", null, { get disabled() { return false; } }),
    /disabled cannot be an accessor/u,
  );
  assert.throws(
    () => defineAction("safe", null, { future: true }),
    /unknown field future/u,
  );
});

test("oversized automatic action ids fail at the originating TSX element", () => {
  const source = { fileName: "oversized.tsx", lineNumber: 4, columnNumber: 9 };
  assert.throws(
    () => compileFrameV1(
      "oversized-action",
      jsxDEV(
        Button,
        { onPress: () => undefined, children: "Save" },
        "k".repeat(1_010),
        false,
        source,
      ),
    ),
    /oversized\.tsx:4:9: onPress automatic action id exceeds the 1024-byte protocol limit/u,
  );
});
