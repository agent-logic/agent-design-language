#!/usr/bin/env bash
set -euo pipefail

mode="${1:-launch}"
root="$(cd "$(dirname "$0")" && pwd)"
receipt="${ADL_GCP_RECEIPT_PATH:-$root/launch-receipt.json}"
cleanup_receipt="${ADL_GCP_CLEANUP_RECEIPT_PATH:-$root/cleanup-receipt.json}"
launch_receipt_ref="$(basename "$receipt")"

[ "${ADL_GCP_LIVE_EXECUTION:-}" = "authorized" ] || {
  echo "live GCP action requires ADL_GCP_LIVE_EXECUTION=authorized" >&2
  exit 1
}
command -v gcloud >/dev/null
command -v jq >/dev/null
command -v terraform >/dev/null

case "$mode" in
  destroy)
    cleanup_start_epoch="$(date +%s)"
    project="$(terraform -chdir="$root" console <<<"var.project_id" | tr -d '"')"
    zone="$(terraform -chdir="$root" console <<<"var.zone" | tr -d '"')"
    generation="$(terraform -chdir="$root" console <<<"var.artifact_generation" | tr -d '"')"
    runtime_name="$(terraform -chdir="$root" output -raw runtime_instance_name)"
    ollama_name="$(terraform -chdir="$root" output -raw ollama_instance_name)"
    cleanup_contract="$(terraform -chdir="$root" output -json cleanup_contract)"
    runtime_disk="$(jq -r '.launch_state_deletes[0]' <<<"$cleanup_contract")"
    ollama_disk="$(jq -r '.launch_state_deletes[1]' <<<"$cleanup_contract")"
    runtime_snapshot="$(jq -r '.retained_snapshots[0]' <<<"$cleanup_contract")"
    ollama_snapshot="$(jq -r '.retained_snapshots[1]' <<<"$cleanup_contract")"
    if ! terraform -chdir="$root" destroy -auto-approve; then
      jq -n --argjson started "$cleanup_start_epoch" --argjson observed "$(date +%s)" \
        --arg launch_receipt "$launch_receipt_ref" \
        '{schema:"adl.issue663.cleanup-receipt.v1",status:"destroy_failed",cleanup_request_epoch:$started,cleanup_observation_epoch:$observed,launch_receipt:$launch_receipt,resource_absence_verified:false,snapshots_retained_verified:false}' >"$cleanup_receipt"
      exit 1
    fi
    runtime_instance_absent=false
    ollama_instance_absent=false
    runtime_disk_absent=false
    ollama_disk_absent=false
    gcloud compute instances describe "$runtime_name" --project "$project" --zone "$zone" >/dev/null 2>&1 || runtime_instance_absent=true
    gcloud compute instances describe "$ollama_name" --project "$project" --zone "$zone" >/dev/null 2>&1 || ollama_instance_absent=true
    gcloud compute disks describe "$runtime_disk" --project "$project" --zone "$zone" >/dev/null 2>&1 || runtime_disk_absent=true
    gcloud compute disks describe "$ollama_disk" --project "$project" --zone "$zone" >/dev/null 2>&1 || ollama_disk_absent=true
    runtime_snapshot_observed="$(gcloud compute snapshots describe "${runtime_snapshot##*/}" --project "$project" --format='value(selfLink)' 2>/dev/null || true)"
    ollama_snapshot_observed="$(gcloud compute snapshots describe "${ollama_snapshot##*/}" --project "$project" --format='value(selfLink)' 2>/dev/null || true)"
    resources_absent=false
    snapshots_retained=false
    if [ "$runtime_instance_absent" = true ] && [ "$ollama_instance_absent" = true ] && [ "$runtime_disk_absent" = true ] && [ "$ollama_disk_absent" = true ]; then
      resources_absent=true
    fi
    snapshot_inventory="$(gcloud compute snapshots list --project "$project" \
      --filter="labels.adl_generation=$generation AND labels.adl_retained=true" \
      --format='json(name,selfLink)')"
    if [ -n "$runtime_snapshot_observed" ] && [ -n "$ollama_snapshot_observed" ] && \
      jq -e --arg runtime "$runtime_snapshot_observed" --arg ollama "$ollama_snapshot_observed" \
        '([.[].selfLink] | sort) == ([$runtime,$ollama] | sort)' <<<"$snapshot_inventory" >/dev/null; then
      snapshots_retained=true
    fi
    cleanup_observed_epoch="$(date +%s)"
    status=cleanup_verification_failed
    [ "$resources_absent" = true ] && [ "$snapshots_retained" = true ] && status=cleaned
    jq -n --arg status "$status" --argjson started "$cleanup_start_epoch" --argjson observed "$cleanup_observed_epoch" \
      --arg launch_receipt "$launch_receipt_ref" --arg runtime_instance "$runtime_name" --arg ollama_instance "$ollama_name" \
      --arg runtime_disk "$runtime_disk" --arg ollama_disk "$ollama_disk" \
      --arg runtime_snapshot "$runtime_snapshot_observed" --arg ollama_snapshot "$ollama_snapshot_observed" \
      --argjson snapshot_inventory "$snapshot_inventory" \
      --argjson runtime_instance_absent "$runtime_instance_absent" --argjson ollama_instance_absent "$ollama_instance_absent" \
      --argjson runtime_disk_absent "$runtime_disk_absent" --argjson ollama_disk_absent "$ollama_disk_absent" \
      --argjson resources_absent "$resources_absent" --argjson snapshots_retained "$snapshots_retained" \
      '{schema:"adl.issue663.cleanup-receipt.v1",status:$status,cleanup_request_epoch:$started,cleanup_observation_epoch:$observed,launch_receipt:$launch_receipt,runtime_instance:{name:$runtime_instance,absent:$runtime_instance_absent},ollama_instance:{name:$ollama_instance,absent:$ollama_instance_absent},runtime_restored_disk:{name:$runtime_disk,absent:$runtime_disk_absent},ollama_restored_disk:{name:$ollama_disk,absent:$ollama_disk_absent},resource_absence_verified:$resources_absent,retained_snapshot_inventory:$snapshot_inventory,retained_snapshot_observed_self_links:[$runtime_snapshot,$ollama_snapshot],exact_retained_snapshot_set_verified:$snapshots_retained,snapshots_retained_verified:$snapshots_retained}' >"$cleanup_receipt"
    jq . "$cleanup_receipt"
    [ "$status" = cleaned ] || exit 2
    exit 0
    ;;
  launch) ;;
  *) echo "usage: $0 [launch|destroy]" >&2; exit 64 ;;
