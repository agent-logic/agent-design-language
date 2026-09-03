#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
retire="$root/retire-snapshot-generation.sh"
launch="$root/run-live-snapshot-launch.sh"
bash -n "$retire"
bash -n "$launch"
rg -q '\[ "\$#" -ne 3 \]' "$retire"
rg -q 'actual_generation.*expected_generation' "$retire"
rg -q 'actual_runtime_id.*expected_runtime_id' "$retire"
rg -q 'actual_ollama_id.*expected_ollama_id' "$retire"
rg -q 'verification_resources_enabled' "$retire"
rg -q 'ADL_GCP_LIVE_EXECUTION' "$retire"
rg -q 'ADL_GCP_SNAPSHOT_RETIREMENT' "$retire"
rg -q 'terraform -chdir="\$catalog_dir" destroy' "$retire"
rg -q 'ADL_GCP_LIVE_EXECUTION' "$launch"
authorization_line="$(rg -n 'ADL_GCP_LIVE_EXECUTION' "$launch" | head -1 | cut -d: -f1)"
destroy_line="$(rg -n 'terraform -chdir="\$root" destroy' "$launch" | head -1 | cut -d: -f1)"
[ "$authorization_line" -lt "$destroy_line" ] || {
  echo "live-action authorization must precede launch destroy" >&2
  exit 1
}
for field in runtime_running_observed_epoch ollama_running_observed_epoch runtime_ready_epoch gpu_ollama_ready_epoch runtime_guest_boot_relative_ready_seconds gpu_guest_boot_relative_ready_seconds; do
  rg -q "$field" "$launch"
done
for field in cleanup-receipt resource_absence_verified runtime_instance_absent ollama_instance_absent runtime_disk_absent ollama_disk_absent retained_snapshot_observed_self_links snapshots_retained_verified; do
  rg -q "$field" "$launch"
done
if rg -n 'snapshot-catalog.*destroy|retire-snapshot' "$launch"; then
  echo "ordinary launch teardown must not reach snapshot catalog retirement" >&2
  exit 1
fi
echo "issue663_snapshot_retirement=pass"
