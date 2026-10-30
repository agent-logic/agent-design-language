#!/usr/bin/env bash
set -euo pipefail

# Preserved original historical telemetry contract; not native terminal authority.
source_root="${CSDLC_HISTORICAL_SOURCE_ROOT:-}"
if [[ "$source_root" != /* || ! -d "$source_root" ]]; then
  echo "Select an authentic complete original telemetry fixture with CSDLC_HISTORICAL_SOURCE_ROOT." >&2
  exit 2
fi
python3 - "$source_root" <<'SOURCE_PINS'
import hashlib
from pathlib import Path
import sys
root = Path(sys.argv[1])
pins = {
    "adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_from_codex_session.py": "359c4274fc2498c67b0b97a912d5f573c35a06d7f2fb04309d147353ddfee3d6",
    "adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py": "cea05f335af023b6576faeaef57cd776ee5dcf0c09b67e17e51d50c6f7b869db",
}
for relative, expected in pins.items():
    try:
        actual = hashlib.sha256((root / relative).read_bytes()).hexdigest()
    except OSError:
        raise SystemExit("Missing original telemetry resource: " + relative)
    if actual != expected:
        raise SystemExit("Changed original telemetry resource: " + relative)
SOURCE_PINS
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

home_dir="${tmpdir}/home"
session_root="${home_dir}/.codex/sessions/2026/07/02"
artifacts_dir="${tmpdir}/artifacts"
mkdir -p "${session_root}" "${artifacts_dir}"

thread_id="thread-4666-completed"
transcript_path="${session_root}/rollout-2026-07-02T00-00-00-${thread_id}.jsonl"
cat >"${transcript_path}" <<'EOF_TRANSCRIPT'
{"type":"response_item","payload":{"item":{"type":"function_call","name":"get_goal","call_id":"call_goal_4666"}}}
{"type":"response_item","payload":{"item":{"type":"function_call_output","call_id":"call_goal_4666","output":"{\"goal\":{\"threadId\":\"thread-4666-completed\",\"objective\":\"Issue #4666 session\",\"status\":\"complete\",\"tokensUsed\":2500,\"timeUsedSeconds\":60,\"createdAt\":1783010000,\"updatedAt\":1783010060}}"}}}
EOF_TRANSCRIPT

python3 "${source_root}/adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_from_codex_session.py" \
  --issue-number 4666 \
  --artifacts-dir "${artifacts_dir}" \
  --capture-stage card_repair \
  --issue-goal-ref goal:v0.91.7:issue:4666 \
  --sprint-goal-ref issue-4631 \
  --thread-id "${thread_id}" \
  --session-root "${home_dir}/.codex/sessions" \
  --metrics-confidence high \
  >"${tmpdir}/readiness-prep-non-terminal.out"

python3 - "$artifacts_dir/issue-4666-goal-metrics-summary.json" <<'PY'
import json
import sys
summary = json.loads(open(sys.argv[1], encoding="utf-8").read())
assert summary["selected_stage"] == "card_repair", summary
assert summary["selected_segment"] == "readiness_prep", summary
assert summary["completion_state"] == "unknown", summary
assert summary["goal_terminal_state"]["completion_allowed"] is False, summary
assert summary["sprint_goal_ref"] == "issue-4631", summary
PY

if python3 "${source_root}/adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_from_codex_session.py" \
  --issue-number 4666 \
  --artifacts-dir "${tmpdir}/bound-artifacts" \
  --capture-stage issue_start \
  --issue-goal-ref goal:v0.91.7:issue:4666 \
  --sprint-goal-ref issue-4631 \
  --thread-id "${thread_id}" \
  --session-root "${home_dir}/.codex/sessions" \
  --metrics-confidence high \
  >"${tmpdir}/issue-start.out" 2>"${tmpdir}/issue-start.err"; then
  echo "expected issue_start capture to reject terminal completion without PR truth" >&2
  exit 1
fi

grep -q "issue goal cannot be recorded as completed" "${tmpdir}/issue-start.err"
echo "PASS readiness prep metrics remain non-terminal"
