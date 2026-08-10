# Structured Task Prompt

Template: 1.0.0

Issue: 199

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and publish only governed learner catch-up, OpenRaft joint/final transition orchestration, exact membership/route parity publication, pending removal exclusion, governed rejoin, and crash reconciliation between #201 and #200.

## Deliverables

- adl-runtime/src/distributed/membership_coordinator.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/distributed/membership.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_membership_transition.rs
- .csdlc/prepared/issues/199/produce-proof-receipt.rb
- .csdlc/prepared/issues/199/validate-proof-receipt.rb
- .csdlc/evidence/199
- .csdlc/issues/199

## Acceptance

1. AC-1: A transition starts only from an opaque #201 membership-operation token and exact current MembershipState, AuthorityMembership, verified route cut, and durable OpenRaft membership parity.
2. AC-2: Add, promote, and rejoin enroll the authorized candidate only as a learner and prove exact committed-log or canonical-snapshot catch-up before any voting transition.
3. AC-3: The coordinator invokes the standard OpenRaft membership API and durably observes both the exact joint configuration and the exact final uniform target configuration before authority publication.
4. AC-4: Final publication makes MembershipState, AuthorityMembership, verified routes, Raft ids, voter identities, keys, certificate generations, configurations, and final committed index agree exactly; authorization remains fail closed until parity is complete.
5. AC-5: The shared durable pending-exclusion authority from #202 immediately excludes the target from ordinary endorsement, voter route, renewal, mutation, Shepherd, and Observatory authority while permitting only an explicitly governed replication-only learner-recovery session; #199 emits a pending-exclusion receipt for #200 and does not claim that it mutated FencingStore.
6. AC-6: A removed or restarted node cannot self-promote from local state; governed rejoin requires a new current token and certificate, learner catch-up, joint/final commitment, and parity publication.
7. AC-7: An exclusive bounded canonical journal, exact retry cache, and node-local external checkpoint reconcile leader change, initialization, every transition phase, rollback, corruption, capacity, and unsafe path failures without duplicate side effects.
8. AC-8: Exact focused real-node tests, strict Clippy, merge-safe receipt validation, diff hygiene, and fresh independent exact-head review pass before a ready unmerged PR opens.

## Dependencies

- Issue #191 / PR #197 externally reviewed and merged as an ancestor
- Issue #201 quorum-committed authority protocol externally reviewed and merged as an ancestor
- Issue #202 authority-verified learner route and shared pending-exclusion consultation externally reviewed and merged as an ancestor
- Current MembershipState, AuthorityMembership, certificate identity, verified route cut, and secure OpenRaft APIs
- Issue #199 live GitHub contract
- Issue #200 remains blocked until this issue merges

## Inputs

- agent-logic/agent-design-language#199
- adl-runtime/src/distributed/polis_runtime.rs from merged #191 and #201
- adl-runtime/src/distributed/membership.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/src/distributed/transport.rs and authority_protocol.rs from merged #202 learner/exclusion prerequisite
- adl-runtime/tests/distributed_runtime_transport.rs
- .csdlc/issues/142 operational design as read-only umbrella truth

## Non Goals

- Creating or verifying #201 endorsements or VerifiedAuthorityOperation tokens
- Certificate, lease, FencingStore, owner, Shepherd, Observatory, migration, or recovery side effects (#200)
- Kernel continuity export/import or snapshot catalog materialization
- Guardian/API/WSS or Observatory listener integration
- Models, AWS, live demonstrations, final #142 delivery, merge without operator authorization, or lifecycle closeout
