#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

authorization_file="${ISSUE_728_AUTHORIZATION_FILE:-}"
mode="${ISSUE_728_MODE:-}"

fail() {
  printf 'FAIL #728 disposable proof: %s\n' "$1" >&2
  exit 1
}

note() {
  printf '%s\n' "$*"
}

require_env() {
  local name="$1"
  local value="${!name:-}"
  test -n "$value" || fail "missing required environment variable: $name"
}

auth_value() {
  local key="$1"
  sed -n "s/^${key}=//p" "$authorization_file" | tail -n 1
}

require_auth_value() {
  local key="$1"
  local expected="$2"
  local actual
  actual="$(auth_value "$key")"
  test "$actual" = "$expected" || fail "authorization mismatch for $key"
}

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

ensure_relative_path() {
  local label="$1"
  local value="$2"
  case "$value" in
    ""|/*|../*|*/../*) fail "$label must be a repo-relative path without '..'" ;;
  esac
}

ensure_evidence_path() {
  local label="$1"
  local value="$2"
  ensure_relative_path "$label" "$value"
  case "$value" in
    .csdlc/evidence/728/*|docs/milestones/v0.92.1/evidence/cloud/aws-f/*) ;;
    *) fail "$label must live under #728 evidence" ;;
  esac
}

ensure_stack_path() {
  local label="$1"
  local value="$2"
  ensure_relative_path "$label" "$value"
  test -d "$value" || fail "$label directory not found: $value"
}

ensure_file() {
  local label="$1"
  local value="$2"
  ensure_relative_path "$label" "$value"
  test -f "$value" || fail "$label not found: $value"
}

check_deadline() {
  require_env ISSUE_728_DEADLINE_UTC
  local now_epoch deadline_epoch
  now_epoch="$(date -u '+%s')"
  deadline_epoch="$(date -u -j -f '%Y-%m-%dT%H:%M:%SZ' "$ISSUE_728_DEADLINE_UTC" '+%s' 2>/dev/null || true)"
  test -n "$deadline_epoch" || deadline_epoch="$(date -u -d "$ISSUE_728_DEADLINE_UTC" '+%s' 2>/dev/null || true)"
  test -n "$deadline_epoch" || fail "ISSUE_728_DEADLINE_UTC must be parseable UTC, for example 2026-09-08T23:59:00Z"
  test "$now_epoch" -lt "$deadline_epoch" || fail "mutation deadline has passed"
}

terraform_workspace() {
  local root="$1"
  local workspace="$2"
  terraform -chdir="$root" workspace select "$workspace" >/dev/null 2>&1 || terraform -chdir="$root" workspace new "$workspace" >/dev/null
}

terraform_init_stack() {
  local root="$1"
  local backend_config="$2"
  ensure_file "backend config" "$backend_config"
  terraform -chdir="$root" init -backend-config="$repo_root/$backend_config" -input=false
}

terraform_plan_stack() {
  local root="$1"
  local workspace="$2"
  local var_file="$3"
  local plan_file="$4"
  shift 4
  ensure_file "tfvars" "$var_file"
  ensure_evidence_path "plan file" "$plan_file"
  mkdir -p "$(dirname "$plan_file")"
  terraform_workspace "$root" "$workspace"
  terraform -chdir="$root" plan -input=false -var-file="$repo_root/$var_file" -out="$repo_root/$plan_file" "$@"
  sha256_file "$plan_file"
}

terraform_apply_plan() {
  local root="$1"
  local workspace="$2"
  local plan_file="$3"
  ensure_file "plan file" "$plan_file"
  terraform_workspace "$root" "$workspace"
  terraform -chdir="$root" apply -input=false "$repo_root/$plan_file"
}

terraform_destroy_stack() {
  local root="$1"
  local workspace="$2"
  local var_file="$3"
  shift 3
  ensure_file "tfvars" "$var_file"
  terraform_workspace "$root" "$workspace"
  terraform -chdir="$root" destroy -input=false -auto-approve -var-file="$repo_root/$var_file" "$@"
}

output_raw() {
  local root="$1"
  local name="$2"
  terraform -chdir="$root" output -raw "$name"
}

cleanup_all_best_effort() {
  note "cleanup_trap=started" >&2
  terraform_destroy_stack "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_WORKSPACE" "$ISSUE_728_NODE_VAR_FILE" >/dev/null 2>&1 || true
  terraform_destroy_stack "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_WORKSPACE" "$ISSUE_728_ALB_VAR_FILE" -var "target_instance_id=null" >/dev/null 2>&1 || true
  note "cleanup_trap=finished" >&2
}

arm_cleanup_trap() {
  trap cleanup_all_best_effort ERR INT TERM
}

disarm_cleanup_trap() {
  trap - ERR INT TERM
}

require_empty_terraform_state() {
  local root="$1"
  local workspace="$2"
  local label="$3"
  terraform_workspace "$root" "$workspace"
  local state_listing
  state_listing="$(terraform -chdir="$root" state list 2>/dev/null || true)"
  test -z "$state_listing" || fail "$label Terraform state still has resources after destroy"
}

require_instance_absent() {
  local instance_id="$1"
  local live
  live="$(aws ec2 describe-instances \
    --instance-ids "$instance_id" \
    --query "Reservations[].Instances[?State.Name!='terminated'].InstanceId" \
    --output text 2>/dev/null || true)"
  test -z "$live" || fail "instance still exists outside terminated state: $instance_id"
  note "aws_absent_instance=$instance_id"
}

require_security_group_absent() {
  local security_group_id="$1"
  if aws ec2 describe-security-groups --group-ids "$security_group_id" >/dev/null 2>&1; then
    fail "security group still exists: $security_group_id"
  fi
  note "aws_absent_security_group=$security_group_id"
}

require_elbv2_resource_absent() {
  local label="$1"
  shift
  if aws elbv2 "$@" >/dev/null 2>&1; then
    fail "$label still exists"
  fi
  note "aws_absent_$label=true"
}

require_target_registration_absent() {
  local target_group_arn="$1"
  local instance_id="$2"
  if aws elbv2 describe-target-health --target-group-arn "$target_group_arn" --targets "Id=$instance_id" >/dev/null 2>&1; then
    fail "target registration still exists for $instance_id"
  fi
  note "aws_absent_target_registration=$instance_id"
}

wait_target_healthy() {
  local target_group_arn="$1"
  local instance_id="$2"
  local attempt state
  for attempt in $(seq 1 40); do
    state="$(aws elbv2 describe-target-health \
      --target-group-arn "$target_group_arn" \
      --targets "Id=$instance_id" \
      --query 'TargetHealthDescriptions[0].TargetHealth.State' \
      --output text 2>/dev/null || true)"
    if [[ "$state" = "healthy" ]]; then
      note "target_health=healthy"
      return 0
    fi
    sleep 15
  done
  fail "target did not become healthy"
}

common_preflight() {
  require_env AWS_PROFILE
  require_env AWS_REGION
  require_env ISSUE_728_EXPECTED_ACCOUNT_ID
  require_env ISSUE_728_ALB_ROOT
  require_env ISSUE_728_NODE_ROOT
  require_env ISSUE_728_ALB_WORKSPACE
  require_env ISSUE_728_NODE_WORKSPACE
  require_env ISSUE_728_ALB_BACKEND_CONFIG
  require_env ISSUE_728_NODE_BACKEND_CONFIG
  require_env ISSUE_728_ALB_VAR_FILE
  require_env ISSUE_728_NODE_VAR_FILE
  require_env ISSUE_728_ALB_PLAN
  require_env ISSUE_728_NODE_PLAN
  require_env ISSUE_728_ATTACH_PLAN
  require_env ISSUE_728_EXTERNAL_HEALTH_URL
  require_env ISSUE_728_EXPECTED_RECEIPT_MARKER
  require_env ISSUE_728_COST_CEILING_USD

  test "$AWS_PROFILE" = "agent-logic-admin" || fail "AWS_PROFILE must be agent-logic-admin"
  test "$AWS_REGION" = "us-west-2" || fail "AWS_REGION must be us-west-2"
  test "$ISSUE_728_ALB_ROOT" = "infra/aws/runtime/alb-origin" || fail "unexpected ALB root"
  test "$ISSUE_728_NODE_ROOT" = "infra/aws/runtime/private-node" || fail "unexpected node root"

  ensure_stack_path ISSUE_728_ALB_ROOT "$ISSUE_728_ALB_ROOT"
  ensure_stack_path ISSUE_728_NODE_ROOT "$ISSUE_728_NODE_ROOT"
  ensure_evidence_path ISSUE_728_ALB_PLAN "$ISSUE_728_ALB_PLAN"
  ensure_evidence_path ISSUE_728_NODE_PLAN "$ISSUE_728_NODE_PLAN"
  ensure_evidence_path ISSUE_728_ATTACH_PLAN "$ISSUE_728_ATTACH_PLAN"
  ensure_file ISSUE_728_ALB_VAR_FILE "$ISSUE_728_ALB_VAR_FILE"
  ensure_file ISSUE_728_NODE_VAR_FILE "$ISSUE_728_NODE_VAR_FILE"
  ensure_file ISSUE_728_ALB_BACKEND_CONFIG "$ISSUE_728_ALB_BACKEND_CONFIG"
  ensure_file ISSUE_728_NODE_BACKEND_CONFIG "$ISSUE_728_NODE_BACKEND_CONFIG"

  local account
  account="$(aws sts get-caller-identity --query Account --output text)"
  test "$account" = "$ISSUE_728_EXPECTED_ACCOUNT_ID" || fail "AWS account mismatch"
}

require_authorization_file() {
  if [[ -z "$authorization_file" ]]; then
    fail "missing ISSUE_728_AUTHORIZATION_FILE; refusing AWS mutation"
  fi

  ensure_evidence_path ISSUE_728_AUTHORIZATION_FILE "$authorization_file"
  test -f "$authorization_file" || fail "authorization file not found: $authorization_file"

  require_auth_value issue "728"
  require_auth_value approved "true"
  require_auth_value production_traffic "false"
  require_auth_value aws_profile "$AWS_PROFILE"
  require_auth_value aws_region "$AWS_REGION"
  require_auth_value expected_account_id "$ISSUE_728_EXPECTED_ACCOUNT_ID"
  require_auth_value alb_root "$ISSUE_728_ALB_ROOT"
  require_auth_value node_root "$ISSUE_728_NODE_ROOT"
  require_auth_value alb_workspace "$ISSUE_728_ALB_WORKSPACE"
  require_auth_value node_workspace "$ISSUE_728_NODE_WORKSPACE"
  require_auth_value alb_backend_config "$ISSUE_728_ALB_BACKEND_CONFIG"
  require_auth_value node_backend_config "$ISSUE_728_NODE_BACKEND_CONFIG"
  require_auth_value alb_var_file "$ISSUE_728_ALB_VAR_FILE"
  require_auth_value node_var_file "$ISSUE_728_NODE_VAR_FILE"
  require_auth_value alb_plan "$ISSUE_728_ALB_PLAN"
  require_auth_value node_plan "$ISSUE_728_NODE_PLAN"
  require_auth_value attach_plan "$ISSUE_728_ATTACH_PLAN"
  require_auth_value external_health_url "$ISSUE_728_EXTERNAL_HEALTH_URL"
  require_auth_value expected_receipt_marker "$ISSUE_728_EXPECTED_RECEIPT_MARKER"
  if [[ -n "${ISSUE_728_RECEIPT_FILE:-}" ]]; then
    require_auth_value receipt_file "$ISSUE_728_RECEIPT_FILE"
  fi
  require_auth_value cost_ceiling_usd "$ISSUE_728_COST_CEILING_USD"
  require_auth_value deadline_utc "$ISSUE_728_DEADLINE_UTC"
}

require_plan_digest() {
  local key="$1"
  local plan_file="$2"
  local expected actual
  expected="$(auth_value "$key")"
  test -n "$expected" || fail "authorization missing $key"
  actual="$(sha256_file "$plan_file")"
  test "$actual" = "$expected" || fail "saved plan digest mismatch for $key"
}

write_receipt() {
  local receipt="${ISSUE_728_RECEIPT_FILE:-.csdlc/evidence/728/disposable-proof-receipt.txt}"
  ensure_evidence_path ISSUE_728_RECEIPT_FILE "$receipt"
  mkdir -p "$(dirname "$receipt")"
  {
    printf 'issue=728\n'
    printf 'production_traffic=false\n'
    printf 'mode=%s\n' "$mode"
    printf 'alb_root=%s\n' "$ISSUE_728_ALB_ROOT"
    printf 'node_root=%s\n' "$ISSUE_728_NODE_ROOT"
    printf 'alb_workspace=%s\n' "$ISSUE_728_ALB_WORKSPACE"
    printf 'node_workspace=%s\n' "$ISSUE_728_NODE_WORKSPACE"
    printf 'external_health_url=%s\n' "$ISSUE_728_EXTERNAL_HEALTH_URL"
    printf 'expected_receipt_marker=%s\n' "$ISSUE_728_EXPECTED_RECEIPT_MARKER"
  } >> "$receipt"
  note "receipt_file=$receipt"
}

plan_alb() {
  common_preflight
  terraform_init_stack "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_BACKEND_CONFIG"
  local digest
  digest="$(terraform_plan_stack "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_WORKSPACE" "$ISSUE_728_ALB_VAR_FILE" "$ISSUE_728_ALB_PLAN" -var "target_instance_id=null")"
  note "PASS #728 plan-alb"
  note "alb_plan=$ISSUE_728_ALB_PLAN"
  note "alb_plan_sha256=$digest"
  note "next_mode=apply-alb"
}

apply_alb() {
  common_preflight
  require_authorization_file
  require_plan_digest alb_plan_sha256 "$ISSUE_728_ALB_PLAN"
  check_deadline
  local alb_security_group_id target_group_arn
  arm_cleanup_trap
  terraform_init_stack "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_BACKEND_CONFIG"
  terraform_apply_plan "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_WORKSPACE" "$ISSUE_728_ALB_PLAN"
  alb_security_group_id="$(output_raw "$ISSUE_728_ALB_ROOT" alb_security_group_id)"
  target_group_arn="$(output_raw "$ISSUE_728_ALB_ROOT" target_group_arn)"
  note "PASS #728 apply-alb"
  note "alb_security_group_id=$alb_security_group_id"
  note "target_group_arn=$target_group_arn"
  note "next_mode=plan-node"
  disarm_cleanup_trap
}

plan_node() {
  common_preflight
  terraform_init_stack "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_BACKEND_CONFIG"
  local alb_security_group_id digest
  terraform_workspace "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_WORKSPACE"
  alb_security_group_id="${ISSUE_728_ALB_SECURITY_GROUP_ID:-$(output_raw "$ISSUE_728_ALB_ROOT" alb_security_group_id)}"
  test -n "$alb_security_group_id" || fail "missing ALB security group id"
  digest="$(terraform_plan_stack "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_WORKSPACE" "$ISSUE_728_NODE_VAR_FILE" "$ISSUE_728_NODE_PLAN" -var "alb_security_group_id=$alb_security_group_id")"
  note "PASS #728 plan-node"
  note "node_plan=$ISSUE_728_NODE_PLAN"
  note "node_plan_sha256=$digest"
  note "next_mode=apply-node"
}

apply_node() {
  common_preflight
  require_authorization_file
  require_plan_digest node_plan_sha256 "$ISSUE_728_NODE_PLAN"
  check_deadline
  local instance_id
  arm_cleanup_trap
  terraform_init_stack "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_BACKEND_CONFIG"
  terraform_apply_plan "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_WORKSPACE" "$ISSUE_728_NODE_PLAN"
  instance_id="$(output_raw "$ISSUE_728_NODE_ROOT" instance_id)"
  note "PASS #728 apply-node"
  note "instance_id=$instance_id"
  note "next_mode=plan-attach"
  disarm_cleanup_trap
}

plan_attach() {
  common_preflight
  terraform_init_stack "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_BACKEND_CONFIG"
  local instance_id digest
  terraform_workspace "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_WORKSPACE"
  instance_id="${ISSUE_728_TARGET_INSTANCE_ID:-$(output_raw "$ISSUE_728_NODE_ROOT" instance_id)}"
  test -n "$instance_id" || fail "missing target instance id"
  digest="$(terraform_plan_stack "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_WORKSPACE" "$ISSUE_728_ALB_VAR_FILE" "$ISSUE_728_ATTACH_PLAN" -var "target_instance_id=$instance_id")"
  note "PASS #728 plan-attach"
  note "attach_plan=$ISSUE_728_ATTACH_PLAN"
  note "attach_plan_sha256=$digest"
  note "next_mode=apply-attach-prove-destroy"
}

apply_attach_prove_destroy() {
  common_preflight
  require_authorization_file
  require_plan_digest attach_plan_sha256 "$ISSUE_728_ATTACH_PLAN"
  check_deadline
  arm_cleanup_trap

  terraform_init_stack "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_BACKEND_CONFIG"
  terraform_init_stack "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_BACKEND_CONFIG"

  terraform_apply_plan "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_WORKSPACE" "$ISSUE_728_ATTACH_PLAN"

  terraform_workspace "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_WORKSPACE"
  terraform_workspace "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_WORKSPACE"
  local response target_group_arn instance_id listener_arn alb_arn alb_security_group_id node_security_group_id
  target_group_arn="$(output_raw "$ISSUE_728_ALB_ROOT" target_group_arn)"
  instance_id="$(output_raw "$ISSUE_728_NODE_ROOT" instance_id)"
  listener_arn="$(output_raw "$ISSUE_728_ALB_ROOT" listener_arn)"
  alb_security_group_id="$(output_raw "$ISSUE_728_ALB_ROOT" alb_security_group_id)"
  node_security_group_id="$(output_raw "$ISSUE_728_NODE_ROOT" security_group_id)"
  alb_arn="$(aws elbv2 describe-target-groups \
    --target-group-arns "$target_group_arn" \
    --query 'TargetGroups[0].LoadBalancerArns[0]' \
    --output text)"
  test -n "$alb_arn" && test "$alb_arn" != "None" || fail "missing ALB ARN for zero-residue readback"
  wait_target_healthy "$target_group_arn" "$instance_id"

  response="$(curl --fail --silent --show-error "$ISSUE_728_EXTERNAL_HEALTH_URL")"
  printf '%s\n' "$response" | grep -F "$ISSUE_728_EXPECTED_RECEIPT_MARKER" >/dev/null || fail "external receipt missing expected target marker"

  write_receipt
  note "PASS #728 external receipt"

  terraform_destroy_stack "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_WORKSPACE" "$ISSUE_728_NODE_VAR_FILE"
  terraform_destroy_stack "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_WORKSPACE" "$ISSUE_728_ALB_VAR_FILE" -var "target_instance_id=null"
  require_empty_terraform_state "$ISSUE_728_NODE_ROOT" "$ISSUE_728_NODE_WORKSPACE" "private-node"
  require_empty_terraform_state "$ISSUE_728_ALB_ROOT" "$ISSUE_728_ALB_WORKSPACE" "alb-origin"
  require_target_registration_absent "$target_group_arn" "$instance_id"
  require_elbv2_resource_absent listener describe-listeners --listener-arns "$listener_arn"
  require_elbv2_resource_absent target_group describe-target-groups --target-group-arns "$target_group_arn"
  require_elbv2_resource_absent load_balancer describe-load-balancers --load-balancer-arns "$alb_arn"
  require_security_group_absent "$node_security_group_id"
  require_security_group_absent "$alb_security_group_id"
  require_instance_absent "$instance_id"
  disarm_cleanup_trap

  note "PASS #728 reverse destroy requested"
  note "zero_residue_readback=terraform_state_empty_and_aws_absence"
}

if [[ -z "$authorization_file" ]]; then
  fail "missing ISSUE_728_AUTHORIZATION_FILE; refusing AWS mutation"
fi

test -n "$mode" || fail "missing ISSUE_728_MODE; expected one of plan-alb, apply-alb, plan-node, apply-node, plan-attach, apply-attach-prove-destroy"

case "$mode" in
  plan-alb) plan_alb ;;
  apply-alb) apply_alb ;;
  plan-node) plan_node ;;
  apply-node) apply_node ;;
  plan-attach) plan_attach ;;
  apply-attach-prove-destroy) apply_attach_prove_destroy ;;
  *) fail "unknown ISSUE_728_MODE: $mode" ;;
esac
