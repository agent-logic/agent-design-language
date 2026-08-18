# Structured Review Prompt

Template: 1.0.0

Issue: 417

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/417
.csdlc/prepared/issues/417
.csdlc/evidence/417
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs

## Prompts

- Does eligibility bind to the originating recover_review recovery epoch rather than any historical recovery?
- Are all supported intervening operations bounded explicitly and unsafe operations rejected?
- Do tests prove recover_design_review followed by authored refresh and downstream authority clearing?
- Do immediate and iterative refresh paths retain compatibility?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI, merge, terminal finish, owner-binary installation, and #414 resumption remain future gates.

## Review Result

Revision: Some("git-blake3:646e8bf81ca89d9ef1c8a3ee44c1e7da463238b4:4472c6b1c174b70c5663ef0ae34383c5c4977297f8f783d7a8951e3d3daeb4fb")

Reviewer: Some("fresh-session:26ed0de7-6bd6-48f0-aa48-642a9ee19634")

Result: pass
