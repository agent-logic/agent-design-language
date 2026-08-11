# Structured Task Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare and later implement only issue #112 Runtime authority, refusal, replay, redacted audit, and narrow pre-delivery API integration over the exact merged gated contracts.

## Deliverables

- Runtime-owned Layer 8 principal, authority, refusal, replay, and redacted audit module at the execution-planned source path
- Narrow Runtime module export and pre-delivery API integration at the execution-planned source paths
- Focused nonzero authority and Runtime API integration test targets required after #111
- Dedicated real-browser Observatory authority-state contract required after #111
- Feature contract for Layer 8 conversation authority under v0.92
- .csdlc/prepared/issues/112/design.md
- .csdlc/prepared/issues/112/diagram.mmd
- .csdlc/prepared/issues/112/validate-preparation.rb

## Acceptance

1. AC-1: Every governed action derives a stable Layer 8 principal only from authenticated, unexpired, non-revoked Runtime evidence bound to one Polis and credential generation.
2. AC-2: Discovery, contact, continuation, attachment, and exact multi-recipient actions require separate least-privilege capabilities intersected with current agent and Polis policy before sequence reservation or delivery.
3. AC-3: Identity expiry, rotation, revocation, stale capability epoch, malformed input, policy unavailability, replay ambiguity, and audit failure fail closed without fallback authority.
4. AC-4: Recipient substitution, recipient-set widening, implicit broadcast, action or conversation scope escalation, replay, and cross-Polis attempts are rejected atomically before provider execution.
5. AC-5: Operator, recipient-agent, reviewer, and public projections expose only audience-allowed decision, refusal, retry, correlation, recipient, conversation, and outcome fields.
6. AC-6: Restart restores replay and redacted audit integrity before admission; corrupt, truncated, reordered, discontinuous, or unwritable audit state remains unavailable.
7. AC-7: All three exact issue-owned authority, Runtime API integration, and real-browser Observatory targets pass at one reviewed implementation revision, with nonzero Rust test selection and no preparation-text substitution.

## Dependencies

- Hard serial gate: #111 must be closed by a merged PR and ancestral to the execution base

## Inputs

- AGENTS.md
- csdlc-v2/AGENTS.md
- docs/templates/prompts/current.json
- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/src/acip.rs
- adl-runtime/src/lib.rs
- adl/src/csm_runtime_api.rs
- Merged #111 conversation contract once terminal and ancestral

## Non Goals

- Implementing or mutating either gated issue, the umbrella issue, or any sibling work package
- Canonical session schemas, durable history or search, rooms, roster or presence, attention inbox, or final product hardening
- Constitutional governance, TLS, ACIP, identity, provider, transport, or general control redesign
- Browser-owned authorization, unrestricted operator actuation, private-policy exposure, or retention of forbidden content
- Product implementation, binding, push, PR, publication, merge, GitHub mutation, or closeout during this preparation run
