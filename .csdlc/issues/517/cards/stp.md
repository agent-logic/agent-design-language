# Structured Task Prompt

Template: 1.0.0

Issue: 517

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Deliver only issue #517: Quality gate.

## Deliverables

- Every required proving lane passes for the exact candidate and the gate reports zero unowned exceptions.
- Issue-specific retained validation evidence
- Independent exact-head review and truthful terminal record

## Acceptance

1. AC-1: The exact required quality denominator is complete for the admitted candidate.
2. AC-2: Every required proving lane passes with a nonzero exact denominator.
3. AC-3: Skipped, missing, stale, filtered-to-zero, non-proving, or ambiguous results deny the gate.
4. AC-4: The decision reports zero unowned exceptions.
5. AC-5: Exact-head review has no unresolved actionable findings.

## Dependencies

- Terminal reviewed and ancestral INT-01 issue #516
- Sprint 9 umbrella #537

## Inputs

- docs/milestones/v0.92.1/evidence/release/tail-01
- docs/milestones/v0.92.1/QUALITY_GATE_v0.92.1.md
- .csdlc/prepared/issues/517/validate-quality-gate.rb
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- .csdlc/prepared/issues/537/sprint-execution-packet.yaml

## Non Goals

- Documentation repair
- Release ceremony
- Implementing failed-lane fixes
