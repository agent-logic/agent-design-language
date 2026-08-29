#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

failures=0

require_file_contains() {
  local file="$1"
  local pattern="$2"
  local label="$3"
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
  local content
  content="$(tr '\n' ' ' < "$file")"
  if grep -Eiq "$pattern" <<<"$content"; then
    echo "stale-current-route: $label in $file" >&2
    failures=$((failures + 1))
  fi
}

reject_current_route_lines() {
  local file="$1"
  local pattern="$2"
  local label="$3"
  local matches
  matches="$(
    grep -Ein "$pattern" "$file" \
      | grep -Eiv 'historical|former|retired|not a current|not executable guidance|blocked by policy|do not|refuse|evidence only|not executable|not a runnable current procedure' \
      || true
  )"
  if [[ -n "$matches" ]]; then
    echo "stale-current-route: $label in $file" >&2
    echo "$matches" >&2
    failures=$((failures + 1))
  fi
}

docs=(
  "AGENTS.md"
  "docs/onboarding.md"
  "docs/architecture/ADL_ARCHITECTURE.md"
  "docs/tooling/adl_pr_cycle_skill.md"
  "csdlc-v3/README.md"
  "csdlc-v3/AGENTS.md"
)

for file in "${docs[@]}"; do
  if [[ ! -f "$file" ]]; then
    echo "missing file: $file" >&2
    failures=$((failures + 1))
    continue
  fi
  require_file_contains "$file" "v2.*(live|sole|current).*authority|authority.*v2" "v2 live/current authority"
  require_file_contains "$file" "v3.*(construction|non-authoritative|non-authority)|construction.*v3" "v3 construction non-authority"
done

require_file_contains "AGENTS.md" "three[- ]minute|3[- ]minute|3 m" "three-minute prepared issue start target"
require_file_contains "docs/onboarding.md" "three[- ]minute|3[- ]minute|3 m" "three-minute prepared issue start target"
require_file_contains "csdlc-v3/README.md" "V3-A|V3-B|V3-C" "V3-A/B/C construction state"
require_file_contains "csdlc-v3/README.md" "V3-F|#505" "V3-F/#505 cutover boundary"
require_file_contains "docs/architecture/ADL_ARCHITECTURE.md" "pr init.*historical|historical.*pr init" "architecture legacy pr-route historical wording"
require_file_contains "docs/tooling/adl_pr_cycle_skill.md" "historical compatibility|not a current workflow entrypoint|blocked by policy" "adl_pr_cycle historical-only wording"

current_route_pattern='(^|[^`[:alnum:]_-])(adl_pr_cycle|pr\.sh|pr init|pr ready|pr run|pr finish|pr janitor|pr closeout|pr preflight)([^`[:alnum:]_-]|$).*(current|normal|use|invoke|run|route|default)|default.*(^|[^`[:alnum:]_-])(adl_pr_cycle|pr\.sh|pr init|pr ready|pr run|pr finish|pr janitor|pr closeout|pr preflight)([^`[:alnum:]_-]|$)'
for file in "${docs[@]}"; do
  reject_current_route_lines "$file" "$current_route_pattern" "legacy wrapper presented as current route"
done

reject_file_contains "AGENTS.md" "v3[^.]{0,80}(is|becomes|remains)[^.]{0,80}(sole|current|live)[^.]{0,80}authority" "premature v3 authority"
reject_file_contains "docs/onboarding.md" "v3[^.]{0,80}(is|becomes|remains)[^.]{0,80}(sole|current|live)[^.]{0,80}authority" "premature v3 authority"

if (( failures > 0 )); then
  exit 1
fi

echo "docs route scan passed"
