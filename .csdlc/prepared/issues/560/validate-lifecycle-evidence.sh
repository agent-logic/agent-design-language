#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"

test -f .csdlc/issues/560/index.json
test -d .csdlc/evidence/560
test -f .csdlc/prepared/issues/560/design.md
test -f .csdlc/prepared/issues/560/diagram.mmd

echo "lifecycle evidence shape present for #560"
