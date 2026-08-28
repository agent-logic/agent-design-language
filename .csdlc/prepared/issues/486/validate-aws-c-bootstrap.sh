#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$repo_root"

issue_dir=".csdlc/issues/486"
prepared_dir=".csdlc/prepared/issues/486"
receipt="$prepared_dir/aws-b-terminal-receipt.md"
design="$prepared_dir/design.md"
vpp="$issue_dir/cards/vpp.md"

fail() {
  printf 'aws-c-bootstrap-prebind: FAIL: %s\n' "$*" >&2
  exit 1
}

test -f "$design" || fail "missing design: $design"
test -f "$receipt" || fail "missing #485 terminal receipt: $receipt"
test -f "$vpp" || fail "missing VPP card: $vpp"

grep -q 'Issue state: `CLOSED`' "$receipt" || fail "#485 receipt does not prove CLOSED issue state"
grep -q 'Pull request state: `MERGED`' "$receipt" || fail "#485 receipt does not prove MERGED PR state"
grep -q 'Merge commit: `a71d699d52831b32bb68ed9c7c7e837925949de4`' "$receipt" || fail "#485 merge SHA mismatch"
grep -q 'Closing linkage: PR #564 closes issue #485' "$receipt" || fail "#485 closing linkage missing"

grep -q 'infra/aws/bootstrap' "$design" || fail "design does not declare bootstrap Terraform path"
grep -q 'docs/milestones/v0.92.1/evidence/cloud/aws-c' "$design" || fail "design does not declare AWS-C evidence path"

grep -q 'prebind-bootstrap-packet' "$vpp" || fail "VPP does not include the prebind bootstrap packet lane"
grep -q 'defer_reason' "$vpp" || fail "VPP does not record deferred post-bind proof lanes"

printf 'aws-c-bootstrap-prebind: PASS\n'
