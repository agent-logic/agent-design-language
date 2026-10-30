#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const script = resolve(repoRoot, ".csdlc/prepared/issues/262/record-podcast-http-playback.rb");
const result = spawnSync("ruby", [script, "--profile", "browser", ...process.argv.slice(2)], {
  cwd: repoRoot,
  stdio: "inherit"
});

process.exit(result.status ?? 1);
