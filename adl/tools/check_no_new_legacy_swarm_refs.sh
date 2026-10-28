#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
cd "$ROOT_DIR"

base_ref=""
if [[ $# -ge 1 && -n "${1:-}" ]]; then
  base_ref="$1"
elif [[ -n "${GITHUB_BASE_REF:-}" ]]; then
  candidate="origin/${GITHUB_BASE_REF}"
  if git rev-parse --verify "$candidate" >/dev/null 2>&1; then
    base_ref="$candidate"
  else
    git fetch --no-tags --depth=1 origin "$GITHUB_BASE_REF" >/dev/null 2>&1 || true
    if git rev-parse --verify "$candidate" >/dev/null 2>&1; then
      base_ref="$candidate"
    else
      base_ref="FETCH_HEAD"
    fi
  fi
elif git rev-parse --verify origin/main >/dev/null 2>&1; then
  base_ref="origin/main"
else
  base_ref="$(git rev-parse HEAD~1)"
fi

# A reviewed root-history transition has no merge base. Compare its source trees
# explicitly; a failed Git observation must never masquerade as an empty diff.
if git merge-base "$base_ref" HEAD >/dev/null 2>&1; then
  range_spec="${base_ref}...HEAD"
else
  range_spec="${base_ref}..HEAD"
fi
changed_files="$(git diff --name-only --diff-filter=AMRT "$range_spec")"

echo "Guardrail base ref: ${base_ref}"
echo "Guardrail diff range: ${range_spec}"

# Allow explicit legacy-name references only in bounded historical or migration-planning surfaces.
allowlist_regex='^(docs/milestones/v0\.[0-6]/|docs/milestones/v0\.7/SWARM_NAME_CHANGE_PLANNING_v0\.7\.md$|docs/milestones/v0\.85/README\.md$|docs/milestones/v0\.85/SWARM_REMOVAL_PLANNING\.md$|docs/milestones/v0\.85/DECISIONS_v0\.85\.md$|docs/milestones/v0\.85/DESIGN_v0\.85\.md$|docs/milestones/v0\.85/MILESTONE_CHECKLIST_v0\.85\.md$|docs/milestones/v0\.85/RELEASE_PLAN_v0\.85\.md$|docs/milestones/v0\.85/SPRINT_v0\.85\.md$|docs/milestones/v0\.85/WBS_v0\.85\.md$|docs/records/v0\.87\.1/tasks/issue-1477/sor\.md$|adl/tools/check_no_new_legacy_swarm_refs\.sh$|adl/tools/check_release_notes_commands\.sh$)'
# RD02 retains exact historical source paths for the repository split audit.
# Allow only this frozen inventory, not new operational legacy-name references.
allowlist_regex="${allowlist_regex%)}|docs/milestones/v0\.93\.1/repository-decomposition/rd02-1182/mapping-inputs/docs-dispositions\.json$)"
# The naming guard's negative fixture deliberately adds the forbidden name.
allowlist_regex="${allowlist_regex%)}|tools/ci_contracts/tests/ci\.rs$)"
legacy_regex='(^|[^A-Za-z0-9_])(swarm-remote|SWARM_[A-Z0-9_]+)([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])swarm([^:/A-Za-z0-9_]|$)'

violations=0

while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  if [[ "$file" =~ $allowlist_regex ]]; then
    continue
  fi

  added_lines="$(git diff -U0 "$range_spec" -- "$file" | awk '/^\+[^+]/ {print substr($0,2)}')"
  [[ -z "$added_lines" ]] && continue

  if grep -En "$legacy_regex" <<<"$added_lines" >/tmp/adl_legacy_ref_hits.txt; then
    violations=1
    echo "Legacy naming additions are not allowed in: $file"
    sed 's/^/  /' /tmp/adl_legacy_ref_hits.txt
  fi
done <<<"$changed_files"

if [[ "$violations" -ne 0 ]]; then
  echo
  echo "Guardrail failed: found new legacy-name references outside allowlist."
  echo "If intentional, add a scoped allowlist entry in this script with rationale."
  exit 1
fi

echo "Guardrail passed: no new unintended legacy-name references."
