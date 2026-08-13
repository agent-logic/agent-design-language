# Structured Review Prompt

Template: 1.0.0

Issue: 331

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/migration.rs
csdlc-v2/src/store.rs
csdlc-v2/src/bin/csdlc-issue.rs
csdlc-v2/src/schema.rs
csdlc-v2/src/lib.rs
csdlc-v2/tests/code_repository_migration.rs
.csdlc/evidence/331
.csdlc/prepared/issues/331/design.md
.csdlc/prepared/issues/331/diagram.mmd
.csdlc/prepared/issues/331/finalize-implementation.json
.csdlc/prepared/issues/331/validate_initialized_code_repository_migration.py
.csdlc/prepared/issues/331/review-recover-after-main-merge.json

## Prompts

- Does the new route only allow initialized repository declaration without broad lifecycle identity rewrite?
- Are stale CAS, existing code_repository, branch/worktree, publication, terminal, and ambiguous repository cases fail-closed?
- Does doctor/readiness clear repository_identity_drift for explicit canonical code_repository while preserving issue repository authority?
- Is existing bound/implemented/reviewed migration behavior unchanged?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Read-only exact-head review only; no mutation performed.
- Worktree had dirty lifecycle/review metadata outside the assigned acceptance scope, so this PASS is limited to the assigned scoped paths at HEAD efb0183888b4bd3f5fac5d525da4564aca123a10.

## Review Result

Revision: Some("git-blake3:efb0183888b4bd3f5fac5d525da4564aca123a10:db15387873e558955ca6c77b94261880a8c013dd84642c7670e84d6fd5f60a5d")

Reviewer: Some("fresh-session:8cf72c63-c5f6-4615-84d2-086f56cad1ce")

Result: pass
