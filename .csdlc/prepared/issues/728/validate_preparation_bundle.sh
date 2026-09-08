#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

fail() {
  printf 'FAIL #728 preparation: %s\n' "$1" >&2
  exit 1
}

require_file() {
  test -f "$1" || fail "missing required file: $1"
}

require_text() {
  local path="$1"
  local needle="$2"
  grep -Fq "$needle" "$path" || fail "missing required text in $path: $needle"
}

require_executable() {
  test -x "$1" || fail "required executable is not executable: $1"
}

require_file ".csdlc/issues/728/index.json"
require_file ".csdlc/prepared/issues/728/design.md"
require_file ".csdlc/prepared/issues/728/diagram.mmd"
require_file ".csdlc/prepared/issues/728/validate_preparation_bundle.sh"
require_file "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh"
require_executable ".csdlc/prepared/issues/728/validate_preparation_bundle.sh"
require_executable "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh"

require_text ".csdlc/prepared/issues/728/design.md" "agent-logic-admin"
require_text ".csdlc/prepared/issues/728/design.md" "No production traffic"
require_text ".csdlc/prepared/issues/728/design.md" "destroy"
require_text ".csdlc/prepared/issues/728/design.md" "zero-residue"
require_text ".csdlc/prepared/issues/728/design.md" "Static validation or ALB-only health"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "ISSUE_728_AUTHORIZATION_FILE"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "production_traffic=false"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "require_auth_value alb_backend_config"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "require_auth_value node_backend_config"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "require_auth_value alb_var_file"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "require_auth_value node_var_file"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "require_auth_value external_health_url"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "require_auth_value expected_receipt_marker"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "destroy"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "require_instance_absent"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "require_security_group_absent"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "require_target_registration_absent"
require_text "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh" "terraform_state_empty_and_aws_absence"

test -d "infra/aws/runtime/alb-origin" || fail "missing ALB-origin root"
test -d "infra/aws/runtime/private-node" || fail "missing private-node root"
test -d "infra/aws/runtime/modules/private-runtime-node" || fail "missing private runtime node module"
test -d "infra/aws/modules/csm-runtime-alb" || fail "missing runtime ALB module"

require_text "infra/aws/runtime/modules/private-runtime-node/main.tf" "resource \"aws_vpc_security_group_ingress_rule\" \"runtime_from_alb\""
require_text "infra/aws/runtime/modules/private-runtime-node/main.tf" "referenced_security_group_id = var.alb_security_group_id"
require_text "infra/aws/modules/csm-runtime-alb/main.tf" "resource \"aws_vpc_security_group_ingress_rule\" \"https\""
require_text "infra/aws/modules/csm-runtime-alb/main.tf" "for_each          = toset(var.allowed_ingress_cidrs)"

if sed -n '/resource "aws_vpc_security_group_ingress_rule" "runtime_from_alb"/,/^}/p' infra/aws/runtime/modules/private-runtime-node/main.tf | grep -F 'cidr_ipv4' >/dev/null; then
  fail "private Runtime node ingress must not use cidr_ipv4"
fi

if sed -n '/resource "aws_vpc_security_group_ingress_rule" "https"/,/^}/p' infra/aws/modules/csm-runtime-alb/main.tf | grep -F '0.0.0.0/0' >/dev/null; then
  fail "ALB ingress rule must not hard-code world-open ingress"
fi

printf 'PASS #728 preparation bundle validator\n'
