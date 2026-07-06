#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

BIN="$TMP/bin"
mkdir -p "$BIN"

cat >"$BIN/aws" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "aws $*" >>"${FAKE_AWS_LOG:?}"
case "$1 $2" in
  "sts get-caller-identity")
    printf '%s\n' "123456789012"
    ;;
  "sns create-topic")
    printf '%s\n' "arn:aws:sns:us-west-2:123456789012:adl-v0917-wp08-acip-sns-4685"
    ;;
  "sns delete-topic")
    exit 0
    ;;
  *)
    exit 0
    ;;
esac
SH
chmod +x "$BIN/aws"

cat >"$BIN/proof" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
OUT=""
RUN_ID=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --out)
      OUT="${2:?--out requires a value}"
      shift
      ;;
    --run-id)
      RUN_ID="${2:?--run-id requires a value}"
      shift
      ;;
  esac
  shift
done
test "${ADL_AWS_SIGNAL_MODE:-}" = "live"
test "${ADL_AWS_SIGNAL_APPROVED:-}" = "true"
test "${ADL_AWS_REGION:-}" = "us-west-2"
test "${ADL_AWS_PROFILE:-}" = "agent-logic-admin"
test "${AWS_PROFILE:-}" = "agent-logic-admin"
test -n "${ADL_AWS_SNS_TOPIC_ARN:-}"
test -n "${ADL_AWS_ACCOUNT_HASH:-}"
test -n "${ADL_AWS_ACCOUNT_SHA256:-}"
cat >"$OUT/acip_sns_summary.json" <<JSON
{
  "schema": "adl.wp08.acip_sns_live_proof.v1",
  "issue": 4685,
  "status": "passed",
  "run_id": "$RUN_ID",
  "aws_profile": "agent-logic-admin",
  "aws_region": "us-west-2",
  "aws_account_hash": "$ADL_AWS_ACCOUNT_HASH",
  "sns": {
    "topic_arn_hash": "b2ae781a59c36abf",
    "topic_name": "adl-v0917-wp08-acip-sns-4685",
    "message_id": "00000000-0000-4000-8000-000000000001"
  },
  "acip_projection": {
    "schema_version": "adl.runtime.aws_signal.v1",
    "signal_kind": "acip_projection",
    "runtime_id": "wp08-acip-sns-4685",
    "cycle_id": "cycle-wp08-acip-sns-0001",
    "route_class": "cross_boundary_deferred",
    "projection_level": "content_summary",
    "message_ref": "runtime/wp08/acip/messages/msg-wp08-acip-sns-0001.json",
    "trace_ref": "runtime/wp08/acip/traces/public-summary.json",
    "correlation_id": "wp08-acip-sns-correlation-0001",
    "content_sha256_recorded": true
  },
  "negative_case_policy": {
    "missing_profile": "aws_acip_sns_profile_missing",
    "missing_topic": "aws_acip_sns_topic_missing",
    "malformed_or_denied_projection": "projection_denied",
    "sns_unavailable_or_access_denied": "aws_acip_sns_publish_failed"
  },
  "redaction": {
    "raw_account_id_recorded": false,
    "raw_topic_arn_recorded": false,
    "credentials_recorded": false,
    "raw_message_content_recorded": false
  }
}
JSON
printf '{"status":"passed"}\n'
SH
chmod +x "$BIN/proof"

export PATH="$BIN:$PATH"
export FAKE_AWS_LOG="$TMP/aws.log"

bash "$ROOT/adl/tools/run_wp08_acip_sns_live_proof.sh" \
  --out "$TMP/proof" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --run-id fixture-run \
  --proof-bin "$BIN/proof" \
  --expected-account-sha256 2a33349e7e606a8ad2e30e3c84521f9377450cf09083e162e0a9b1480ce0f972 \
  --cleanup >/tmp/wp08-acip-sns-test-output.json

python3 - "$TMP/proof/acip_sns_summary.json" "$TMP/proof/sns_resource_summary.json" "$FAKE_AWS_LOG" <<'PY'
import json
import sys
from pathlib import Path

summary = json.loads(Path(sys.argv[1]).read_text())
resource = json.loads(Path(sys.argv[2]).read_text())
aws_log = Path(sys.argv[3]).read_text()

assert summary["schema"] == "adl.wp08.acip_sns_live_proof.v1"
assert summary["status"] == "passed"
assert summary["aws_profile"] == "agent-logic-admin"
assert summary["aws_account_hash"] != "123456789012"
assert "aws_account_sha256" not in summary
assert summary["sns"]["topic_name"] == "adl-v0917-wp08-acip-sns-4685"
assert resource["schema"] == "adl.wp08.acip_sns_resource.v1"
assert "aws_account_sha256" not in resource
assert resource["redaction"]["raw_topic_arn_recorded"] is False
for required in [
    "sts get-caller-identity",
    "sns create-topic",
    "sns delete-topic",
]:
    assert required in aws_log, required
assert "agent-logic-admin" in aws_log
PY

python3 "$ROOT/adl/tools/validate_wp08_acip_sns_live_proof.py" \
  "$TMP/proof/acip_sns_summary.json" "$TMP/proof/sns_resource_summary.json" >/dev/null

create_count_before="$(grep -c "sns create-topic" "$FAKE_AWS_LOG" || true)"
if bash "$ROOT/adl/tools/run_wp08_acip_sns_live_proof.sh" \
  --out "$TMP/bad-proof" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --run-id bad-fixture-run \
  --proof-bin "$BIN/proof" \
  --expected-account-sha256 ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff \
  >"$TMP/bad.out" 2>"$TMP/bad.err"; then
  echo "expected mismatched account hash to fail" >&2
  exit 1
fi
grep -F "AWS profile did not resolve to the approved Agent Logic account hash" "$TMP/bad.err" >/dev/null
create_count_after="$(grep -c "sns create-topic" "$FAKE_AWS_LOG" || true)"
if [ "$create_count_before" != "$create_count_after" ]; then
  echo "mismatched account hash must fail before SNS mutation" >&2
  exit 1
fi

echo "PASS test_run_wp08_acip_sns_live_proof"
