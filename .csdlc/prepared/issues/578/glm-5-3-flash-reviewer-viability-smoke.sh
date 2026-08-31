#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

out_dir=".adl/provider-smoke/glm-5-3-flash"
mkdir -p "$out_dir"

request="$out_dir/request.json"
result="$out_dir/result.json"
log="$out_dir/provider-run.jsonl"

if [ -z "${ZAI_API_KEY:-}" ]; then
  printf '{"schema":"adl.provider_smoke_result.v1","status":"skipped","reason":"ZAI_API_KEY not set","profile":"z_ai:glm-5.3-flash","live_provider_call":false}\n'
  exit 0
fi

cat > "$request" <<'JSON'
{
  "route": {
    "provider_kind": "hosted",
    "provider": "z_ai",
    "runtime_surface": "hosted_api",
    "provider_model_id": "glm-5.3-flash",
    "credential_ref": "env:ZAI_API_KEY",
    "source_registry": "z_ai:glm-5.3-flash"
  },
  "model_identity": {
    "provider_kind": "hosted",
    "provider": "z_ai",
    "model_ref": "hosted:adl-z-ai:glm-5.3-flash",
    "provider_model_id": "glm-5.3-flash",
    "runtime_surface": "hosted_api",
    "identity_strength": "provider_asserted",
    "observed_at": "unix:0"
  },
  "prompt_contract_ref": "adl.review_provider_viability_smoke.v1",
  "lane_ref": "reviewer_viability",
  "run_id": "glm-5-3-flash-reviewer-viability",
  "request_id": "glm-5-3-flash-reviewer-viability-001",
  "attempt_policy": {
    "max_attempts": 1,
    "timeout_ms": 45000,
    "retry_backoff_ms": 1
  },
  "input_text": "Reply with exactly: GLM reviewer smoke ok",
  "max_output_tokens": 64,
  "reasoning_effort": "low",
  "clear_thinking": true,
  "temperature": 0.2,
  "top_p": 0.8,
  "inference_parameter_fingerprint": "glm-5.3-flash-reviewer-fast-v1",
  "governance_surface": "reviewer_viability_timeout_45s"
}
JSON

cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- \
  --request "$request" \
  --out "$result" \
  --log "$log"

python3 - "$result" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
status = data.get("final_status")
if status != "ok":
    failure = data.get("failure") or {}
    print(json.dumps({
        "schema": "adl.provider_smoke_result.v1",
        "status": "failed",
        "profile": "z_ai:glm-5.3-flash",
        "live_provider_call": True,
        "final_status": status,
        "failure_kind": failure.get("kind"),
        "message": failure.get("message"),
    }, sort_keys=True))
    raise SystemExit(1)
print(json.dumps({
    "schema": "adl.provider_smoke_result.v1",
    "status": "passed",
    "profile": "z_ai:glm-5.3-flash",
    "live_provider_call": True,
    "duration_ms": data.get("duration_ms"),
}, sort_keys=True))
PY
