# Structured Task Prompt

Template: 1.0.0

Issue: 210

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and publish only the authenticated typed continuity-transfer session on #191, consuming #201's sealed ContinuityTransferGrantProjection and #208's sealed ContinuityBundleSourcePort/TargetContinuityEffectPort. Before any target write, independently verify the retained signed manifest/catalog key generation and exact entry/chunk/range commitments; advance bytes, incremental verifier, and durable accepted prefix only through a crash-reconcilable effect. Return VerifiedTransferPossession only after full verification. On abort, request #208 cleanup through a separate TargetCleanupPermit that survives transfer expiry/cancellation; #210 never deletes, activates, fences, owns, serves, or decides migration. Prove the exact 45 cases and SHA-256-bound 8-acceptance/84-subassertion map serially through tests, Clippy, exact base-to-source diff, producer, independent review, and validator. Every one of the 45 cases must have at least one direct machine subassertion mapping.

## Deliverables

- adl-runtime/src/distributed/continuity_transfer.rs
- adl-runtime/src/distributed/transport.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/distributed/snapshot_catalog.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_continuity_transfer.rs
- .csdlc/prepared/issues/210/continuity-transfer-acceptance-map.json
- .csdlc/prepared/issues/210/verify-diff-hygiene.rb
- .csdlc/prepared/issues/210/produce-proof-receipt.rb
- .csdlc/prepared/issues/210/validate-proof-receipt.rb
- .csdlc/evidence/210
- .csdlc/issues/210

## Acceptance

1. AC-1: Only the exact current authorized source and target route/certificate/boot/membership cut may establish a private transfer session; wrong or stale identity, lineage, domain, polis, cut, token, generic send, Raft, or unknown-kind input fails before bytes move.
2. AC-2: Every canonical frame binds transfer, manifest, index, offset, length, payload and predecessor digests and session generations; all frame, queue, file, count and total-byte bounds reject N+1 before prefix mutation.
3. AC-3: The target durably records each exact accepted prefix before acknowledgment; exact duplicate is cached, conflict/gap/reorder/overlap fails closed, and restart resumes only at the exact next offset.
4. AC-4: Incremental verification proves the signed catalog/manifest, service/file order, schemas, chunk list, total length and whole digest without whole-bundle allocation before #208 possession evidence.
5. AC-5: Deadline, cancellation, partition, source/target restart, disk-full, crash and reply loss reconcile the exact prefix/result or leave isolated denied state; no false terminal success or duplicate write is possible.
6. AC-6: Authorized abort closes the session, invokes exact #208 discard, removes only transfer-owned state and returns a live independently verified zero-residue receipt.
7. AC-7: Evidence contains only opaque refs, digests, bounded counts and outcomes, with no raw content, identity, certificate, token, address, path, key, signature or secret.
8. AC-8: Exact forty-five-case focused proof, strict Clippy, merge-safe immutable receipt validation, diff hygiene and fresh independent exact-head review pass before a ready unmerged PR opens.

## Dependencies

- Merged fixed ancestor: issue #191 / PR #197 at 8bd475cf18eb77cc7402220f69282f64a4a1a1e5
- Merged fixed ancestor: issue #201 / PR #229 at 3ffc4c402c57e167fb9943221c9dac24f96f8895
- Merged fixed ancestor: issue #200 / PR #231 at 507d9a1e3a74c2c9c6cce14259b095139aa3bdfa
- Merged fixed boundary: issue #208 / PR #230 at 5e25dccebde3bdd608e3ecb80d3d60a0c40e3a90; #210 consumes only its sealed ports and cannot take over kernel/filesystem effects or cleanup
- Serial stop: issue #202 must be independently reviewed, merged, and ancestral, followed by issue #199, followed by issue #203; #210 may not bind or edit product source before all three conditions hold in that order
- After #202, #199, and #203 merge, resync #210 onto the resulting exact origin/main and rerun typed csdlc-validate issue plus csdlc-doctor before bind
- #205 and #210 are graph-parallel preparation siblings but their implementation PRs land serially because both touch adl-runtime/src/distributed/mod.rs; the second branch resyncs and revalidates after the first merge
- Future issue #204 consumes #210 transfer outcomes for migration/control decisions and remains blocked until the required authority and transfer predecessors merge

## Inputs

- agent-logic/agent-design-language#210
- adl-runtime/src/distributed/authority_protocol.rs from merged #201
- adl-runtime/src/distributed/authority_store_adapters.rs from merged #203
- adl-runtime/src/kernel_continuity_client.rs from merged #208
- adl-runtime/src/distributed/transport.rs and polis_runtime.rs from merged #191
- adl-runtime/src/distributed/snapshot_catalog.rs

## Non Goals

- Consensus, authority issuance, membership transitions, local kernel continuity implementation (#208), migration/recovery policy (#204), fence/activation/OwnerCommit/serving authority, models, AWS, live qualification, final #142 delivery, merge without operator authorization, or lifecycle closeout
