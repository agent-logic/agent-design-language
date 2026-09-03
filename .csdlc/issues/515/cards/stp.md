# Structured Task Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one non-authoritative shadow-execution path; comparison cases are proof inputs.

## Deliverables

- Bounded local-model shadow-execution path
- Deterministic authority-versus-shadow comparison input and rule set
- Fallback behavior that preserves authoritative results on shadow failure
- Redacted evidence under docs/milestones/v0.92.1/evidence/provider/prov-b/
- adl/tests/provider_shadow_isolation.rs
- adl/tests/provider_shadow_comparison.rs
- adl/tests/provider_shadow_fallback.rs
- .csdlc/prepared/issues/515/validate-provider-shadow-readiness.sh
- .csdlc/prepared/issues/515/validate-provider-shadow-redaction.sh
- docs/milestones/v0.92.1/evidence/provider/prov-b

## Acceptance

1. AC-1: Shadow and authority paths are distinguishable
2. AC-2: Inputs and comparison rules are exact
3. AC-3: Failures preserve the authoritative result
4. AC-4: Evidence is redacted

## Dependencies

- PROV-A/#514 closed and merged before execution

## Inputs

- agent-logic/agent-design-language#515
- agent-logic/agent-design-language#514
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#PROV-B
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#PROV-B
- adl/src/provider/**

## Non Goals

- Provider benchmark marketing claims
- Production cutover
- Authority-path replacement
- Live paid cloud or provider execution
