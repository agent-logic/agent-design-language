#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
python3 adl/tools/generate_active_command_reference_scan.py --check
echo "active command reference scan check-only gate: ok"
