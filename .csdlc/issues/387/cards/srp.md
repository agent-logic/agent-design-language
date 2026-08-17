# Structured Review Prompt

Template: 1.0.0

Issue: 387

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/387
csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs

## Prompts

- Verify the implemented-phase repair route is narrow and does not weaken reviewed/published/publication guards.
- Verify the regression covers the #114-shaped sequence and negative guard behavior.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Only the requested focused test was rerun; broader Gate 5 and workspace validation were not repeated.
- Subsequent PASS metadata must be handled by typed lifecycle exact-revision authority rather than assumed to preserve prior review.

## Review Result

Revision: Some("git-blake3:1190fea9976e9e71ce287c821d5d45309de1d5f3:5ab2778d30294bbb5a9ccef78efecb6be1f5038ed3a968a6225cee61c762215f")

Reviewer: Some("fresh-session:13dcbdf2-6f65-46a1-8aa2-48c6d70c72d4")

Result: pass
