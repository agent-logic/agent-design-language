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
.csdlc/evidence/330/r3

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

Revision: Some("git-blake3:399b3103340a33c4d5c8a243b73556118a8a3c94:62602bab709fcc192ed27adcbb7f4a882f7fe8ad7a7b528b4a02a06d4ec9a518")

Reviewer: Some("fresh-session:330-r3-exact-implementation-review")

Result: pass
