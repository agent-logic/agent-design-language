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
rg -q 'terraform -chdir="\$catalog_dir" destroy' "$retire"
if rg -n 'snapshot-catalog.*destroy|retire-snapshot' "$launch"; then
  echo "ordinary launch teardown must not reach snapshot catalog retirement" >&2
  exit 1
fi
echo "issue663_snapshot_retirement=pass"
