#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
python3 adl/tools/generate_active_command_reference_scan.py --self-test

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
cat >"$tmpdir/AGENTS.md" <<'EOF'
owner_skill: workflow-conductor
Route the selected issue after readiness through the workflow_conductor owner.
VALIDATOR="$ROOT/adl/tools/validate_structured_prompt.sh"
EOF
if python3 adl/tools/generate_active_command_reference_scan.py --check --fixture-root "$tmpdir" >/dev/null 2>&1; then
  echo "active fixture unexpectedly passed" >&2
  exit 1
fi
cat >"$tmpdir/AGENTS.md" <<'EOF'
The workflow-conductor route is retired and must not be used.
[[ ! -e "$ROOT/adl/tools/review_card_surface.sh" ]]
EOF
python3 adl/tools/generate_active_command_reference_scan.py --check --fixture-root "$tmpdir"
python3 adl/tools/generate_active_command_reference_scan.py --check
echo "active command reference scan check-only gate: ok"
