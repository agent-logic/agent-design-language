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
  require_file_contains "$file" "C-SDLC v2 remains the live[^.]{0,80}authority" "explicit live v2 authority wording"
  require_file_contains "$file" "until.*(v3|V3-F|#505).*cutover|v3.*cutover" "until-v3-cutover wording"
  require_file_contains "$file" "v3.*(construction|non-authoritative|non-authority)|construction.*v3" "v3 construction non-authority"
  reject_file_contains "$file" "v3[^.]{0,80}(is|becomes|remains)[^.]{0,80}(the )?(current|live|sole|final)[^.]{0,80}(lifecycle )?authority" "plain premature v3 authority claim"
  reject_file_contains "$file" "v3[^.]{0,80}(is|becomes|remains)[^.]{0,80}(publish|finish|clean|mutate)[^.]{0,80}live[^.]{0,80}lifecycle" "premature v3 lifecycle mutation authority"
  reject_file_contains "$file" "v3[^.]{0,80}(may|can|should)[^.]{0,80}(publish|finish|clean|mutate|bind)[^.]{0,80}(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)" "pre-cutover v3 lifecycle authority"
  reject_file_contains "$file" "(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)[^.]{0,80}v3[^.]{0,80}(may|can|should)[^.]{0,80}(publish|finish|clean|mutate|bind|advance|validate)" "pre-cutover v3 lifecycle authority with leading cutover clause"
  reject_file_contains "$file" "v3[^.]{0,80}(replaces|supersedes|retires)[^.]{0,80}v2[^.]{0,80}(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)" "pre-cutover v3 replacement authority"
  reject_file_contains "$file" "(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)[^.]{0,80}v3[^.]{0,80}(replaces|supersedes|retires)[^.]{0,80}v2" "pre-cutover v3 replacement authority with leading cutover clause"
done

installed_pr_skills=(
  "pr-init/SKILL.md"
  "pr-ready/SKILL.md"
  "pr-run/SKILL.md"
  "pr-finish/SKILL.md"
  "pr-janitor/SKILL.md"
  "adl_pr_cycle/SKILL.md"
)

tracked_skill_guidance=(
  "docs/tooling/adl_pr_cycle_skill.md"
)

for file in "${tracked_skill_guidance[@]}"; do
  require_file_contains "$file" "historical compatibility|not a current workflow entrypoint|blocked by policy" "tracked adl_pr_cycle historical-only wording"
  reject_file_contains "$file" "Purpose:[^.]{0,160}Route one tracked issue|Procedure:[^.]{0,240}Preflight[^.]{0,120}Init[^.]{0,120}Bind" "tracked adl_pr_cycle executable route"
done

if [[ -n "${CODEX_SKILLS_ROOT:-}" ]]; then
  skills_root="$CODEX_SKILLS_ROOT"
elif [[ -d "$HOME/.codex/skills" ]]; then
  skills_root="$HOME/.codex/skills"
else
  skills_root=""
fi

if [[ -n "$skills_root" ]]; then
  echo "checking local installed skills under $skills_root" >&2
else
  echo "local installed skill root unavailable; tracked skill guidance remains proving surface" >&2
fi

for relative in "${installed_pr_skills[@]}"; do
  if [[ -z "$skills_root" ]]; then
    continue
  fi
  file="$skills_root/$relative"
  require_file_contains "$file" "C-SDLC v2 remains the live[^.]{0,80}authority" "installed skill explicit live v2 authority wording"
  require_file_contains "$file" "v3.*(construction|non-authoritative|non-authority)|construction.*v3" "installed skill v3 construction non-authority"
  if [[ "$relative" == "adl_pr_cycle/SKILL.md" ]]; then
    require_file_contains "$file" "historical compatibility|not a current workflow entrypoint|blocked by policy" "installed adl_pr_cycle historical-only wording"
    reject_file_contains "$file" "Purpose:[^.]{0,160}Route one tracked issue|Procedure:[^.]{0,240}Preflight[^.]{0,120}Init[^.]{0,120}Bind" "installed adl_pr_cycle executable route"
  fi
  reject_file_contains "$file" "v3[^.]{0,80}(is|becomes|remains)[^.]{0,80}(the )?(current|live|sole|final)[^.]{0,80}(lifecycle )?authority" "installed skill plain premature v3 authority claim"
  reject_file_contains "$file" "v3[^.]{0,80}(is|becomes|remains)[^.]{0,80}(publish|finish|clean|mutate)[^.]{0,80}live[^.]{0,80}lifecycle" "installed skill premature v3 lifecycle mutation authority"
  reject_file_contains "$file" "v3[^.]{0,80}(may|can|should)[^.]{0,80}(publish|finish|clean|mutate|bind)[^.]{0,80}(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)" "installed skill pre-cutover v3 lifecycle authority"
  reject_file_contains "$file" "(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)[^.]{0,80}v3[^.]{0,80}(may|can|should)[^.]{0,80}(publish|finish|clean|mutate|bind|advance|validate)" "installed skill pre-cutover v3 lifecycle authority with leading cutover clause"
  reject_file_contains "$file" "v3[^.]{0,80}(replaces|supersedes|retires)[^.]{0,80}v2[^.]{0,80}(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)" "installed skill pre-cutover v3 replacement authority"
  reject_file_contains "$file" "(before|prior to|ahead of)[^.]{0,80}(V3-F|#505|cutover)[^.]{0,80}v3[^.]{0,80}(replaces|supersedes|retires)[^.]{0,80}v2" "installed skill pre-cutover v3 replacement authority with leading cutover clause"
done

if (( failures > 0 )); then
  exit 1
fi

echo "skill guidance scan passed"
