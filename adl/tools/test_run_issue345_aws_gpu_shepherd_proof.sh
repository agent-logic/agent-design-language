#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER="$ROOT/adl/tools/run_issue345_aws_gpu_shepherd_proof.sh"

fail() {
  echo "FAIL issue345-runner-contract: $*" >&2
  exit 1
}

expect_failure() {
  local needle="$1"
  shift
  local out="$tmp/failure-$RANDOM.out" err="$tmp/failure-$RANDOM.err"
  if "$@" >"$out" 2>"$err"; then
    fail "expected failure containing: $needle"
  fi
  grep -q "$needle" "$err" || fail "failure did not contain: $needle"
}

bash -n "$RUNNER"
git -C "$ROOT" rev-parse --verify HEAD >/dev/null

help="$($RUNNER preflight --help)"
grep -q -- '--authorization-file' <<<"$help" || fail "authorization-file interface is missing"
grep -q -- '--owner-token' <<<"$help" || fail "owner-bound cleanup interface is missing"

model_request_line="$(grep -n 'cargo test --locked' "$RUNNER" | tail -1 | cut -d: -f1)"
residency_line="$(grep -n '/api/ps' "$RUNNER" | tail -1 | cut -d: -f1)"
[[ "$model_request_line" -lt "$residency_line" ]] || fail "GPU residency is checked before model execution"
grep -q 'OLLAMA_KEEP_ALIVE=-1' "$RUNNER" || fail "models are not kept resident"
grep -q 'OLLAMA_MAX_LOADED_MODELS=' "$RUNNER" || fail "multi-model capacity is not configured"
grep -q 'length >= 2' "$RUNNER" || fail "multi-model input is not required"
grep -q 'adl.issue345.aws_gpu_proof.v2' "$RUNNER" || fail "multi-model proof schema is missing"
grep -q 'estimated_compute_cost_usd' "$RUNNER" || fail "cost evidence is missing"
grep -q 'validate_v092_runtime_guardian_lifecycle.sh --suite preflight_1x' "$RUNNER" || fail "Runtime Guardian launch proof is missing"
grep -q 'run_issue268_six_resident_uts_cycle.py' "$RUNNER" || fail "real Runtime agent UTS/ACC path is missing"
grep -q 'runtime_agent_acc_proofs' "$RUNNER" || fail "Runtime agent ACC proof receipt is missing"
grep -q 'components_exercised:\["guardian_supervised_runtime_v3","governed_runtime_agents","ollama_gpu"\]' "$RUNNER" || fail "component-complete architecture receipt is missing"
grep -q 'runtime_v3_to_ollama_transit_proved:false' "$RUNNER" || fail "Runtime v3 provider-boundary non-claim is missing"
grep -q 'issue345-authorizations/\$AUTHORIZATION_SHA256.json' "$RUNNER" || fail "durable single-use authorization marker is missing"
grep -q -- "--if-none-match '\*'" "$RUNNER" || fail "authorization replay guard is missing"
grep -q 'instance role assume-role trust policy drifted' "$RUNNER" || fail "instance-role trust verification is missing"
grep -q 'deadline reaper assume-role trust policy drifted' "$RUNNER" || fail "reaper-role trust verification is missing"
grep -q 'active_issue_volumes' "$RUNNER" || fail "stale EBS detection is missing"
grep -q 'VolumeSize=\$GP3_VOLUME_SIZE_GIB' "$RUNNER" || fail "bounded 10x disk setting is not used at launch"
grep -q 'conservative_worst_case_total_cost_usd' "$RUNNER" || fail "complete worst-case cost receipt is missing"

tmp="$(mktemp -d "${TMPDIR:-/tmp}/issue345-contract.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT

# Load only the runner's pure validators. This never invokes AWS or a paid path.
ADL_ISSUE345_LIBRARY_MODE=1 source "$RUNNER"

