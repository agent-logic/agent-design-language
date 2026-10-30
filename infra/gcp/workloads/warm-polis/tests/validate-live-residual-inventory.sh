#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
receipt="$root/.csdlc/evidence/670/live/residual-inventory-g670b.json"
project="$(jq -r '.project' "$receipt")"
name_filter="$(jq -r '.filters.name_regex' "$receipt")"
generation="$(jq -r '.filters.retained_snapshot_generation' "$receipt")"

for resource in instances disks firewall-rules images addresses; do
  observed="$(gcloud compute "$resource" list --project "$project" --filter="name~'$name_filter'" --format=json)"
  jq -e 'length == 0' <<<"$observed" >/dev/null
done

observed_snapshots="$(gcloud compute snapshots list --project "$project" \
  --filter="labels.adl_generation=$generation AND labels.adl_retained=true" \
  --format='json(name,status,selfLink,storageBytes)')"
jq -e --argjson observed "$observed_snapshots" '
  ($observed | sort_by(.name)) == (.retained_snapshots | sort_by(.name))
' "$receipt" >/dev/null

echo "issue670_live_residual_inventory=pass"
