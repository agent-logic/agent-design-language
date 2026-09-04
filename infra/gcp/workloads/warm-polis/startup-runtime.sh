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
ollama_ip="$(metadata adl-ollama-private-ip)"
mount_path="/mnt/adl-runtime"
state_path="/var/lib/adl/issue663"

for command in curl jq mount sha256sum systemctl; do
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
[ -x "$mount_path/install/bin/adl-runtime-guardian" ]
runtime_state="$state_path/runtime-state"
rm -rf "$runtime_state"
cp -a "$mount_path/install/runtime-state" "$runtime_state"
init="$runtime_state/runtime-init.toml"
[ -f "$init" ]
[ -f "$mount_path/install/config/tls/ca.pem" ]

systemd-run --unit=adl-runtime-guardian --property=Restart=always \
  --property=RestartSec=1s --property=StartLimitIntervalSec=0 \
  --setenv="OLLAMA_HOST=http://$ollama_ip:11434" \
  /bin/bash -c 'find "$1" -type f -name "*.lock" -delete; exec "$2" --init "$3"' \
  _ "$runtime_state" "$mount_path/install/bin/adl-runtime-guardian" "$init"

for _ in $(seq 1 120); do
  curl --fail --silent --show-error https://127.0.0.1:20997/v1/health \
    --cacert "$mount_path/install/config/tls/ca.pem" >/dev/null 2>&1 && break
  sleep 1
done
curl --fail --silent --show-error https://127.0.0.1:20997/v1/health \
  --cacert "$mount_path/install/config/tls/ca.pem" >/dev/null
ready_seconds="$(cut -d. -f1 /proc/uptime)"
jq -n --arg generation "$generation" --argjson boot_seconds "$boot_seconds" \
  --argjson ready_seconds "$ready_seconds" \
  '{schema:"adl.issue663.runtime-ready.v1",status:"ready",artifact_generation:$generation,clock_source:"CLOCK_BOOTTIME_linux_proc_uptime",guest_start_seconds:$boot_seconds,guest_ready_seconds:$ready_seconds,guardian_supervised:true}' \
  >"$state_path/runtime-ready.json"
echo "ADL_ISSUE663_RUNTIME_READY=PASS generation=$generation ready_seconds=$ready_seconds"
