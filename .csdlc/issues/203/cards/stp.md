# Structured Task Prompt

Template: 1.0.0

Issue: 203

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and publish only sealed existing-store adapters, store-bound live grant enforcement, deterministic canonical lease-time state, conservative local safety anchors, and focused proof surfaces.

## Deliverables

- adl-runtime/src/distributed/authority_store_adapters.rs
- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/src/distributed/fencing.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_identity_lease_authority.rs
- .csdlc/prepared/issues/203/produce-proof-receipt.rb
- .csdlc/prepared/issues/203/validate-proof-receipt.rb
- .csdlc/evidence/203
- .csdlc/issues/203

## Acceptance

1. AC-1: Normal-build callers cannot open ungated stores, construct a raw grant, or mutate/authorize certificates, leases, or fencing without current #200 lineage/action/adapter/generation validation on every use.
2. AC-2: Each sealed plan consumes the exact private #201 artifact view, byte-compares its digest and operation binding, and verifies it through the existing store-native signature/quorum path without signing, endorsing, or reconstruction.
3. AC-3: Canonical lease state and receipts bind only committed deterministic time; local monotonic safety anchors are separate and NotReady/Unsafe produces no concrete or barrier progress.
4. AC-4: Certificate enroll/rotate/revoke/compromise and LeaseGrant/Renewal/Revoke/Fence/Activate/OwnerCommit execute only the fixed fail-safe order, with floor before ledger revocation and exact floor/safety/possession checks before activation.
5. AC-5: Every concrete effect has an opaque digest-bound receipt; partial progress remains denied, and cache-first exact retry reconciles rather than duplicating effects and returns only from Published.
6. AC-6: Initialization, dual open, every effect/receipt/anchor/result/checkpoint/marker/view boundary, rollback, corrupt/noncanonical/oversized or replaced state, unsafe paths, and N+1 capacity fail closed without partial publication.
7. AC-7: Existing path-included low-level tests may use cfg(test) raw helpers, but the normal library exposes only authority-bound handles and compile-time proof rejects a production bypass.
8. AC-8: Exact forty-four-case focused proof, strict Clippy, merge-safe immutable receipt validation, diff hygiene, and fresh independent exact-head review pass before a ready unmerged PR opens.

## Dependencies

- Issue #191 / PR #197 externally reviewed and merged as an ancestor
- Issue #201 committed authority protocol externally reviewed and merged as an ancestor
- Issue #202 learner transport and exclusion externally reviewed and merged as an ancestor
- Issue #199 governed membership transitions externally reviewed and merged as an ancestor
- Issue #200 reconciliation barrier externally reviewed and merged as an ancestor
- Issue #205 and #204 remain blocked until this issue merges

## Inputs

- agent-logic/agent-design-language#203
- adl-runtime/src/distributed/authority_protocol.rs from merged #201
- adl-runtime/src/distributed/authority_reconciliation.rs from merged #200
- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/src/distributed/fencing.rs
- .csdlc/issues/142 operational design as read-only umbrella truth

## Non Goals

- Shepherd or Observatory serving eligibility (#205)
- Migration or recovery workflow execution (#204)
- OpenRaft membership (#199) or learner transport/exclusion (#202)
- Guardian/kernel/API/WSS, models, AWS, live qualification, final #142 delivery, merge without operator authorization, or lifecycle closeout
