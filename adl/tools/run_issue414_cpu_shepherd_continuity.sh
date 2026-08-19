#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANAGED_OLLAMA="${ADL_ISSUE414_MANAGED_OLLAMA:-0}"
if [[ "$MANAGED_OLLAMA" == 1 ]]; then
  OLLAMA_URL="${ADL_ISSUE414_OLLAMA_URL:-http://127.0.0.1:11435}"
else
  OLLAMA_URL="${ADL_ISSUE414_OLLAMA_URL:-http://127.0.0.1:11434}"
fi
OUT="${ADL_ISSUE414_OUT:-$ROOT/.csdlc/evidence/414/cpu-shepherd-reference.json}"
HOST_CLASS="${ADL_ISSUE414_HOST_CLASS:-reference}"
MODELS=("llama3.1:8b" "qwen3:8b" "phi4-mini")
CONTINUITY_BIN="${ADL_ISSUE414_CONTINUITY_BIN:-$ROOT/adl/target/debug/adl_resident_shepherd_continuity}"
PREFLIGHT_ONLY=0
BOOTSTRAP_MANIFEST="${ADL_ISSUE414_BOOTSTRAP_MANIFEST:-}"

usage() {
  cat <<'USAGE'
Usage: run_issue414_cpu_shepherd_continuity.sh [--out PATH] [--host-class reference|r7i.2xlarge] [--preflight-only]

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
    --preflight-only) PREFLIGHT_ONLY=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

