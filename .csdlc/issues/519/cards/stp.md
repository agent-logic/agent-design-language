# Structured Task Prompt

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Deliver only issue #519: Publication finalization.

## Deliverables

- The packet binds the exact reviewed candidate, correct closing relationships, publication linkage, and redacted artifacts while leaving merge, tag, release, and external publication untouched.
- Issue-specific retained validation evidence
- Independent exact-head review and truthful terminal record

## Acceptance

1. AC-1: The publication-candidate packet records the exact reviewed revision and artifact denominator.
2. AC-2: Issue and pull-request linkage, including closing relationships, is correct and unambiguous.
3. AC-3: Publication artifacts are redacted and contain no private paths, credentials, or unsupported claims.
4. AC-4: Stale review, ambiguous linkage, missing artifacts, or digest mismatch denies candidate readiness.
5. AC-5: Exact-head review has no unresolved actionable findings.

## Dependencies

- Terminal reviewed and ancestral TAIL-02 issue #518
- Sprint 9 umbrella #537

## Inputs

- docs/milestones/v0.92.1/evidence/release/tail-03
- .csdlc/prepared/issues/519/validate-publication-candidate.rb
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- .csdlc/prepared/issues/537/sprint-execution-packet.yaml

## Non Goals

- Merge
- Tag
- Release
- External publication
- Release ceremony
