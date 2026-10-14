#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cd "$ROOT_DIR"
historical="docs/milestones/v0.91.5/TOOL_SURFACE_REGISTRY_3734.md"
historical_before="$(git hash-object "$historical")"
python3 adl/tools/test_generate_tool_surface_registry.py
python3 adl/tools/generate_tool_surface_registry.py
python3 adl/tools/generate_tool_surface_registry.py --check
[[ "$(git hash-object "$historical")" == "$historical_before" ]]
git ls-files --error-unmatch docs/tooling/TOOL_SURFACE_REGISTRY.md >/dev/null
git diff --exit-code -- docs/tooling/TOOL_SURFACE_REGISTRY.md >/dev/null

echo "tool surface registry generation/check: ok"
