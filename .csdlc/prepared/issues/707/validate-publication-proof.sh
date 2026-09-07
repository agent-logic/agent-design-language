#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

test -x adl/tools/test_runtime_v3_cross_binary_generation.sh
test -f .csdlc/evidence/707/live-a2a-client.mjs
jq -e '
  .schema == "adl.issue707.live_a2a_evidence.v1" and
  (.attempts | length) >= 2 and
  ([.attempts[] | select(.sender_id == "beacon" and .recipient_id == "gemma-e4b" and .status == "delivered" and .recipient_reply_nonempty == true)] | length) >= 2
' .csdlc/evidence/707/live-a2a-results.json >/dev/null
git diff --check origin/main...HEAD

echo "issue 707 retained publication proof: PASS"
