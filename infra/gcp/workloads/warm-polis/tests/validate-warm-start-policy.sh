#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
scripts=("$root/startup-runtime.sh" "$root/startup-ollama.sh")
for script in "${scripts[@]}"; do
  bash -n "$script"
  if rg -n -i '\b(git (clone|fetch|pull)|cargo (build|install)|rustup|apt(-get)? (install|upgrade)|dnf install|yum install|ollama pull|gcloud storage cp)\b' "$script"; then
    echo "forbidden mutable startup action in $script" >&2
    exit 1
  fi
  rg -q 'adl-artifact-generation|artifact_generation|generation' "$script"
  rg -q '/dev/disk/by-id/google-' "$script"
done
echo "issue663_warm_start_policy=pass"
