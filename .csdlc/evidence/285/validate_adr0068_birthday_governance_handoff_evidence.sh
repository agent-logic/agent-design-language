#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
git_common_dir="$(git rev-parse --git-common-dir)"

manifest="$root/.csdlc/evidence/285/evidence-manifest.json"
observations="$root/.csdlc/evidence/285/live-observations.json"
reconciliation="$root/.csdlc/evidence/285/adr0068-birthday-governance-handoff-reconciliation.md"
terminal_5839="$git_common_dir/csdlc-v2/derived-terminal/5839.json"
terminal_5836="$git_common_dir/csdlc-v2/derived-terminal/5836.json"
retained_5836="$root/.csdlc/issues/5836/index.json"

for required in "$manifest" "$observations" "$reconciliation" "$terminal_5839" "$retained_5836"; do
  test -f "$required"
done

test ! -e "$terminal_5836"

jq -e '
  .issue == 5839
  and .disposition == "merged"
  and .pull_request == 289
  and .merge_sha == "7f88697ce82215188af941e15cf02a6220c9ad63"
  and .head_sha == "042710838de804f4ccd85a46b48e8e6b7daab1a4"
  and .canonical_generation == 39
  and .canonical_digest == "28c8aa03cd5a88bf612aac78d74e0e2fdd387037f5c5727c0c89445d1ccddc24"
' "$terminal_5839" >/dev/null

jq -e '
  .issue == 5836
  and .phase == "initialized"
  and .generation == 44
  and .digest == "e45f9365f8eaf2252922d7d7bd052791c616558816935b4d33ef2f865f47ca62"
  and .publication == null
  and .terminal == null
' "$retained_5836" >/dev/null

jq -e '
  .schema == "adl.csdlc.evidence.adr0068_handoff_manifest.v1"
  and .issue == 285
  and .wp19_handoff.terminal == true
  and .wp19_handoff.issue == 5839
  and .wp19_handoff.merge_sha == "7f88697ce82215188af941e15cf02a6220c9ad63"
  and .wp18_birthday.terminal == false
  and .wp18_birthday.issue == 5836
  and .wp18_birthday.retained_phase == "initialized"
  and .residual_gaps[0] == "WP-18/#5836 has retained current-main initialized state but no current derived-terminal cache or current-repo GitHub issue identity."
  and (.non_claims | index("ADR 0068 acceptance") != null)
  and (.non_claims | index("#207 closeout") != null)
  and (.non_claims | index("#288 final ADR serialization") != null)
' "$manifest" >/dev/null

jq -e '
  .schema == "adl.csdlc.evidence.adr0068_live_observations.v1"
  and .issue == 285
  and .observations.wp19_pr_289.state == "MERGED"
  and .observations.wp18_issue_5836_current_repo == "not_found"
  and .observations.wp18_retained_record.phase == "initialized"
' "$observations" >/dev/null

grep -F "WP-19/#5839 terminal handoff evidence is present" "$reconciliation" >/dev/null
grep -F "WP-18/#5836 terminal birthday proof is not present in current derived-terminal authority" "$reconciliation" >/dev/null
grep -F "does not accept ADR 0068" "$reconciliation" >/dev/null

echo "adr0068 birthday-governance handoff evidence validation PASS"
