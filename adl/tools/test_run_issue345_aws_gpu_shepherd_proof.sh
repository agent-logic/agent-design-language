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
if AWS_PROFILE=default AWS_REGION=us-west-2 "$RUNNER" preflight >"$tmp/wrong-profile.out" 2>"$tmp/wrong-profile.err"; then
  fail "wrong AWS profile unexpectedly passed"
fi
grep -q 'AWS profile must be agent-logic-admin' "$tmp/wrong-profile.err" || fail "wrong profile did not fail closed"

head="$(git -C "$ROOT" rev-parse HEAD)"
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
    negative_cases:(if $live_preflight == "passed" then 7 else 2 end)}'
