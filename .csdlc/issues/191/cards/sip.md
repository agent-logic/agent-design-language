# Structured Intent Prompt

Template: 1.0.0

Issue: 191

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Deliver the production three-voter consensus substrate required by #142 using mutually authenticated encrypted transport and crash-safe durable OpenRaft storage.

## Required Outcome

Three real voters elect, commit, snapshot and restart only through domain-bound Quinn/rustls RPCs; storage failure, replay, corruption and one-of-three topology fail closed without live state moving ahead of durable truth.

## Scope

- Quinn/rustls OpenRaft network adapter with exact polis, trust-domain, node and certificate binding
- Crash-safe durable OpenRaft log, state-machine, vote, replay and snapshot storage
- Three-voter transport/storage integration and adversarial tests
- Issue-owned exact proof and independent review

## Authority

- This issue transports and durably applies already-typed consensus data; it does not invent lease, membership, fencing, activation, migration or recovery authority.
- Every voter has a unique non-exported transport identity and private key; signed envelopes do not substitute for encryption.
- A failed durable write cannot leave accepted in-memory state ahead of recoverable disk state.
- One reachable voter of three cannot commit or apply a client mutation.

## Assumptions

- none

## Operator Constraints

- Remain a serial prerequisite of #192 and #142.
- Do not open the PR before independent exact-head review and resolution of all actionable findings.
- Do not merge without the operator's normal reviewed-PR authority.