head="$(git -C "$ROOT" rev-parse HEAD)"
run_id="adl-issue345-contract"
expires_epoch="$(( $(date +%s) + 3600 ))"
jq -n \
  --arg commit "$head" --arg run_id "$run_id" --arg expires "$expires_epoch" \
  --arg account "$EXPECTED_ACCOUNT_SHA256" \
  --arg bucket "$ARTIFACT_BUCKET" --arg manifest_key "$ARTIFACT_MANIFEST_KEY" \
  --arg manifest_version "$ARTIFACT_MANIFEST_VERSION_ID" --arg manifest_sha "$ARTIFACT_MANIFEST_SHA256" \
  --arg profile "$INSTANCE_PROFILE" --arg role "$INSTANCE_PROFILE_ROLE" \
  --arg inline_policy "$INSTANCE_REQUIRED_INLINE_POLICY" --arg inline_sha "$INSTANCE_REQUIRED_INLINE_POLICY_SHA256" \
  --arg managed_policy "$INSTANCE_REQUIRED_MANAGED_POLICY_ARN" --arg security_group "$NO_INGRESS_SECURITY_GROUP" \
  --arg reaper_function "$DEADLINE_REAPER_FUNCTION" --arg reaper_rule "$DEADLINE_REAPER_RULE" \
  --arg reaper_role "$DEADLINE_REAPER_ROLE" --arg reaper_code "$DEADLINE_REAPER_CODE_SHA256_B64" \
  --arg dlami_parameter "$DLAMI_PARAMETER" --argjson models "$MODEL_IDENTITIES_JSON" '
  {
    schema:"adl.issue345.paid_run_authorization.v2",authorized:true,
    authorization_id:"issue345-contract",source_commit:$commit,
    reviewed_revision:("git-blake3:" + $commit + ":" + ("0" * 64)),run_id:$run_id,
    region:"us-west-2",instance_type:"g6.xlarge",model_identities:$models,
    max_instance_seconds:3300,max_reaper_lag_seconds:300,max_billable_seconds:3600,
    max_gpu_hourly_usd:0.85,max_total_cost_usd:20,
    cost_overheads:{gp3_volume_size_gib:200,gp3_monthly_usd_per_gib:0.08,
      public_ipv4_hourly_usd:0.005,aws_request_overhead_usd:0.05},
    expires_epoch:($expires | tonumber),
    bindings:{aws_account_sha256:$account,
      artifact_manifest:{bucket:$bucket,key:$manifest_key,version_id:$manifest_version,sha256:$manifest_sha},
      instance_profile:{name:$profile,role:$role,inline_policy:$inline_policy,
        inline_policy_sha256:$inline_sha,managed_policy_arn:$managed_policy},
      no_ingress_security_group:{name:$security_group,resolved_id_sha256:("0" * 64)},
      deadline_reaper:{function:$reaper_function,rule:$reaper_rule,role:$reaper_role,code_sha256_b64:$reaper_code},
      dlami:{parameter:$dlami_parameter,resolved_ami_sha256:("0" * 64)},
      subnet_sha256:("0" * 64)}}' >"$tmp/authorization.json"
jq -S . "$tmp/authorization.json" >"$tmp/authorization-reformatted.json"
digest_a="$(authorization_canonical_sha256 "$tmp/authorization.json")"
digest_b="$(authorization_canonical_sha256 "$tmp/authorization-reformatted.json")"
[[ "$digest_a" == "$digest_b" ]] || fail "semantic authorization formatting changes its consumption key"

SOURCE_COMMIT="$head" RUN_ID="$run_id" AUTHORIZATION_FILE="$tmp/authorization.json" load_authorization
jq '.bindings.aws_account_sha256 = ("f" * 64)' "$tmp/authorization.json" >"$tmp/authorization-wrong-account.json"
if (SOURCE_COMMIT="$head" RUN_ID="$run_id" AUTHORIZATION_FILE="$tmp/authorization-wrong-account.json" load_authorization) 2>/dev/null; then
  fail "authorization with another account binding unexpectedly passed"
fi
jq '.max_reaper_lag_seconds = 299 | .max_billable_seconds = 3599' "$tmp/authorization.json" >"$tmp/authorization-short-reaper.json"
if (SOURCE_COMMIT="$head" RUN_ID="$run_id" AUTHORIZATION_FILE="$tmp/authorization-short-reaper.json" load_authorization) 2>/dev/null; then
  fail "authorization with insufficient reaper bound unexpectedly passed"
fi
jq '.reviewed_revision = ("git-blake3:" + .source_commit + ":not-an-immutable-digest")' \
  "$tmp/authorization.json" >"$tmp/authorization-bad-review.json"
if (SOURCE_COMMIT="$head" RUN_ID="$run_id" AUTHORIZATION_FILE="$tmp/authorization-bad-review.json" load_authorization) 2>/dev/null; then
  fail "authorization with a fabricated reviewed revision unexpectedly passed"
fi
AUTHORIZATION_FILE="$tmp/authorization.json"
bound_preflight="$(jq -n --arg account "$EXPECTED_ACCOUNT_SHA256" '{account_sha256:$account,
  no_ingress_security_group_sha256:("0" * 64),ami_sha256:("0" * 64),subnet_sha256:("0" * 64)}')"
verify_authorized_preflight_bindings "$bound_preflight"
if (verify_authorized_preflight_bindings "$(jq '.ami_sha256 = ("f" * 64)' <<<"$bound_preflight")") 2>/dev/null; then
  fail "resolved AMI authorization mismatch unexpectedly passed"
