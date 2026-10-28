#!/usr/bin/env bash
set -euo pipefail

repo="agent-logic/agent-design-language"
artifact="docs/milestones/v0.92.1/evidence/cloud/sprint-3/sprint-3-cloud-convergence-closeout.md"

test -f "$artifact"

grep -F "Roster membership version: 4" "$artifact" >/dev/null
grep -F "not recorded" "$artifact" >/dev/null
grep -F "does not claim new paid AWS/GCP execution" "$artifact" >/dev/null
grep -F "does not claim production cutover" "$artifact" >/dev/null
grep -F "Skipped lanes remain skipped evidence" "$artifact" >/dev/null

check_child() {
  local issue="$1"
  local pr="$2"
  local merge_commit="$3"

  local issue_state
  issue_state="$(gh issue view "$issue" --repo "$repo" --json state --jq '.state')"
  test "$issue_state" = "CLOSED"

  local pr_state
  pr_state="$(gh pr view "$pr" --repo "$repo" --json state --jq '.state')"
  test "$pr_state" = "MERGED"

  local observed_merge_commit
  observed_merge_commit="$(gh pr view "$pr" --repo "$repo" --json mergeCommit --jq '.mergeCommit.oid')"
  test "$observed_merge_commit" = "$merge_commit"

  git merge-base --is-ancestor "$merge_commit" HEAD
}

check_child 489 577 69ba35e066d1389a9f194659acb066a7dca82a40
check_child 494 595 dc08b5abf10682ed9ace5deefd0e1389ea6899b6
check_child 495 590 c78c60f5a45a87a96159d4910a831b69b62b042c
check_child 496 599 83077ca029d52c9d613ed5a373da30f1dd42d9b3

echo "sprint-3-closeout-live: pass"