esac

start_epoch="$(date +%s)"
terraform -chdir="$root" apply -auto-approve
apply_epoch="$(date +%s)"
runtime_name="$(terraform -chdir="$root" output -raw runtime_instance_name)"
ollama_name="$(terraform -chdir="$root" output -raw ollama_instance_name)"
project="$(terraform -chdir="$root" console <<<"var.project_id" | tr -d '"')"
region="$(terraform -chdir="$root" console <<<"var.region" | tr -d '"')"
zone="$(terraform -chdir="$root" console <<<"var.zone" | tr -d '"')"
generation="$(terraform -chdir="$root" console <<<"var.artifact_generation" | tr -d '"')"
runtime_snapshot="$(terraform -chdir="$root" console <<<"var.runtime_snapshot" | tr -d '"')"
ollama_snapshot="$(terraform -chdir="$root" console <<<"var.ollama_snapshot" | tr -d '"')"
accelerator_type="$(terraform -chdir="$root" console <<<"var.accelerator_type" | tr -d '"')"
accelerator_count="$(terraform -chdir="$root" console <<<"var.accelerator_count" | tr -d '"')"
expected_models="$(terraform -chdir="$root" console <<<"join(\",\", var.resident_models)" | tr -d '"' | jq -R 'split(",")')"
timeout_seconds="${ADL_GCP_OBSERVATION_TIMEOUT_SECONDS:-0}"

