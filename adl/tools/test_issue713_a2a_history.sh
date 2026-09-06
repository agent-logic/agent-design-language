#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

echo "issue713_a2a_history: deterministic validation harness is reserved for the bound implementation worktree" >&2
echo "issue713_a2a_history: expected checks: Runtime transcript persistence, API projection, Observatory restore, replay, restart, checkpoint, rehydration, redaction, all-agent symmetry" >&2

exit 0
