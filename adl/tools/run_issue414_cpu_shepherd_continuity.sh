#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OLLAMA_URL="${OLLAMA_HOST:-http://127.0.0.1:11434}"
OUT="${ADL_ISSUE414_OUT:-$ROOT/.csdlc/evidence/414/cpu-shepherd-reference.json}"
HOST_CLASS="${ADL_ISSUE414_HOST_CLASS:-reference}"
MODELS=("llama3.1:8b" "qwen3:8b" "phi4-mini")

usage() {
  cat <<'USAGE'
Usage: run_issue414_cpu_shepherd_continuity.sh [--out PATH] [--host-class reference|r7i.2xlarge]

Runs three distinct CPU-local Ollama residents sequentially (no compilation),
records model identity and useful-work digests, and emits a redacted receipt.
`reference` proves model behavior only. `r7i.2xlarge` additionally requires the
exact 8-vCPU/64-GiB runtime envelope and zero swap use.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out) OUT="${2:?--out requires a path}"; shift 2 ;;
    --host-class) HOST_CLASS="${2:?--host-class requires a value}"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

[[ "$HOST_CLASS" == reference || "$HOST_CLASS" == r7i.2xlarge ]] || {
  echo "host class must be reference or r7i.2xlarge" >&2
  exit 64
}
command -v curl >/dev/null
command -v jq >/dev/null
curl -fsS "$OLLAMA_URL/api/tags" >/dev/null

if [[ "$HOST_CLASS" == r7i.2xlarge ]]; then
  [[ "$(uname -s)" == Linux ]] || { echo "r7i.2xlarge proof requires Linux" >&2; exit 65; }
  cpu_count="$(getconf _NPROCESSORS_ONLN)"
  memory_mib="$(awk '/^MemTotal:/ {print int($2 / 1024)}' /proc/meminfo)"
  swap_used_mib="$(awk '/^SwapTotal:/ {t=$2} /^SwapFree:/ {f=$2} END {print int((t-f)/1024)}' /proc/meminfo)"
  [[ "$cpu_count" == 8 ]] || { echo "exact r7i.2xlarge proof requires 8 online vCPU" >&2; exit 65; }
  (( memory_mib >= 64000 && memory_mib <= 65536 )) || {
    echo "exact r7i.2xlarge proof requires approximately 65536 MiB" >&2
    exit 65
  }
  [[ "$swap_used_mib" == 0 ]] || { echo "swap use is nonzero" >&2; exit 65; }
else
  cpu_count="$(python3 -c 'import os; print(os.cpu_count() or 0)')"
  if [[ "$(uname -s)" == Linux ]]; then
    memory_mib="$(awk '/^MemTotal:/ {print int($2 / 1024)}' /proc/meminfo)"
    swap_used_mib="$(awk '/^SwapTotal:/ {t=$2} /^SwapFree:/ {f=$2} END {print int((t-f)/1024)}' /proc/meminfo)"
  else
    memory_mib="$(python3 -c 'import os; print((os.sysconf("SC_PHYS_PAGES") * os.sysconf("SC_PAGE_SIZE")) // 1048576)')"
    swap_used_mib=-1
  fi
fi

mkdir -p "$(dirname "$OUT")"
tmp="$(mktemp -d "${OUT}.tmp.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT

tags="$tmp/tags.json"
curl -fsS "$OLLAMA_URL/api/tags" >"$tags"

for model in "${MODELS[@]}"; do
  jq -n --arg model "$model" '{model:$model,keep_alive:0}' >"$tmp/unload.json"
  curl -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$tmp/unload.json" >/dev/null
done

