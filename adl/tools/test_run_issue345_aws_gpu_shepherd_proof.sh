#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER="$ROOT/adl/tools/run_issue345_aws_gpu_shepherd_proof.sh"
GIT_COMMON_DIR="$(git -C "$ROOT" rev-parse --git-common-dir 2>/dev/null || true)"
if [[ -n "$GIT_COMMON_DIR" ]]; then
  DEFAULT_FIXTURE_ROOT="$GIT_COMMON_DIR/csdlc-v2/issue345/test-fixtures"
else
  DEFAULT_FIXTURE_ROOT="$ROOT/.adl/local/issue345/test-fixtures"
fi
FIXTURE_ROOT="${ADL_ISSUE345_TEST_FIXTURE_ROOT:-$DEFAULT_FIXTURE_ROOT}"
CASE_DIR="$FIXTURE_ROOT/issue345-runner-contract"
BIN_DIR="$CASE_DIR/bin"
STATE_DIR="$CASE_DIR/state"
CALL_LOG="$CASE_DIR/aws-calls.log"
ACCOUNT_ID="123456789012"
ACCOUNT_SHA="$(printf '%s' "$ACCOUNT_ID" | shasum -a 256 | awk '{print $1}')"
MANIFEST="$CASE_DIR/artifact-manifest.json"
MANIFEST_SHA=""

fail() {
  echo "FAIL issue345-runner-contract: $*" >&2
  exit 1
}

reset_fixture() {
  rm -rf "$CASE_DIR"
  mkdir -p "$BIN_DIR" "$STATE_DIR"
  cat >"$MANIFEST" <<'JSON'
{
  "schema": "adl.shepherd.portable_model_bundle.v1",
  "model_identity": "gemma4:12b",
  "model_digest_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "artifacts": [
    {
      "kind": "ollama_model_store",
      "key": "models/gemma4/store.zst",
      "version_id": "model-version",
      "relative_path": "model-store/store.zst",
      "sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    }
  ]
}
JSON
  MANIFEST_SHA="$(shasum -a 256 "$MANIFEST" | awk '{print $1}')"
  cat >"$BIN_DIR/aws" <<'AWS'
#!/usr/bin/env bash
set -euo pipefail
LOG="${ADL_ISSUE345_FAKE_AWS_LOG:?}"
printf '%s\n' "$*" >>"$LOG"
if [[ "${ADL_ISSUE345_FAKE_AWS_FAIL_LOCK:-}" == "1" && "$*" == *"s3api put-object"* ]]; then
  echo "An error occurred (PreconditionFailed) when calling the PutObject operation" >&2
  exit 255
fi
case "$*" in
  *"sts get-caller-identity"*)
    printf '123456789012\n'
    ;;
  *"ec2 describe-security-groups"*)
    printf '%s\n' '[{"GroupId":"sg-0secret","GroupName":"adl-issue345-no-ingress","IpPermissions":[]}]'
    ;;
  *"iam get-instance-profile"*)
    printf '%s\n' '{"InstanceProfile":{"InstanceProfileName":"ADLRemoteValidationPermanentProfile","Roles":[{"RoleName":"ADLIssue345Role"}]}}'
    ;;
  *"lambda get-function-configuration"*)
    printf '%s\n' '{"State":"Active","LastUpdateStatus":"Successful","Timeout":30}'
    ;;
  *"events describe-rule"*)
    printf '%s\n' '{"State":"ENABLED"}'
    ;;
  *"events list-targets-by-rule"*)
    printf '%s\n' '{"Targets":[{"Id":"target","Arn":"arn"}]}'
    ;;
  *"s3api get-object"*)
    destination="${@: -1}"
    cp "${ADL_ISSUE345_FAKE_MANIFEST:?}" "$destination"
    printf '%s\n' '{"VersionId":"manifest-version"}'
    ;;
  *"service-quotas get-service-quota"*)
    printf '4\n'
    ;;
  *"pricing get-products"*)
    printf '%s\n' '["{\"terms\":{\"OnDemand\":{\"x\":{\"priceDimensions\":{\"y\":{\"pricePerUnit\":{\"USD\":\"0.80\"}}}}}}}"]'
    ;;
  *"ec2 describe-instances"*)
    printf '\n'
    ;;
  *"s3api put-object"*)
    printf '%s\n' '{"VersionId":"lock-version-secret"}'
    ;;
  *"ec2 run-instances"*)
    printf '%s\n' '{"Instances":[{"InstanceId":"i-secret"}]}'
    ;;
  *"ec2 terminate-instances"*|*"ec2 wait instance-terminated"*|*"ec2 describe-volumes"*|*"s3api delete-object"*)
    printf '\n'
    ;;
  *)
    printf '%s\n' '{}'
    ;;
esac
AWS
  chmod +x "$BIN_DIR/aws"
}

run_with_fake_aws() {
  PATH="$BIN_DIR:$PATH" \
  AWS_PROFILE=agent-logic-admin \
  AWS_REGION=us-west-2 \
  ADL_ISSUE345_STATE_ROOT="$STATE_DIR" \
  ADL_ISSUE345_EXPECTED_ACCOUNT_SHA256="$ACCOUNT_SHA" \
  ADL_ISSUE345_ARTIFACT_BUCKET=issue345-artifacts \
  ADL_ISSUE345_ARTIFACT_MANIFEST_VERSION_ID=manifest-version \
  ADL_ISSUE345_ARTIFACT_MANIFEST_SHA256="$MANIFEST_SHA" \
  ADL_ISSUE345_FAKE_AWS_LOG="$CALL_LOG" \
  ADL_ISSUE345_FAKE_MANIFEST="$MANIFEST" \
  "$RUNNER" "$@"
}

