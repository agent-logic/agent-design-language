# Structured Task Prompt

Template: 1.0.0

Issue: 191

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Replace the unencrypted prototype OpenRaft RPC and mutate-before-persist stores with one production encrypted authenticated transport and crash-safe storage vertical slice, without implementing higher-level distributed authority or cloud orchestration.

## Deliverables

- adl-runtime/Cargo.toml
- adl-runtime/Cargo.lock
- adl-runtime/src/distributed/mod.rs
- adl-runtime/src/distributed/transport.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/tests/distributed_runtime_transport.rs
- adl-runtime/tests/distributed_transport.rs
- adl-runtime/tests/distributed_discovery.rs
- .csdlc/prepared/issues/191/produce-proof-receipt.rb
- .csdlc/prepared/issues/191/validate-proof-receipt.rb

## Acceptance

1. AC-1: Three real voters elect and commit over the existing authoritative mutually authenticated encrypted Quinn/rustls transport only.
2. AC-2: Exact polis, trust-domain, sender, receiver, boot generation, committed membership index, certificate generation and node identity are bound; unauthorized, expired, or superseded-after-overlap values fail closed while an authority-approved overlap remains valid.
3. AC-3: An exact duplicate returns its durable cached response without Raft redispatch; a conflicting duplicate, reordered new sequence, cross-polis, oversized or truncated RPC is rejected before dispatch with bounded reads.
4. AC-4: Vote, log, state-machine, replay-response and snapshot persistence are fail-atomic per OpenRaft callback and crash-recoverable; injected pre/post-write failures cannot create partial accepted state.
5. AC-5: Three-to-two continues, one-of-three cannot commit, and a restarted voter recovers the exact authority-derived committed prefix and snapshot.
6. AC-6: State, key, certificate and lock paths reject symlinked ancestors, nonordinary leaves and oversized data; an external ConsensusCheckpointAuthority rejects coherent rollback and lower committed generations.
7. AC-7: Initial and changed routes derive only from exact MembershipState and AuthorityMembership parity; caller routing hints cannot create voters or membership authority.
8. AC-8: Unique voter private keys remain absent from logs/evidence, and focused tests, strict Clippy, exact receipt validation and independent exact-head review pass with no unresolved actionable findings.

## Dependencies

- agent-logic/agent-design-language#142 split authority
- Merged Sprint 3 transport, certificate and membership modules on main
- OpenRaft, Quinn and rustls dependencies already present in adl-runtime

## Inputs

- adl-runtime/src/distributed/transport.rs
- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/membership.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/Cargo.toml

## Non Goals

- Distributed lease, fencing, activation, migration or recovery policy
- Runtime kernel continuity export/import
- AWS provisioning, models, Observatory presentation or final #142 demonstration
- Permanent infrastructure or multi-region consensus
