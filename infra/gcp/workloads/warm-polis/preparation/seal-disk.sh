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
paid_deadline_epoch="$(metadata adl-paid-deadline-epoch)"
budget_stop_seconds="$((paid_deadline_epoch - $(date +%s)))"
[ "$budget_stop_seconds" -gt 0 ]
for command in curl file gcloud jq mkfs.ext4 mount sha256sum systemd-run tar; do command -v "$command" >/dev/null; done
systemd-run --unit="adl-issue670-preparation-budget-stop-$(cat /proc/sys/kernel/random/boot_id)" --on-active="${budget_stop_seconds}s" /usr/sbin/poweroff >/dev/null
mount_path="/mnt/adl-staging"
work_path="/var/lib/adl/issue663"

for _ in $(seq 1 120); do [ -e "$device" ] && break; sleep 1; done
[ -e "$device" ]
mkfs.ext4 -F "$device"
install -d -m 0755 "$mount_path" "$work_path"
mount "$device" "$mount_path"
gcloud storage cp "$bundle_uri" "$work_path/bundle.tar"
[ "$(sha256sum "$work_path/bundle.tar" | awk '{print $1}')" = "$bundle_sha" ]
if jq -e '.schema == "adl.shepherd.portable_model_bundle.v2"' "$work_path/bundle.tar" >/dev/null 2>&1; then
  bucket="${bundle_uri#gs://}"
  bucket="${bucket%%/*}"
  install -d -m 0755 "$mount_path/install/bin" "$mount_path/install/models"
  while IFS= read -r artifact; do
    kind="$(jq -r '.kind' <<<"$artifact")"
    object="$(jq -r '.object' <<<"$artifact")"
    relative_path="$(jq -r '.relative_path' <<<"$artifact")"
    expected_artifact_sha="$(jq -r '.sha256' <<<"$artifact")"
    case "$kind" in
      ollama_runtime)
        target="$work_path/ollama-runtime.tar"
        gcloud storage cp "gs://$bucket/$object" "$target"
        [ "$(sha256sum "$target" | awk '{print $1}')" = "$expected_artifact_sha" ]
        tar -xf "$target" -C "$mount_path/install"
        ;;
      ollama_model_blob|ollama_model_manifest)
        case "$relative_path" in
          models/*) ;;
          *) echo "invalid portable model path: $relative_path" >&2; exit 2 ;;
        esac
        target="$mount_path/install/$relative_path"
        install -d -m 0755 "$(dirname "$target")"
        gcloud storage cp "gs://$bucket/$object" "$target"
        [ "$(sha256sum "$target" | awk '{print $1}')" = "$expected_artifact_sha" ]
        ;;
    esac
  done < <(jq -c '.artifacts[]' "$work_path/bundle.tar")
  [ -x "$mount_path/install/bin/ollama" ]
  [ "$(find "$mount_path/install/models/manifests" -type f | wc -l | tr -d ' ')" -ge 2 ]
else
  tar -xf "$work_path/bundle.tar" -C "$mount_path"
  for binary in adl csm adl-runtime-guardian adl-runtime-kernel vector; do
    target="$mount_path/install/bin/$binary"
    [ -x "$target" ]
    file "$target" | grep -Eq 'ELF 64-bit.*x86-64'
  done
  [ -f "$mount_path/install/runtime-state/runtime-init.toml" ]
  [ -f "$mount_path/install/config/tls/ca.pem" ]
fi
jq -n --arg generation "$generation" --arg bundle_uri "$bundle_uri" --arg bundle_sha "$bundle_sha" \
  '{schema:"adl.issue663.generation.v1",artifact_generation:$generation,bundle_uri:$bundle_uri,bundle_sha256:$bundle_sha}' \
  >"$mount_path/.adl-generation.json"
sync
umount "$mount_path"
sync
echo "ADL_ISSUE663_SEAL=PASS generation=$generation"
trap - EXIT
exit 0
