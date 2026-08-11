# Structured Intent Prompt

Template: 1.0.0

Issue: 200

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Add one opaque crash-safe reconciliation barrier between finalized #201 operation tokens and later concrete authority adapters without claiming a transaction across independent stores.

## Required Outcome

Exact token-bound adapter steps reconcile through a journal, result cache, node-local checkpoint, and one atomic published generation; authority-restoring reads and mutations remain denied until publication, and exact retry returns the retained result without re-execution.

## Scope

- Private AuthorityReconciliationBarrier and sealed adapter/result contracts
- Pending, Reconciling, Checkpointed, and Published durable phases
- Deterministic committed-time evidence transport from #201
- Fail-safe read and mutation permit boundary
- Crash, retry, rollback, corruption, capacity, and path-safety proof with a test-only adapter

## Authority

- Only an opaque finalized #201 operation token can create a reconciliation plan
- Production callers cannot inject an adapter trait object, closure, step receipt, completion boolean, or permit
- The barrier transports committed time evidence unchanged and never uses a local clock to choose replicated results
- No permit exists before exact state, result, checkpoint, marker, and published view agree
- No cross-store atomicity or concrete #203/#204 side effect is claimed
- Legacy direct PolisCommand variants cannot create authority or reconciliation success

## Assumptions

- none

## Operator Constraints

- Do not bind or edit product source until PR #197 and #201 are externally reviewed, merged, and ancestral
- Keep this issue limited to the reusable barrier; concrete stores are #203 and migration/recovery is #204
- Use opaque/private authority types and a sealed registry rather than caller-provided adapters or receipts
- Run fresh independent exact-head review before publication
- Open a ready PR for visibility but never merge before operator review and authorization
- No AWS use and no lifecycle closeout
