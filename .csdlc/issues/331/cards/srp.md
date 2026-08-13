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
.csdlc/prepared/issues/331/review-recover-after-publication-evidence-drift.json

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

- Review was read-only and bounded to assigned paths at immutable commit c4848ee3d3fd040d65dc67f6e247f4fa2870156d.
- Validation logs were inspected as evidence, but tests were not rerun by this reviewer.

## Review Result

Revision: Some("git-blake3:c4848ee3d3fd040d65dc67f6e247f4fa2870156d:8f3e2afda1d8c3640af279612899fffca2de1e64d58de2085d9c183b705c8920")

Reviewer: Some("fresh-session:4f6d4dcc-c379-4544-9c8a-bb34eaeb80f9")

Result: pass
