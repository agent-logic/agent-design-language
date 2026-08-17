# Structured Review Prompt

Template: 1.0.0

Issue: 388

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/388
.csdlc/prepared/issues/388
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

- Validation was assessed from committed evidence rather than rerun in the read-only review session.
- The empty issue lock is lifecycle binding metadata outside the substantive review surface.

## Review Result

Revision: Some("git-blake3:32cb34b353c4e7b4347fdef359083337cab3f3d2:6d89c521962edcc8d1ae2c0dab15a3f0b19837fc71dddb2c94d9b258b450e8cb")

Reviewer: Some("fresh-session:da20f923-1403-48e4-b279-34ce98bd558f")

Result: pass
