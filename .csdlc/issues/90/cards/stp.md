# Structured Task Prompt

Template: 1.0.0

Issue: 90

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove the narrow pre-code_repository migration route, obtain exact-head review, and publish a closing PR; stop before merge unless explicitly authorized.

## Deliverables

- Versioned typed code_repository migration request and report
- csdlc-issue migrate-code-repository command
- Fail-closed topology, origin, phase, CAS, cleanliness, and prior-state guards
- Durable audit evidence preserving previous and adopted identity
- csdlc-v2/tests/code_repository_migration.rs
- Focused regression proving reviewed split-authority publication
- Updated operator recovery documentation

## Acceptance

1. AC-1: A Bound, Implemented, or Reviewed record with absent code_repository can adopt only the exact effective GitHub origin repository through a typed request.
2. AC-2: The operation requires the exact registered issue branch and canonical worktree, a clean tracked and untracked worktree, and current generation and digest.
3. AC-3: Wrong origin, mismatched requested identity, missing or ambiguous topology, dirty worktree, unsupported phase, stale CAS, and existing conflicting code_repository fail closed.
4. AC-4: The atomic update changes only code_repository, record/card identity generations, projection/canonical digests, and complete audit evidence; semantic card content, phase, review, publication, readiness, and terminal truth remain unchanged.
5. AC-5: A reviewed pre-field record can migrate and pass the existing split-authority publication preflight without review recovery or weakened freshness checks.
6. AC-6: Repeating the exact successful request deterministically fails with stale_digest and never changes generation or duplicates audit evidence.
7. AC-7: Focused tests, formatting, strict Clippy, schema exposure, and installed-command smoke proof pass.
8. AC-8: Operator documentation replaces hand-edit fallback with the typed recovery command and states every stop condition.

## Dependencies

- Canonical GitHub issue #90
- Existing csdlc-issue migration command surface
- Existing Git topology and GitHub origin identity helpers
- Existing csdlc-publish split-repository identity and exact-review checks

## Inputs

- AGENTS.md
- csdlc-v2/AGENTS.md
- csdlc-v2/src/migration.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/doctor.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/topology_migration.rs
- agent-logic/agent-design-language#90

## Non Goals

- Weakening or bypassing csdlc-publish repository checks
- Changing issue repository identity
- Retargeting arbitrary remotes, branches, or worktrees
- Editing cards or substantive reviewed files
- Migrating published, merge-ready, merged, closed-out, or unbound initialized records
- General administrative record mutation
