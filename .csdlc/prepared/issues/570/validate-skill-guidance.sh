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

v2_skill_files=(
  "csdlc-v2/AGENTS.md"
  "csdlc-v2/operator/SKILLS.md"
)

while IFS= read -r file; do
  v2_skill_files+=("$file")
done < <(find csdlc-v2/operator/skills -mindepth 2 -maxdepth 2 -name SKILL.md | sort)

if [[ "${#v2_skill_files[@]}" -ne 13 ]]; then
  echo "unexpected v2 skill guidance denominator: ${#v2_skill_files[@]} files" >&2
  failures=$((failures + 1))
fi

for file in "${v2_skill_files[@]}"; do
  require_file_contains "$file" "v2.*(live|sole|current).*authority|authority.*v2" "v2 authority wording"
  require_file_contains "$file" "until.*(v3|V3-F|#505).*cutover|v3.*cutover" "until-v3-cutover wording"
  require_file_contains "$file" "v3.*(construction|non-authoritative|non-authority)|construction.*v3" "v3 construction non-authority"
done

installed_pr_skills=(
  "/Users/daniel/.codex/skills/pr-init/SKILL.md"
  "/Users/daniel/.codex/skills/pr-ready/SKILL.md"
  "/Users/daniel/.codex/skills/pr-run/SKILL.md"
  "/Users/daniel/.codex/skills/pr-finish/SKILL.md"
  "/Users/daniel/.codex/skills/pr-janitor/SKILL.md"
)

for file in "${installed_pr_skills[@]}"; do
  require_file_contains "$file" "v2.*(live|sole|current).*authority|authority.*v2" "installed skill v2 authority wording"
  require_file_contains "$file" "v3.*(construction|non-authoritative|non-authority)|construction.*v3" "installed skill v3 construction non-authority"
done

if (( failures > 0 )); then
  exit 1
fi

echo "skill guidance scan passed"
