#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const commonDir = execFileSync(
  "git",
  ["rev-parse", "--path-format=absolute", "--git-common-dir"],
  { cwd: root, encoding: "utf8" },
).trim();

const receipt = JSON.parse(
  readFileSync(join(commonDir, "csdlc-v2/closeout/5662.json"), "utf8"),
);
const projection = JSON.parse(
  readFileSync(join(root, ".csdlc/issues/5662/index.json"), "utf8"),
);

const canonicalize = (value) => {
  if (Array.isArray(value)) {
    return value.map(canonicalize);
  }
  if (value !== null && typeof value === "object") {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonicalize(value[key])]),
    );
  }
  return value;
};
const canonicalJson = (value) => JSON.stringify(canonicalize(value));
const failures = [];
const firstRetainedRevision = "6487f1ef8d97549c5ccf092946d93a7aa67c60de";
const retainedProjectionRevision = "d95b4b0c5ebcc4c4fa95d8dccf19558296c53c6c";
const projectionPaths = [
  ".csdlc/issues/5662/",
  ".csdlc/publication/5662.intent.json",
];
const allowedPaths = [
  ".csdlc/issues/5686/",
  ".csdlc/prepared/issues/5686/",
  ".csdlc/evidence/5686/",
  ".csdlc/locks/5686.lock",
];

if (canonicalJson(projection) !== canonicalJson(receipt.record)) {
  failures.push("projected issue record differs from receipt.record");
}

for (const [card, expected] of Object.entries(receipt.cards)) {
  const actual = JSON.parse(
    readFileSync(join(root, `.csdlc/issues/5662/cards/${card}.values.json`), "utf8"),
  );
  if (canonicalJson(actual) !== canonicalJson(expected)) {
    failures.push(`${card}.values.json differs from receipt.cards.${card}`);
  }
}

for (const [relativePath, expected] of Object.entries(receipt.authored_artifacts)) {
  const actual = readFileSync(join(root, relativePath), "utf8");
  if (actual !== expected) {
    failures.push(`${relativePath} differs from receipt.authored_artifacts`);
  }
}

const baseRevision = execFileSync(
  "git",
  ["merge-base", "origin/main", "HEAD"],
  { cwd: root, encoding: "utf8" },
).trim();
const committedPaths = execFileSync(
  "git",
  ["diff", "--name-only", baseRevision, "HEAD"],
  { cwd: root, encoding: "utf8" },
)
  .trimEnd()
  .split("\n")
  .filter(Boolean);
const worktreePaths = execFileSync(
  "git",
  ["status", "--porcelain=v1", "--untracked-files=all"],
  { cwd: root, encoding: "utf8" },
)
  .trimEnd()
  .split("\n")
  .filter(Boolean)
  .map((line) => line.slice(3));
const changedPaths = [...new Set([...committedPaths, ...worktreePaths])].sort();
for (const relativePath of changedPaths) {
  if (
    !projectionPaths.some((allowed) => relativePath.startsWith(allowed)) &&
    !allowedPaths.some((allowed) => relativePath.startsWith(allowed))
  ) {
    failures.push(`out-of-scope path: ${relativePath}`);
  }
}

const retainedParent = execFileSync(
  "git",
  ["rev-parse", `${firstRetainedRevision}^`],
  { cwd: root, encoding: "utf8" },
).trim();
const expectedProjectionPaths = execFileSync(
  "git",
  [
    "diff",
    "--name-only",
    retainedParent,
    retainedProjectionRevision,
    "--",
    ".csdlc/issues/5662",
    ".csdlc/publication/5662.intent.json",
  ],
  { cwd: root, encoding: "utf8" },
)
  .trimEnd()
  .split("\n")
  .filter(Boolean)
  .sort();
const actualProjectionPaths = changedPaths.filter((relativePath) =>
  projectionPaths.some((allowed) => relativePath.startsWith(allowed)),
);
if (canonicalJson(actualProjectionPaths) !== canonicalJson(expectedProjectionPaths)) {
  failures.push("projected path set differs from the two retained commits");
}

try {
  execFileSync(
    "git",
    [
      "diff",
      "--quiet",
      retainedProjectionRevision,
      "HEAD",
      "--",
      ".csdlc/issues/5662",
      ".csdlc/publication/5662.intent.json",
    ],
    { cwd: root, stdio: "ignore" },
  );
} catch {
  failures.push("projected files differ from retained terminal revision");
}

const result = {
  schema: "adl.csdlc_terminal_projection_parity.v1",
  issue: 5662,
  receipt_ref: receipt.receipt_ref,
  receipt_digest: receipt.record.digest,
  projection_digest: projection.digest,
  phase: projection.phase,
  generation: projection.generation,
  base_revision: baseRevision,
  retained_projection_revision: retainedProjectionRevision,
  expected_projection_path_count: expectedProjectionPaths.length,
  changed_path_count: changedPaths.length,
  parity: failures.length === 0,
  failures,
};

process.stdout.write(`${JSON.stringify(result)}\n`);
if (failures.length > 0) {
  process.exitCode = 1;
}
