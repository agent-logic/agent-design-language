#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$repo_root"

if [[ "${ADL_LIVE_WUJI_A2A_HISTORY:-0}" != "1" ]]; then
  cat <<'JSON'
{
  "schema": "adl.issue713.live_wuji_a2a_history.v1",
  "status": "deferred",
  "reason": "Set ADL_LIVE_WUJI_A2A_HISTORY=1 only when the operator authorizes a live Wuji Runtime/ACIP acceptance run.",
  "required_proof": [
    "raw_redacted_bidirectional_non_shepherd_a2a_runtime_evidence",
    "verbatim_user_visible_outbound_and_reply_content",
    "causal_order_conversation_turn_work_correlation_ids",
    "authenticated_history_api_projection_after_reconnect",
    "checkpoint_restart_rehydration_recovery_without_duplication",
    "redaction_excludes_system_developer_prompts_credentials_secrets_provider_internal_material"
  ]
}
JSON
  exit 0
fi

if [[ ! -x "adl/tools/test_issue713_live_a2a_history.sh" ]]; then
  echo "missing executable live proof runner: adl/tools/test_issue713_live_a2a_history.sh" >&2
  exit 2
fi

exec bash adl/tools/test_issue713_live_a2a_history.sh
