# Structured Task Prompt

Template: 1.0.0

Issue: 292

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Tooling-only csdlc-edit implementation and tests; #112 is a read-only fixture and must not be mutated.

## Deliverables

- Implemented csdlc-edit operation
- Authorization predicates and sibling-scope rejects
- Audit event recording previous/new identity
- Tests for atomic update, stale CAS, phase/review/publication/readiness/terminal rejects, incompatible latest review-related audit rejects, malformed/sibling title/slug rejects, live evidence mismatch, cross-card equality, and validation on isolated #112 fixture
- Fresh exact-head review record
- Published ready PR, not merged

## Acceptance

1. AC-1 csdlc-edit accepts correct_identity_title_slug_after_decomposition only for implemented-phase records with no review assignment, review, publication, readiness, or terminal truth, and with latest review-related audit state compatible with review recovery.
2. AC-2 The operation requires live typed issue evidence and rejects when the evidence title does not equal the requested title.
3. AC-3 The operation rejects sibling-scope claims and malformed, empty, or colliding slugs.
4. AC-4 The operation atomically updates title and slug across all six card value envelopes while preserving non-identity card content.
5. AC-5 The audit event records previous_title, new_title, previous_slug, new_slug, and live issue evidence.
6. AC-6 Stale generation/digest requests fail closed.
7. AC-7 An isolated #112-derived fixture validates without mutating any #112 worktree.
8. AC-8 A #119-compliant fresh-session exact-head review is assigned and recorded before publication.
9. AC-9 The ready PR is published green and the session stops before merge.

## Dependencies

- Issue #292
- Issue #119 fresh-session review procedure
- Read-only #112 fixture evidence

## Inputs

- GitHub issue #292 typed-read body
- /Volumes/FastWork/adl-worktrees/adl-issue-112-layer8-authority-preparation-v2 as read-only fixture evidence
- csdlc-v2 csdlc-edit implementation and tests
- existing card identity tests and gate2 identity preservation tests

## Non Goals

- Do not mutate #112 lifecycle, branch, worktree, cards, implementation, review, or publication.
- Do not implement #112 product behavior.
- Do not widen or replace #291 initialized-phase recovery.
- Do not directly edit rendered Markdown or C-SDLC state files.
- Do not merge or close #292.
