# Structured Task Prompt

Template: 1.0.0

Issue: 78

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement, prove, review, and publish the narrow post-recovery STP deliverable correction; stop before merge unless explicitly authorized.

## Deliverables

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs

## Acceptance

1. AC-1: A recovered implemented issue can correct only its STP deliverables through csdlc-edit apply.
2. AC-2: An implemented issue without typed review recovery cannot use the operation.
3. AC-3: Every non-implemented phase and every non-STP card rejects the operation.
4. AC-4: Stale CAS, projection drift, blank values, and duplicate values fail closed.
5. AC-5: The atomic commit preserves unrelated STP fields and regenerates all projections and digests.
6. AC-6: Durable audit evidence includes actor, reason, previous deliverables, and replacements.
7. AC-7: Focused tests, formatting, strict Clippy, and installation proof pass from /Volumes/FastWork.
8. AC-8: Issue #73 successfully consumes the installed operation without direct card edits.

## Dependencies

- Canonical GitHub issue #78
- Existing csdlc-review recover audit semantics
- Existing csdlc-edit atomic store and projection validation
- Issue #73 recovered implemented record

## Inputs

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-edit.rs
- csdlc-v2/tests
- csdlc-v2/AGENTS.md
- AGENTS.md

## Non Goals

- General post-implementation STP editing
- Phase rollback from implemented to bound
- Direct Markdown or JSON mutation
- Changing issue #73 architecture content
- Implementing C-SDLC v3
