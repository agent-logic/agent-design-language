#!/usr/bin/env bash
set -euo pipefail

evidence_dir=docs/milestones/v0.92.1/evidence/release/tail-09
test -d "$evidence_dir"
find "$evidence_dir" -type f -size +0c | grep -q .
ruby .csdlc/prepared/issues/316/validate-v0922-codefriend-plan.rb
git diff --check
printf 'issue 525 exact-revision review artifact contract passed\n'
