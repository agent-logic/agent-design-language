# Structured Task Prompt

Template: 1.0.0

Issue: 356

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add minimal accessors and focused proof only; no Observatory eligibility state machine or predecessor redesign.

## Deliverables

- adl-runtime/src/distributed/serving_authority.rs
- adl-runtime/tests/distributed_observatory_authority_projection.rs
- .csdlc/evidence/356
- .csdlc/issues/356

## Acceptance

1. AC-1: Borrowed/copy accessors expose exactly the existing redacted projection fields required by #274.
2. AC-2: No constructor, mutation, raw quorum/membership, OwnerCommit, lease, artifact, token, signature, key, endpoint, path, or provider data becomes accessible.
3. AC-3: A/A accessor values match sealed verification and A/B substitution fails before projection creation.
4. AC-4: Debug/serialization proof remains redacted.
5. AC-5: Focused tests, strict Clippy, diff hygiene, exact-head review, hosted CI, and typed finish pass.

## Dependencies

- #350 terminal cache canonical and merge ancestral
- Blocks #274 until terminal and ancestral

## Inputs

- agent-logic/agent-design-language#356
- adl-runtime/src/distributed/serving_authority.rs at terminal #350
- adl-runtime/tests/distributed_observatory_authority_projection.rs at terminal #350

## Non Goals

- #274 state machine
- #273, #272, #203, #205, or #275 behavior
- UI, listener, transport, cloud, provider, deployment
