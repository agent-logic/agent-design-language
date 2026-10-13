#!/usr/bin/env bash
set -euo pipefail

# #1223: transferred demo producers are selected explicitly before any effects.
ADL_RUNTIME_BIN="${ADL_RUNTIME_BIN:-}"
if [[ "$ADL_RUNTIME_BIN" != /* || ! -x "$ADL_RUNTIME_BIN" ]]; then
  echo "Select the reviewed Runtime executable with absolute executable ADL_RUNTIME_BIN before running retained demo callers." >&2
  exit 2
fi
RUNTIME_DEMO_SOURCE_ROOT="${RUNTIME_DEMO_SOURCE_ROOT:-}"
if [[ "$RUNTIME_DEMO_SOURCE_ROOT" != /* ]]; then
  echo "Select the reviewed Runtime source checkout with absolute RUNTIME_DEMO_SOURCE_ROOT; ADL no longer owns these demo producers." >&2
  exit 2
fi
if [[ ! -f "$RUNTIME_DEMO_SOURCE_ROOT/adl/tools/demo_v087_control_plane.sh" ]]; then
  echo "Missing selected Runtime demo resource: demo_v087_control_plane.sh" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cd "$ROOT_DIR"

bash adl/tools/demo_v087_trace_truth.sh
bash adl/tools/demo_v087_provider_portability.sh
bash adl/tools/demo_v087_shared_obsmem.sh
bash adl/tools/demo_v087_operational_skills.sh
bash "$RUNTIME_DEMO_SOURCE_ROOT/adl/tools/demo_v087_control_plane.sh"
bash adl/tools/demo_v087_reviewer_package.sh

echo "v0.87 demo suite complete: artifacts/v087/"

