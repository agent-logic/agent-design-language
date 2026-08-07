# Structured Intent Prompt

Template: 1.0.0

Issue: 5905

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make every closed v0.92 issue truthfully terminal under the sole csdlc-finish authority.

## Required Outcome

Add a bounded historical reconciliation operation and use #5800 as the first exact live canary before reconciling the remaining inventory.

## Scope

- Typed csdlc-finish historical reconciliation contract
- Focused terminal authority tests
- Closed v0.92 derived terminal envelopes

## Authority

- csdlc-finish remains the sole terminal authority
- Historical reconciliation is limited to already-terminal live GitHub outcomes
- Routine review and publication gates remain unchanged

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Reconcile #5800 first
- No hand-edited cards or terminal envelopes
- No AWS use
