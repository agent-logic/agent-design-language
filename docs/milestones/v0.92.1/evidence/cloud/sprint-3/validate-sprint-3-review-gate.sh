#!/usr/bin/env bash
set -euo pipefail

review_record=".csdlc/issues/531/index.json"

test -f "$review_record"

jq -e '
  .review != null and
  .review.completed == true and
  (.review.findings | type == "array") and
  (.review.findings | length == 0)
' "$review_record" >/dev/null

echo "sprint-3-review-gate: pass"
