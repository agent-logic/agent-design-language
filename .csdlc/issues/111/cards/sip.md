# Structured Intent Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide one versioned, provider-neutral, Runtime-owned human-agent conversation session contract through canonical ingress and egress.

## Required Outcome

A Layer 8 operator can start or resume a bounded one-to-one conversation with a policy-reachable agent, submit ordered correlated turns, and observe authoritative accepted, delivered, refused, timed-out, cancelled, and failed outcomes without browser- or provider-owned protocol authority.

## Scope

- Versioned conversation, turn, delivery, and response contracts
- Runtime-owned bounded session registry, ordered turns, idempotency, reconnect cursor, cancellation, timeout, and restart semantics
- Provider-neutral agent execution adapter boundary
- Authenticated Observatory client integration over the canonical #83 ingress and egress path
- Focused deterministic positive and negative proof

## Authority

- Runtime is the sole authority for sessions, ordering, outcomes, recipient eligibility, and public response correlation
- Live #110 authorizes the bounded #111 execution slice; #83 remains asynchronous decomposition reconciliation and is not an execution gate
- The browser is a non-authoritative authenticated client and never signs agent work or synthesizes delivery or agent responses
- Provider payloads, credentials, private cognition, and private agent state remain outside public contracts
- Downstream #112 consumes #111 later and supplies no authority or dependency to this issue
- Issue #83, umbrella #110, deferred #122, and tooling issue #213 must not be mutated by this execution

## Assumptions

- none

## Operator Constraints

- Work only on issue #111 in the dedicated FastWork preparation worktree
- Use only C-SDLC v2 owner binaries and typed card operations for lifecycle truth
- Leave root main and unrelated issue #143 artifacts untouched
- Do not mutate issue #83 or any other WP-18C child
- Stop before binding while live serial dependencies or issue-graph ambiguity remain
