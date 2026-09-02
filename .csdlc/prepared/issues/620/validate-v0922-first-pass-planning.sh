#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
root="$(git rev-parse --show-toplevel)"
cd "$root"

required=(
  docs/milestones/v0.92.2/README.md
  docs/milestones/v0.92.2/VISION_v0.92.2.md
  docs/milestones/v0.92.2/DESIGN_v0.92.2.md
  docs/milestones/v0.92.2/DECISIONS_v0.92.2.md
  docs/milestones/v0.92.2/WBS_v0.92.2.md
  docs/milestones/v0.92.2/SPRINT_v0.92.2.md
  docs/milestones/v0.92.2/PLANNED_ISSUE_CATALOG_v0.92.2.md
  docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml
  docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml
  docs/milestones/v0.92.2/CANONICAL_DOC_INVENTORY_v0.92.2.md
)

structure() {
  for path in "${required[@]}"; do test -f "$path" || { echo "missing: $path" >&2; return 1; }; done
  ruby -e 'require "yaml"; ARGV.each { |p| YAML.safe_load(File.read(p), aliases: true) }' \
    docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml \
    docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml
}

scheduling() {
  test -f .adl/docs/TBD/TBD_DOC_STATUS_INVENTORY.md
  test -f .adl/docs/TBD/LOCAL_BACKLOG.md
  test -f .adl/docs/TBD/planning/NEW_FEATURE_MILESTONE_ASSIGNMENT_PLAN.md
  if test -f docs/milestones/v0.92.2/TBD_SCHEDULING_RECONCILIATION_v0.92.2.md; then
    rg -q 'Source|source' docs/milestones/v0.92.2/TBD_SCHEDULING_RECONCILIATION_v0.92.2.md
    rg -q 'Disposition|disposition' docs/milestones/v0.92.2/TBD_SCHEDULING_RECONCILIATION_v0.92.2.md
    rg -q 'Unresolved|unresolved' docs/milestones/v0.92.2/TBD_SCHEDULING_RECONCILIATION_v0.92.2.md
  fi
}

consistency() {
  if rg -n '/Users/|/Volumes/|<this-issue>|TBD-ISSUE|TODO-ISSUE' docs/milestones/v0.92.2; then
    echo "machine-local path or unresolved issue placeholder found" >&2
    return 1
  fi
  git diff --check
}

case "$mode" in
  structure) structure ;;
  scheduling) scheduling ;;
  consistency) consistency ;;
  all) structure; scheduling; consistency ;;
  *) echo "usage: $0 {structure|scheduling|consistency|all}" >&2; exit 2 ;;
esac
