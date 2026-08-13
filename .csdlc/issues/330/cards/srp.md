# Structured Review Prompt

Template: 1.0.0

Issue: 330

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/projection_cleanup.rs
csdlc-v2/src/projection_recovery.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/issue_330_bridge_cleanup_defect.rs
.csdlc/issues/330
.csdlc/evidence/330/r2

## Prompts

- Does the recovery validator accept post-cleanup retained attempts only under exact cleanup authority?
- Does the cleanup final-receipt race reject before mutation and preserve byte-exact state?
- Are #299 cleanup authority checks preserved or strengthened?
- Does the #300 bridge-fed target pass without synthetic authority?

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
