# Structured Task Prompt

Template: 1.0.0

Issue: 365

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only opaque store-derived committed-projection seams and their exact focused proof without changing eligibility policy or exposing raw authority.

## Deliverables

- adl-runtime/src/distributed/shepherd_serving_eligibility.rs
- adl-runtime/src/distributed/observatory_serving_eligibility.rs
- adl-runtime/tests/distributed_shepherd_serving_eligibility.rs
- adl-runtime/tests/distributed_observatory_serving_eligibility.rs
- .csdlc/prepared/issues/365/design.md
- .csdlc/prepared/issues/365/diagram.mmd
- .csdlc/prepared/issues/365/validate_preparation.py
- .csdlc/issues/365
- .csdlc/evidence/365

## Acceptance

1. AC-1: Authentic committed #273 and #274 stores return opaque sealed read-only inputs usable by #275.
2. AC-2: Public projection fields raw DTOs and caller-supplied digests cannot construct or convert into sealed inputs.
3. AC-3: State receipt child kind durable payload checkpoint generation/index and redacted projection truth share one canonical provenance binding and are revalidated.
4. AC-4: A/B substitution malformed/corrupt checkpoint or provenance stale generation/index and fabricated public projections fail closed.
5. AC-5: Restart/reopen preserves exact sealed bytes and public bytes remain fully redacted.
6. AC-6: Acquire replace renew transfer revoke and expiry policy behavior remains unchanged and existing denominators stay green.
7. AC-7: Exact four-path scope strict Clippy exact review hosted CI typed finish cache and ancestry pass before #275 resumes.

## Dependencies

- #272 terminal cache canonical and ancestral
- #273 terminal cache canonical and ancestral
- #274 terminal cache canonical and ancestral
- Blocks #275 and is part of #205 serialized wave

## Inputs

- agent-logic/agent-design-language#365
- terminal #273 Shepherd module and focused test
- terminal #274 Observatory module and focused test
- CheckpointedJson durable envelope/checkpoint authority contract as read-only input

## Non Goals

- New authority issuance policy state transition or verifier bypass
- Public constructor raw projection ingestion raw state or receipt exposure
- Any #275 #205 mod.rs authority protocol serving_authority adapter listener transport UI migration cloud or provider change
