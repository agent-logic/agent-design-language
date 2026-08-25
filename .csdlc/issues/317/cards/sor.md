# Structured Output Record

Template: 1.0.0

Issue: 317

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated the #317 R1 findings by expanding the denominator to the complete 13-row Sprint 6 canonical universe, retaining and recomputing raw GitHub response digests, separating each reviewed revision from its later publication head, recording lifecycle/receipt/topology/cleanup truth per row, and expanding executable negative proof to 19 cases. No GitHub write, review, publication, merge, finish, cleanup, tag, release, close, or activation mutation occurred.

## Artifacts

- docs/milestones/v0.92/V092_TERMINAL_CLOSEOUT_PLAN_317.md
- .csdlc/evidence/317/issue-universe.json
- .csdlc/evidence/317/closeout-dag.json
- .csdlc/evidence/317/negative-cases.json
- .csdlc/evidence/317/github-observation-envelope.json
- .csdlc/evidence/317/github-raw
- .csdlc/prepared/issues/317/validate-closeout-plan.rb

## Execution

- Expanded the explicit legacy-to-canonical reconciliation from six tail rows to umbrella #307 plus children #308-#319.
- Bound every canonical row to tracked wave and Sprint 6 authority and to retained raw issue/PR response bytes with recomputed SHA-256.
- Recorded typed-record, phase, receipt, branch, worktree, cleanliness, cleanup, release-dependency, owner, and next-action truth for every row.
- Separated #316 reviewed revision d9aaaadbf5fc8e425e7099485fc006857feffd1e from publication head 8478f11e21a34530ba07bf64afc260e7a6eedd33 and proved review-head and merge ancestry.
- Expanded negative replay to missing receipt, active/dirty worktree, partial release identity, duplicate mutation, arbitrary envelope digest, non-ancestral review, and the original identity/proof/DAG cases.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/317/validate-closeout-plan.rb",
      "all"
    ],
    "purpose": "Validate the tracked-authority 13-row universe, recomputed raw provenance, reviewed-revision ancestry, acyclic merge gates, lifecycle truth, and 19 negative mutations.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/317"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