assert_no_secret_output() {
  local file="$1"
  if grep -Eq '123456789012|sg-0secret|i-secret|lock-version-secret|[A-Z0-9]{20,}' "$file"; then
    fail "output leaked raw account/resource/token material: $file"
  fi
}

reset_fixture
bash -n "$RUNNER"

preflight_out="$CASE_DIR/preflight.json"
run_with_fake_aws preflight >"$preflight_out"
jq -e '.schema == "adl.issue345.aws_gpu_preflight.v1"
  and .profile == "agent-logic-admin"
  and .region == "us-west-2"
  and .price_ready == true
  and .quota_ready == true
  and .public_ingress == false
  and .paid_launch == false
  and .active_issue_instance_count == 0
  and (.account_sha256 | test("^[0-9a-f]{64}$"))
  and (.no_ingress_security_group_sha256 | test("^[0-9a-f]{64}$"))' "$preflight_out" >/dev/null ||
  fail "preflight JSON contract failed"
assert_no_secret_output "$preflight_out"
grep -q 'ec2 run-instances' "$CALL_LOG" && fail "preflight must not launch compute"

wrong_profile_err="$CASE_DIR/wrong-profile.err"
if PATH="$BIN_DIR:$PATH" AWS_PROFILE=default AWS_REGION=us-west-2 "$RUNNER" preflight >"$CASE_DIR/wrong-profile.out" 2>"$wrong_profile_err"; then
  fail "wrong profile unexpectedly passed"
fi
grep -q 'AWS profile must be agent-logic-admin' "$wrong_profile_err" ||
  fail "wrong profile error was not fail-closed"

no_execute_err="$CASE_DIR/no-execute.err"
if run_with_fake_aws run --commit 0123456789abcdef0123456789abcdef01234567 --run-id adl-issue345-test >"$CASE_DIR/no-execute.out" 2>"$no_execute_err"; then
  fail "run without --execute unexpectedly passed"
fi
grep -q 'paid execution requires --execute' "$no_execute_err" ||
  fail "missing --execute did not fail closed"

no_auth_err="$CASE_DIR/no-auth.err"
if run_with_fake_aws run --commit 0123456789abcdef0123456789abcdef01234567 --run-id adl-issue345-test --execute >"$CASE_DIR/no-auth.out" 2>"$no_auth_err"; then
  fail "run without retained authorization unexpectedly passed"
fi
grep -q 'ADL_ISSUE345_PAID_RUN_AUTHORIZATION=authorized' "$no_auth_err" ||
  fail "missing authorization did not fail closed"

bad_commit_err="$CASE_DIR/bad-commit.err"
if PATH="$BIN_DIR:$PATH" AWS_PROFILE=agent-logic-admin AWS_REGION=us-west-2 ADL_ISSUE345_PAID_RUN_AUTHORIZATION=authorized "$RUNNER" run --commit main --run-id adl-issue345-test --execute >"$CASE_DIR/bad-commit.out" 2>"$bad_commit_err"; then
  fail "non-exact commit unexpectedly passed"
fi
grep -q 'exact 40-character Git commit' "$bad_commit_err" ||
  fail "bad commit did not fail closed"

lock_err="$CASE_DIR/lock-collision.err"
if PATH="$BIN_DIR:$PATH" \
  AWS_PROFILE=agent-logic-admin \
  AWS_REGION=us-west-2 \
  ADL_ISSUE345_PAID_RUN_AUTHORIZATION=authorized \
  ADL_ISSUE345_STATE_ROOT="$STATE_DIR" \
  ADL_ISSUE345_EXPECTED_ACCOUNT_SHA256="$ACCOUNT_SHA" \
  ADL_ISSUE345_ARTIFACT_BUCKET=issue345-artifacts \
  ADL_ISSUE345_ARTIFACT_MANIFEST_VERSION_ID=manifest-version \
  ADL_ISSUE345_ARTIFACT_MANIFEST_SHA256="$MANIFEST_SHA" \
  ADL_ISSUE345_FAKE_AWS_LOG="$CALL_LOG" \
  ADL_ISSUE345_FAKE_MANIFEST="$MANIFEST" \
  ADL_ISSUE345_FAKE_AWS_FAIL_LOCK=1 \
  "$RUNNER" run --commit 0123456789abcdef0123456789abcdef01234567 --run-id adl-issue345-test --execute >"$CASE_DIR/lock-collision.out" 2>"$lock_err"; then
  fail "lock collision unexpectedly passed"
fi
grep -q 'PreconditionFailed' "$lock_err" || fail "lock collision did not expose AWS conditional failure"
grep -q 's3api put-object' "$CALL_LOG" || fail "lock acquisition was not attempted"

cleanup_err="$CASE_DIR/cleanup-owner.err"
if run_with_fake_aws cleanup --run-id adl-issue345-test --owner-token nothex --lock-version-id lock-version >"$CASE_DIR/cleanup-owner.out" 2>"$cleanup_err"; then
  fail "bad owner token cleanup unexpectedly passed"
fi
grep -q 'owner token must be the exact 32-character execution token' "$cleanup_err" ||
  fail "cleanup owner-token guard did not fail closed"

printf '%s\n' '{"schema":"adl.issue345.runner_contract_test.v1","status":"pass","paid_launches":0,"preflight":"pass","negative_cases":5}'
