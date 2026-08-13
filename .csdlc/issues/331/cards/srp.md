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
.csdlc/prepared/issues/331/sor-record-post-p1-remediation.json

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

- Review was read-only and bounded to the assigned scope and immutable HEAD; uncommitted lifecycle/review bookkeeping in .csdlc/issues/331 was not treated as substantive implementation scope.
- Validation evidence was inspected as recorded proof, but validation commands were not rerun by this reviewer.

## Review Result

Revision: Some("git-blake3:6b9e81098a7620a263f7fc7cb5901ef76673b669:9c5cb7ecdf54bc47807a625fa65a1fb96e6da38df61c02ecb90d231535c5fc46")

Reviewer: Some("fresh-session:a5d0d458-5c86-405f-98cd-0d240a3e5433")

Result: pass
