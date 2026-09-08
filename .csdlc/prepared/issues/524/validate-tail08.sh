#!/usr/bin/env bash
set -euo pipefail

release_plan=docs/milestones/v0.92.2/RELEASE_PLAN_v0.92.2.md
checklist=docs/milestones/v0.92.2/MILESTONE_CHECKLIST_v0.92.2.md
test -s "$release_plan"
test -s "$checklist"
for id in TAIL-01 TAIL-02 TAIL-03 TAIL-04 TAIL-05 TAIL-06 TAIL-07 TAIL-08 TAIL-09 TAIL-10; do
  grep -q "$id" "$release_plan"
  grep -q "$id" "$checklist"
done
ruby .csdlc/prepared/issues/316/validate-v0922-codefriend-plan.rb
git diff --check
printf 'issue 524 closeout-plan contract passed\n'
