#!/usr/bin/env bash
set -euo pipefail

manifest="${1:-.csdlc/prepared/issues/695/acceptance-manifest.json}"
results="${2:-.csdlc/evidence/695/acceptance-results.json}"

test -f "$results"
head_sha="$(git rev-parse HEAD)"
jq -e '
  .schema == "adl.runtime.agent-partial-checkpoint.acceptance.v1" and
  .issue == 695 and
  .zero_test_policy == "reject" and
  ([.rows[].id] | sort) == ([range(1; 11) | "AC-\(.)"] | sort) and
  all(.rows[]; (.proof | length) > 0 and all(.proof[]; length > 0))
' "$manifest" >/dev/null

jq -e --arg head "$head_sha" --slurpfile manifest "$manifest" '
  .schema == "adl.runtime.agent-partial-checkpoint.results.v1" and
  .issue == 695 and
  .head_sha == $head and
  ([.results[].proof_id] | length) == ([.results[].proof_id] | unique | length) and
  all($manifest[0].rows[] as $row;
    all($row.proof[] as $proof;
      any(.results[];
        .proof_id == $proof and
        .outcome == "passed" and
        ((.test_count // 0) + (.assertion_count // 0)) > 0
      )
    )
  )
' "$results" >/dev/null
