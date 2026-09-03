#!/usr/bin/env bash
set -euo pipefail

metadata() {
  curl --fail --silent --show-error -H 'Metadata-Flavor: Google' \
    "http://metadata.google.internal/computeMetadata/v1/instance/attributes/$1"
}

generation="$(metadata adl-generation)"
runtime_sha="$(metadata adl-runtime-manifest-sha256)"
ollama_sha="$(metadata adl-ollama-manifest-sha256)"
install -d -m 0755 /mnt/runtime-verify /mnt/ollama-verify
mount -o ro /dev/disk/by-id/google-adl-runtime-verify /mnt/runtime-verify
mount -o ro /dev/disk/by-id/google-adl-ollama-verify /mnt/ollama-verify
[ "$(sha256sum /mnt/runtime-verify/.adl-generation.json | awk '{print $1}')" = "$runtime_sha" ]
[ "$(sha256sum /mnt/ollama-verify/.adl-generation.json | awk '{print $1}')" = "$ollama_sha" ]
[ "$(jq -r '.artifact_generation' /mnt/runtime-verify/.adl-generation.json)" = "$generation" ]
[ "$(jq -r '.artifact_generation' /mnt/ollama-verify/.adl-generation.json)" = "$generation" ]
echo "ADL_ISSUE663_SNAPSHOT_VERIFY=PASS generation=$generation"
sync
umount /mnt/runtime-verify /mnt/ollama-verify
shutdown -h now
