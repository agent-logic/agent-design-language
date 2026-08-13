# Structured Intent Prompt

Template: 1.0.0

Issue: 323

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Add a bounded typed C-SDLC v2 recovery operation for active bound issue identity/repository migration so split-authority lifecycle defects can be repaired without hand-editing records.

## Required Outcome

C-SDLC v2 exposes a fail-closed owner operation that can migrate an active bound lifecycle record from an incorrect issue identity to a canonical current-repo issue identity while preserving provenance and finish invariants.

## Scope

- C-SDLC v2 issue owner command surface
- Bound issue identity/repository migration request schema
- Record/card projection migration and provenance
- Regression coverage for the live #5913 -> #322 recovery shape

## Authority

- Typed C-SDLC v2 remains lifecycle authority
- No raw .csdlc state edits
- No weakening of csdlc-finish canonical identity checks
- No merge of PR #320 until the new recovery route lands and is used

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Keep #5913 PR #320 green state preserved
- Do not touch #112, #298, projection_recovery.rs, store.rs, or gate5.rs
