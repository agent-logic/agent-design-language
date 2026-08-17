# Structured Review Prompt

Template: 1.0.0

Issue: 292

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/card_identity.rs
.csdlc/issues/292
.csdlc/prepared/issues/292

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

Revision: Some("git-blake3:ee18d8e987bab6d8971dfda00ce4ce0b76d1a025:7f95158a6170c35f2e96da29328ba351f66125b5ea758bde0fd31a81f56cb19d")

Reviewer: Some("fresh-session:a6458bb4-3c16-4a60-8bcb-ca881691672c")

Result: pass
