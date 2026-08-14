# Structured Review Prompt

Template: 1.0.0

Issue: 298

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/projection_recovery.rs
csdlc-v2/src/store.rs
csdlc-v2/src/schema.rs
csdlc-v2/src/bin/csdlc-issue.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/298
.csdlc/evidence/298

## Prompts

- no-follow retained-handle identity and mount authority
- tagged CAS and failed-operation lineage permissions
- immutable receipt chain and exact crash adoption
- complete candidate construction and atomic install
- evidence preservation and ordinary-commit release gate
- scope exclusion of #299 cleanup and #300 integrated proof

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Inspection-only review; reviewer did not rerun validation or recheck GitHub CI. Parent observed local validation at the unchanged source tree after merge reconciliation: cargo fmt --check, git diff --check, focused gate5 preserved_projection_recovery, cargo test --lib, and strict all-target Clippy all passed. Hosted CI remains deferred to republication.

## Review Result

Revision: Some("git-blake3:222abd504d99a3bedc224699c0b68d302aac25b7:84e0aadc005c93ddf4bf88ac30533059fc92a2038fae8d23f727d44885c3e4fd")

Reviewer: Some("fresh-session:aee63902-3a08-4f0b-8f9f-6676a5ef1352")

Result: pass
