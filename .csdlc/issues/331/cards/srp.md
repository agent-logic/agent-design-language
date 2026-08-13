# Structured Review Prompt

Template: 1.0.0

Issue: 331

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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

- Review was read-only; validation evidence was inspected from retained logs rather than re-executed.
- GitHub/PR state and hosted CI were outside this exact-head implementation review.

## Review Result

Revision: Some("git-blake3:24e99238dd88daa437a6adf973baf4a1741e2de8:4e283a5a997f1e74d13dbedcb1b6e8d8d02f45a8c51dcb0eba96a2133df6bfb7")

Reviewer: Some("fresh-session:f74e271b-ec76-4ab6-bfcb-2f81865f8327")

Result: pass
