#!/usr/bin/env bash
set -euo pipefail

manifest="${1:-.csdlc/prepared/issues/695/acceptance-manifest.json}"
results="${2:-.csdlc/evidence/695/acceptance-results.json}"

test -f "$results"
implementation_digest="$({
  jq -r '.implementation_paths[]' "$results" | while IFS= read -r file_path; do
    test -f "$file_path"
    printf '%s %s\n' "$file_path" "$(git hash-object "$file_path")"
  done
} | git hash-object --stdin)"
jq -e '
  .schema == "adl.runtime.agent-partial-checkpoint.acceptance.v1" and
  .issue == 695 and
  .zero_test_policy == "reject" and
  ([.rows[].id] | sort) == ([range(1; 11) | "AC-\(.)"] | sort) and
  all(.rows[]; (.proof | length) > 0 and all(.proof[]; length > 0))
' "$manifest" >/dev/null

jq -e --arg implementation_digest "$implementation_digest" --slurpfile manifest "$manifest" '
  . as $result_doc |
  ($result_doc.schema == "adl.runtime.agent-partial-checkpoint.results.v1" and
   $result_doc.issue == 695 and
   $result_doc.evidence_binding.schema == "adl.runtime.agent-partial-checkpoint.content-digest.v1" and
   $result_doc.evidence_binding.implementation_digest == $implementation_digest and
   ($result_doc.implementation_paths | length) > 0 and
   ([$result_doc.results[].proof_id] | length) == ([$result_doc.results[].proof_id] | unique | length) and
   all($manifest[0].rows[];
     . as $row |
     all($row.proof[];
       . as $proof |
       any($result_doc.results[];
         .proof_id == $proof and
         .outcome == "passed" and
         ((.test_count // 0) + (.assertion_count // 0)) > 0))))
' "$results" >/dev/null
