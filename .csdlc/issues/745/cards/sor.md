# Structured Output Record

Template: 1.0.0

Issue: 745

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Completed four Proposed ADR candidates, preserved Deferred ADR 0069, and mapped all eight v0.92.1 topics with source-bound evidence. Defect #744 and explicitly authorized typed v2 recovery are recorded. Independent review, publication, CI and terminal delivery follow separately.

## Artifacts

- docs/architecture/adr/0069-observatory-governed-runtime-consumer-boundary.md
- docs/architecture/adr/0072-csdlc-v3-native-authority.md
- docs/architecture/adr/0073-validated-configuration-snapshot-reload.md
- docs/architecture/adr/0074-runtime-generation-source-ownership.md
- docs/architecture/adr/0075-provider-profile-and-shadow-authority.md
- docs/architecture/adr/README.md
- docs/milestones/v0.92.1/ADR_PLAN_v0.92.1.md
- .csdlc/prepared/issues/745/source-manifest.json
- .csdlc/prepared/issues/745/v2-transition-recovery.md
- .csdlc/prepared/issues/745/validate-adrs.py

## Execution

- Added ADR candidates 0072-0075 without accepting new architecture.
- Updated Deferred ADR 0069 and mapped existing ADRs 0066/0070 with their promotion gates.
- Recorded eight topic dispositions, 41 source hashes and the bounded #744 recovery.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/745/validate-adrs.py"
    ],
    "purpose": "Validate eight-topic coverage, ADR statuses, source hashes and relative links after preserving inputs outside generated evidence.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/745/adr-docs.log"
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
