#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
repo_root="$(git -C "$root" rev-parse --show-toplevel)"
retire="$root/retire-snapshot-generation.sh"
launch="$root/run-live-snapshot-launch.sh"
prepare="$root/prepare-snapshot-generation.sh"
module="$root/../modules/two-node-ollama-runtime/main.tf"
bash -n "$retire"
bash -n "$launch"
bash -n "$prepare"
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
for field in ADL_ISSUE663_RUNTIME_BOOT ADL_ISSUE663_OLLAMA_BOOT current_runtime_boot_id current_ollama_boot_id final_runtime_boot_id final_ollama_boot_id ADL_ISSUE670_AGENT_SUMMARY resident_models agent_tool_count; do
  rg -q "$field" "$root"
done
for field in cleanup-receipt resource_absence_verified runtime_instance_absent ollama_instance_absent runtime_disk_absent ollama_disk_absent retained_snapshot_inventory exact_retained_snapshot_set_verified retained_snapshot_observed_self_links snapshots_retained_verified; do
  rg -q "$field" "$launch"
done
for field in expected_project preflight_receipt residual_instances residual_disks residual_firewalls residual_images residual_addresses residual_issue_inventory; do
  rg -q "$field" "$launch"
done
! rg -q 'ADL_GCP_EXPECTED_PROJECT|ADL_GCP_PREFLIGHT_RECEIPT_PATH|ADL_GCP_CONSERVATIVE_HOURLY_USD|ADL_GCP_MAX_PAID_HOURS|ADL_GCP_STORAGE_RESERVE_USD' "$launch" "$prepare" "$repo_root/.csdlc/prepared/issues/670/run-live-preflight.sh" || {
  echo "issue #670 project, preflight path, and cost-envelope authority must not be caller-overridable" >&2
  exit 1
}
for field in catalog_project catalog_region catalog_zone catalog_generation paid_deadline_epoch require_paid_time; do
  rg -q "$field" "$prepare"
done
for startup in "$root/startup-runtime.sh" "$root/startup-ollama.sh"; do
  rg -q 'budget_stop_seconds=28800' "$startup"
  rg -q 'adl-issue670-budget-stop' "$startup"
done
launch_project_guard_line="$(rg -n 'not authorized project' "$launch" | head -1 | cut -d: -f1)"
launch_apply_line="$(rg -n 'terraform -chdir="\$root" apply' "$launch" | head -1 | cut -d: -f1)"
[ "$launch_project_guard_line" -lt "$launch_apply_line" ] || {
  echo "exact-project launch guard must precede paid apply" >&2
  exit 1
}
prepare_project_guard_line="$(rg -n 'not authorized project' "$prepare" | head -1 | cut -d: -f1)"
prepare_apply_line="$(rg -n 'terraform -chdir="\$root/preparation" apply' "$prepare" | head -1 | cut -d: -f1)"
[ "$prepare_project_guard_line" -lt "$prepare_apply_line" ] || {
  echo "exact-project preparation guard must precede paid apply" >&2
  exit 1
}
[ "$(rg -c 'ignore_changes = \[attached_disk\]' "$module")" -eq 2 ] || {
  echo "both nodes must preserve separately managed warm-disk attachments on repeat apply" >&2
  exit 1
}
if rg -n 'snapshot-catalog.*destroy|retire-snapshot' "$launch"; then
  echo "ordinary launch teardown must not reach snapshot catalog retirement" >&2
  exit 1
fi
echo "issue663_snapshot_retirement=pass"
