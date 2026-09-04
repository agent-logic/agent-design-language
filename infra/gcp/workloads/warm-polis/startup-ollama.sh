#!/usr/bin/env bash
set -euo pipefail

metadata() {
  curl --fail --silent --show-error -H 'Metadata-Flavor: Google' \
    "http://metadata.google.internal/computeMetadata/v1/instance/attributes/$1"
}

boot_seconds="$(cut -d. -f1 /proc/uptime)"
device="/dev/disk/by-id/google-$(metadata adl-data-device-name)"
generation="$(metadata adl-artifact-generation)"
expected_sha="$(metadata adl-content-manifest-sha256)"
models_json="$(metadata adl-resident-models)"
mount_path="/mnt/adl-ollama"
state_path="/var/lib/adl/issue663"

for command in curl jq mount nvidia-smi sha256sum systemctl; do
  command -v "$command" >/dev/null
done
for _ in $(seq 1 120); do
  [ -e "$device" ] && break
  sleep 1
done
[ -e "$device" ]
install -d -m 0755 "$mount_path" "$state_path"
mount -o ro "$device" "$mount_path"
manifest="$mount_path/.adl-generation.json"
[ "$(sha256sum "$manifest" | awk '{print $1}')" = "$expected_sha" ]
[ "$(jq -r '.artifact_generation' "$manifest")" = "$generation" ]
[ -x "$mount_path/install/bin/ollama" ]
nvidia-smi >/dev/null

systemd-run --unit=adl-ollama --property=Restart=always \
  --property=RestartSec=1s --property=StartLimitIntervalSec=0 \
  --setenv=HOME="$state_path" --setenv=OLLAMA_HOST=0.0.0.0:11434 \
  --setenv=OLLAMA_KEEP_ALIVE=-1 --setenv=OLLAMA_MODELS="$mount_path/install/models" \
  --setenv="OLLAMA_MAX_LOADED_MODELS=$(jq 'length' <<<"$models_json")" \
  "$mount_path/install/bin/ollama" serve

for _ in $(seq 1 120); do
  curl --fail --silent --show-error http://127.0.0.1:11434/api/tags >/dev/null 2>&1 && break
  sleep 1
done
warm_pids=()
while IFS= read -r model; do
  jq -n --arg model "$model" '{model:$model,prompt:"ready",stream:false,keep_alive:-1}' \
    | curl --fail --silent --show-error http://127.0.0.1:11434/api/generate \
        -H 'Content-Type: application/json' --data-binary @- >/dev/null &
  warm_pids+=("$!")
done < <(jq -r '.[]' <<<"$models_json")
for pid in "${warm_pids[@]}"; do wait "$pid"; done

loaded="$(curl --fail --silent --show-error http://127.0.0.1:11434/api/ps)"
jq -e --argjson expected "$models_json" \
  '([.models[].name] as $loaded | all($expected[]; . as $model | any($loaded[]; startswith($model))))' \
  <<<"$loaded" >/dev/null
ready_seconds="$(cut -d. -f1 /proc/uptime)"
jq -n --arg generation "$generation" --argjson models "$models_json" \
  --argjson boot_seconds "$boot_seconds" --argjson ready_seconds "$ready_seconds" \
  '{schema:"adl.issue663.ollama-ready.v1",status:"ready",artifact_generation:$generation,clock_source:"CLOCK_BOOTTIME_linux_proc_uptime",guest_start_seconds:$boot_seconds,guest_ready_seconds:$ready_seconds,models:$models,model_count:($models|length),ollama_public:false}' \
  >"$state_path/ollama-ready.json"
echo "ADL_ISSUE663_OLLAMA_READY=PASS generation=$generation ready_seconds=$ready_seconds model_count=$(jq 'length' <<<"$models_json")"
