# Structured Intent Prompt

Template: 1.0.0

Issue: 272

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Build the durable reconcile-before-publish serving-authority foundation consumed by #273, #274, and #275 without implementing any eligibility lifecycle.

## Required Outcome

Only an exact sealed #200 authority cut and terminal #203 read-only PublishedStoreAuthorityReceiptView can reconcile and publish bounded durable serving-authority foundation state; OwnerCommit/fence/lease and prior/candidate state digests are accepted only from the exact bounded DOMAIN-NUL/u32be-length/RFC8785-JCS preimage whose SHA-256 equals the sealed view result digest; incomplete, stale, corrupt, rolled-back, conflicting, or unsafe state fails closed and the base projection is deterministic and redacted.

## Scope

- Durable bounded serving-authority foundation store
- Exact sealed authority, OwnerCommit, fence, lease, lineage, operation, generation, receipt, result, and prior-state bindings
- Pending then reconcile then publish state machine with cache-first retry and restart reconciliation
- Deterministic redacted base projection
- Focused foundation tests, strict Clippy, scope guard, exact-head review, and hosted CI

## Authority

- #205 remains coordination-only and owns no product implementation
- #203 exclusively owns authority_store_adapters.rs and AuthorityStoreAdapterRegistry; #272 consumes only PublishedStoreAuthorityReceiptView and never edits or widens that registry
- #272 owns only the durable foundation and no Shepherd or Observatory eligibility lifecycle
- Node-local bytes, configuration, retained permits, cached booleans, caller DTOs, raw tokens, and raw stores are never authority
- OwnerCommit, fence, lease, prior-state, and candidate-state fields are untrusted until an exact <=4096-byte ADL-SERVING-AUTHORITY-FOUNDATION-BINDING-V1-NUL/u32be-length/RFC8785-JCS preimage hashes to the sealed #203 result_sha256; lineage, operation, adapter_kind, action_class, adapter_version, published_generation, and receipt digest also match direct accessors; no #203 widening is required

## Assumptions

- none

## Operator Constraints

- Preparation base is exact origin/main a0d7b2bb58f80762610972ca945678b696640df4
- Do not touch dirty primary main or the preserved codex/205-serving-authority-preparation branch
- Do not bind or edit product source before a new #119-compliant fresh design review PASS, typed approval, validate, and doctor
- Stop for #265 only on a real same-file collision; its current Runtime-kernel files do not overlap
- Do not edit #203 authority_store_adapters.rs or absorb #273, #274, #275, #204, process/listener, Runtime-kernel, projection, UI, cloud, provider, or paid-runner scope
- Use branch codex/272-serving-authority-foundation and a FastWork worktree only through typed bind
