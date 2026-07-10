#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: bash adl/tools/run_wp08_heartbeat_live_proof.sh --out <dir> [options]

Proves WP-08 #4684 live runtime heartbeat publication by creating or reusing a
bounded CloudWatch Logs group/stream, running the standalone csm daemon with
ADL_AWS_SIGNAL_MODE=live, fetching the emitted heartbeat event, and writing a
redacted summary.

Options:
  --out <dir>             Required proof output directory.
  --profile <name>        AWS profile. Default: agent-logic-admin.
  --region <region>       AWS region. Default: us-west-2.
  --run-id <id>           Run id suffix. Default: wp08-4684-<utc>.
  --csm-bin <path>        csm binary. Default: ADL_CSM_BIN or adl/target/debug/csm.
  --cleanup               Delete the issue log group after verification.
  --help                  Show this help.
USAGE
}

OUT=""
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
RUN_ID="wp08-4684-$(date -u +%Y%m%dT%H%M%SZ)"
CSM_BIN="${ADL_CSM_BIN:-adl/target/debug/csm}"
CLEANUP=0

while [ "$#" -gt 0 ]; do
  case "$1" in
    --out)
      OUT="${2:?--out requires a value}"
      shift
      ;;
    --profile)
      PROFILE="${2:?--profile requires a value}"
      shift
      ;;
    --region)
      REGION="${2:?--region requires a value}"
      shift
      ;;
    --run-id)
      RUN_ID="${2:?--run-id requires a value}"
      shift
      ;;
    --csm-bin)
      CSM_BIN="${2:?--csm-bin requires a value}"
      shift
      ;;
    --cleanup)
      CLEANUP=1
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

if [ -z "$OUT" ]; then
  echo "--out is required" >&2
  usage >&2
  exit 2
fi

mkdir -p "$OUT"
# shellcheck source=adl/tools/csm_binary_availability.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/csm_binary_availability.sh"
CSM_BIN="$(adl_resolve_csm_binary "$CSM_BIN" "$OUT/csm_binary_availability.json")"

AWS_BIN="${AWS_BIN:-aws}"
if ! command -v "$AWS_BIN" >/dev/null 2>&1; then
  echo "aws CLI not found; set AWS_BIN or install aws CLI" >&2
  exit 2
fi

LOG_GROUP="/adl/v0917/wp08/4684/runtime-heartbeat"
LOG_STREAM="run-${RUN_ID//[^A-Za-z0-9_.-]/-}"
SUMMARY="$OUT/live_heartbeat_summary.json"
EVENTS_JSON="$OUT/cloudwatch_get_log_events.redacted.json"
OBS_LOG="$OUT/observability_${RUN_ID//[^A-Za-z0-9_.-]/-}.log"
SPEC="$OUT/agent.yaml"
RUN_STATE_ROOT="state/${RUN_ID//[^A-Za-z0-9_.-]/-}"
CLEANUP_TARGET="stream"

rm -f "$SUMMARY" "$EVENTS_JSON" "$OBS_LOG" "$OUT/csm_stdout.json" "$OUT/csm_stderr.log"

ACCOUNT="$("$AWS_BIN" sts get-caller-identity --profile "$PROFILE" --query Account --output text)"
ACCOUNT_HASH="$(printf '%s' "$ACCOUNT" | shasum -a 256 | awk '{print substr($1,1,16)}')"

"$AWS_BIN" logs create-log-group \
  --profile "$PROFILE" \
  --region "$REGION" \
  --log-group-name "$LOG_GROUP" >/dev/null 2>&1 || true
"$AWS_BIN" logs put-retention-policy \
  --profile "$PROFILE" \
  --region "$REGION" \
  --log-group-name "$LOG_GROUP" \
  --retention-in-days 7 >/dev/null
"$AWS_BIN" logs create-log-stream \
  --profile "$PROFILE" \
  --region "$REGION" \
  --log-group-name "$LOG_GROUP" \
  --log-stream-name "$LOG_STREAM" >/dev/null 2>&1 || true

cat >"$SPEC" <<'YAML'
schema: adl.long_lived_agent_spec.v1
agent_instance_id: wp08-heartbeat-4684
display_name: WP08 Heartbeat 4684
state_root: __RUN_STATE_ROOT__
workflow:
  kind: demo_adapter
  name: wp08_heartbeat_live_proof
  run_args:
    provider_id: local_fixture
    model: none
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 60
  max_consecutive_failures: 1
memory:
  namespace: runtime/wp08/heartbeat/4684
  write_policy: append_only
YAML
python3 - "$SPEC" "$RUN_STATE_ROOT" <<'PY'
import sys
from pathlib import Path

path = Path(sys.argv[1])
state_root = sys.argv[2]
path.write_text(path.read_text().replace("__RUN_STATE_ROOT__", state_root))
PY

START_MS="$(python3 - <<'PY'
import time
print(int((time.time() - 30) * 1000))
PY
)"

