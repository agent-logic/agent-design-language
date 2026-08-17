# Structured Review Prompt

Template: 1.0.0

Issue: 387

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/387
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

- Fresh reviewer performed focused exact-head source/evidence review; broader CI remains required after republishing PR #389.
- Reviewer started focused gate5 validation and observed the #387 regression pass before stopping to return the bounded immediate verdict.

## Review Result

Revision: Some("git-blake3:6e05f1c11124dc1d60122562861c38dea6f249af:c944f98d5992f93212abd0bac4448034f9ce166754d40965a8e7f01450d55d56")

Reviewer: Some("fresh-session:02ce1362-e859-4102-8014-c8773758f08e")

Result: pass
