# Structured Output Record

Template: 1.0.0

Issue: 513

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the DEC-01 Runtime v2/v3 authority-separation topology without Runtime v4 scope or sibling Sprint 1 changes.

## Artifacts

- docs/runtime/runtime-v2-v3-authority-topology.md
- docs/milestones/v0.92.1/evidence/runtime-decoupling/runtime-authority-topology.json
- docs/milestones/v0.92.1/evidence/runtime-decoupling/validate-runtime-authority-topology.sh
- .csdlc/prepared/issues/513
- .csdlc/issues/513

## Execution

- Added a repository-local Runtime v2/v3 authority topology document for v0.92.1 DEC-01.
- Added a machine-readable topology manifest that assigns Runtime v2, Runtime v3 guardian, Runtime v3 kernel, support-surface, documentation, and issue-lifecycle ownership without authority transfer.
- Added an executable validator for source-root existence, Runtime v4 exclusion, reverse-reference classification, compatibility behavior, migration dry-run, and rollback dry-run.
- Kept Runtime v2 source authoritative under adl/src/runtime_v2/** and Runtime v3 authority under adl-runtime/** plus adl-runtime-kernel/**; Runtime v4 remains excluded.

## Validation

[
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/runtime-decoupling/validate-runtime-authority-topology.sh"
    ],
    "purpose": "Run the focused DEC-01 executable validator against the repository-local topology contract.",
    "outcome": "passed",
    "evidence_ref": "dec-01-runtime-authority-topology.log"
  },
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/runtime-decoupling/validate-runtime-authority-topology.sh"
    ],
    "purpose": "Prove the DEC-01 topology after replacing first-match reverse-reference classification with longest-prefix ambiguity detection and non-overlapping support-surface rows.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/513/dec-01-runtime-authority-topology-p1-rerun.log"
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