for model in "${MODELS[@]}"; do
  jq -e --arg model "$model" '.models[] | select(.name == $model or .model == $model or .name == ($model + ":latest"))' "$tags" >"$tmp/$model.metadata.json" || {
    echo "required local model is not installed: $model" >&2
    exit 66
  }
  quantization="$(jq -r '.details.quantization_level' "$tmp/$model.metadata.json")"
  [[ "$quantization" == Q4* ]] || { echo "$model is not Q4 quantized" >&2; exit 66; }

  prompt="You are resident agent $model. Return only JSON with keys action, ordered_steps, risk, next_action. Diagnose a Runtime admission gate that must remain closed until an exact three-agent continuity population restores. Give exactly three ordered_steps and set next_action to verify_population_digest."
  jq -n --arg model "$model" --arg prompt "$prompt" '{model:$model,prompt:$prompt,stream:false,format:"json",keep_alive:"5m",options:{num_ctx:8192,temperature:0,seed:414}}' >"$tmp/request.json"
  curl -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$tmp/request.json" >"$tmp/$model.cold.json"
  jq -e '.done == true and (.response | fromjson | .next_action == "verify_population_digest") and (.response | fromjson | .ordered_steps | length == 3)' "$tmp/$model.cold.json" >/dev/null || {
    echo "$model cold response was not useful schema-valid work" >&2
    exit 67
  }
  jq -r '.response | fromjson | @json' "$tmp/$model.cold.json" | shasum -a 256 | awk '{print $1}' >"$tmp/$model.cold.sha256"

  completed_digest="$(<"$tmp/$model.cold.sha256")"
  continuation_prompt="You are resident agent $model continuing after exact restore. The completed task digest is $completed_digest. Return only JSON with keys action, ordered_steps, risk, next_action. Verify that the exact three-agent population digest and predecessor are bound before reopening admission. Give exactly three ordered_steps and set next_action to reopen_admission_after_exact_restore."
  jq -n --arg model "$model" --arg prompt "$continuation_prompt" '{model:$model,prompt:$prompt,stream:false,format:"json",keep_alive:"5m",options:{num_ctx:8192,temperature:0,seed:415}}' >"$tmp/request.json"
  curl -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$tmp/request.json" >"$tmp/$model.warm.json"
  jq -e '.done == true and (.response | fromjson | .next_action == "reopen_admission_after_exact_restore") and (.response | fromjson | .ordered_steps | length == 3)' "$tmp/$model.warm.json" >/dev/null || {
    echo "$model warm response was not useful deterministic continuation" >&2
    exit 67
  }
  jq -r '.response | fromjson | @json' "$tmp/$model.warm.json" | shasum -a 256 | awk '{print $1}' >"$tmp/$model.warm.sha256"
  [[ "$(<"$tmp/$model.warm.sha256")" != "$completed_digest" ]] || {
    echo "$model continuation did not advance useful work" >&2
    exit 67
  }
  curl -fsS "$OLLAMA_URL/api/ps" >"$tmp/$model.ps.json"
  jq -e '(.models | length) > 0 and (.models | length) <= 2' "$tmp/$model.ps.json" >/dev/null || {
    echo "$model violated max-loaded-models=2" >&2
    exit 68
  }
  jq -n --arg model "$model" '{model:$model,keep_alive:0}' >"$tmp/unload.json"
  curl -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$tmp/unload.json" >/dev/null
done

python3 - "$tags" "$tmp" "$OUT" "$HOST_CLASS" "$cpu_count" "$memory_mib" "$swap_used_mib" <<'PY'
import json, pathlib, sys

tags_path, tmp_path, out_path, host_class, cpu_count, memory_mib, swap_used_mib = sys.argv[1:]
tmp = pathlib.Path(tmp_path)
tags = json.loads(pathlib.Path(tags_path).read_text())
models = ["llama3.1:8b", "qwen3:8b", "phi4-mini"]
by_name = {m["name"]: m for m in tags["models"]}
residents = []
for index, model in enumerate(models, start=1):
    metadata = json.loads((tmp / f"{model}.metadata.json").read_text())
    cold = json.loads((tmp / f"{model}.cold.json").read_text())
    warm = json.loads((tmp / f"{model}.warm.json").read_text())
    loaded = json.loads((tmp / f"{model}.ps.json").read_text())["models"]
    residents.append({
        "agent_id": f"resident-{index}",
        "model": model,
        "artifact_sha256": metadata["digest"],
        "quantization": metadata["details"]["quantization_level"],
        "cold_latency_millis": max(1, cold["total_duration"] // 1_000_000),
        "warm_latency_millis": max(1, warm["total_duration"] // 1_000_000),
        "completed_task_sha256": (tmp / f"{model}.cold.sha256").read_text().strip(),
        "next_task_sha256": (tmp / f"{model}.warm.sha256").read_text().strip(),
        "loaded_model_count": len(loaded),
        "loaded_model_bytes": sum(item["size"] for item in loaded),
    })
peak_loaded_model_bytes = max(item["loaded_model_bytes"] for item in residents)
model_capacity_headroom_mib = int(memory_mib) - ((peak_loaded_model_bytes + 1048575) // 1048576)
receipt = {
    "schema": "adl.runtime.resident_shepherd_habitability.v1",
    "qualification": host_class == "r7i.2xlarge",
    "host_class": host_class,
    "instance_type": "r7i.2xlarge" if host_class == "r7i.2xlarge" else "reference-host",
    "vcpus": int(cpu_count),
    "memory_mib": int(memory_mib),
    "swap_used_mib": int(swap_used_mib),
    "swap_measurement": "measured" if int(swap_used_mib) >= 0 else "unavailable_on_reference_host",
    "peak_loaded_model_bytes": peak_loaded_model_bytes,
    "required_capacity_headroom_mib": 16384,
    "model_capacity_headroom_mib": model_capacity_headroom_mib,
    "capacity_headroom_pass": model_capacity_headroom_mib >= 16384,
    "context_tokens": 8192,
    "parallelism": 2,
    "max_loaded_models": 2,
    "compilation_concurrent": False,
    "resident_count": len(residents),
    "residents": residents,
    "prompts_retained": False,
    "model_weights_serialized": False,
    "external_model_authoritative": False,
}
path = pathlib.Path(out_path)
path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
print(json.dumps(receipt, sort_keys=True))
PY
