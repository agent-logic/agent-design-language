#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 3 ]; then
  echo "usage: $0 EXPECTED_GENERATION EXPECTED_RUNTIME_SNAPSHOT_ID EXPECTED_OLLAMA_SNAPSHOT_ID" >&2
  exit 64
fi
expected_generation="$1"
expected_runtime_id="$2"
expected_ollama_id="$3"
catalog_dir="$(cd "$(dirname "$0")/snapshot-catalog" && pwd)"

actual_generation="$(terraform -chdir="$catalog_dir" output -raw generation)"
actual_runtime_id="$(terraform -chdir="$catalog_dir" output -raw runtime_snapshot_id)"
actual_ollama_id="$(terraform -chdir="$catalog_dir" output -raw ollama_snapshot_id)"
verifier_enabled="$(terraform -chdir="$catalog_dir" output -raw verification_resources_enabled)"

[ "$actual_generation" = "$expected_generation" ] || { echo "generation mismatch" >&2; exit 1; }
[ "$actual_runtime_id" = "$expected_runtime_id" ] || { echo "runtime snapshot ID mismatch" >&2; exit 1; }
[ "$actual_ollama_id" = "$expected_ollama_id" ] || { echo "ollama snapshot ID mismatch" >&2; exit 1; }
[ "$verifier_enabled" = "false" ] || { echo "verifier resources must be removed before retirement" >&2; exit 1; }

terraform -chdir="$catalog_dir" destroy
