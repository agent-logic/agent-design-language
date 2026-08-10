# Structured Review Prompt

Template: 1.0.0

Issue: 191

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Quinn/rustls handshake and identity binding, bounded RPC framing/replay, OpenRaft semantics, crash-safe persistence, restart/snapshot recovery, path safety, secret hygiene and exact proof.

## Prompts

- Can any node send or accept OpenRaft traffic without an authenticated encrypted session bound to the exact polis/domain/node generation?
- Can any persistence or snapshot failure leave live or recovered state ahead of the last durable commit?
- Do the tests exercise real network encryption, three-to-two, one-of-three halt, restart, replay, corruption and path attacks?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
