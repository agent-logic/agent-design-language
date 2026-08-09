# Structured Review Prompt

Template: 1.0.0

Issue: 73

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md
.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd
.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_ARCHITECTURE.md
.adl/docs/TBD/CSDLC_V3_RUST_PLAN_REVIEW.md

## Prompts

- Does the architecture genuinely simplify C-SDLC v2 instead of combining existing binaries behind a dispatcher?
- Does every proposed implementation issue have a complete independent proof boundary and correct dependency ordering?
- Are state, transaction, async, cancellation, Git, GitHub, migration, and recovery semantics correct and non-overstated?
- Are any architectural decisions deferred in a way that would force implementation issues to invent scope?
- Can the plan reach cutover without dual authority and defer v2 deletion to a separately authorized issue?

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
