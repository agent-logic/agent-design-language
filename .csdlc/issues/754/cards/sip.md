# Structured Intent Prompt

Template: 1.0.0

Issue: 754

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Restore standalone CI registry compatibility while preserving native authority guards.

## Required Outcome

Coherent registries1.0.3/1.0.4 accepted; mismatches rejected; full standalone checks pass.

## Scope

- csdlc-v2/src/registry.rs
- csdlc-v2/tests/gate9.rs
- csdlc-v2/tests/gate10a.rs
- csdlc-v2/tests/gate_github_route_policy.rs

## Authority

- Explicit operator authorization for typed-v2 transition; preserve root native754 state.

## Assumptions

- none

## Operator Constraints

- Separate tooling issue754; no PR753 merge.
