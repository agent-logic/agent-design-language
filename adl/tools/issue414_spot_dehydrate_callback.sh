#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CALLBACK_STARTED_EPOCH="$(date +%s)"
BIN="${ADL_ISSUE414_CONTINUITY_BIN:-$ROOT/adl/target/debug/adl_resident_shepherd_continuity}"
BASE_INPUT="${ADL_SPOT_RESIDENT_INPUT:-}"
RUNTIME_ROOT="${ADL_SPOT_RETAINED_RUNTIME_ROOT:-}"
NOTICE_FILE=""
DEADLINE_UTC=""
RUN_ROOT=""
EXPECTED_VOLUME_SHA256="${ADL_SPOT_RUNTIME_VOLUME_ID_SHA256:-}"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --notice-file) NOTICE_FILE="${2:?}"; shift 2 ;;
    --deadline-utc) DEADLINE_UTC="${2:?}"; shift 2 ;;
    --run-root) RUN_ROOT="${2:?}"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done
[[ -x "$BIN" && -f "$BASE_INPUT" && -d "$RUNTIME_ROOT" && -f "$NOTICE_FILE" && -n "$RUN_ROOT" && "$EXPECTED_VOLUME_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
  echo "Spot dehydration callback configuration is incomplete" >&2
  exit 66
}
command -v findmnt >/dev/null
command -v lsblk >/dev/null
mount_source="$(findmnt -no SOURCE --target "$RUNTIME_ROOT")"
volume_serial="$(lsblk -ndo SERIAL "$mount_source" | head -1 | tr -d '[:space:]')"
[[ -n "$volume_serial" ]] || { echo "retained Runtime volume serial is unavailable" >&2; exit 66; }
if [[ "$volume_serial" == vol* && "$volume_serial" != vol-* ]]; then
  volume_serial="vol-${volume_serial#vol}"
fi
if command -v sha256sum >/dev/null 2>&1; then
  observed_volume_sha256="$(printf '%s' "$volume_serial" | sha256sum | awk '{print $1}')"
else
  observed_volume_sha256="$(printf '%s' "$volume_serial" | shasum -a 256 | awk '{print $1}')"
fi
[[ "$observed_volume_sha256" == "$EXPECTED_VOLUME_SHA256" ]] || {
  echo "mounted Runtime volume identity does not match the approved retained volume" >&2
  exit 66
}
jq -e --arg identity "$EXPECTED_VOLUME_SHA256" '.runtime_volume_identity_sha256 == $identity' "$BASE_INPUT" >/dev/null
jq -e --arg deadline "$DEADLINE_UTC" '(.action == "terminate" or .action == "stop") and .time == $deadline' "$NOTICE_FILE" >/dev/null
if command -v sha256sum >/dev/null 2>&1; then
  notice_sha256="$(sha256sum "$NOTICE_FILE" | awk '{print $1}')"
else
  notice_sha256="$(shasum -a 256 "$NOTICE_FILE" | awk '{print $1}')"
fi
callback_input="$RUN_ROOT/spot-dehydration-input.json"
jq --arg source "aws_imdsv2_spot_instance_action" \
   --arg action "$(jq -r '.action' "$NOTICE_FILE")" \
   --arg deadline "$DEADLINE_UTC" \
   --arg digest "$notice_sha256" \
   '.spot_notice={source:$source,action:$action,deadline_utc:$deadline,notice_sha256:$digest}' \
   "$BASE_INPUT" >"$callback_input"
receipt="$RUN_ROOT/spot-dehydration-command-receipt.json"
# TERM is followed by KILL after two seconds, so a synchronous status/capsule
# call cannot outlive the bounded child. Failure preserves all stop intents and
# closed admission; clearing them could erase concurrent operator authority.
remaining_seconds="$(python3 - "$DEADLINE_UTC" <<'PY'
import datetime, sys
deadline = datetime.datetime.fromisoformat(sys.argv[1].replace("Z", "+00:00"))
now = datetime.datetime.now(datetime.timezone.utc)
print(max(0, int((deadline - now).total_seconds())))
PY
)"
elapsed_seconds=$(( $(date +%s) - CALLBACK_STARTED_EPOCH ))
outer_remaining=$((85 - elapsed_seconds))
(( remaining_seconds > 3 && outer_remaining > 2 )) || { echo "Spot deadline leaves no bounded dehydration window" >&2; exit 70; }
work_seconds=$((remaining_seconds - 3))
(( work_seconds <= outer_remaining )) || work_seconds="$outer_remaining"
set +e
timeout --signal=TERM --kill-after=2 "$work_seconds" "$BIN" dehydrate \
  --input "$callback_input" --runtime-root "$RUNTIME_ROOT" --output "$receipt"
dehydrate_status=$?
set -e
if (( dehydrate_status != 0 )); then
  echo "Spot dehydration failed before termination readiness; admission remains closed and stop intents are preserved" >&2
  exit "$dehydrate_status"
fi
cat "$receipt"
