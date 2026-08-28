#!/usr/bin/env bash
set -euo pipefail

evidence_dir=".csdlc/evidence/551"
output="$evidence_dir/html-polis-node.tap"
mkdir -p "$evidence_dir"

node --test demos/html-observatory/tests/polis_identity.test.mjs | tee "$output"
grep -Eq '^# tests [1-9][0-9]*$' "$output"
