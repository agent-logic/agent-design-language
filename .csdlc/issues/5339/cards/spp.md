# Structured Planning Prompt

Template: 1.0.0

Issue: 5339

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the six-primitives language boundary and COTS/budget decisions during preparation; after #5337 is merged and typed closed_out, construct the independent crate, implement strict parsing/schema/semantic validation/canonicalization, prove mapped corpus parity and budgets, then complete exact-revision review and typed publication.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Complete and review issue-specific cards, clean-room design, dependency boundary, protected paths, COTS decisions, and budgets without product implementation",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "After #5337 is merged and typed closed_out, create the independent adl-language crate and implement the six typed primitives plus strict YAML and JSON parsing",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement checked schemas, semantic validation, deterministic canonicalization, stable diagnostics, and focused positive and negative fixtures",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Map and prove the reviewed #5337 corpus, dependency and size budgets, warm/full latency, strict Clippy, exact-revision review, and publication readiness",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- The six primitives remain provider, tool, agent, task, workflow, and singular run
- Parsing and validation are pure and deterministic
- Unknown fields and unresolved references fail closed
- Canonicalization preserves meaning and never suppresses a characterized semantic difference
- The language crate does not gain compiler, runtime, provider, control-plane, or cloud authority
- No incumbent implementation or test code is reused in the clean-room crate
- No implementation before #5337 merged and typed closed_out

## Risks

- The characterization contract may expose compatibility behavior that conflicts with a minimal clean-room model
- Schema generation and deserialization may drift if their contracts are tested independently rather than jointly
- YAML coercion or duplicate-key behavior may create JSON/YAML inconsistency
- Semantic reference checks may accidentally absorb compiler-owned resolution behavior
- A per-crate provisional budget can become a target that pressures proof quality

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5339/design.md

Digest: 4ba65266b7f69d5c946b730ee2bd9c0a0b1b861f3ca87adb790a9018ba576913

## Diagram

.csdlc/prepared/issues/5339/diagram.mmd

Digest: 278ead07dcaa2e0ebd12c720e78f4efb6e1cf8f75982314af828d3dabb128cfa

## Stop Conditions

- #5337 is not both merged and typed closed_out when product implementation would begin
- The current #5337 corpus or normalization contract is missing, stale, or has unreviewed ambiguity
- Required behavior would copy or link incumbent ADL implementation
- A proposed dependency introduces runtime, network, control-plane, cloud, database, or provider authority
- Language semantics cannot be separated cleanly from #5338 compiler ownership
- Budget variance lacks evidence-backed review or would weaken required proof

## Handoff

Proceed only after doctor readiness.
