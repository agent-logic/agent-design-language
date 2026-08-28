# Structured Task Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Deliver only issue #515: Local-model shadow execution.

## Deliverables

- Shadow execution is distinguishable, deterministic, redacted, and unable to mutate or replace the authoritative provider result.
- Issue-specific retained validation evidence
- Independent exact-head review and truthful terminal record

## Acceptance

1. AC-1: Shadow and authoritative provider paths are unambiguously distinguishable.
2. AC-2: Inputs and comparison rules are exact and deterministic.
3. AC-3: Shadow failures cannot mutate or replace the authoritative result.
4. AC-4: Retained comparison evidence is redacted and source-revision bound.
5. AC-5: Exact-head review has no unresolved actionable findings.

## Dependencies

- Terminal reviewed and ancestral PROV-A issue #514
- Sprint 9 umbrella #537

## Inputs

- adl/src/provider
- docs/milestones/v0.92.1/evidence/provider/prov-b
- .csdlc/prepared/issues/515/validate-provider-shadow.rb
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- .csdlc/prepared/issues/537/sprint-execution-packet.yaml

## Non Goals

- Provider benchmark marketing claims
- Production provider cutover
- Changing provider authority
