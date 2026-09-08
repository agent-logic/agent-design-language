#!/usr/bin/env bash
set -euo pipefail

test -s docs/milestones/v0.92.2/README.md
test -s docs/milestones/v0.92.2/WBS_v0.92.2.md
test -s docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml
test -s docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml
test -s docs/planning/ADL_FEATURE_LIST.md
ruby .csdlc/prepared/issues/316/validate-v0922-codefriend-plan.rb
git diff --check
printf 'issue 523 planning package contract passed\n'
