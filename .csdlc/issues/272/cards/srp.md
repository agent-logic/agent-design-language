# Structured Review Prompt

Template: 1.0.0

Issue: 272

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/serving_authority.rs
adl-runtime/src/distributed/mod.rs
adl-runtime/tests/distributed_serving_authority_foundation.rs
.csdlc/issues/272
.csdlc/prepared/issues/272
.csdlc/evidence/272

## Prompts

- Can any caller publish foundation state without the exact sealed #200 authority cut and terminal #203 PublishedStoreAuthorityReceiptView?
- Does #272 avoid all #203 registry edits and every #273/#274 eligibility lifecycle?
- Can Pending, restart, retry, corruption, rollback, capacity, or unsafe path expose a new or contradictory published view?
- Are every lineage, operation, action, version, generation, receipt/result, OwnerCommit, fence, lease, and prior-state binding exact and replay-safe?
- Does the base projection reveal only bounded redacted values and make no eligibility decision?
- Are the product/test allowlist, validation lanes, fresh-session review, publication, CI, and terminal gates exact and truthful?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Broad all-target strict Clippy remains baseline-red in unrelated pre-existing Runtime test surfaces; the production library and exact #272 integration target are strict-Clippy clean.
- Review is limited to #272 foundation behavior and does not approve #273-#275 eligibility or integrated behavior.

## Review Result

Revision: Some("git-blake3:08015fbb363ef8d31b31b17844f856fd48f3b6d0:a58ceb4c66c321372482bf1dc84b4d6a3c6bff97f323ced8ed3601aefc6846c9")

Reviewer: Some("fresh-session:97b655fd-7cc7-4b6b-8785-4994b6df35b7")

Result: pass
