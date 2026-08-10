# Structured Intent Prompt

Template: 1.0.0

Issue: 141

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Close the two remaining PR #120 findings with committed terminal truth and fail-closed strict-Clippy receipt validation.

## Required Outcome

Issue #5909 records the live merged and closed outcome, and the shared proof contract rejects digest-only Clippy artifacts unless an exact successful structured command is present.

## Scope

- csdlc-v2/src/store.rs
- .csdlc/prepared/issues/5862/proof-receipt-contract.rb
- .csdlc/prepared/issues/5909/validate-proof-receipt.rb
- .csdlc/prepared/issues/141
- .csdlc/evidence/141
- .csdlc/issues/141
- .csdlc/issues/5909

## Authority

- Issue and code authority are agent-logic/agent-design-language#141
- PR #120 and legacy issue #5909 live state are terminal outcome authority
- No Runtime product code is owned
- Only structured command evidence can establish strict Clippy success

## Assumptions

- none

## Operator Constraints

- Never write on main
- Use typed C-SDLC v2 cards
- Run only focused validation
- Publish after independent exact-head review
