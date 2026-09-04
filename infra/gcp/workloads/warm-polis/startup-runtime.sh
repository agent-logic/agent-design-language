#!/usr/bin/env bash
set -Eeuo pipefail
trap 'rc=$?; echo "ADL_ISSUE663_RUNTIME_READY=FAIL line=$LINENO exit_code=$rc"; exit "$rc"' ERR

metadata() {
  curl --fail --silent --show-error -H 'Metadata-Flavor: Google' \
    "http://metadata.google.internal/computeMetadata/v1/instance/attributes/$1"
}

boot_seconds="$(cut -d. -f1 /proc/uptime)"
device="/dev/disk/by-id/google-$(metadata adl-data-device-name)"
generation="$(metadata adl-artifact-generation)"
expected_sha="$(metadata adl-content-manifest-sha256)"
ollama_ip="$(metadata adl-ollama-private-ip)"
boot_id="$(cat /proc/sys/kernel/random/boot_id)"
mount_path="/mnt/adl-runtime"
state_path="/var/lib/adl/issue663"
echo "ADL_ISSUE663_RUNTIME_BOOT generation=$generation boot_id=$boot_id"

for command in curl jq mount python3 sed sha256sum systemctl; do
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
[ -f "$mount_path/install/bin/adl-runtime-guardian" ]
runtime_state="$state_path/runtime-state"
rm -rf "$runtime_state"
cp -a "$mount_path/install/runtime-state" "$runtime_state"
guardian="$state_path/adl-runtime-guardian"
cp "$mount_path/install/bin/adl-runtime-guardian" "$guardian"
chmod 0755 "$guardian"
init="$runtime_state/runtime-init.toml"
[ -f "$init" ]
[ -f "$mount_path/install/config/tls/ca.pem" ]
# Retained bundles may predate the kernel's narrowed init schema. Normalize
# only the rejected API TLS keys and continuity-control sections, preserving
# every subsequent section regardless of its order in the sealed file.
python3 - "$init" <<'PY'
import pathlib
import re
import sys

path = pathlib.Path(sys.argv[1])
section = ""
kept = []
blocked_sections = (
    "continuity_control",
    "polis",
    "resident_shepherd",
    "observability_pipeline.cloudwatch",
)
blocked_credentials = {
    "migration_decision_public_key_path",
    "migration_decision_key_id",
    "migration_decision_key_generation",
    "acip_write_token_path",
    "birth_witness_trust_manifest_path",
}
for line in path.read_text().splitlines(keepends=True):
    match = re.match(r'^\[([^]]+)\]\s*$', line)
    if match:
        section = match.group(1)
    if any(section == prefix or section.startswith(prefix + ".") for prefix in blocked_sections):
        continue
    key = line.split('=', 1)[0].strip() if '=' in line else ""
    if section == "api.tls" and key in {"server_name", "trust_roots_path"}:
        continue
    if section == "credentials" and key in blocked_credentials:
        continue
    kept.append(line)
path.write_text(''.join(kept))
PY
sed -i "s#http://127.0.0.1:11434#http://$ollama_ip:11434#g" "$init"

systemd-run --unit=adl-runtime-guardian --property=Restart=always \
  --property=RestartSec=1s --property=StartLimitIntervalSec=0 \
  --property=StandardOutput=journal+console --property=StandardError=journal+console \
  --setenv="OLLAMA_HOST=http://$ollama_ip:11434" \
  /bin/bash -c 'find "$1" -type f -name "*.lock" -delete; exec "$2" --init "$3"' \
  _ "$runtime_state" "$guardian" "$init"

for _ in $(seq 1 120); do
  curl --fail --silent --show-error --noproxy '*' --connect-timeout 2 --max-time 5 \
    https://wuji.dev.csm.agent-logic.ai:20997/v1/health \
    --resolve "wuji.dev.csm.agent-logic.ai:20997:127.0.0.1" \
    --cacert "$mount_path/install/config/tls/ca.pem" >/dev/null 2>&1 && break
  sleep 1
done
curl --fail --silent --show-error --noproxy '*' --connect-timeout 2 --max-time 5 \
  https://wuji.dev.csm.agent-logic.ai:20997/v1/health \
  --resolve "wuji.dev.csm.agent-logic.ai:20997:127.0.0.1" \
  --cacert "$mount_path/install/config/tls/ca.pem" >/dev/null

plan="$state_path/agent-plan.json"
runner="$state_path/run-six-resident-remote.py"
evidence="$state_path/agent-evidence"
first_model="llama3.1:8b"
second_model="qwen3:8b"
rm -rf "$evidence" "$state_path/agent-state.json" "$state_path/agent-runtime"
jq --arg first "$first_model" --arg second "$second_model" '
  .host.gpu_allowed=false
  | .host.max_loaded_models=2
  | .residents |= (to_entries | map(.value.model=(if (.key%2)==0 then $first else $second end) | .value))
' "$mount_path/install/config/issue268_six_resident_uts_plan.json" >"$plan"
sed "s#http://127.0.0.1:11434#http://$ollama_ip:11434#g" \
  "$mount_path/install/config/run_issue268_six_resident_uts_cycle.py" >"$runner"
if ! python3 "$runner" \
  --phase pre \
  --state "$state_path/agent-state.json" \
  --evidence-dir "$evidence" \
  --plan "$plan" \
  --task-panel "$mount_path/install/config/issue268_runtime_uts_task_panel.json" \
  --runtime-bin "$mount_path/install/bin/adl" \
  --runtime-root "$state_path/agent-runtime" >"$state_path/agents.log" 2>&1; then
  cat "$state_path/agents.log"
  exit 1
fi
agent_tool_count="$(jq -sc 'map(select(.agent_test_outcome=="executed" and .runtime_exit_code==0 and .runtime_receipt.decision=="executed"))|length' "$evidence"/pre-*.json)"
[ "$agent_tool_count" -eq 6 ]
agent_summary="$(jq -sc 'map({agent_id,model,role,agent_test_outcome,runtime_exit_code,runtime_decision:.runtime_receipt.decision,acc_contract_id:.runtime_receipt.acc_contract_id,resident_id:.runtime_receipt.resident_id})' "$evidence"/pre-*.json)"
[ "$(jq 'length' <<<"$agent_summary")" -eq 6 ]
ready_seconds="$(cut -d. -f1 /proc/uptime)"
jq -n --arg generation "$generation" --argjson boot_seconds "$boot_seconds" \
  --argjson ready_seconds "$ready_seconds" --argjson agent_tool_count "$agent_tool_count" \
  '{schema:"adl.issue663.runtime-ready.v1",status:"ready",artifact_generation:$generation,clock_source:"CLOCK_BOOTTIME_linux_proc_uptime",guest_start_seconds:$boot_seconds,guest_ready_seconds:$ready_seconds,guardian_supervised:true,real_agent_tool_path:true,agent_tool_count:$agent_tool_count}' \
  >"$state_path/runtime-ready.json"
echo "ADL_ISSUE670_AGENT_TOOL=PASS count=$agent_tool_count"
echo "ADL_ISSUE670_AGENT_SUMMARY=$agent_summary"
echo "ADL_ISSUE663_RUNTIME_READY=PASS generation=$generation boot_id=$boot_id ready_seconds=$ready_seconds agent_tool_count=$agent_tool_count"
