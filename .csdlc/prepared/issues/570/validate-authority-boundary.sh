#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

failures=0

require_file_contains() {
  local file="$1"
  local pattern="$2"
  local label="$3"
  if [[ ! -f "$file" ]]; then
    echo "missing file: $file" >&2
    failures=$((failures + 1))
    return
  fi
  if ! grep -Eiq "$pattern" "$file"; then
    echo "missing: $label in $file" >&2
    failures=$((failures + 1))
  fi
}

reject_file_contains() {
  local file="$1"
  local pattern="$2"
  local label="$3"
  if [[ -f "$file" ]] && grep -Eiq "$pattern" "$file"; then
    echo "forbidden: $label in $file" >&2
    failures=$((failures + 1))
  fi
}

surfaces=(
  "AGENTS.md"
  "docs/onboarding.md"
  "csdlc-v2/AGENTS.md"
  "csdlc-v2/operator/SKILLS.md"
  "csdlc-v3/README.md"
  "csdlc-v3/AGENTS.md"
)

for file in "${surfaces[@]}"; do
  require_file_contains "$file" "V3-F|#505|cutover" "explicit V3-F/#505 cutover boundary"
  reject_file_contains "$file" "v3.*(publish|finish|clean|mutate).*live.*lifecycle" "premature v3 lifecycle mutation authority"
done

require_file_contains "csdlc-v3/README.md" "clean replacement|replace" "clean replacement target"
require_file_contains "docs/onboarding.md" "prepared.*issue.*(three|3).*minute|(three|3).*minute.*prepared.*issue" "three-minute prepared issue guidance"

if (( failures > 0 )); then
  exit 1
fi

echo "authority boundary scan passed"
