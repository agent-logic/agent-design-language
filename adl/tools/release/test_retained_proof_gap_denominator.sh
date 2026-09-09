#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$repo_root"

tool="adl/tools/release/build_retained_proof_gap_denominator.rb"
artifact="docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json"

ruby -c "$tool" >/dev/null

positive="$(ruby "$tool" --check)"
echo "$positive" | jq -e '
  .status == "pass" and
  .remediation_total == 198 and
  .preserved_total == 29 and
  .accounted_non_764_eligible_total == 3 and
  .eligible_partition_total == .eligible_total
' >/dev/null

if ADL_764_EXPECT_REMEDIATION_NON_PROVING=142 ruby "$tool" --check | jq -e '.status == "pass"' >/dev/null; then
  echo "expected drifted non_proving denominator to fail" >&2
  exit 1
fi

if ADL_764_EXPECT_ACCOUNTED_NON_764_ELIGIBLE_REVIEW_FRESHNESS_RESOLVED=2 ruby "$tool" --check | jq -e '.status == "pass"' >/dev/null; then
  echo "expected drifted review_freshness_resolved accounting to fail" >&2
  exit 1
fi

ruby "$tool" >/dev/null

jq -e '
  .schema == "adl.v0921.tail06.issue764.retained_proof_gap_denominator.v1" and
  .validation.status == "pass" and
  .partition.remediation_total == 198 and
  .partition.preserved_total == 29 and
  .partition.accounted_non_764_eligible_total == 3 and
  .partition.eligible_partition_total == .partition.eligible_total and
  (.remediation_rows | length) == 198 and
  (.preserved_rows | length) == 29 and
  (.accounted_non_764_eligible_rows | length) == 3 and
  (.child_buckets | length) == 4
' "$artifact" >/dev/null

echo "issue 764 retained proof-gap denominator validation passed"
