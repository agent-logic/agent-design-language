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
  "logs get-log-events")
    cat <<'JSON'
{
  "events": [
    {
      "timestamp": 1783320000000,
      "message": "{\"schema_version\":\"adl.runtime.aws_signal.v1\",\"signal_kind\":\"heartbeat\",\"runtime_id\":\"wp08-heartbeat-4684\",\"cycle_id\":\"cycle-000001\",\"heartbeat_seq\":1,\"status\":\"completed\",\"projection_level\":\"operations_safe\",\"transport\":{\"mode\":\"live\",\"target_kind\":\"cloudwatch_logs\",\"region\":\"us-west-2\",\"approved\":true},\"payload\":{\"state\":\"idle\",\"elapsed_ms\":0,\"next_cycle_hint\":\"sleep_until_next_heartbeat\",\"stop_requested\":false,\"lease_state\":\"clear\"}}"
    }
  ]
}
JSON
    ;;
  *)
    exit 0
    ;;
esac
SH
chmod +x "$BIN/aws"

cat >"$BIN/csm" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "csm $*" >>"${FAKE_CSM_LOG:?}"
test "${ADL_AWS_SIGNAL_MODE:-}" = "live"
test "${ADL_AWS_SIGNAL_APPROVED:-}" = "true"
test "${ADL_AWS_HEARTBEAT_TARGET:-}" = "cloudwatch_logs"
test -n "${ADL_AWS_HEARTBEAT_LOG_GROUP:-}"
test -n "${ADL_AWS_HEARTBEAT_LOG_STREAM:-}"
printf '{"status":"completed"}\n'
if [ -n "${ADL_OBSERVABILITY_LOG:-}" ]; then
  printf 'adl_event schema=adl.observability.event.v1 command=agent stage=aws_runtime_heartbeat result=completed mode=live\n' >"$ADL_OBSERVABILITY_LOG"
fi
SH
chmod +x "$BIN/csm"

export PATH="$BIN:$PATH"
export FAKE_AWS_LOG="$TMP/aws.log"
export FAKE_CSM_LOG="$TMP/csm.log"

bash "$ROOT/adl/tools/run_wp08_heartbeat_live_proof.sh" \
  --out "$TMP/proof" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --run-id fixture-run \
  --csm-bin "$BIN/csm" \
  --cleanup >/tmp/wp08-heartbeat-test-output.json

python3 - "$TMP/proof/live_heartbeat_summary.json" "$FAKE_AWS_LOG" "$FAKE_CSM_LOG" <<'PY'
import json
import sys
from pathlib import Path

summary = json.loads(Path(sys.argv[1]).read_text())
aws_log = Path(sys.argv[2]).read_text()
csm_log = Path(sys.argv[3]).read_text()

assert summary["schema"] == "adl.wp08.heartbeat_live_proof.v1"
assert summary["status"] == "passed"
assert summary["aws_profile"] == "agent-logic-admin"
assert summary["aws_account_hash"] != "123456789012"
assert summary["heartbeat"]["transport_mode"] == "live"
assert summary["heartbeat"]["target_kind"] == "cloudwatch_logs"
assert summary["cloudwatch"]["cleanup_requested"] is True
for required in [
    "logs create-log-group",
    "logs put-retention-policy",
    "logs create-log-stream",
    "logs get-log-events",
    "logs delete-log-stream",
]:
    assert required in aws_log, required
assert "logs delete-log-group" not in aws_log
assert "daemon --spec" in csm_log
PY

python3 "$ROOT/adl/tools/validate_wp08_heartbeat_live_proof.py" \
  "$TMP/proof/live_heartbeat_summary.json" >/dev/null

echo "PASS test_run_wp08_heartbeat_live_proof"