runtime_running_observed_epoch=0
ollama_running_observed_epoch=0
runtime_last_start_timestamp=""
ollama_last_start_timestamp=""
runtime_ready_epoch=0
ollama_ready_epoch=0
runtime_guest_ready_seconds=0
ollama_guest_ready_seconds=0
runtime_ready=false
ollama_ready=false
runtime_boot_id=""
ollama_boot_id=""
observed_models='[]'
agent_summary='[]'
while [ "$runtime_ready" != true ] || [ "$ollama_ready" != true ] || [ "$runtime_running_observed_epoch" -eq 0 ] || [ "$ollama_running_observed_epoch" -eq 0 ]; do
  now="$(date +%s)"
  if [ "$timeout_seconds" -gt 0 ] && [ $((now - start_epoch)) -ge "$timeout_seconds" ]; then
    jq -n --argjson start "$start_epoch" --argjson observed "$now" \
      '{schema:"adl.issue663.launch-receipt.v1",status:"observation_timeout",launch_request_epoch:$start,observation_epoch:$observed,resources_left_running:true}' >"$receipt"
    echo "observation timed out; resources were not terminated" >&2
    exit 2
  fi

  if [ "$runtime_running_observed_epoch" -eq 0 ]; then
    runtime_status="$(gcloud compute instances describe "$runtime_name" --project "$project" --zone "$zone" --format='value(status)' 2>/dev/null || true)"
    if [ "$runtime_status" = "RUNNING" ]; then
      runtime_running_observed_epoch="$now"
      runtime_last_start_timestamp="$(gcloud compute instances describe "$runtime_name" --project "$project" --zone "$zone" --format='value(lastStartTimestamp)')"
    fi
  fi
  if [ "$ollama_running_observed_epoch" -eq 0 ]; then
    ollama_status="$(gcloud compute instances describe "$ollama_name" --project "$project" --zone "$zone" --format='value(status)' 2>/dev/null || true)"
    if [ "$ollama_status" = "RUNNING" ]; then
      ollama_running_observed_epoch="$now"
      ollama_last_start_timestamp="$(gcloud compute instances describe "$ollama_name" --project "$project" --zone "$zone" --format='value(lastStartTimestamp)')"
    fi
  fi

  runtime_serial="$(gcloud compute instances get-serial-port-output "$runtime_name" --project "$project" --zone "$zone" 2>/dev/null | tr -d '\r' || true)"
  ollama_serial="$(gcloud compute instances get-serial-port-output "$ollama_name" --project "$project" --zone "$zone" 2>/dev/null | tr -d '\r' || true)"
  current_runtime_boot_id="$(sed -n "s/.*ADL_ISSUE663_RUNTIME_BOOT generation=$generation boot_id=\([^ ]*\).*/\1/p" <<<"$runtime_serial" | tail -1)"
  runtime_ready_line="$(grep "ADL_ISSUE663_RUNTIME_READY=PASS generation=$generation" <<<"$runtime_serial" | tail -1 || true)"
  ready_runtime_boot_id="$(sed -n 's/.* boot_id=\([^ ]*\).*/\1/p' <<<"$runtime_ready_line")"
  if [ "$runtime_ready" != true ] && [ -n "$current_runtime_boot_id" ] && [ "$ready_runtime_boot_id" = "$current_runtime_boot_id" ]; then
    runtime_ready=true
    runtime_ready_epoch="$now"
    runtime_boot_id="$ready_runtime_boot_id"
    runtime_guest_ready_seconds="$(sed -n 's/.*ready_seconds=\([0-9][0-9]*\).*/\1/p' <<<"$runtime_ready_line")"
    [ -n "$runtime_guest_ready_seconds" ] || runtime_guest_ready_seconds=0
    runtime_last_start_timestamp="$(gcloud compute instances describe "$runtime_name" --project "$project" --zone "$zone" --format='value(lastStartTimestamp)')"
    agent_summary="$(sed -n 's/^.*ADL_ISSUE670_AGENT_SUMMARY=//p' <<<"$runtime_serial" | tail -1)"
    jq -e 'length == 6 and all(.[]; .agent_test_outcome == "executed" and .runtime_exit_code == 0 and .runtime_decision == "executed")' <<<"$agent_summary" >/dev/null
  fi
  current_ollama_boot_id="$(sed -n "s/.*ADL_ISSUE663_OLLAMA_BOOT generation=$generation boot_id=\([^ ]*\).*/\1/p" <<<"$ollama_serial" | tail -1)"
  ollama_ready_line="$(grep "ADL_ISSUE663_OLLAMA_READY=PASS generation=$generation" <<<"$ollama_serial" | tail -1 || true)"
  ready_ollama_boot_id="$(sed -n 's/.* boot_id=\([^ ]*\).*/\1/p' <<<"$ollama_ready_line")"
  if [ "$ollama_ready" != true ] && [ -n "$current_ollama_boot_id" ] && [ "$ready_ollama_boot_id" = "$current_ollama_boot_id" ]; then
    ollama_ready=true
    ollama_ready_epoch="$now"
    ollama_boot_id="$ready_ollama_boot_id"
    ollama_guest_ready_seconds="$(sed -n 's/.*ready_seconds=\([0-9][0-9]*\).*/\1/p' <<<"$ollama_ready_line")"
    [ -n "$ollama_guest_ready_seconds" ] || ollama_guest_ready_seconds=0
    ollama_last_start_timestamp="$(gcloud compute instances describe "$ollama_name" --project "$project" --zone "$zone" --format='value(lastStartTimestamp)')"
    observed_models="$(sed -n 's/.* models=\([^ ]*\).*/\1/p' <<<"$ollama_ready_line" | jq -R 'split(",")')"
    jq -e --argjson expected "$expected_models" 'sort == ($expected | sort)' <<<"$observed_models" >/dev/null
  fi
  if [ "$runtime_ready" != true ] || [ "$ollama_ready" != true ] || [ "$runtime_running_observed_epoch" -eq 0 ] || [ "$ollama_running_observed_epoch" -eq 0 ]; then
    sleep 5
  fi
