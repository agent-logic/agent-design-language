import assert from "node:assert/strict";
import { accessSync, constants, mkdtempSync, rmSync, symlinkSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const validator = join(repoRoot, "adl/tools/test_milestone_dashboard.sh");

function executable(name) {
  for (const directory of (process.env.PATH ?? "").split(":")) {
    if (!directory) continue;
    const candidate = join(directory, name);
    try {
      accessSync(candidate, constants.X_OK);
      return candidate;
    } catch {
      // Continue until the executable is found in the current test environment.
    }
  }
  throw new Error(`required test executable is unavailable: ${name}`);
}

function runDashboardValidator(path) {
  return spawnSync(validator, [], {
    cwd: repoRoot,
    env: { ...process.env, PATH: path },
    encoding: "utf8",
  });
}

test("Node-present dashboard validation executes behavioral assertions", () => {
  const result = runDashboardValidator(process.env.PATH ?? "");

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /^PASS test_milestone_dashboard javascript_validation=executed$/m);
  assert.doesNotMatch(result.stdout, /^SKIP test_milestone_dashboard/m);
});

test("Node-absent dashboard validation reports behavioral assertions skipped", () => {
  const isolatedBin = mkdtempSync(join(tmpdir(), "adl-dashboard-node-modes-"));
  try {
    for (const name of ["bash", "dirname", "grep", "head", "sed"]) {
      symlinkSync(executable(name), join(isolatedBin, name));
    }

    const result = runDashboardValidator(isolatedBin);

    assert.equal(result.status, 0, result.stderr);
    assert.match(
      result.stdout,
      /^SKIP test_milestone_dashboard javascript_validation=skipped reason=node_unavailable static_validation=passed$/m,
    );
    assert.doesNotMatch(result.stdout, /^PASS test_milestone_dashboard/m);
  } finally {
    rmSync(isolatedBin, { force: true, recursive: true });
  }
});
