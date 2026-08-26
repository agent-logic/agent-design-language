# Structured Review Prompt

Template: 1.0.0

Issue: 365

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/365
.csdlc/prepared/issues/365
.csdlc/evidence/365
adl-runtime/src/distributed/shepherd_serving_eligibility.rs
adl-runtime/src/distributed/observatory_serving_eligibility.rs
adl-runtime/tests/distributed_shepherd_serving_eligibility.rs
adl-runtime/tests/distributed_observatory_serving_eligibility.rs

## Prompts

- Can any caller fabricate or convert a public projection into the opaque type?
- Does private construction recompute receipt state payload generation/index and child-kind binding from store-owned truth?
- Do reopen corruption rollback and A/B tests use real persistence boundaries?
- Are getters/canonical bytes fully redacted?
- Are existing transition policies and exact four-path ownership unchanged?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI, publication, merge, terminal cache, and ancestry remain later typed gates.

## Review Result

Revision: Some("git-blake3:e8d3c6a08e7586a2c4bae6f0d996720abfe15afd:3ef477e631427c4c54068c7daa319f3485d17f024850c1131ee2fa9398ed6d37")

Reviewer: Some("fresh-session:e7143fdd-d9fc-47ef-b82d-859b8426fc39")

Result: pass