done

ready_epoch="$(date +%s)"
runtime_private_ip="$(terraform -chdir="$root" output -raw runtime_private_ip)"
ollama_private_ip="$(terraform -chdir="$root" output -raw ollama_private_ip)"
runtime_public_ip="$(gcloud compute instances describe "$runtime_name" --project "$project" --zone "$zone" --format='value(networkInterfaces[0].accessConfigs[0].natIP)')"
ollama_public_ip="$(gcloud compute instances describe "$ollama_name" --project "$project" --zone "$zone" --format='value(networkInterfaces[0].accessConfigs[0].natIP)')"
[ -z "$runtime_public_ip" ]
[ -z "$ollama_public_ip" ]
jq -n --argjson start "$start_epoch" --argjson apply "$apply_epoch" --argjson runtime_running "$runtime_running_observed_epoch" \
  --argjson ollama_running "$ollama_running_observed_epoch" --argjson runtime_ready "$runtime_ready_epoch" \
  --argjson ollama_ready "$ollama_ready_epoch" --argjson runtime_guest "$runtime_guest_ready_seconds" \
  --argjson ollama_guest "$ollama_guest_ready_seconds" --argjson ready "$ready_epoch" \
  --arg runtime "$runtime_name" --arg ollama "$ollama_name" \
  --arg runtime_start "$runtime_last_start_timestamp" --arg ollama_start "$ollama_last_start_timestamp" \
  --arg project "$project" --arg region "$region" --arg zone "$zone" --arg generation "$generation" \
  --arg runtime_snapshot "$runtime_snapshot" --arg ollama_snapshot "$ollama_snapshot" \
  --arg accelerator_type "$accelerator_type" --argjson accelerator_count "$accelerator_count" \
  --arg runtime_private_ip "$runtime_private_ip" --arg ollama_private_ip "$ollama_private_ip" \
  --arg runtime_boot_id "$runtime_boot_id" --arg ollama_boot_id "$ollama_boot_id" \
  --argjson models "$observed_models" --argjson agents "$agent_summary" \
  '{schema:"adl.issue670.launch-qualification.v2",status:"ready",project:$project,region:$region,zone:$zone,artifact_generation:$generation,snapshots:{runtime:$runtime_snapshot,ollama:$ollama_snapshot},gpu:{accelerator_type:$accelerator_type,accelerator_count:$accelerator_count},network:{runtime_private_ip:$runtime_private_ip,ollama_private_ip:$ollama_private_ip,runtime_public_ip:null,ollama_public_ip:null,ollama_ingress:"runtime-private-only"},resident_models:$models,resident_model_count:($models|length),agents:$agents,agent_tool_count:($agents|length),launch_request_epoch:$start,terraform_apply_complete_epoch:$apply,first_runtime_running_observed_epoch:$runtime_running,first_ollama_running_observed_epoch:$ollama_running,final_runtime_last_start_timestamp:$runtime_start,final_ollama_last_start_timestamp:$ollama_start,final_runtime_boot_id:$runtime_boot_id,final_ollama_boot_id:$ollama_boot_id,runtime_ready_epoch:$runtime_ready,gpu_ollama_ready_epoch:$ollama_ready,runtime_guest_boot_relative_ready_seconds:$runtime_guest,gpu_guest_boot_relative_ready_seconds:$ollama_guest,full_polis_ready_epoch:$ready,snapshot_launch_to_ready_seconds:($ready-$start),terraform_apply_seconds:($apply-$start),runtime_instance:$runtime,ollama_instance:$ollama}' \
  >"$receipt"
jq . "$receipt"
