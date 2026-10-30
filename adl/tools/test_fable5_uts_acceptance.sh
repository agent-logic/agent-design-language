#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP/uts/tools/benchmark" "$TMP/adl-bin"

cat >"$TMP/uts/tools/benchmark/deterministic_self_check.py" <<'PY'
#!/usr/bin/env python3
print('{"passed": true, "fixture_count": 22, "task_count": 11}')
PY
chmod +x "$TMP/uts/tools/benchmark/deterministic_self_check.py"

cat >"$TMP/uts/tools/uts_benchmark_runner.py" <<'PY'
#!/usr/bin/env python3
import json
import os
import subprocess
import sys
from pathlib import Path

mode = sys.argv[1]
selector = Path(sys.argv[2]).read_text(encoding="utf-8").strip()
out = Path(sys.argv[3])
out.parent.mkdir(parents=True, exist_ok=True)

request = out.with_suffix(".request.json")
result = out.with_suffix(".adapter-result.json")
log = out.with_suffix(".jsonl")
request.write_text(json.dumps({
    "route": {
        "provider_kind": "hosted",
        "provider": "anthropic",
        "runtime_surface": "hosted_api",
        "provider_model_id": selector.split(":")[-1],
        "credential_ref": "env:ANTHROPIC_API_KEY"
    },
    "model_identity": {
        "provider_kind": "hosted",
        "provider": "anthropic",
        "model_ref": selector.split(":")[-1],
        "provider_model_id": selector.split(":")[-1],
        "runtime_surface": "hosted_api",
        "identity_strength": "provider_asserted",
        "observed_at": "2026-07-07T00:00:00Z",
        "source_registry": "test",
        "inference_parameter_fingerprint": "provider_default",
        "tool_surface": "uts.v1.1",
        "evaluator_ref": "test",
        "benchmark_ref": "test"
    },
    "prompt_contract_ref": "test",
    "lane_ref": "test",
    "attempt_policy": {"max_attempts": 1, "timeout_ms": 1000},
    "input_text": "return json"
}), encoding="utf-8")
cmd = os.environ["UTS_ADL_PROVIDER_ADAPTER_COMMAND"].split() + ["--request", str(request), "--out", str(result), "--log", str(log)]
completed = subprocess.run(cmd, check=False)
if completed.returncode != 0:
    raise SystemExit(completed.returncode)
payload = json.loads(request.read_text(encoding="utf-8"))
out.write_text(json.dumps({
    "mode": mode,
    "selector": selector,
    "max_output_tokens": payload.get("max_output_tokens"),
    "inference_parameter_fingerprint": payload.get("inference_parameter_fingerprint"),
}), encoding="utf-8")
PY
chmod +x "$TMP/uts/tools/uts_benchmark_runner.py"

cat >"$TMP/adl-bin/adl-provider-adapter" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
request=""
out=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --request) request="$2"; shift 2 ;;
    --out) out="$2"; shift 2 ;;
    --log) shift 2 ;;
    *) shift ;;
  esac
done
python3 - "$request" "$out" <<'PY'
import json
import sys
from pathlib import Path
request = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert request["max_output_tokens"] == 1024
Path(sys.argv[2]).write_text(json.dumps({"final_status": "ok", "output_text": "{}"}), encoding="utf-8")
PY
SH
chmod +x "$TMP/adl-bin/adl-provider-adapter"

printf 'test-key\n' >"$TMP/key"

ADL_PROVIDER_ADAPTER_BIN="$TMP/adl-bin/adl-provider-adapter" \
bash "$ROOT/adl/tools/run_fable5_uts_acceptance.sh" \
  --uts-root "$TMP/uts" \
  --artifact-dir "$TMP/artifacts" \
  --key-file "$TMP/key" \
  --skip-probe >"$TMP/run.out"

grep -F "PASS fable5_uts_acceptance" "$TMP/run.out" >/dev/null
grep -F "hosted:adl-anthropic:claude-fable-5" "$TMP/artifacts/fable5_selector.txt" >/dev/null
python3 - <<'PY' "$TMP/artifacts/fable5_uts_results.json"
import json
import sys
from pathlib import Path
payload = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert payload["max_output_tokens"] == 1024
assert "max_output_tokens=1024" in payload["inference_parameter_fingerprint"]
PY

REQ="$TMP/request.json"
OUT="$TMP/out.json"
LOG="$TMP/log.jsonl"
printf '{"route":{},"attempt_policy":{"max_attempts":1,"timeout_ms":1}}\n' >"$REQ"
python3 "$ROOT/adl/tools/adl_provider_adapter_with_budget.py" \
  --adapter "$TMP/adl-bin/adl-provider-adapter" \
  --max-output-tokens 1024 \
  -- --request "$REQ" --out "$OUT" --log "$LOG"
python3 - <<'PY' "$REQ"
import json
import sys
from pathlib import Path
payload = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert payload["max_output_tokens"] == 1024
assert payload["inference_parameter_fingerprint"] == "max_output_tokens=1024"
PY
