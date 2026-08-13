# Structured Review Prompt

Template: 1.0.0

Issue: 331

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/migration.rs
csdlc-v2/src/store.rs
csdlc-v2/src/doctor.rs
csdlc-v2/src/bin/csdlc-issue.rs if changed
csdlc-v2/tests/code_repository_migration.rs
focused gate2/doctor tests if changed
.csdlc/issues/331
.csdlc/evidence/331

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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
