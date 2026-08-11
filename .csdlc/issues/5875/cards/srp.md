# Structured Review Prompt

Template: 1.0.0

Issue: 5875

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/migration.rs
adl-runtime/tests/distributed_migration.rs
.csdlc/evidence/5875
.csdlc/issues/5875

## Prompts

- Is the implementation confined to exclusive paths?
- Do exact tests prove the named behavior and negatives?
- Are receipts exact-revision and digest bound?
- Does rollback restore one authoritative owner without weakening security?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Production module registration remains intentionally owned by integration issue #5878.

## Review Result

Revision: Some("git-blake3:85c4023c5fdd0776177b3dc87adbdd203cf8c375:778d772d22b56bc27032efc48c7271c61293f255489ea48e9d97b98237e63c46")

Reviewer: Some("codex:independent-5875-exact-head-review")

Result: pass
