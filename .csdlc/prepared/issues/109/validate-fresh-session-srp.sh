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
index=".csdlc/issues/109/index.json"

for path in "$skill" "$runbook" "$srp" "$index"; do
  test -s "$path"
done

# Policy assertions must examine the exact committed blobs, never mutable
# working-tree content presented under an unchanged HEAD.
git diff --quiet "$expected_head" -- "$skill" "$runbook" "$srp" "$index" \
  .csdlc/issues/109/cards/stp.md
git diff --cached --quiet "$expected_head" -- "$skill" "$runbook" "$srp" "$index" \
  .csdlc/issues/109/cards/stp.md

ruby - "$skill" "$runbook" "$srp" "$index" "$expected_head" <<'RUBY'
require "json"
require "open3"

skill_path, runbook_path, srp_path, index_path, expected_head = ARGV
skill, runbook, srp = [skill_path, runbook_path, srp_path].map { |path| File.read(path) }
index = JSON.parse(File.read(index_path))

requirements = {
  "AC-1 standard SRP authority" => skill.include?("standard SRP, which remains\nthe sole review-result authority"),
  "AC-2 fresh exact-head handoff" => skill.include?("exact commit SHA to a fresh\nexternal review session that does not inherit the implementation conversation"),
  "AC-3 read-only findings-first evidence" => skill.include?("report findings first, ordered P0 through P3, with repository-relative file and\nline evidence") &&
    skill.include?("must state explicit limitations and operate\nread-only"),
  "AC-4 resolution and mandatory re-review" => runbook.include?("Resolve every actionable finding in the implementation session.") &&
    runbook.include?("If the fix changes the substantive commit, generate a current SRP and send\n   it to a new review session at the new exact SHA."),
  "AC-5 authority-critical precedence" => skill.include?("Follow `docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md`; it is mandatory review\nprocedure") &&
    skill.include?("require code, security, and evidence coverage even\nwhen every changed file is documentation") &&
    runbook.include?("Classify authority first."),
  "AC-6 no new orchestration" => runbook.include?("Do not add a review daemon, scheduler, registry, claim, persistent reviewer,\nparallel review record, or new lifecycle phase."),
  "AC-7 no redundant broad validation" => runbook.include?("Do not rerun broad validation\nsolely to prepare the review.")
}

failed = requirements.reject { |_name, passed| passed }.keys
abort("failed policy assertions: #{failed.join(', ')}") unless failed.empty?
requirements.each_key { |name| puts "assertion=PASS #{name}" }

assignment = index["review_assignment"]
review = index["review"]
abort("fresh-session review assignment missing") unless assignment.is_a?(Hash)
abort("completed review evidence missing") unless review.is_a?(Hash) && review["completed"] == true
reviewer = review["reviewer"].to_s
abort("reviewer is not a fresh session identity") unless reviewer.match?(/\Afresh-session:[0-9a-f]{8}-[0-9a-f-]{27,}\z/)
abort("review evidence does not match assignment") unless
  assignment.values_at("reviewer", "revision", "scope") == review.values_at("reviewer", "reviewed_revision", "scope")

revision = review["reviewed_revision"].to_s
match = revision.match(/\Agit-blake3:([0-9a-f]{40}):[0-9a-f]{64}\z/)
abort("reviewed revision is malformed") unless match
reviewed_commit = match[1]
_stdout, _stderr, ancestor = Open3.capture3("git", "merge-base", "--is-ancestor", reviewed_commit, expected_head)
abort("reviewed commit is not an ancestor of exact head") unless ancestor.success?
scope = Array(review["scope"])
abort("review scope missing") if scope.empty? || scope.any? { |path| path.to_s.empty? }
_stdout, _stderr, unchanged = Open3.capture3("git", "diff", "--quiet", reviewed_commit, expected_head, "--", *scope)
abort("reviewed scope changed after review") unless unchanged.success?

findings = Array(review["findings"])
open_findings = findings.select do |finding|
  finding["actionable"] && finding.fetch("in_scope", true) &&
    !%w[fixed accepted_risk].include?(finding["disposition"])
end
abort("actionable review findings lack terminal dispositions") unless open_findings.empty?
findings.each do |finding|
  next unless finding["actionable"] && finding.fetch("in_scope", true) && finding["disposition"] == "fixed"
  abort("fixed finding does not name reviewed revision") unless finding["fix_revision"] == revision
end
abort("typed lifecycle does not record reviewed truth") unless %w[reviewed published].include?(index["phase"])
abort("standard SRP does not record PASS") unless srp.include?("Result: pass") && srp.include?(reviewer)
puts "assertion=PASS completed fresh-session review evidence"
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
