import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import test from "node:test";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import {
  TSX_PROTOCOL_DECLARATION_FINGERPRINT_V1,
  TSX_PROTOCOL_NAME,
  TSX_PROTOCOL_V1_MAX_SAFE_INTEGER,
  TSX_PROTOCOL_VERSION_V1,
} from "../src/generated/protocol.ts";

const packageRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repositoryRoot = resolve(packageRoot, "..", "..");
const fixtureRoot = resolve(repositoryRoot, "tests", "fixtures", "tsx-protocol");
const declarationPath = resolve(packageRoot, "src", "generated", "protocol.ts");

function assertJsonNumbersAreLossless(value, path = "payload") {
  if (typeof value === "number") {
    assert.ok(Number.isFinite(value), `${path} must be finite`);
    if (Number.isInteger(value)) {
      assert.ok(Number.isSafeInteger(value), `${path} must be a safe integer`);
    }
    return;
  }
  if (Array.isArray(value)) {
    value.forEach((item, index) => assertJsonNumbersAreLossless(item, `${path}[${index}]`));
    return;
  }
  if (value !== null && typeof value === "object") {
    for (const [key, item] of Object.entries(value)) {
      assertJsonNumbersAreLossless(item, `${path}.${key}`);
    }
  }
}

function fnv1a64(input) {
  let hash = 0xcbf29ce484222325n;
  for (const byte of Buffer.from(input, "utf8")) {
    hash ^= BigInt(byte);
    hash = BigInt.asUintN(64, hash * 0x100000001b3n);
  }
  return hash.toString(16).padStart(16, "0");
}

for (const filename of readdirSync(fixtureRoot).filter((name) => name.endsWith(".json")).sort()) {
  test(`${filename} is canonical and JavaScript-lossless`, () => {
    const source = readFileSync(resolve(fixtureRoot, filename), "utf8").trim();
    const decoded = JSON.parse(source);
    assert.equal(JSON.stringify(decoded), source);
    assertJsonNumbersAreLossless(decoded);
  });
}

test("committed fingerprints are fixed lowercase hexadecimal strings", () => {
  const committed = JSON.parse(
    readFileSync(resolve(fixtureRoot, "committed-counter-v1.json"), "utf8"),
  );
  assert.match(committed.payload.layoutFingerprint, /^[0-9a-f]{16}$/u);
  assert.match(committed.payload.sceneFingerprint, /^[0-9a-f]{16}$/u);
});

test("generated declarations have a valid Rust-owned fingerprint", () => {
  const source = readFileSync(declarationPath, "utf8");
  const marker = "export const TSX_PROTOCOL_NAME";
  const bodyOffset = source.indexOf(marker);
  assert.notEqual(bodyOffset, -1, "declaration body marker is missing");
  const body = source.slice(bodyOffset);
  const fingerprint = fnv1a64(body);

  assert.match(source, new RegExp(`Declaration fingerprint: fnv1a64:${fingerprint}`, "u"));
  assert.match(
    source,
    new RegExp(`TSX_PROTOCOL_DECLARATION_FINGERPRINT_V1 = "fnv1a64:${fingerprint}"`, "u"),
  );
  assert.match(source, /export type TsxClientMessageV1 =/u);
  assert.match(source, /export type TsxHostMessageV1 =/u);
  assert.match(source, /layoutFingerprint: TsxFingerprintV1/u);
  assert.match(source, /importSource\?: string \| null/u);
  assert.doesNotMatch(source, /\bbigint\b/u);
  assert.doesNotMatch(source, /import_source/u);
});

test("generated protocol runtime constants load directly in Node", () => {
  assert.equal(TSX_PROTOCOL_NAME, "a3s.gui.tsx");
  assert.equal(TSX_PROTOCOL_VERSION_V1, 1);
  assert.equal(TSX_PROTOCOL_V1_MAX_SAFE_INTEGER, Number.MAX_SAFE_INTEGER);
  assert.match(TSX_PROTOCOL_DECLARATION_FINGERPRINT_V1, /^fnv1a64:[0-9a-f]{16}$/u);
});
