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

- Replacement hosted CI and operator external review remain required before merge.

## Review Result

Revision: Some("git-blake3:fe948c207dfa2d64071bd6beac4eb4e0df415dfa:8c69febe67f3a65cdbc047acf3bef19a24b853fc973a8acf808ead0e521d1855")

Reviewer: Some("subagent:prepare_5875_release")

Result: pass
