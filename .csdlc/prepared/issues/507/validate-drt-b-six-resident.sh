#!/usr/bin/env bash
set -euo pipefail

lane="${1:-}"
case "${lane}" in
  --lane=prebind|--lane=six-resident-uts|--lane=continuity-reclamation)
    ;;
  *)
    echo "usage: $0 --lane=<prebind|six-resident-uts|continuity-reclamation>" >&2
    exit 64
    ;;
esac

repo_root="$(git rev-parse --show-toplevel)"
git_common_dir="$(git rev-parse --git-common-dir)"
issue_dir="${repo_root}/.csdlc/issues/507"
prepared_dir="${repo_root}/.csdlc/prepared/issues/507"

test -f "${issue_dir}/index.json"
test -f "${prepared_dir}/design.md"
test -f "${prepared_dir}/diagram.mmd"
test -f "${git_common_dir}/csdlc-v2/derived-terminal/506.json"
if ! test -f "${git_common_dir}/csdlc-v2/derived-terminal/345.json"; then
    state="$(gh api repos/agent-logic/agent-design-language/issues/345 --jq '.state')"
    test "${state}" = "closed"
fi

grep -q "six-resident UTS qualification" "${prepared_dir}/design.md"
grep -q "DRT-B" "${prepared_dir}/design.md"
grep -q "#508" "${prepared_dir}/design.md"
grep -q "#509" "${prepared_dir}/design.md"
grep -q "paid/GPU" "${prepared_dir}/design.md"

case "${lane}" in
  --lane=prebind)
    grep -q '"phase":"initialized"' "${issue_dir}/index.json" || grep -q '"phase": "initialized"' "${issue_dir}/index.json"
    ;;
  --lane=six-resident-uts)
    contract="${repo_root}/docs/milestones/v0.92.1/evidence/runtime/drt-b/qualification-contract.json"
    test -f "${contract}"
    jq -e '
      .resident_count == 6
      and (.residents | type == "array" and length == 6)
      and ([.residents[].resident_id] | unique | length == 6)
      and ([.residents[].workload_receipt_id] | unique | length == 6)
      and all(.residents[];
        (.resident_id | type == "string" and length > 0)
        and (.workload_receipt_id | type == "string" and length > 0)
        and (.lineage_digest | type == "string" and length > 0)
        and ((.replay_cursor | type == "string") or (.replay_cursor | type == "number"))
      )
    ' "${contract}" >/dev/null
    ;;
  --lane=continuity-reclamation)
    contract="${repo_root}/docs/milestones/v0.92.1/evidence/runtime/drt-b/qualification-contract.json"
    test -f "${contract}"
    jq -e '
      .dehydrate_restore == "exact"
      and .cleanup_zero == true
      and (.resource_envelope | type == "object")
      and (.cleanup_selectors | type == "array" and length > 0)
      and (.negative_matrix | type == "array")
      and (([.negative_matrix[].case] | index("duplicate_resident_identity")) != null)
      and (([.negative_matrix[].case] | index("missing_workload_receipt")) != null)
      and (([.negative_matrix[].case] | index("mutated_lineage")) != null)
      and (([.negative_matrix[].case] | index("replay_cursor_regression")) != null)
      and (([.negative_matrix[].case] | index("cleanup_selector_mismatch")) != null)
      and all(.negative_matrix[]; .decision == "fail_closed")
    ' "${contract}" >/dev/null
    ;;
esac

echo "DRT-B validator PASS ${lane#--lane=}"
