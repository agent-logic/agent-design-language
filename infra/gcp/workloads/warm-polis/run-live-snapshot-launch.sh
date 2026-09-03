#!/usr/bin/env bash
set -euo pipefail

mode="${1:-launch}"
root="$(cd "$(dirname "$0")" && pwd)"
receipt="${ADL_GCP_RECEIPT_PATH:-$root/launch-receipt.json}"

[ "${ADL_GCP_LIVE_EXECUTION:-}" = "authorized" ] || {
  echo "live GCP action requires ADL_GCP_LIVE_EXECUTION=authorized" >&2
  exit 1
}

case "$mode" in
  destroy)
    terraform -chdir="$root" destroy
    exit 0
    ;;
  launch) ;;
  *) echo "usage: $0 [launch|destroy]" >&2; exit 64 ;;
esac
command -v gcloud >/dev/null
command -v jq >/dev/null
command -v terraform >/dev/null

start_epoch="$(date +%s)"
terraform -chdir="$root" apply
apply_epoch="$(date +%s)"
runtime_name="$(terraform -chdir="$root" output -raw runtime_instance_name)"
ollama_name="$(terraform -chdir="$root" output -raw ollama_instance_name)"
project="$(terraform -chdir="$root" console <<<"var.project_id" | tr -d '"')"
zone="$(terraform -chdir="$root" console <<<"var.zone" | tr -d '"')"
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

  runtime_serial="$(gcloud compute instances get-serial-port-output "$runtime_name" --project "$project" --zone "$zone" 2>/dev/null || true)"
  ollama_serial="$(gcloud compute instances get-serial-port-output "$ollama_name" --project "$project" --zone "$zone" 2>/dev/null || true)"
  if [ "$runtime_ready" != true ] && grep -q 'ADL_ISSUE663_RUNTIME_READY=PASS' <<<"$runtime_serial"; then
    runtime_ready=true
    runtime_ready_epoch="$now"
    runtime_guest_ready_seconds="$(sed -n 's/.*ADL_ISSUE663_RUNTIME_READY=PASS.*ready_seconds=\([0-9][0-9]*\).*/\1/p' <<<"$runtime_serial" | tail -1)"
    [ -n "$runtime_guest_ready_seconds" ] || runtime_guest_ready_seconds=0
  fi
  if [ "$ollama_ready" != true ] && grep -q 'ADL_ISSUE663_OLLAMA_READY=PASS' <<<"$ollama_serial"; then
    ollama_ready=true
    ollama_ready_epoch="$now"
    ollama_guest_ready_seconds="$(sed -n 's/.*ADL_ISSUE663_OLLAMA_READY=PASS.*ready_seconds=\([0-9][0-9]*\).*/\1/p' <<<"$ollama_serial" | tail -1)"
    [ -n "$ollama_guest_ready_seconds" ] || ollama_guest_ready_seconds=0
  fi
  if [ "$runtime_ready" != true ] || [ "$ollama_ready" != true ] || [ "$runtime_running_observed_epoch" -eq 0 ] || [ "$ollama_running_observed_epoch" -eq 0 ]; then
    sleep 5
  fi
done

ready_epoch="$(date +%s)"
jq -n --argjson start "$start_epoch" --argjson apply "$apply_epoch" --argjson runtime_running "$runtime_running_observed_epoch" \
  --argjson ollama_running "$ollama_running_observed_epoch" --argjson runtime_ready "$runtime_ready_epoch" \
  --argjson ollama_ready "$ollama_ready_epoch" --argjson runtime_guest "$runtime_guest_ready_seconds" \
  --argjson ollama_guest "$ollama_guest_ready_seconds" --argjson ready "$ready_epoch" \
  --arg runtime "$runtime_name" --arg ollama "$ollama_name" \
  --arg runtime_start "$runtime_last_start_timestamp" --arg ollama_start "$ollama_last_start_timestamp" \
  '{schema:"adl.issue663.launch-receipt.v1",status:"ready",launch_request_epoch:$start,terraform_apply_complete_epoch:$apply,runtime_running_observed_epoch:$runtime_running,ollama_running_observed_epoch:$ollama_running,runtime_last_start_timestamp:$runtime_start,ollama_last_start_timestamp:$ollama_start,runtime_ready_epoch:$runtime_ready,gpu_ollama_ready_epoch:$ollama_ready,runtime_guest_boot_relative_ready_seconds:$runtime_guest,gpu_guest_boot_relative_ready_seconds:$ollama_guest,full_polis_ready_epoch:$ready,snapshot_launch_to_ready_seconds:($ready-$start),terraform_apply_seconds:($apply-$start),runtime_instance:$runtime,ollama_instance:$ollama}' \
  >"$receipt"
jq . "$receipt"
