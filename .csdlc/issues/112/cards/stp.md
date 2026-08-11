# Structured Task Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare and later implement only issue #112 Runtime authority, refusal, replay, redacted audit, and narrow pre-delivery API integration over the exact merged gated contracts.

## Deliverables

- adl-runtime/src/layer8_authority.rs
- adl-runtime/src/lib.rs
- adl-runtime/tests/layer8_authority.rs
- adl/src/csm_runtime_api.rs
- adl/tests/layer8_authority_runtime_api.rs
- adl/tools/validate_layer8_authority_observatory_ui.sh
- docs/milestones/v0.92/features/LAYER8_CONVERSATION_AUTHORITY.md
- .csdlc/prepared/issues/112/design.md
- .csdlc/prepared/issues/112/diagram.mmd

## Acceptance

1. AC-1: Every governed action derives a stable Layer 8 principal only from authenticated, unexpired, non-revoked Runtime evidence bound to one Polis and credential generation.
2. AC-2: Discovery, contact, continuation, attachment, and exact multi-recipient actions require separate least-privilege capabilities intersected with current agent and Polis policy before sequence reservation or delivery.
3. AC-3: Identity expiry, rotation, revocation, stale capability epoch, malformed input, policy unavailability, replay ambiguity, and audit failure fail closed without fallback authority.
4. AC-4: Recipient substitution, recipient-set widening, implicit broadcast, action or conversation scope escalation, replay, and cross-Polis attempts are rejected atomically before provider execution.
5. AC-5: Operator, recipient-agent, reviewer, and public projections expose only audience-allowed decision, refusal, retry, correlation, recipient, conversation, and outcome fields.
6. AC-6: Restart restores replay and redacted audit integrity before admission; corrupt, truncated, reordered, discontinuous, or unwritable audit state remains unavailable.
7. AC-7: The exact issue-owned Layer 8 authority and Runtime API integration test targets each select and pass nonzero tests at one reviewed implementation revision; preparation text is never accepted as product proof.

## Dependencies

- Hard serial gate: #83 must be closed by a merged PR and ancestral to the execution base
- Hard serial gate: #111 must be closed by a merged PR and ancestral to the execution base

## Inputs

- AGENTS.md
- csdlc-v2/AGENTS.md
- docs/templates/prompts/current.json
- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/src/acip.rs
- adl-runtime/src/lib.rs
- adl/src/csm_runtime_api.rs
- Merged gated contracts when both serial gates become terminal and ancestral

## Non Goals

- Implementing or mutating either gated issue, the umbrella issue, or any sibling work package
- Canonical session schemas, durable history or search, rooms, roster or presence, attention inbox, or final product hardening
- Constitutional governance, TLS, ACIP, identity, provider, transport, or general control redesign
- Browser-owned authorization, unrestricted operator actuation, private-policy exposure, or retention of forbidden content
- Product implementation, binding, push, PR, publication, merge, GitHub mutation, or closeout during this preparation run
