# Structured Task Prompt

Template: 1.0.0

Issue: 508

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #508 DRT-C only; produce the final distributed Runtime qualification decision after #507 terminal, without absorbing #509 or Observatory product redesign.

## Deliverables

- Final distributed Runtime qualification decision
- Failure, Observatory, soak, synthesis, and cleanup-zero evidence
- Issue-owned readiness and implementation validators
- Exact-head review receipt

## Acceptance

1. AC-1: Requirements 185 through 187 are mapped
2. AC-2: Identity, provider, and transport failures fail closed
3. AC-3: Observatory evidence is Runtime-authentic and redacted
4. AC-4: Soak, cleanup, and synthesis bind exact revisions

## Dependencies

- #507 terminal and ancestral: merge d022d6c198669bcbc10cd98bee4d7c8520f9c4d4, terminal digest 68e94be0e0f9addbc95dcf21c8579a35672ecf4c51b02fb5cc67b9c2c02f5328

## Inputs

- agent-logic/agent-design-language#508
- agent-logic/agent-design-language#507
- .git/csdlc-v2/derived-terminal/507.json
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#DRT-C
- docs/milestones/v0.92.1/evidence/runtime/drt-b/qualification-contract.json

## Non Goals

- Observatory product redesign
- Unbounded soak
- GCP portability qualification
- Replacing AWS qualification
