# Structured Review Prompt

Template: 1.0.0

Issue: 292

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Exact-head tooling implementation review for csdlc-edit operation, predicates, audit, atomic all-six-card update, tests, and lifecycle evidence.

## Prompts

- Verify the operation cannot run outside the intended implemented-phase pre-review/pre-publication/pre-readiness/pre-terminal window and rejects incompatible latest review-related audit state.
- Verify live issue evidence binds the requested title and sibling-scope claims are rejected.
- Verify all six card values update atomically and no non-identity content changes.
- Verify #112 fixture use is isolated and read-only.
- Verify tests cover stale CAS, phase/review/publication/readiness/terminal rejects, incompatible latest review-related audit state, malformed or sibling identity rejects, evidence mismatch, audit fields, and validation.

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
