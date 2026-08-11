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

- Runtime is the sole authority for sessions, ordering, outcomes, and public response correlation
- Existing #83 and #112 identity and policy boundaries decide operator and recipient reachability; #111 does not widen authority
- The browser is a non-authoritative client and never synthesizes delivery or agent responses
- Provider payloads, credentials, private cognition, and private agent state remain outside public contracts
- No product implementation, binding, publication, push, PR, merge, close, or #83/#110 mutation is authorized during preparation

## Assumptions

- none

## Operator Constraints

- Work only on issue #111 in the dedicated FastWork preparation worktree
- Use only C-SDLC v2 owner binaries and typed card operations for lifecycle truth
- Leave root main and unrelated issue #143 artifacts untouched
- Do not mutate issue #83 or any other WP-18C child
- Stop before binding while live serial dependencies or issue-graph ambiguity remain
