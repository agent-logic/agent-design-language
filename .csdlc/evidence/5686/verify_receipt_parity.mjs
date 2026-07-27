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
const allowedPaths = [
  ".csdlc/issues/5662/",
  ".csdlc/publication/5662.intent.json",
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
  if (!allowedPaths.some((allowed) => relativePath.startsWith(allowed))) {
    failures.push(`out-of-scope path: ${relativePath}`);
  }
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
  changed_paths: changedPaths,
  parity: failures.length === 0,
  failures,
};

process.stdout.write(`${JSON.stringify(result)}\n`);
if (failures.length > 0) {
  process.exitCode = 1;
}
