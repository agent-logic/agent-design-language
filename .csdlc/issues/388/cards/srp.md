# Structured Review Prompt

Template: 1.0.0

Issue: 388

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
focused csdlc-v2 regression tests
.csdlc/issues/388
.csdlc/prepared/issues/388/validate_preparation_bundle.py
.csdlc/evidence/388

## Prompts

- Are the new repair operations narrow and field-specific?
- Do guards fail closed on stale CAS and active downstream truth?
- Does the #114-like sequence pass without weakening #363 behavior?
- Are SOR follow-up replacement, empty-vector removal, and blank-entry refusal explicitly covered?
- Are audit records complete and append-only?

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
