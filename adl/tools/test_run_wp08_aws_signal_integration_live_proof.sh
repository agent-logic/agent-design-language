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
  "sts get-caller-identity") printf '123456789012\n' ;;
  *) echo "unexpected aws call $*" >&2; exit 1 ;;
esac
SH
chmod +x "$BIN/aws"

cat >"$TMP/fake_heartbeat.sh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "heartbeat $*" >>"${FAKE_CHILD_LOG:?}"
OUT=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --out) OUT="$2"; shift ;;
  esac
  shift
done
mkdir -p "$OUT"
cat >"$OUT/live_heartbeat_summary.json" <<'JSON'
{
  "schema": "adl.wp08.heartbeat_live_proof.v1",
  "issue": 4684,
  "status": "passed",
  "aws_profile": "agent-logic-admin",
  "aws_region": "us-west-2",
  "aws_account_hash": "2a33349e7e606a8a",
  "cloudwatch": {
    "log_group": "/adl/v0917/wp08/4684/runtime-heartbeat",
    "log_stream": "run-wp08-4686-fake-heartbeat",
    "retention_days": 7,
    "event_count": 1
  },
  "heartbeat": {
    "signal_kind": "heartbeat",
    "transport_mode": "live",
    "target_kind": "cloudwatch_logs"
  },
  "redaction": {
    "raw_account_id_recorded": false,
    "credentials_recorded": false
  }
}
JSON
SH
chmod +x "$TMP/fake_heartbeat.sh"

cat >"$TMP/fake_acip.sh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "acip $*" >>"${FAKE_CHILD_LOG:?}"
OUT=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --out) OUT="$2"; shift ;;
  esac
  shift
done
mkdir -p "$OUT"
cat >"$OUT/acip_sns_summary.json" <<'JSON'
{
  "schema": "adl.wp08.acip_sns_live_proof.v1",
  "issue": 4685,
  "status": "passed",
  "aws_profile": "agent-logic-admin",
  "aws_region": "us-west-2",
  "aws_account_hash": "2a33349e7e606a8a",
  "sns": {
    "topic_name": "adl-v0917-wp08-acip-sns-4685",
    "topic_arn_hash": "aaaaaaaaaaaaaaaa",
    "message_id": "fake-message-id"
  },
  "acip_projection": {
    "signal_kind": "acip_projection",
    "route_class": "cross_boundary_deferred",
    "projection_level": "content_summary"
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
cat >"$OUT/sns_resource_summary.json" <<'JSON'
{
  "schema": "adl.wp08.acip_sns_resource.v1",
  "issue": 4685,
  "aws_profile": "agent-logic-admin",
  "aws_region": "us-west-2",
  "aws_account_hash": "2a33349e7e606a8a",
  "sns": {
    "topic_name": "adl-v0917-wp08-acip-sns-4685",
    "topic_arn_hash": "aaaaaaaaaaaaaaaa",
    "cleanup_requested": false
  }
}
JSON
SH
chmod +x "$TMP/fake_acip.sh"

export AWS_BIN="$BIN/aws"
export FAKE_AWS_LOG="$TMP/aws.log"
export FAKE_CHILD_LOG="$TMP/child.log"
export ADL_WP08_HEARTBEAT_PROOF_SCRIPT="$TMP/fake_heartbeat.sh"
export ADL_WP08_ACIP_SNS_PROOF_SCRIPT="$TMP/fake_acip.sh"

EXPECTED="$(printf '123456789012' | shasum -a 256 | awk '{print $1}')"
bash "$ROOT/adl/tools/run_wp08_aws_signal_integration_live_proof.sh" \
  --out "$TMP/proof" \
  --expected-account-sha256 "$EXPECTED" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --csm-bin "$TMP/fake-csm" \
  --acip-proof-bin "$TMP/fake-acip-bin" >/dev/null

python3 "$ROOT/adl/tools/validate_wp08_aws_signal_integration_live_proof.py" \
  "$TMP/proof/aws_signal_integration_summary.json" >/dev/null
grep -F "heartbeat" "$FAKE_CHILD_LOG" >/dev/null
grep -F "acip" "$FAKE_CHILD_LOG" >/dev/null

: >"$FAKE_CHILD_LOG"
if bash "$ROOT/adl/tools/run_wp08_aws_signal_integration_live_proof.sh" \
  --out "$TMP/bad" \
  --expected-account-sha256 0000000000000000000000000000000000000000000000000000000000000000 \
  --profile agent-logic-admin \
  --region us-west-2 \
  --csm-bin "$TMP/fake-csm" \
  --acip-proof-bin "$TMP/fake-acip-bin" 2>/tmp/wp08-4686-bad.err; then
  echo "expected mismatch failure" >&2
  exit 1
fi
if [ -s "$FAKE_CHILD_LOG" ]; then
  echo "account mismatch reached child proof scripts" >&2
  cat "$FAKE_CHILD_LOG" >&2
  exit 1
fi

echo "PASS test_run_wp08_aws_signal_integration_live_proof"
