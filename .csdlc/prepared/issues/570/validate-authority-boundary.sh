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
  local content
  content="$(tr '\n' ' ' < "$file")"
  if ! grep -Eiq "$pattern" <<<"$content"; then
    echo "missing: $label in $file" >&2
    failures=$((failures + 1))
  fi
}

reject_file_contains() {
  local file="$1"
  local pattern="$2"
  local label="$3"
  if [[ -f "$file" ]]; then
    local content
    content="$(tr '\n' ' ' < "$file")"
    if grep -Eiq "$pattern" <<<"$content"; then
    echo "forbidden: $label in $file" >&2
    failures=$((failures + 1))
    fi
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

while IFS= read -r file; do
  surfaces+=("$file")
done < <(find csdlc-v2/operator/skills -mindepth 2 -maxdepth 2 -name SKILL.md | sort)

for file in "${surfaces[@]}"; do
  require_file_contains "$file" "V3-F|#505|cutover" "explicit V3-F/#505 cutover boundary"
  reject_file_contains "$file" "v3[^.]{0,80}(is|becomes|remains)[^.]{0,80}(publish|finish|clean|mutate)[^.]{0,80}live[^.]{0,80}lifecycle" "premature v3 lifecycle mutation authority"
  reject_file_contains "$file" "v3[^.]{0,80}(may|can|should)[^.]{0,80}(publish|finish|clean|mutate|bind)[^.]{0,80}(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)" "pre-cutover v3 lifecycle authority"
  reject_file_contains "$file" "(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)[^.]{0,80}v3[^.]{0,80}(may|can|should)[^.]{0,80}(publish|finish|clean|mutate|bind|advance|validate)" "pre-cutover v3 lifecycle authority with leading cutover clause"
  reject_file_contains "$file" "v3[^.]{0,80}(replaces|supersedes|retires)[^.]{0,80}v2[^.]{0,80}(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)" "pre-cutover v3 replacement authority"
  reject_file_contains "$file" "(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)[^.]{0,80}v3[^.]{0,80}(replaces|supersedes|retires)[^.]{0,80}v2" "pre-cutover v3 replacement authority with leading cutover clause"
done

require_file_contains "csdlc-v3/README.md" "clean replacement|replace" "clean replacement target"
require_file_contains "docs/onboarding.md" "prepared.*issue.*(three|3).*minute|(three|3).*minute.*prepared.*issue" "three-minute prepared issue guidance"

if (( failures > 0 )); then
  exit 1
fi

echo "authority boundary scan passed"