[[ "$HOST_CLASS" == reference || "$HOST_CLASS" == r7i.2xlarge ]] || {
  echo "host class must be reference or r7i.2xlarge" >&2
  exit 64
}
[[ "$OUT" == /* ]] || OUT="$ROOT/$OUT"
command -v curl >/dev/null
command -v jq >/dev/null
if [[ -n "$BOOTSTRAP_MANIFEST" ]]; then
  reviewed_head="$(git -C "$ROOT" rev-parse HEAD)"
  python3 "$ROOT/adl/tools/issue414_s3_linux_bootstrap.py" validate "$BOOTSTRAP_MANIFEST" \
    --expected-reviewed-git-sha "$reviewed_head" >/dev/null
  expected_runner_sha="$(jq -r '.runner_sha256' "$BOOTSTRAP_MANIFEST")"
  expected_binary_sha="$(jq -r '.continuity_binary_sha256' "$BOOTSTRAP_MANIFEST")"
  [[ "$(shasum -a 256 "$0" | awk '{print $1}')" == "$expected_runner_sha" ]] || {
    echo "runner does not match reviewed bootstrap provenance" >&2; exit 65;
  }
  [[ "$(shasum -a 256 "$CONTINUITY_BIN" | awk '{print $1}')" == "$expected_binary_sha" ]] || {
    echo "continuity binary does not match reviewed bootstrap provenance" >&2; exit 65;
  }
elif [[ "$HOST_CLASS" == r7i.2xlarge ]]; then
  echo "r7i.2xlarge qualification requires a reviewed Linux/x86 bootstrap manifest" >&2
  exit 65
fi
[[ "$MANAGED_OLLAMA" == 0 || "$MANAGED_OLLAMA" == 1 ]] || {
  echo "ADL_ISSUE414_MANAGED_OLLAMA must be 0 or 1" >&2
  exit 64
}
[[ -x "$CONTINUITY_BIN" ]] || {
  echo "resident continuity proof binary is missing; build adl-runtime-resident-shepherd-continuity before model execution" >&2
  exit 66
}

OLLAMA_PID=""
cleanup() {
  if [[ -n "$OLLAMA_PID" ]]; then
    kill "$OLLAMA_PID" >/dev/null 2>&1 || true
    wait "$OLLAMA_PID" >/dev/null 2>&1 || true
  fi
  [[ -z "$tmp" ]] || rm -rf "$tmp"
}
tmp=""
trap cleanup EXIT
if [[ "$MANAGED_OLLAMA" == 1 ]]; then
  command -v ollama >/dev/null
  # The managed lane is the qualification contract: its PID, CPU selection,
  # resource use, and environment are measurable. Exact r7 execution is #268.
  OLLAMA_HOST="${OLLAMA_URL#http://}" \
  OLLAMA_NUM_PARALLEL=2 \
  OLLAMA_MAX_LOADED_MODELS=2 \
  OLLAMA_CONTEXT_LENGTH=8192 \
  OLLAMA_LLM_LIBRARY=cpu \
  ollama serve >"${OUT}.ollama.log" 2>&1 &
  OLLAMA_PID="$!"
  for _ in $(seq 1 30); do
    curl -fsS "$OLLAMA_URL/api/tags" >/dev/null 2>&1 && break
    sleep 1
  done
fi
curl -fsS "$OLLAMA_URL/api/tags" >/dev/null

if [[ "$HOST_CLASS" == r7i.2xlarge ]]; then
  [[ "$MANAGED_OLLAMA" == 1 ]] || {
    echo "r7i.2xlarge qualification requires proof-owned managed Ollama" >&2
    exit 65
  }
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
build_cache_root="$tmp/ephemeral-build-cache"
if [[ "$HOST_CLASS" == r7i.2xlarge ]]; then
  runtime_root="${ADL_ISSUE414_RETAINED_RUNTIME_ROOT:?r7 qualification requires retained Runtime root}"
  command -v findmnt >/dev/null; command -v lsblk >/dev/null
  runtime_source="$(findmnt -no SOURCE --target "$runtime_root")"
  runtime_serial="$(lsblk -ndo SERIAL "$runtime_source" | head -1 | tr -d '[:space:]')"
  [[ -n "$runtime_serial" ]] || { echo "retained Runtime EBS serial is unavailable" >&2; exit 65; }
  if [[ "$runtime_serial" == vol* && "$runtime_serial" != vol-* ]]; then runtime_serial="vol-${runtime_serial#vol}"; fi
  ADL_ISSUE414_RUNTIME_VOLUME_IDENTITY_SHA256="$(printf '%s' "$runtime_serial" | shasum -a 256 | awk '{print $1}')"
else
  runtime_root="$tmp/runtime-continuity"
  ADL_ISSUE414_RUNTIME_VOLUME_IDENTITY_SHA256="$(printf '%s' reference-volume | shasum -a 256 | awk '{print $1}')"
fi
export ADL_ISSUE414_RUNTIME_VOLUME_IDENTITY_SHA256
mkdir -p "$runtime_root" "$build_cache_root"

tags="$tmp/tags.json"
curl -fsS "$OLLAMA_URL/api/tags" >"$tags"

export ADL_ISSUE414_SIGNING_KEY_HEX="${ADL_ISSUE414_SIGNING_KEY_HEX:-9999999999999999999999999999999999999999999999999999999999999999}"
python3 - "$tags" "$runtime_root" "$build_cache_root" >"$tmp/preflight.json" <<'PY'
import hashlib, json, pathlib, sys
tags = json.loads(pathlib.Path(sys.argv[1]).read_text())
models = ["llama3.1:8b", "qwen3:8b", "phi4-mini"]
by_name = {m["name"]: m for m in tags["models"]}
residents = []
for index, model in enumerate(models, start=1):
    metadata = by_name.get(model) or by_name.get(model + ":latest")
    if metadata is None:
        raise SystemExit(f"missing required model {model}")
    quantization = metadata["details"]["quantization_level"]
    thinking = "think=false" if model == "qwen3:8b" else "think=unsupported"
    configuration = hashlib.sha256(f'{model}:{metadata["digest"]}:{quantization}:8192:2:2:{thinking}'.encode()).hexdigest()
    residents.append({
        "agent_id": f"resident-{index}", "model": model,
        "artifact_sha256": metadata["digest"], "quantization": quantization,
        "configuration_sha256": configuration,
        "completed_task_sha256": hashlib.sha256(f"preflight-cold-{index}".encode()).hexdigest(),
        "continuation_request_sha256": hashlib.sha256(f"preflight-next-{index}".encode()).hexdigest(),
    })
print(json.dumps({
    "residents": residents, "existing_agent_specs": [],
    "retained_runtime_root": sys.argv[2], "build_cache_root": sys.argv[3],
    "runtime_volume_identity_sha256": __import__("os").environ["ADL_ISSUE414_RUNTIME_VOLUME_IDENTITY_SHA256"],
    "source_host": "preflight", "target_host": "local"
}, sort_keys=True))
PY
"$CONTINUITY_BIN" preflight \
  --input "$tmp/preflight.json" --runtime-root "$runtime_root" \
  --output "$tmp/preflight-receipt.json"
jq -e '.status == "passed" and .resident_count == 3 and .signing_key_exact_bytes == 32' \
  "$tmp/preflight-receipt.json" >/dev/null
if [[ "$PREFLIGHT_ONLY" == 1 ]]; then
  jq -c '.' "$tmp/preflight-receipt.json"
  exit 0
fi

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
  if [[ "$model" == qwen3:8b ]]; then
    jq -n --arg model "$model" --arg prompt "$prompt" '{model:$model,prompt:$prompt,stream:false,format:"json",think:false,keep_alive:"5m",options:{num_ctx:8192,num_predict:128,num_gpu:0,temperature:0,seed:414}}' >"$tmp/request.json"
  else
    jq -n --arg model "$model" --arg prompt "$prompt" '{model:$model,prompt:$prompt,stream:false,format:"json",keep_alive:"5m",options:{num_ctx:8192,num_predict:128,num_gpu:0,temperature:0,seed:414}}' >"$tmp/request.json"
  fi
  curl --max-time 300 -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$tmp/request.json" >"$tmp/$model.cold.json"
  jq -e '.done == true and (.response | fromjson | .next_action == "verify_population_digest") and (.response | fromjson | .ordered_steps | length == 3)' "$tmp/$model.cold.json" >/dev/null || {
    echo "$model cold response was not useful schema-valid work" >&2
    exit 67
  }
  jq -r '.response | fromjson | @json' "$tmp/$model.cold.json" | shasum -a 256 | awk '{print $1}' >"$tmp/$model.cold.sha256"

  completed_digest="$(<"$tmp/$model.cold.sha256")"
  continuation_prompt="You are resident agent $model continuing after exact restore. The completed task digest is $completed_digest. Return only JSON with keys action, ordered_steps, risk, next_action. Verify that the exact three-agent population digest and predecessor are bound before reopening admission. Give exactly three ordered_steps and set next_action to reopen_admission_after_exact_restore."
  printf '%s' "$continuation_prompt" | shasum -a 256 | awk '{print $1}' >"$tmp/$model.continuation-request.sha256"
  printf '%s' "$continuation_prompt" >"$tmp/$model.continuation-prompt.txt"
  jq -n --arg model "$model" '{model:$model,keep_alive:0}' >"$tmp/unload.json"
  curl -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$tmp/unload.json" >/dev/null
done

python3 - "$tags" "$tmp" "$runtime_root" "$build_cache_root" >"$tmp/pre-recovery.json" <<'PY'
import json, os, pathlib, sys
tags = json.loads(pathlib.Path(sys.argv[1]).read_text())
tmp = pathlib.Path(sys.argv[2])
runtime_root = sys.argv[3]
build_cache_root = sys.argv[4]
models = ["llama3.1:8b", "qwen3:8b", "phi4-mini"]
residents = []
for index, model in enumerate(models, start=1):
    metadata = json.loads((tmp / f"{model}.metadata.json").read_text())
    residents.append({
        "agent_id": f"resident-{index}",
        "model": model,
        "artifact_sha256": metadata["digest"],
        "quantization": metadata["details"]["quantization_level"],
        "configuration_sha256": __import__("hashlib").sha256(
            f'{model}:{metadata["digest"]}:{metadata["details"]["quantization_level"]}:8192:2:2:{"think=false" if model == "qwen3:8b" else "think=unsupported"}'.encode()
        ).hexdigest(),
        "completed_task_sha256": (tmp / f"{model}.cold.sha256").read_text().strip(),
        "continuation_request_sha256": (tmp / f"{model}.continuation-request.sha256").read_text().strip(),
    })
print(json.dumps({
    "residents": residents,
    "existing_agent_specs": [],
    "retained_runtime_root": runtime_root,
    "build_cache_root": build_cache_root,
    "runtime_volume_identity_sha256": __import__("os").environ["ADL_ISSUE414_RUNTIME_VOLUME_IDENTITY_SHA256"],
    "source_host": "local-managed-ollama",
    "target_host": "local",
}, sort_keys=True))
PY

"$CONTINUITY_BIN" dehydrate \
  --input "$tmp/pre-recovery.json" \
  --runtime-root "$runtime_root" \
  --output "$runtime_root/dehydration-command-receipt.json"

# Restore validates the signed Runtime-v2/capsule integration and exact model
# bindings, applies any existing-agent capsules, then and only then opens admission.
ADL_ISSUE414_RESTORE_HOST_CLASS="$(if [[ "$HOST_CLASS" == r7i.2xlarge ]]; then printf aws; else printf reference; fi)" \
ADL_ISSUE414_CONTINUITY_BIN="$CONTINUITY_BIN" \
  bash "$ROOT/adl/tools/issue414_restore_and_admit.sh" \
  "$tmp/pre-recovery.json" "$runtime_root" "$runtime_root/admission-receipt.json"
jq -e '.admission_open == true and .continuation_verified == false' \
  "$runtime_root/admission-receipt.json" >/dev/null

for model in "${MODELS[@]}"; do
  completed_digest="$(<"$tmp/$model.cold.sha256")"
  continuation_prompt="$(<"$tmp/$model.continuation-prompt.txt")"
  if [[ "$model" == qwen3:8b ]]; then
    jq -n --arg model "$model" --arg prompt "$continuation_prompt" '{model:$model,prompt:$prompt,stream:false,format:"json",think:false,keep_alive:"5m",options:{num_ctx:8192,num_predict:128,num_gpu:0,temperature:0,seed:415}}' >"$tmp/request.json"
  else
    jq -n --arg model "$model" --arg prompt "$continuation_prompt" '{model:$model,prompt:$prompt,stream:false,format:"json",keep_alive:"5m",options:{num_ctx:8192,num_predict:128,num_gpu:0,temperature:0,seed:415}}' >"$tmp/request.json"
  fi
  curl --max-time 300 -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$tmp/request.json" >"$tmp/$model.warm.json"
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
  if [[ -z "$OLLAMA_PID" ]]; then
    printf '%s\n' -1 >"$tmp/$model.rss-kib"
  elif [[ "$(uname -s)" == Linux ]]; then
    awk '/^VmRSS:/ {print $2}' "/proc/$OLLAMA_PID/status" >"$tmp/$model.rss-kib"
  else
    ps -o rss= -p "$OLLAMA_PID" | awk '{print $1}' >"$tmp/$model.rss-kib"
  fi
  [[ "$(<"$tmp/$model.rss-kib")" == -1 || "$(<"$tmp/$model.rss-kib")" =~ ^[1-9][0-9]*$ ]] || {
    echo "$model Ollama RSS measurement failed" >&2
    exit 68
  }
  jq -n --arg model "$model" '{model:$model,keep_alive:0}' >"$tmp/unload.json"
  curl -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$tmp/unload.json" >/dev/null
done

python3 - "$tags" "$tmp" "$OUT" "$HOST_CLASS" "$cpu_count" "$memory_mib" "$swap_used_mib" "$MANAGED_OLLAMA" <<'PY'
import json, os, pathlib, sys

tags_path, tmp_path, out_path, host_class, cpu_count, memory_mib, swap_used_mib, managed_ollama = sys.argv[1:]
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
        "configuration_sha256": __import__("hashlib").sha256(
            f'{model}:{metadata["digest"]}:{metadata["details"]["quantization_level"]}:8192:2:2:{"think=false" if model == "qwen3:8b" else "think=unsupported"}'.encode()
        ).hexdigest(),
        "cold_latency_millis": max(1, cold["total_duration"] // 1_000_000),
        "warm_latency_millis": max(1, warm["total_duration"] // 1_000_000),
        "completed_task_sha256": (tmp / f"{model}.cold.sha256").read_text().strip(),
        "continuation_request_sha256": (tmp / f"{model}.continuation-request.sha256").read_text().strip(),
        "next_task_sha256": (tmp / f"{model}.warm.sha256").read_text().strip(),
        "loaded_model_count": len(loaded),
        "loaded_model_bytes": sum(item["size"] for item in loaded),
        "ollama_rss_kib": int((tmp / f"{model}.rss-kib").read_text()),
    })
peak_loaded_model_bytes = max(item["loaded_model_bytes"] for item in residents)
measured_rss = [item["ollama_rss_kib"] for item in residents if item["ollama_rss_kib"] > 0]
peak_ollama_rss_kib = max(measured_rss) if measured_rss else -1
accounted_bytes = peak_ollama_rss_kib * 1024 if peak_ollama_rss_kib > 0 else peak_loaded_model_bytes
model_capacity_headroom_mib = int(memory_mib) - ((accounted_bytes + 1048575) // 1048576)
receipt = {
    "schema": "adl.runtime.resident_shepherd_habitability.v2",
    "qualification": host_class == "r7i.2xlarge",
    "host_class": host_class,
    "instance_type": "r7i.2xlarge" if host_class == "r7i.2xlarge" else "reference-host",
    "vcpus": int(cpu_count),
    "memory_mib": int(memory_mib),
    "swap_used_mib": int(swap_used_mib),
    "swap_measurement": "measured" if int(swap_used_mib) >= 0 else "unavailable_on_reference_host",
    "peak_loaded_model_bytes": peak_loaded_model_bytes,
    "peak_ollama_rss_kib": peak_ollama_rss_kib,
    "rss_measurement": "exact_managed_ollama_pid" if managed_ollama == "1" else "unavailable_external_reference_server",
    "required_capacity_headroom_mib": 16384,
    "model_capacity_headroom_mib": model_capacity_headroom_mib,
    "capacity_headroom_pass": model_capacity_headroom_mib >= 16384,
    "context_tokens": 8192,
    "parallelism": int(2),
    "max_loaded_models": int(2),
    "ollama_configuration_source": "proof_owned_process_environment_and_api_ps" if managed_ollama == "1" else "api_ps_and_request_contract_server_environment_unverified",
    "compilation_concurrent": False,
    "resident_count": len(residents),
    "residents": residents,
    "prompts_retained": False,
    "model_weights_serialized": False,
    "external_model_authoritative": False,
    "bootstrap_manifest": os.environ.get("ADL_ISSUE414_BOOTSTRAP_MANIFEST") or None,
    "bootstrap_role": "non_authoritative_s3_cache" if os.environ.get("ADL_ISSUE414_BOOTSTRAP_MANIFEST") else "not_used_reference_host",
}
path = pathlib.Path(out_path)
path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
print(json.dumps(receipt, sort_keys=True))
PY

"$CONTINUITY_BIN" complete \
  --input "$OUT" \
  --runtime-root "$runtime_root" \
  --output "${OUT%.json}-end-to-end.json"
jq -e '.admission_open == true and .continuation_verified == true' \
  "${OUT%.json}-end-to-end.json" >/dev/null
"$CONTINUITY_BIN" validate-receipt \
  --input "$OUT" \
  --runtime-root "$runtime_root" \
  --output "${OUT%.json}-validation.json"
jq -e '.status == "passed" and .capacity_headroom_pass == true' \
  "${OUT%.json}-validation.json" >/dev/null
jq -c '.' "${OUT%.json}-end-to-end.json"
