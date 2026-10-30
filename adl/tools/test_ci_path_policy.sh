#!/usr/bin/env bash
set -euo pipefail
# PVF: deterministic local Git/routing contracts; no builds/providers/network.
# The former private-job routing fixtures are historical #1223 custody, not
# passing assertions for the public-only selector introduced by the split.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
exec bash "$ROOT_DIR/tools/public_adl/test_ci_routing.sh"
