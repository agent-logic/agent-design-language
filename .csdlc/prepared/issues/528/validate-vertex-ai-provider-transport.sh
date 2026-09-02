#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$root"

required_files=(
  ".csdlc/prepared/issues/528/design.md"
  ".csdlc/prepared/issues/528/diagram.mmd"
  "adl/src/provider_adapter.rs"
  "adl/src/provider_substrate.rs"
  "adl/src/provider_communication.rs"
  "docs/validation/pvf_lanes.json"
)

for path in "${required_files[@]}"; do
  if [[ ! -f "$path" ]]; then
    echo "missing required #528 file: $path" >&2
    exit 1
  fi
done

grep -q "Vertex AI" ".csdlc/prepared/issues/528/design.md"
grep -q "ADC / workload identity" ".csdlc/prepared/issues/528/diagram.mmd"
grep -q "provider" "adl/src/provider_adapter.rs"

if grep -R --line-number -E '(ya29\\.|BEGIN PRIVATE KEY|-----BEGIN|GOOGLE_APPLICATION_CREDENTIALS=.*\\.json|access_token)' \
  --exclude='validate-vertex-ai-provider-transport.sh' \
  ".csdlc/prepared/issues/528" \
  "docs/operations" 2>/dev/null; then
  echo "#528 prepared/documentation surfaces contain credential-shaped material" >&2
  exit 1
fi

echo "#528 prepared Vertex AI provider transport validator: PASS"
