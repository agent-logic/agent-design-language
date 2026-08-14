# Structured Intent Prompt

Template: 1.0.0

Issue: 369

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Add a typed fail-closed recovery that clears falsely recorded design approval on bound or implemented issues.

## Required Outcome

An exact CAS-guarded operation preserves audit/topology, sets current design review pending, and grants no replacement lifecycle authority.

## Scope

- Typed recovery request and schema
- Store transition and csdlc-edit CLI dispatch
- Focused bound/implemented success and refusal regressions

## Authority

- Exact prior approval identity and revision
- Bound or implemented only with no later review/publication authority
- No generic audit or card mutation

## Assumptions

- none

## Operator Constraints

- Blocks #275 and #205
- No Runtime product paths
- Preserve false audit event and append explicit correction
