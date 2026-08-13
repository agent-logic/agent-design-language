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
.csdlc/issues/331
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

- Read-only review; validation commands were not re-run by this review session.
- Working tree had lifecycle/index/SRP noise and an untracked lock/assignment file, so review was pinned to immutable HEAD objects.

## Review Result

Revision: Some("git-blake3:a9c11f378b70949a887adc211781057d08c9af31:46f4f67ae0dddfb8df6c1fdc24ff7adae71d316399946a7644f8ebb77c906473")

Reviewer: Some("fresh-session:4589c316-ed35-4957-b82d-c9f6f5131fd5")

Result: pass
