#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
node "$root/.csdlc/prepared/issues/5500/check-diagram.mjs" \
  "$root/.csdlc/prepared/issues/5500/diagram.mmd"
