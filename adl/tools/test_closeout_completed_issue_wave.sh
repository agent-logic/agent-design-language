#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

[[ ! -e "$ROOT/adl/tools/closeout_completed_issue_wave.sh" ]] || {
  echo "retired wave-closeout helper unexpectedly exists" >&2
  exit 1
}

# The legacy main-sync helper is a historical retirement candidate. Current
# terminal guidance comes from retained repository policy, not its source text.
grep -Fq 'Use native `finish` for terminal authority, then run native `clean`' "$ROOT/AGENTS.md"
grep -Fq '`finish` establishes terminal truth and `clean` separately guards cleanup.' "$ROOT/docs/csdlc-v3/CURRENT_AUTHORITY.md"

echo "PASS test_closeout_completed_issue_wave"
