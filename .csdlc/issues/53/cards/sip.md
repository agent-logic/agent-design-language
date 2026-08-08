# Structured Intent Prompt

Template: 1.0.0

Issue: 53

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make tracked execution-proof receipts bind distinct substantive and evidence revisions without weakening exact-revision or digest verification.

## Required Outcome

A committed receipt names machine-verified substantive and evidence revisions with valid ancestry and evidence-only intervening changes, while all existing proof digests remain fail closed.

## Scope

- .csdlc/prepared/issues/5862/proof-receipt-contract.rb
- .csdlc/prepared/issues/53/test-proof-receipt-contract.rb
- .csdlc/prepared/issues/53
- .csdlc/issues/53

## Authority

- Issue and code authority are agent-logic/agent-design-language#53
- Retained v2 receipts remain immutable and keep their existing validation semantics
- Only the shared WP-04 proof receipt contract and issue-local regression are implementation scope
- No distributed Runtime product behavior is owned

## Assumptions

- none

## Operator Constraints

- Never write on main
- Do not weaken exact revision or artifact digest verification
- Use the typed C-SDLC v2 lifecycle
- Publish only after exact-head independent review
