#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$repo_root"
git_common_dir="$(git rev-parse --git-common-dir)"

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -s "$path" ]] || fail "missing or empty file: $path"
}

sha256_of() {
  shasum -a 256 "$1" | awk '{print $1}'
}

expect_sha() {
  local path="$1"
  local expected="$2"
  require_file "$path"
  local actual
  actual="$(sha256_of "$path")"
  [[ "$actual" == "$expected" ]] || fail "sha mismatch for $path: expected $expected got $actual"
}

expect_jq() {
  local path="$1"
  local expr="$2"
  jq -e "$expr" "$path" >/dev/null || fail "jq assertion failed for $path: $expr"
}

terminal_5878="$git_common_dir/csdlc-v2/derived-terminal/5878.json"
proof_5878=".csdlc/evidence/5878/execution-proof.json"
summary_194=".csdlc/evidence/194/private-wuji-aws-recovery-live-summary.redacted.json"
preflight_194=".csdlc/evidence/194/live-preflight/live-private-network-preflight.redacted.json"
manifest_284=".csdlc/evidence/284/evidence-manifest.json"
observations_284=".csdlc/evidence/284/live-observations.json"
reconciliation_284=".csdlc/evidence/284/adr0066-guardian-authority-reconciliation.md"

expect_sha "$terminal_5878" "7c02bca8850112812b40199148e8b3c3bdb0df94c26c8eae23367c179368b947"
expect_sha "$proof_5878" "c6eef86743055f0af725193dc3c0b610c6aa0da66a0ba30cde7accb0d468d24f"
expect_sha "$summary_194" "4598758414dea21b8232b878a05043634858fadc8066872f24645e44dbb37286"
expect_sha "$preflight_194" "7f7414e06df08f21dc32c2309e76a87376306656582f5b638f8906a551a313bf"

expect_jq "$terminal_5878" '
  .issue == 5878 and
  .repository == "danielbaustin/agent-design-language" and
  .pull_request == 140 and
  .head_sha == "1288f89499d26a1a607b96cd96e0b71051194af6" and
  .merge_sha == "d3a0d69a4c1507eb038392741d163d8341bd95d1" and
  .issue_state == "closed_by_merged_pr" and
  (.digest | type == "string" and length == 64)
'

expect_jq "$proof_5878" '
  .schema == "adl.wp04.execution_proof.v3" and
  .issue == 5878 and
  .wp == "WP-04.16" and
  .source_revision == "413c1e09992e8e1d996858f8b4a70d210b3eb0d8" and
  ([.commands[].exit_code] | all(. == 0)) and
  ([.negative_cases[] | select(.case == "authority_replay" and .result == "rejected")] | length == 1) and
  ([.negative_cases[] | select(.case == "oversized_protobuf_frame" and .result == "rejected")] | length == 1) and
  ([.negative_cases[] | select(.case == "wrong_authority_domain" and .result == "rejected")] | length == 1)
'

while IFS=$'\t' read -r artifact_path artifact_sha; do
  [[ -n "$artifact_path" && -n "$artifact_sha" ]] || fail "blank artifact entry in $proof_5878"
  expect_sha "$artifact_path" "$artifact_sha"
done < <(jq -r '.artifacts[] | [.path, .sha256] | @tsv' "$proof_5878")

expect_jq "$summary_194" '
  .schema == "adl.issue194.private_wuji_aws_recovery.live_summary.v1" and
  .issue == 194 and
  .status == "partial_private_qualification_passed" and
  ([.passed_profiles[] | select(.profile == "two_voter_private_network_smoke" and .status == "passed")] | length == 1) and
  ([.passed_profiles[] | select(.profile == "single_gpu_private_model_health" and .status == "passed")] | length == 1) and
  ([.passed_profiles[] | select(.profile == "wuji_local_recovery" and .status == "passed")] | length == 1) and
  ([.quota_limitations[] | select(.status == "failed_no_feasible_two_voter_model_shape")] | length == 1) and
  (.non_claims | index("does not claim #142 completion")) and
  (.remaining_acceptance_gaps | index("same serial hybrid run combining Wuji receipt plus two AWS model-health voters"))
'

while IFS= read -r receipt_path; do
  require_file "$receipt_path"
  jq -e 'type == "object"' "$receipt_path" >/dev/null || fail "receipt is not JSON object: $receipt_path"
done < <(jq -r '.passed_profiles[].receipt, .quota_limitations[].receipt' "$summary_194")

require_file "$manifest_284"
require_file "$observations_284"
require_file "$reconciliation_284"

expect_jq "$manifest_284" '
  .schema == "adl.issue284.adr0066_guardian_authority_evidence_manifest.v1" and
  .issue == 284 and
  .inputs[".git/csdlc-v2/derived-terminal/5878.json"].sha256 == "7c02bca8850112812b40199148e8b3c3bdb0df94c26c8eae23367c179368b947" and
  .inputs[".csdlc/evidence/5878/execution-proof.json"].sha256 == "c6eef86743055f0af725193dc3c0b610c6aa0da66a0ba30cde7accb0d468d24f" and
  .inputs[".csdlc/evidence/194/private-wuji-aws-recovery-live-summary.redacted.json"].sha256 == "4598758414dea21b8232b878a05043634858fadc8066872f24645e44dbb37286" and
  .inputs[".csdlc/evidence/194/live-preflight/live-private-network-preflight.redacted.json"].sha256 == "7f7414e06df08f21dc32c2309e76a87376306656582f5b638f8906a551a313bf" and
  (.classifications.terminal_proof | index("#5878 / PR #140")) and
  (.classifications.partial_proof | index("#194 / PR #397")) and
  (.classifications.residual_gaps | index("two simultaneous model-capable AWS GPU voters under current quota")) and
  (.classifications.non_claims | index("#284 does not accept ADR 0066 or edit shared ADR serialization surfaces"))
'

expect_jq "$observations_284" '
  .schema == "adl.issue284.live_observations.v1" and
  .issue == 284 and
  .observed_head == "af49f8f674722bee671d65db5b6a49ea08eeb4b0" and
  .github.issue_142.state == "CLOSED" and
  .github.issue_142.closed_by_pull_requests_count == 0 and
  .github.pr_140.state == "MERGED" and
  .github.pr_140.head_sha == "801ea3f26917421e0f315b17ba0aa299f6c64c39" and
  .github.pr_140.merge_commit == "7415b1577b67c3b1a5b62d5d4a81790b77f85193" and
  .github.retained_5878_terminal.merge_sha == "d3a0d69a4c1507eb038392741d163d8341bd95d1" and
  .github.issue_194.state == "CLOSED" and
  .github.pr_397.state == "MERGED" and
  .github.pr_397.merge_commit == "974b8cbb27215d6d2e232d207fe5ffb3a2cfc04c"
'

grep -Fq "This packet does not claim #142 completion." "$reconciliation_284" || fail "missing #142 non-claim prose"
grep -Fq "Shared ADR docs, index, final plan, and manifest remain untouched for #288." "$reconciliation_284" || fail "missing #288 serialization boundary prose"
grep -Fq "#194 is partial private qualification evidence, not full #142 completion evidence." "$reconciliation_284" || fail "missing #194 partial-proof classification prose"
grep -Fq "Live legacy PR #140 currently differs from the retained #5878 terminal cache" "$reconciliation_284" || fail "missing retained/live PR #140 drift classification prose"

echo "PASS issue #284 ADR 0066 Guardian authority evidence reconciliation"
