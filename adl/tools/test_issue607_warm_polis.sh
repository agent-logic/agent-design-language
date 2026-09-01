#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LANE="${1:-all}"
CASE_ROOT="$ROOT/.adl/local/issue607/test-fixtures"
mkdir -p "$CASE_ROOT"

write_plan() {
  local path="$1" changes="$2"
  jq -n --argjson changes "$changes" '{format_version:"1.2",resource_changes:$changes}' >"$path"
}

run_contracts() {
  local guard="$ROOT/adl/tools/issue607_validate_saved_plan.sh"
  local good_compute="$CASE_ROOT/good-compute.json"
  local bad_compute="$CASE_ROOT/bad-compute.json"
  local good_storage="$CASE_ROOT/good-storage.json"
  local bad_storage="$CASE_ROOT/bad-storage.json"

  write_plan "$good_compute" '[{"type":"aws_instance","change":{"actions":["create"]}},{"type":"aws_volume_attachment","change":{"actions":["create"]}}]'
  write_plan "$bad_compute" '[{"type":"aws_ebs_volume","change":{"actions":["delete"]}}]'
  write_plan "$good_storage" '[{"type":"aws_ebs_volume","change":{"actions":["create"]}}]'
  write_plan "$bad_storage" '[{"type":"aws_instance","change":{"actions":["create"]}}]'

  "$guard" compute "$good_compute" >/dev/null
  ! "$guard" compute "$bad_compute" >/dev/null 2>&1
  "$guard" warm-storage "$good_storage" >/dev/null
  ! "$guard" warm-storage "$bad_storage" >/dev/null 2>&1

  rg -q 'http_tokens[[:space:]]*=[[:space:]]*"required"' "$ROOT/infra/aws/runtime/gpu-proof/main.tf"
  rg -q 'http_tokens[[:space:]]*=[[:space:]]*"required"' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  rg -q 'Operator SSH recovery from exact public IPv4 /32' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  ! rg -n 'from_port[[:space:]]*=[[:space:]]*(443|11434|20997|20998)' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation/main.tf"
  rg -q 'prevent_destroy[[:space:]]*=[[:space:]]*true' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/main.tf"
  rg -q 'stop_instance_before_detaching[[:space:]]*=[[:space:]]*true' "$ROOT/infra/aws/runtime/gpu-proof/main.tf"
  ! rg -n 'apt-get|dnf |yum |cargo build|rustup|git clone|ollama pull|snap install' \
    "$ROOT/infra/aws/runtime/gpu-proof/warm-storage" --glob '*.tf'
}

run_terraform() {
  terraform fmt -check -recursive "$ROOT/infra/aws/runtime/gpu-proof"
  terraform -chdir="$ROOT/infra/aws/runtime/gpu-proof" validate
  terraform -chdir="$ROOT/infra/aws/runtime/gpu-proof/warm-storage" validate
  terraform -chdir="$ROOT/infra/aws/runtime/gpu-proof/warm-storage/preparation" validate
}

run_artifacts() {
  rg -q 'artifact_generation' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/main.tf"
  rg -q 'runtime_seal_sha256' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/main.tf"
  rg -q 'gpu_seal_sha256' "$ROOT/infra/aws/runtime/gpu-proof/warm-storage/main.tf"
  rg -q 'runtime_warm_seal_sha256' "$ROOT/infra/aws/runtime/gpu-proof/variables.tf"
  rg -q 'gpu_warm_seal_sha256' "$ROOT/infra/aws/runtime/gpu-proof/variables.tf"
}

case "$LANE" in
  contracts) run_contracts ;;
  terraform) run_terraform ;;
  artifacts) run_artifacts ;;
  all) run_contracts; run_terraform; run_artifacts ;;
  *) echo "usage: test_issue607_warm_polis.sh contracts|terraform|artifacts|all" >&2; exit 2 ;;
esac

printf 'issue607_%s=pass\n' "$LANE"
