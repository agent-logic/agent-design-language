#!/usr/bin/env bash
set -euo pipefail

mode="${1:-all}"
case "${mode}" in
  membership|readiness|all) ;;
  *) echo "usage: $0 [membership|readiness|all]" >&2; exit 64 ;;
esac

packet="docs/milestones/v0.92.1/evidence/integration/sprint-10/sprint-execution-packet.md"
issues=(516 517 518 519 520 521 522 523 524 525 526)
scoped_issues=(523 524 525 526)

if [[ "${mode}" == "membership" || "${mode}" == "all" ]]; then
  test -f "${packet}"
  for issue in "${issues[@]}"; do
    rg -q "#${issue}" "${packet}"
  done
  rg -q '#516.*#517.*#518.*#519.*#520.*#521.*#522.*#523.*#524.*#525.*#526' "${packet}"
  for heading in \
    '## Child Issue Wave' \
    '## Recommended Execution Order' \
    '## Watcher Policy' \
    '## Budget And Goal Accounting' \
    '## Watcher Plan' \
    '## Sprint Closeout Rollup Expectations' \
    '## Review-remediation loop'; do
    rg -q -F "${heading}" "${packet}"
  done
  if rg -q 'monolithic.*#538|#538.*monolithic' "${packet}"; then
    echo "Sprint 10 packet must not authorize a monolithic #538 implementation PR" >&2
    exit 1
  fi
fi

if [[ "${mode}" == "readiness" || "${mode}" == "all" ]]; then
  missing=()
  not_ready=()
  for issue in "${scoped_issues[@]}"; do
    index=".csdlc/issues/${issue}/index.json"
    if [[ ! -f "${index}" ]]; then
      missing+=("${issue}")
      continue
    fi
    phase="$(jq -r '.phase' "${index}")"
    approved="$(jq -r '(.design_review | type) == "object" and (.design_review.approved.reviewer | type) == "string"' "${index}")"
    if [[ ! "${phase}" =~ ^(ready|bound|implemented|reviewed|published|merged|closed)$ ]] || [[ "${approved}" != "true" ]]; then
      not_ready+=("${issue}:${phase}:design-approved=${approved}")
    fi
  done
  if ((${#missing[@]} > 0 || ${#not_ready[@]} > 0)); then
    ((${#missing[@]} == 0)) || echo "missing typed issue records: ${missing[*]}" >&2
    ((${#not_ready[@]} == 0)) || echo "typed issue records not ready: ${not_ready[*]}" >&2
    exit 1
  fi
  absent_full_sprint=()
  for issue in "${issues[@]}"; do
    [[ -f ".csdlc/issues/${issue}/index.json" ]] || absent_full_sprint+=("${issue}")
  done
  printf '{"schema":"adl.sprint-readiness.v1","sprint_issue":538,"scope_status":"ready","full_sprint_status":"%s","scoped_issues":[' "$([[ ${#absent_full_sprint[@]} -eq 0 ]] && echo ready || echo incomplete)"
  separator=''
  for issue in "${scoped_issues[@]}"; do
    printf '%s%s' "${separator}" "${issue}"
    separator=','
  done
  printf '],"absent_full_sprint_records":['
  separator=''
  for issue in "${absent_full_sprint[@]}"; do
    printf '%s%s' "${separator}" "${issue}"
    separator=','
  done
  printf ']}\n'
fi
