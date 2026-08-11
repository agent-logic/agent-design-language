# Structured Task Prompt

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and publish only the committed quorum authority protocol, its private exact-artifact views for sealed #199/#200/#203 consumers, and a separate sealed #210-only ContinuityTransferGrantProjection for exact continuity-transfer variants; the #210 projection is borrowed, read-only, operation-bound, nonconstructible, and cannot authorize generic payload conversion, signing, migration, fencing, activation, serving, or concrete store effects.

## Deliverables

- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_authority_protocol.rs
- .csdlc/prepared/issues/201/produce-proof-receipt.rb
- .csdlc/prepared/issues/201/validate-proof-receipt.rb
- .csdlc/evidence/201
- .csdlc/issues/201

## Acceptance

1. AC-1: Prepare and finalize entries bind polis, trust domain, exact current membership epoch/index and voter-set digest, operation kind, expected prior protocol checkpoint, payload digest, canonical quorum-attested time token, and a unique bounded operation id.
2. AC-2: Finalization requires an opaque local VoterEndorsementAuthority and a strict quorum of distinct current voter endorsements over the exact committed intent; raw signing keys and caller-produced endorsements are rejected.
3. AC-3: Replicated apply is deterministic: every voter consumes identical committed logical/time evidence, while replica-local clocks may gate endorsement but cannot change the applied result.
4. AC-4: A symlink-safe, exclusively locked, bounded canonical journal and external ConsensusCheckpointAuthority reconcile initialization, intent, finalized result, and exact retry state across crash without claiming a transaction across independent downstream stores.
5. AC-5: Exact retries return the retained canonical result; conflicting reuse, reordered finalize, cross-polis/domain, stale membership, wrong signer, duplicate signer, expired intent, rollback, corruption, and capacity violations fail before new protocol publication.
6. AC-6: Legacy direct PolisCommand authority variants cannot mint membership, lease, fence, owner, Shepherd, Observatory, migration, or recovery authority; retained log/snapshot replay is versioned or fails closed explicitly.
7. AC-7: Successful finalization produces an opaque quorum-approved operation token for #199/#200 and does not itself execute OpenRaft membership change or concrete authority-store side effects.
8. AC-8: Exact nonzero focused tests, strict Clippy, merge-safe receipt validation, diff hygiene, and fresh independent exact-head review pass before a ready unmerged PR opens.

## Dependencies

- Issue #191 / PR #197 externally reviewed and merged as an ancestor
- Current merged MembershipState, AuthorityMembership, certificate identity, and secure OpenRaft transport contracts
- Issue #201 live GitHub contract
- Issues #199 and #200 remain blocked until this issue merges

## Inputs

- agent-logic/agent-design-language#201
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/distributed/transport.rs
- adl-runtime/src/distributed/membership.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_runtime_transport.rs
- .csdlc/issues/142 and its reviewed operational design as read-only umbrella truth

## Non Goals

- OpenRaft learner, joint, final, demotion, or rejoin membership coordination (#199)
- Certificate, lease, fence, owner, Shepherd, migration, or recovery store side effects (#200)
- Kernel continuity export/import or snapshot catalog materialization
- Guardian/API/WSS or Observatory listener integration
- Models, AWS, live demonstration, final #142 delivery, merge without operator authorization, or lifecycle closeout
