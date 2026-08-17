# Structured Review Prompt

Template: 1.0.0

Issue: 387

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/387
.csdlc/prepared/issues/387

## Prompts

- Verify the implemented-phase repair route is narrow and does not weaken reviewed/published/publication guards.
- Verify the regression covers the #114-shaped sequence and negative guard behavior.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review validation was intentionally limited to the permitted focused regression and Clippy; the complete crate test suite was not run.

## Review Result

Revision: Some("git-blake3:5572ce98b8d859cb55d1f518d36ed871c948ce2b:b6af6fa5650526cd0e2441c5b2a284ac579bc3ec58fbee1702f59a632e11df27")

Reviewer: Some("fresh-session:fad332cc-e5b4-4f8c-9fbb-0924645f56b7")

Result: pass
