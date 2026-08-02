import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  A3sActionRegistryError,
  Button,
  RevisionActionRegistryV1,
  View,
  compileFrameV1,
  defineAction,
} from "../src/index.ts";
import { jsx, jsxs } from "../src/jsx-runtime.ts";

const packageRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repositoryRoot = resolve(packageRoot, "..", "..");
const protocolFixtureRoot = resolve(
  repositoryRoot,
  "tests",
  "fixtures",
  "tsx-protocol",
);
const committedFixture = readFixture("committed-counter-v1.json");
const eventFixture = readFixture("event-counter-v1.json");
const invocationTemplate = eventFixture.payload.invocations[0];

test("shared Rust event fixture dispatches through the committed counter scope", async () => {
  const received = [];
  const compiled = compileFrameV1(
    "counter",
    jsx(
      Button,
      {
        onPress: defineAction("increment", (invocation) => received.push(invocation)),
        children: "Count 0",
      },
      "increment",
    ),
  );
  const registry = new RevisionActionRegistryV1();

  registry.stage(1, compiled);
  assert.deepEqual(registry.state, {
    pending: scope(1, null, "counter", 1, 1),
    active: null,
    rollback: null,
    lastEventSequence: 0,
    dispatching: false,
  });
  registry.commit(committedFixture);

  const result = await registry.dispatch(eventFixture);
  assert.deepEqual(result, {
    renderRevision: 1,
    hostRevision: 1,
    eventSequence: 1,
    invocationCount: 1,
    callbackCount: 1,
  });
  assert.deepEqual(received, eventFixture.payload.invocations);
  assert.ok(Object.isFrozen(received[0]));
  assert.ok(Object.isFrozen(received[0].context));
  assert.ok(Object.isFrozen(received[0].context.modifiers));
});

test("reject preserves the active callbacks and permits the same revision retry", async () => {
  const calls = [];
  const registry = new RevisionActionRegistryV1();
  registry.stage(1, compileSingle("first", "action", () => calls.push("first")));
  registry.commit(committed(1, 4, "first"));

  registry.stage(2, compileSingle("second", "action", () => calls.push("second")));
  registry.reject(2);
  assert.equal(registry.state.pending, null);
  assert.equal(registry.state.active.renderRevision, 1);

  registry.stage(2, compileSingle("retry", "action", () => calls.push("retry")));
  registry.reject(2);
  await registry.dispatch(event(1, 4, 1, ["action"]));
  assert.deepEqual(calls, ["first"]);
});

test("commit mismatches leave pending and active scopes unchanged", () => {
  const registry = new RevisionActionRegistryV1();
  registry.stage(1, compileSingle("one", "one", () => undefined));
  registry.commit(committed(1, 7, "one"));
  registry.stage(2, compileSingle("two", "two", () => undefined));

  assert.throws(
    () => registry.commit(committed(2, 8, "wrong-frame")),
    /does not match pending frame/u,
  );
  assert.equal(registry.state.active.renderRevision, 1);
  assert.equal(registry.state.pending.renderRevision, 2);

  assert.throws(
    () => registry.commit(committed(3, 8, "two")),
    /revision 2 is pending/u,
  );
  assert.equal(registry.state.active.renderRevision, 1);
  assert.equal(registry.state.pending.renderRevision, 2);

  registry.commit(committed(2, 8, "two"));
  assert.equal(registry.state.active.renderRevision, 2);
  assert.equal(registry.state.rollback.renderRevision, 1);
});

test("stage validates the complete callback set before publishing", () => {
  const registry = new RevisionActionRegistryV1();
  const valid = compileSingle("atomic-stage", "known", () => undefined);
  const malformed = {
    frame: valid.frame,
    callbacks: new Map([["missing", () => undefined]]),
  };

  assert.throws(
    () => registry.stage(1, malformed),
    /does not match a compiled frame action/u,
  );
  assert.equal(registry.state.pending, null);

  const forgedGeneratedIdentity = {
    frame: {
      ...valid.frame,
      actions: [{ id: "a3s:manual", disabled: false }],
    },
    callbacks: new Map(),
  };
  assert.throws(
    () => registry.stage(1, forgedGeneratedIdentity),
    /compiled frame contains an invalid action id/u,
  );
  assert.equal(registry.state.pending, null);

  registry.stage(1, valid);
  assert.equal(registry.state.pending.renderRevision, 1);
});

test("rollback retention is bounded and stale revisions fail before callbacks", async () => {
  const calls = [];
  const registry = new RevisionActionRegistryV1();
  registry.stage(1, compileSingle("one", "one", () => calls.push("one")));
  registry.commit(committed(1, 10, "one"));
  registry.stage(2, compileSingle("two", "two", () => calls.push("two")));
  registry.commit(committed(2, 11, "two"));

  await assert.rejects(
    registry.dispatch(event(1, 10, 1, ["one"])),
    /event render revision 1 is stale/u,
  );
  assert.deepEqual(calls, []);
  assert.equal(registry.state.lastEventSequence, 0);

  await registry.dispatch(event(2, 11, 1, ["two"]));
  registry.stage(3, compileSingle("three", "three", () => calls.push("three")));
  registry.commit(committed(3, 12, "three"));
  assert.equal(registry.state.active.renderRevision, 3);
  assert.equal(registry.state.rollback.renderRevision, 2);
  assert.equal(registry.state.lastEventSequence, 1);
  assert.deepEqual(calls, ["two"]);
});

