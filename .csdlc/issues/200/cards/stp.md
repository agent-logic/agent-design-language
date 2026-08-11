# Structured Task Prompt

Template: 1.0.0

Issue: 200

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and publish only the generic opaque reconciliation barrier, deterministic time-evidence carrier, sealed adapter/result boundary, current permit boundary, durable state machine, and focused test/proof surfaces consumed later by #203/#204.

## Deliverables

- adl-runtime/src/distributed/authority_reconciliation.rs
- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_authority_reconciliation.rs
- .csdlc/prepared/issues/200/produce-proof-receipt.rb
- .csdlc/prepared/issues/200/validate-proof-receipt.rb
- .csdlc/evidence/200
- .csdlc/issues/200

## Acceptance

1. AC-1: Only an opaque finalized #201 token plus the private sealed adapter registry can create a plan; public constructors, raw payloads, caller receipts/booleans, and legacy direct PolisCommand authority paths fail closed.
2. AC-2: The journal and external checkpoint bind domain, polis, node, guardian, boot, protocol instance, lineage, operation/token/payload/result/time/membership/log/checkpoint/retry digests, adapter kind/version, ordered plan, and exact receipt set.
3. AC-3: Pending is durable before any step; every registered step is individually idempotent and its opaque exact receipt is verified and fsynced in order before the next step.
4. AC-4: While Pending, Reconciling, or Checkpointed, current read and mutation permits are denied; one permit-bearing published generation appears only after all steps, result cache, external checkpoint, and marker reconcile.
5. AC-5: Exact retry is cache-first and returns the byte-exact retained result without re-execution; conflicting reuse, missing/duplicate/reordered/forged receipts, unknown adapter version, or authority/time drift fails closed.
6. AC-6: Initialization and every phase reconcile across crash, dual open, rollback, corruption, noncanonical/oversized/replaced files, capacity N+1, checkpoint collision, and unsafe state/lock paths without partial publication.
7. AC-7: The barrier does not invoke concrete #203/#204 production stores or claim cross-store atomicity; the focused test-only adapter proves one-step, multi-step, fail-safe visibility, and every durable fault window.
8. AC-8: Exact thirty-six-case focused tests, strict Clippy, merge-safe receipt validation, diff hygiene, and fresh independent exact-head review pass before a ready unmerged PR opens.

## Dependencies

- Issue #191 / PR #197 externally reviewed and merged as an ancestor
- Issue #201 quorum-committed authority protocol externally reviewed and merged as an ancestor
- Current secure PolisRuntime checkpoint and durable-state APIs
- Issue #200 live narrowed GitHub contract
- Issues #203 and #204 remain blocked until this issue merges

## Inputs

- agent-logic/agent-design-language#200
- adl-runtime/src/distributed/authority_protocol.rs from merged #201
- adl-runtime/src/distributed/polis_runtime.rs from merged #191
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_runtime_transport.rs
- .csdlc/issues/142 operational design as read-only umbrella truth

## Non Goals

- Concrete certificate, lease, fencing, owner, Shepherd, or Observatory adapters (#203)
- Migration or recovery workflow execution and receipts (#204)
- OpenRaft membership (#199) or learner transport/exclusion (#202)
- Guardian/kernel/API/WSS, models, AWS, live qualification, final #142 delivery, merge without operator authorization, or lifecycle closeout
