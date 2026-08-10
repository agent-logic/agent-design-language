# Structured Review Prompt

Template: 1.0.0

Issue: 191

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/191
.csdlc/prepared/issues/191
.csdlc/evidence/191
adl-runtime/Cargo.toml
adl-runtime/Cargo.lock
adl-runtime/src/distributed/transport.rs
adl-runtime/src/distributed/polis_runtime.rs
adl-runtime/tests/distributed_runtime_transport.rs
adl-runtime/tests/distributed_transport.rs
adl-runtime/tests/distributed_discovery.rs

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