test("ordered vectors await callbacks sequentially and repeat duplicate actions", async () => {
  const calls = [];
  const registry = new RevisionActionRegistryV1();
  const compiled = compileMany("ordered", [
    ["a", async () => {
      calls.push("a:start");
      await Promise.resolve();
      calls.push("a:end");
    }],
    ["b", () => calls.push("b")],
  ]);
  registry.stage(1, compiled);
  registry.commit(committed(1, 20, "ordered"));

  const result = await registry.dispatch(event(1, 20, 1, ["a", "b", "a"]));
  assert.deepEqual(calls, ["a:start", "a:end", "b", "a:start", "a:end"]);
  assert.equal(result.invocationCount, 3);
  assert.equal(result.callbackCount, 3);
});

test("unknown and disabled actions fail complete-vector preflight", async () => {
  const calls = [];
  const registry = new RevisionActionRegistryV1();
  registry.stage(1, compileMany("preflight", [
    ["known", () => calls.push("known")],
    ["disabled", () => calls.push("disabled"), true],
    ["static", null],
  ]));
  registry.commit(committed(1, 30, "preflight"));

  await assert.rejects(
    registry.dispatch(event(1, 30, 1, ["known", "missing"])),
    (error) => error instanceof A3sActionRegistryError && error.code === "unknownAction",
  );
  assert.deepEqual(calls, []);
  assert.equal(registry.state.lastEventSequence, 0);

  await assert.rejects(
    registry.dispatch(event(1, 30, 1, ["known", "disabled"])),
    /references disabled action/u,
  );
  assert.deepEqual(calls, []);
  assert.equal(registry.state.lastEventSequence, 0);

  const result = await registry.dispatch(event(1, 30, 1, ["known", "static"]));
  assert.deepEqual(calls, ["known"]);
  assert.equal(result.invocationCount, 2);
  assert.equal(result.callbackCount, 1);
});

test("callback failures consume the event once and preserve committed scopes", async () => {
  const calls = [];
  const failure = new Error("application failure");
  const registry = new RevisionActionRegistryV1();
  registry.stage(1, compileMany("failure", [
    ["fail", () => {
      calls.push("fail");
      throw failure;
    }],
    ["later", () => calls.push("later")],
  ]));
  registry.commit(committed(1, 40, "failure"));

  await assert.rejects(
    registry.dispatch(event(1, 40, 1, ["fail", "later"])),
    (error) => {
      assert.ok(error instanceof A3sActionRegistryError);
      assert.equal(error.code, "callbackFailed");
      assert.equal(error.invocationIndex, 0);
      assert.equal(error.completedCallbacks, 0);
      assert.equal(error.cause, failure);
      return true;
    },
  );
  assert.deepEqual(calls, ["fail"]);
  assert.equal(registry.state.active.renderRevision, 1);
  assert.equal(registry.state.lastEventSequence, 1);

  await assert.rejects(
    registry.dispatch(event(1, 40, 1, ["later"])),
    /event sequence 1 is invalid; expected 2/u,
  );
  await registry.dispatch(event(1, 40, 2, ["later"]));
  assert.deepEqual(calls, ["fail", "later"]);
});

test("concurrent mutation is rejected and clear releases every retained scope", async () => {
  let release;
  const blocked = new Promise((resolveBlocked) => {
    release = resolveBlocked;
  });
  const registry = new RevisionActionRegistryV1();
  registry.stage(1, compileSingle("busy", "wait", () => blocked));
  registry.commit(committed(1, 50, "busy"));

  const firstDispatch = registry.dispatch(event(1, 50, 1, ["wait"]));
  assert.equal(registry.state.dispatching, true);
  assert.throws(() => registry.clear(), /while an event vector is dispatching/u);
  await assert.rejects(
    registry.dispatch(event(1, 50, 2, ["wait"])),
    /already being dispatched/u,
  );

  release();
  await firstDispatch;
  registry.stage(2, compileSingle("pending", "next", () => undefined));
  registry.clear();
  assert.deepEqual(registry.state, {
    pending: null,
    active: null,
    rollback: null,
    lastEventSequence: 0,
    dispatching: false,
  });
});

function compileSingle(frameId, actionId, handler, disabled = false) {
  return compileFrameV1(
    frameId,
    jsx(
      Button,
      {
        onPress: defineAction(actionId, handler, { disabled }),
        children: actionId,
      },
      actionId,
    ),
  );
}

function compileMany(frameId, definitions) {
  return compileFrameV1(
    frameId,
    jsxs(View, {
      children: definitions.map(([actionId, handler, disabled = false]) =>
        jsx(
          Button,
          {
            onPress: defineAction(actionId, handler, { disabled }),
            children: actionId,
          },
          actionId,
        )
      ),
    }),
  );
}

function committed(renderRevision, hostRevision, frameId) {
  return {
    ...committedFixture,
    renderRevision,
    payload: {
      ...committedFixture.payload,
      frameId,
      hostRevision,
    },
  };
}

function event(renderRevision, hostRevision, eventSequence, actions) {
  return {
    ...eventFixture,
    renderRevision,
    payload: {
      ...eventFixture.payload,
      hostRevision,
      eventSequence,
      invocations: actions.map((action, index) => ({
        ...invocationTemplate,
        node: `${action}-${index}`,
        action,
      })),
      interactionChanges: [],
    },
  };
}

function scope(renderRevision, hostRevision, frameId, actionCount, callbackCount) {
  return { renderRevision, hostRevision, frameId, actionCount, callbackCount };
}

function readFixture(name) {
  return JSON.parse(readFileSync(resolve(protocolFixtureRoot, name), "utf8"));
}
