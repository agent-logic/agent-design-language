#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

require_file() {
  test -f "$1" || fail "missing required file: $1"
}

require_text() {
  local file="$1"
  local needle="$2"
  grep -Fq "$needle" "$file" || fail "missing expected text in $file: $needle"
}

require_file ".csdlc/issues/731/index.json"
require_file ".csdlc/issues/731/cards/sor.md"
require_file ".csdlc/evidence/731/mutation-authorization-request.json"
require_file ".csdlc/evidence/731/terraform-plan-denominator/status.json"
require_file ".csdlc/evidence/731/read-only-preflight/status.json"
require_file ".csdlc/evidence/731/live-foundation-apply/status.json"
require_file ".csdlc/evidence/731/live-disposable-workload/status.json"

jq -e '.phase == "implemented" and .publication == null and .terminal == null' \
  ".csdlc/issues/731/index.json" >/dev/null || fail "issue is not in pre-publication implemented phase"
jq -e '.operator_authorization.status == "approved" and .plan_digest == "e20e83dcafc5d8f7cd963660bbcc249be6fea81ca76d841e8e2e298a4426f6df"' \
  ".csdlc/evidence/731/mutation-authorization-request.json" >/dev/null || fail "authorization packet is not approved or not bound to applied plan"
jq -e '.status == "passed" and .plan_sha256 == "e20e83dcafc5d8f7cd963660bbcc249be6fea81ca76d841e8e2e298a4426f6df" and .expected_resource_count == 20 and .forbidden_resource_count == 0' \
  ".csdlc/evidence/731/terraform-plan-denominator/status.json" >/dev/null || fail "Terraform plan denominator evidence is not current"
jq -e '.status == "passed" and .mutation == "terraform_apply_authorized" and .applied_plan_sha256 == "e20e83dcafc5d8f7cd963660bbcc249be6fea81ca76d841e8e2e298a4426f6df"' \
  ".csdlc/evidence/731/live-foundation-apply/status.json" >/dev/null || fail "live foundation apply evidence is not passed/current"
jq -e '.status == "passed" and .target_network_state == "present" and .target_subnet_state == "present" and .mutation == "none"' \
  ".csdlc/evidence/731/read-only-preflight/status.json" >/dev/null || fail "post-apply read-only preflight evidence is not passed/current"
jq -e '.status == "passed" and .external_ip == false and .machine_type == "e2-micro" and .residue.instances == 0 and .residue.run_labelled_instances == 0 and .residue.disks == 0 and .residue.addresses == 0' \
  ".csdlc/evidence/731/live-disposable-workload/status.json" >/dev/null || fail "disposable workload zero-residue evidence is not passed/current"

require_text ".csdlc/issues/731/cards/sor.md" "Under explicit operator authorization"
require_text ".csdlc/issues/731/cards/sor.md" "applied the bounded GCP-D1 private foundation"
require_text ".csdlc/issues/731/cards/sor.md" "exactly one labelled private e2-micro"
require_text ".csdlc/issues/731/cards/sor.md" "zero residual instances, run-labelled instances, disks, or addresses"
require_text ".csdlc/issues/731/cards/sor.md" "Publication: not_published"
require_text ".csdlc/issues/731/cards/sor.md" "Merge: not_merged"

printf 'PASS: #731 live completion review readiness\n'
