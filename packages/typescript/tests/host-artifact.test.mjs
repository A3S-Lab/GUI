import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";

import {
  A3sHostArtifactError,
  resolveA3sHostArtifactV1,
} from "../src/host-artifact.ts";

test("host artifact resolver accepts an explicit absolute development host", async () => {
  const directory = await mkdtemp(join(tmpdir(), "a3s-gui-host-"));
  try {
    const executable = join(directory, process.platform === "win32" ? "host.exe" : "host");
    await writeFile(executable, "development-host");

    const artifact = await resolveA3sHostArtifactV1({
      environment: {
        A3S_GUI_TSX_HOST: executable,
        A3S_GUI_ALLOW_UNVERIFIED_HOST: "1",
      },
      platform: process.platform,
      arch: process.arch,
    });

    assert.deepEqual(artifact, {
      command: executable,
      source: "environment",
      packageName: null,
    });
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("host artifact resolver requires explicit consent for an unverified override", async () => {
  await assert.rejects(
    resolveA3sHostArtifactV1({
      environment: { A3S_GUI_TSX_HOST: "C:\\unverified-host.exe" },
      platform: "win32",
      arch: "x64",
    }),
    (error) => error instanceof A3sHostArtifactError && error.code === "invalidOverride",
  );
});

test("host artifact resolver validates the platform package manifest and checksum", async () => {
  const directory = await mkdtemp(join(tmpdir(), "a3s-gui-package-"));
  try {
    const executableName = "a3s-gui-tsx-host.exe";
    const executable = join(directory, executableName);
    const bytes = Buffer.from("packaged-self-drawn-host");
    await writeFile(executable, bytes);
    const manifest = join(directory, "host-manifest.json");
    const packageName = "@a3s/gui-host-win32-x64-msvc";
    await writeFile(manifest, JSON.stringify({
      schemaVersion: 1,
      packageName,
      hostVersion: "0.1.0",
      platform: "win32",
      arch: "x64",
      abi: "msvc",
      minimumProtocolVersion: 1,
      maximumProtocolVersion: 1,
      executable: executableName,
      executableBytes: bytes.byteLength,
      sha256: createHash("sha256").update(bytes).digest("hex"),
    }));

    const artifact = await resolveA3sHostArtifactV1({
      environment: {},
      platform: "win32",
      arch: "x64",
      resolvePackageManifest: (requested) => {
        assert.equal(requested, packageName);
        return manifest;
      },
    });
    assert.deepEqual(artifact, {
      command: executable,
      source: "package",
      packageName,
    });

    const invalid = JSON.parse(await readFile(manifest, "utf8"));
    invalid.sha256 = "0".repeat(64);
    await writeFile(manifest, JSON.stringify(invalid));
    await assert.rejects(
      resolveA3sHostArtifactV1({
        environment: {},
        platform: "win32",
        arch: "x64",
        resolvePackageManifest: () => manifest,
      }),
      (error) => error instanceof A3sHostArtifactError && error.code === "checksumMismatch",
    );
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("host artifact resolver rejects unsupported targets before package lookup", async () => {
  let resolverCalls = 0;
  await assert.rejects(
    resolveA3sHostArtifactV1({
      environment: {},
      platform: "freebsd",
      arch: "x64",
      resolvePackageManifest: () => {
        resolverCalls += 1;
        return "unused";
      },
    }),
    (error) => error instanceof A3sHostArtifactError && error.code === "unsupportedTarget",
  );
  assert.equal(resolverCalls, 0);
});

test("host artifact resolver reports a missing exact platform package", async () => {
  await assert.rejects(
    resolveA3sHostArtifactV1({
      environment: {},
      platform: "win32",
      arch: "x64",
      resolvePackageManifest: () => {
        throw new Error("injected package miss");
      },
    }),
    (error) =>
      error instanceof A3sHostArtifactError &&
      error.code === "packageMissing" &&
      error.cause instanceof Error,
  );
});
