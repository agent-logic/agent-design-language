#!/usr/bin/env bash
set -euo pipefail

mode="${1:-launch}"
root="$(cd "$(dirname "$0")" && pwd)"
receipt="${ADL_GCP_RECEIPT_PATH:-$root/launch-receipt.json}"

case "$mode" in
  destroy)
    terraform -chdir="$root" destroy
    exit 0
    ;;
  launch) ;;
  *) echo "usage: $0 [launch|destroy]" >&2; exit 64 ;;
esac

[ "${ADL_GCP_LIVE_EXECUTION:-}" = "authorized" ] || {
  echo "live GCP launch requires ADL_GCP_LIVE_EXECUTION=authorized" >&2
  exit 1
}
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

runtime_ready=false
ollama_ready=false
while [ "$runtime_ready" != true ] || [ "$ollama_ready" != true ]; do
  now="$(date +%s)"
  if [ "$timeout_seconds" -gt 0 ] && [ $((now - start_epoch)) -ge "$timeout_seconds" ]; then
    jq -n --argjson start "$start_epoch" --argjson observed "$now" \
      '{schema:"adl.issue663.launch-receipt.v1",status:"observation_timeout",launch_request_epoch:$start,observation_epoch:$observed,resources_left_running:true}' >"$receipt"
    echo "observation timed out; resources were not terminated" >&2
    exit 2
  fi
  runtime_serial="$(gcloud compute instances get-serial-port-output "$runtime_name" --project "$project" --zone "$zone" 2>/dev/null || true)"
  ollama_serial="$(gcloud compute instances get-serial-port-output "$ollama_name" --project "$project" --zone "$zone" 2>/dev/null || true)"
  grep -q 'ADL_ISSUE663_RUNTIME_READY=PASS' <<<"$runtime_serial" && runtime_ready=true
  grep -q 'ADL_ISSUE663_OLLAMA_READY=PASS' <<<"$ollama_serial" && ollama_ready=true
  [ "$runtime_ready" = true ] && [ "$ollama_ready" = true ] || sleep 5
done

ready_epoch="$(date +%s)"
jq -n --argjson start "$start_epoch" --argjson apply "$apply_epoch" --argjson ready "$ready_epoch" \
  --arg runtime "$runtime_name" --arg ollama "$ollama_name" \
  '{schema:"adl.issue663.launch-receipt.v1",status:"ready",launch_request_epoch:$start,terraform_apply_complete_epoch:$apply,full_polis_ready_epoch:$ready,snapshot_launch_to_ready_seconds:($ready-$start),terraform_apply_seconds:($apply-$start),runtime_instance:$runtime,ollama_instance:$ollama}' \
  >"$receipt"
jq . "$receipt"
