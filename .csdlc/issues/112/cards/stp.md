# Structured Task Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare and later implement only issue #112 Runtime authority, refusal, replay, redacted audit, and narrow pre-delivery API integration over the exact merged gated contracts.

## Deliverables

- Canonical issue identity/title: [v0.92][WP-18C.02a][112.a] Define shared Layer 8 signed authority core
- Runtime-kernel Layer 8 authority module
- Signed identity-message contract with credential-generation binding
- Recipient acknowledgement verification primitives
- Redacted hash-chained audit store
- Focused adversarial and direct exchange tests
- adl-runtime-kernel/tests/layer8_authority.rs

## Acceptance

1. AC-1: Stable principals derive only from authenticated, unexpired, non-revoked runtime identity evidence with current credential generation.
2. AC-2: Core authority decisions require least-privilege capability, current agent policy, and current Polis policy intersection before a grant is returned.
3. AC-3: Expiry, rotation or stale generation, revocation, stale capability epoch, policy unavailability, malformed input, replay ambiguity, invalid signatures, non-canonical payloads, and audit failure fail closed.
4. AC-4: Recipient substitution, recipient-set widening, implicit broadcast, action or conversation scope escalation, replay, and cross-Polis attempts are rejected by the core before a grant.
5. AC-5: Public refusals and audit records expose only bounded decision, refusal, correlation, principal, recipient, conversation, and outcome fields and omit secrets, private cognition, raw provider payloads, private signing keys, and message content.
6. AC-6: Restart restores replay and redacted audit integrity before core admission; corrupt, truncated, reordered, discontinuous, or unwritable audit state remains unavailable.
7. AC-7: Human-agent and direct agent-agent requests share one canonical signed identity-message contract with externally held per-principal keys, credential-generation binding, and exact recipient binding.
8. AC-8: Recipient acknowledgement verification primitives bind sender, recipient, conversation, correlation, causation, replay identity, expiry, and triggering message; signature substitution, stale rotation, revocation, and expiry fail closed.
9. AC-9: Focused runtime-kernel authority-core tests, formatting, clippy, and diff hygiene pass at the exact implementation revision with nonzero test selection.

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
