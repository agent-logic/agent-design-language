#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: $0 <immutable-base-sha> <exact-reviewed-head-sha>" >&2
  exit 2
fi

base="$(git rev-parse --verify "$1^{commit}")"
expected_head="$(git rev-parse --verify "$2^{commit}")"
actual_head="$(git rev-parse HEAD)"
[[ "$actual_head" == "$expected_head" ]]
git merge-base --is-ancestor "$base" "$expected_head"

skill="csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md"
runbook="docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md"
srp=".csdlc/issues/109/cards/srp.md"

for path in "$skill" "$runbook" "$srp"; do
  test -s "$path"
done

# Policy assertions must examine the exact committed blobs, never mutable
# working-tree content presented under an unchanged HEAD.
git diff --quiet "$expected_head" -- "$skill" "$runbook" "$srp" \
  .csdlc/issues/109/cards/stp.md
git diff --cached --quiet "$expected_head" -- "$skill" "$runbook" "$srp" \
  .csdlc/issues/109/cards/stp.md

ruby - "$skill" "$runbook" "$srp" <<'RUBY'
skill, runbook, srp = ARGV.map { |path| File.read(path) }

requirements = {
  "AC-1 standard SRP authority" => skill.include?("standard SRP, which remains\nthe sole review-result authority"),
  "AC-2 fresh exact-head handoff" => skill.include?("exact commit SHA to a fresh\nexternal review session that does not inherit the implementation conversation"),
  "AC-3 read-only findings-first evidence" => srp.include?("Operate read-only: do not edit files, lifecycle state, PR state, or GitHub state.") &&
    srp.include?("Report findings first, ordered P0 through P3, with repository-relative file and line evidence"),
  "AC-4 resolution and mandatory re-review" => runbook.include?("Resolve every actionable finding in the implementation session.") &&
    runbook.include?("If the fix changes the substantive commit, generate a current SRP and send\n   it to a new review session at the new exact SHA."),
  "AC-5 authority-critical precedence" => runbook.include?("Classify authority first.") &&
    runbook.include?("require code, security, and\nevidence review even when every changed file is documentation"),
  "AC-6 no new orchestration" => runbook.include?("Do not add a review daemon, scheduler, registry, claim, persistent reviewer,\nparallel review record, or new lifecycle phase."),
  "AC-7 no redundant broad validation" => runbook.include?("Do not rerun broad validation\nsolely to prepare the review.")
}

failed = requirements.reject { |_name, passed| passed }.keys
abort("failed policy assertions: #{failed.join(', ')}") unless failed.empty?
requirements.each_key { |name| puts "assertion=PASS #{name}" }
RUBY

expected_paths="$(cat <<'PATHS'
.csdlc/evidence/109/focused-srp-contract.log
.csdlc/issues/109/audit.jsonl
.csdlc/issues/109/cards/sip.md
.csdlc/issues/109/cards/sip.values.json
.csdlc/issues/109/cards/sor.md
.csdlc/issues/109/cards/sor.values.json
.csdlc/issues/109/cards/spp.md
.csdlc/issues/109/cards/spp.values.json
.csdlc/issues/109/cards/srp.md
.csdlc/issues/109/cards/srp.values.json
.csdlc/issues/109/cards/stp.md
.csdlc/issues/109/cards/stp.values.json
.csdlc/issues/109/cards/vpp.md
.csdlc/issues/109/cards/vpp.values.json
.csdlc/issues/109/index.json
.csdlc/prepared/issues/109/design.md
.csdlc/prepared/issues/109/diagram.mmd
.csdlc/prepared/issues/109/validate-fresh-session-srp.sh
csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md
docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md
PATHS
)"
actual_paths="$(git diff --name-only "$base" "$expected_head" -- | LC_ALL=C sort)"
expected_paths="$(printf '%s\n' "$expected_paths" | LC_ALL=C sort)"
if [[ "$actual_paths" != "$expected_paths" ]]; then
  echo "exact changed-path manifest mismatch" >&2
  diff -u <(printf '%s\n' "$expected_paths") <(printf '%s\n' "$actual_paths") >&2 || true
  exit 1
fi

if git diff "$base" "$expected_head" -- csdlc-v2/src csdlc-v2/Cargo.toml | grep -q .; then
  echo "forbidden lifecycle implementation change" >&2
  exit 1
fi

printf 'result=PASS\nbase=%s\nhead=%s\npaths=%s\n' \
  "$base" "$expected_head" "$(printf '%s\n' "$actual_paths" | wc -l | tr -d ' ')"
