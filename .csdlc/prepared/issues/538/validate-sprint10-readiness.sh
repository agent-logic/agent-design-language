#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
case "${mode}" in
  membership|readiness|all) ;;
  *) echo "usage: $0 [membership|readiness|all]" >&2; exit 64 ;;
esac

repo_root="$(git rev-parse --show-toplevel)"
packet="docs/milestones/v0.92.1/evidence/integration/sprint-10/sprint-execution-packet.md"
state="docs/milestones/v0.92.1/evidence/integration/sprint-10/state.json"
activity="docs/milestones/v0.92.1/evidence/integration/sprint-10/activity.jsonl"
review="docs/milestones/v0.92.1/evidence/integration/sprint-10/review.md"

if [[ "${mode}" == "membership" || "${mode}" == "all" ]]; then
  test -f "${packet}"
  for issue in 516 517 518 519 520 521 522 523 524 525 526; do
    rg -q "#${issue}" "${packet}"
  done
  rg -q '#516.*#517.*#518.*#519.*#520.*#521.*#522.*#523.*#524.*#525.*#526' "${packet}"
fi

if [[ "${mode}" == "readiness" || "${mode}" == "all" ]]; then
  python3 adl/tools/skills/sprint-conductor/scripts/check_sprint_readiness.py \
    --repo-root "${repo_root}" \
    --ordered-issues 516,517,518,519,520,521,522,523,524,525,526 \
    --execution-mode sequential \
    --execution-packet-path "${packet}" \
    --activity-log-path "${activity}" \
    --review-path "${review}" \
    --state "${state}" \
    --print-json
fi
