#!/usr/bin/env bash
set -euo pipefail

metadata() {
  curl --fail --silent --show-error -H 'Metadata-Flavor: Google' \
    "http://metadata.google.internal/computeMetadata/v1/instance/attributes/$1"
}

fail_and_stop() {
  rc=$?
  trap - EXIT
  echo "ADL_ISSUE663_SEAL=FAIL rc=$rc" >&2
  shutdown -h now || true
  exit "$rc"
}
trap fail_and_stop EXIT

device="/dev/disk/by-id/google-$(metadata adl-data-device)"
generation="$(metadata adl-generation)"
bundle_uri="$(metadata adl-bundle-uri)"
bundle_sha="$(metadata adl-bundle-sha256)"
mount_path="/mnt/adl-staging"
work_path="/var/lib/adl/issue663"

for command in curl gcloud jq mkfs.ext4 mount sha256sum tar; do command -v "$command" >/dev/null; done
for _ in $(seq 1 120); do [ -e "$device" ] && break; sleep 1; done
[ -e "$device" ]
mkfs.ext4 -F "$device"
install -d -m 0755 "$mount_path" "$work_path"
mount "$device" "$mount_path"
gcloud storage cp "$bundle_uri" "$work_path/bundle.tar"
[ "$(sha256sum "$work_path/bundle.tar" | awk '{print $1}')" = "$bundle_sha" ]
tar -xf "$work_path/bundle.tar" -C "$mount_path"
jq -n --arg generation "$generation" --arg bundle_uri "$bundle_uri" --arg bundle_sha "$bundle_sha" \
  '{schema:"adl.issue663.generation.v1",artifact_generation:$generation,bundle_uri:$bundle_uri,bundle_sha256:$bundle_sha}' \
  >"$mount_path/.adl-generation.json"
sync
umount "$mount_path"
sync
echo "ADL_ISSUE663_SEAL=PASS generation=$generation"
trap - EXIT
shutdown -h now