fi

instance_trust='{"Role":{"RoleName":"ADLIssue345GpuProofRole","AssumeRolePolicyDocument":{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Service":"ec2.amazonaws.com"},"Action":"sts:AssumeRole"}]}}}'
instance_trust_is_exact <<<"$instance_trust" >/dev/null || fail "valid instance trust fixture failed"
if instance_trust_is_exact <<<"${instance_trust/ec2.amazonaws.com/lambda.amazonaws.com}" >/dev/null; then
  fail "instance trust drift unexpectedly passed"
fi
reaper_trust='{"Role":{"RoleName":"ADLIssue345GpuDeadlineReaperRole","AssumeRolePolicyDocument":{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Service":"lambda.amazonaws.com"},"Action":"sts:AssumeRole"}]}}}'
reaper_trust_is_exact <<<"$reaper_trust" >/dev/null || fail "valid reaper trust fixture failed"
if reaper_trust_is_exact <<<"${reaper_trust/lambda.amazonaws.com/ec2.amazonaws.com}" >/dev/null; then
  fail "reaper trust drift unexpectedly passed"
fi

owned_records='[{"tags":[{"Key":"adl:owner-token","Value":"0123456789abcdef0123456789abcdef"}]}]'
records_are_owned_by 0123456789abcdef0123456789abcdef <<<"$owned_records" >/dev/null \
  || fail "valid cleanup owner fixture failed"
if records_are_owned_by fedcba9876543210fedcba9876543210 <<<"$owned_records" >/dev/null; then
  fail "cleanup ownership mismatch unexpectedly passed"
fi

if AWS_PROFILE=default AWS_REGION=us-west-2 "$RUNNER" preflight >"$tmp/wrong-profile.out" 2>"$tmp/wrong-profile.err"; then
  fail "wrong AWS profile unexpectedly passed"
fi
grep -q 'AWS profile must be agent-logic-admin' "$tmp/wrong-profile.err" || fail "wrong profile did not fail closed"

if "$RUNNER" run --commit "$head" --run-id adl-issue345-no-authorization --execute \
  >"$tmp/no-authorization.out" 2>"$tmp/no-authorization.err"; then
  fail "paid run without retained authorization unexpectedly passed"
fi
grep -q 'requires --authorization-file' "$tmp/no-authorization.err" || fail "missing authorization did not fail closed"

live_preflight="not_run"
if [[ "${ADL_ISSUE345_LIVE_PREFLIGHT:-0}" == 1 ]]; then
  "$RUNNER" preflight >"$tmp/live-preflight.json"
  jq -e '.schema == "adl.issue345.aws_gpu_preflight.v1"
    and .model_count >= 2 and .price_ready and .total_cost_ready and .quota_ready
    and .active_issue_instance_count == 0 and .active_issue_volume_count == 0
    and .max_billable_seconds == (.max_instance_seconds + .reaper_max_lag_seconds)
    and .cost_overheads.gp3_volume_size_gib >= 200
    and .worst_case_total_cost_usd <= .max_total_cost_usd
    and .public_ingress == false
    and .paid_launch == false' "$tmp/live-preflight.json" >/dev/null || fail "live AWS preflight contract failed"
  live_preflight="passed"
  expect_failure 'approved Agent Logic account hash' env \
    ADL_ISSUE345_EXPECTED_ACCOUNT_SHA256="$(printf '0%.0s' {1..64})" "$RUNNER" preflight
  expect_failure 'zero ingress rules' env \
    ADL_ISSUE345_NO_INGRESS_SECURITY_GROUP=adl-issue345-does-not-exist "$RUNNER" preflight
  expect_failure 'inline policy document drifted' env \
    ADL_ISSUE345_INSTANCE_REQUIRED_INLINE_POLICY_SHA256="$(printf '0%.0s' {1..64})" "$RUNNER" preflight
  expect_failure 'deadline reaper function' env \
    ADL_ISSUE345_DEADLINE_REAPER_CODE_SHA256_B64=wrong-code-digest "$RUNNER" preflight
  expect_failure 'artifact manifest SHA-256 drifted' env \
    ADL_ISSUE345_ARTIFACT_MANIFEST_SHA256="$(printf '0%.0s' {1..64})" "$RUNNER" preflight
fi

jq -n --arg live_preflight "$live_preflight" \
  '{schema:"adl.issue345.runner_contract_test.v2",status:"pass",paid_launches:0,
    real_git:true,fake_aws:false,live_aws_preflight:$live_preflight,
    multi_model_ordering:"request_then_residency",
    negative_cases:(if $live_preflight == "passed" then 16 else 11 end)}'
