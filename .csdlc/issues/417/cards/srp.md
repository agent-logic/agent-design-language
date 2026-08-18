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

- Hosted CI rerun, merge, terminal finish, installation, and #414 resumption remain future gates.

## Review Result

Revision: Some("git-blake3:628c96282b8eeefc9713ef1b54c62614d7460a48:69278fb87097e50b93500f2d128deaf76734a2379baf71fcf8382dcb90268fa7")

Reviewer: Some("fresh-session:805294ca-589b-4159-8cf0-1caf61678101")

Result: pass