ADL_AWS_SIGNAL_MODE=live \
ADL_AWS_SIGNAL_APPROVED=true \
ADL_AWS_REGION="$REGION" \
ADL_AWS_PROFILE="$PROFILE" \
AWS_PROFILE="$PROFILE" \
ADL_AWS_HEARTBEAT_TARGET=cloudwatch_logs \
ADL_AWS_HEARTBEAT_LOG_GROUP="$LOG_GROUP" \
ADL_AWS_HEARTBEAT_LOG_STREAM="$LOG_STREAM" \
ADL_OBSERVABILITY_LOG="$OBS_LOG" \
ADL_OBSERVABILITY_STDERR=0 \
"$CSM_BIN" daemon --spec "$SPEC" --test-supervisor-failure-after-restarts 1 --checkpoint-interval-secs 1 --no-sleep --json \
  >"$OUT/csm_stdout.json" 2>"$OUT/csm_stderr.log"

event_found=0
for attempt in $(seq 1 15); do
  "$AWS_BIN" logs get-log-events \
    --profile "$PROFILE" \
    --region "$REGION" \
    --log-group-name "$LOG_GROUP" \
    --log-stream-name "$LOG_STREAM" \
    --start-from-head \
    --start-time "$START_MS" \
    --output json >"$EVENTS_JSON"
  if python3 - "$EVENTS_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
for event in data.get("events", []):
    try:
        payload = json.loads(event.get("message", ""))
    except json.JSONDecodeError:
        continue
    if payload.get("schema_version") == "adl.runtime.aws_signal.v1" and payload.get("signal_kind") == "heartbeat":
        raise SystemExit(0)
raise SystemExit(1)
PY
  then
    event_found=1
    break
  fi
  if [ "$attempt" -lt 15 ]; then
    sleep 2
  fi
done

if [ "$event_found" -ne 1 ]; then
  echo "CloudWatch heartbeat event was not visible after retry window" >&2
fi

python3 - "$EVENTS_JSON" "$OBS_LOG" "$SUMMARY" "$RUN_ID" "$REGION" "$PROFILE" "$ACCOUNT" "$ACCOUNT_HASH" "$LOG_GROUP" "$LOG_STREAM" "$CLEANUP" "$CLEANUP_TARGET" <<'PY'
import json
import sys
from pathlib import Path

(
    events_path,
    obs_path,
    summary_path,
    run_id,
    region,
    profile,
    account,
    account_hash,
    log_group,
    log_stream,
    cleanup,
    cleanup_target,
) = sys.argv[1:]
raw = json.loads(Path(events_path).read_text())
messages = []
for event in raw.get("events", []):
    message = event.get("message", "")
    try:
        payload = json.loads(message)
    except json.JSONDecodeError:
        continue
    if payload.get("schema_version") == "adl.runtime.aws_signal.v1" and payload.get("signal_kind") == "heartbeat":
        messages.append(payload)

if not messages:
    raise SystemExit("no runtime heartbeat event returned from CloudWatch Logs")

selected = messages[-1]
observability = Path(obs_path).read_text(errors="replace") if Path(obs_path).exists() else ""
events_text = Path(events_path).read_text(errors="replace")
summary = {
    "schema": "adl.wp08.heartbeat_live_proof.v1",
    "issue": 4684,
    "status": "passed",
    "run_id": run_id,
    "aws_profile": profile,
    "aws_region": region,
    "aws_account_hash": account_hash,
    "cloudwatch": {
        "log_group": log_group,
        "log_stream": log_stream,
        "retention_days": 7,
        "cleanup_requested": cleanup == "1",
        "cleanup_target": cleanup_target if cleanup == "1" else "not_requested",
        "event_count": len(messages),
    },
    "heartbeat": {
        "schema_version": selected.get("schema_version"),
        "signal_kind": selected.get("signal_kind"),
        "runtime_id": selected.get("runtime_id"),
        "cycle_id": selected.get("cycle_id"),
        "heartbeat_seq": selected.get("heartbeat_seq"),
        "status": selected.get("status"),
        "projection_level": selected.get("projection_level"),
        "transport_mode": selected.get("transport", {}).get("mode"),
        "target_kind": selected.get("transport", {}).get("target_kind"),
        "payload_state": selected.get("payload", {}).get("state"),
    },
    "negative_cases": {
        "approval_gate": "covered_by_focused_rust_tests",
        "unsupported_target": "covered_by_focused_rust_tests",
        "profile_missing": "covered_by_focused_rust_tests",
    },
    "redaction": {
        "raw_account_id_recorded": False,
        "credentials_recorded": False,
        "observability_contains_account_id": account in observability,
        "cloudwatch_export_contains_account_id": account in events_text,
    },
}
Path(summary_path).write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
print(json.dumps(summary, sort_keys=True))
PY

if [ "$CLEANUP" -eq 1 ]; then
  "$AWS_BIN" logs delete-log-stream \
    --profile "$PROFILE" \
    --region "$REGION" \
    --log-group-name "$LOG_GROUP" \
    --log-stream-name "$LOG_STREAM" >/dev/null
fi
