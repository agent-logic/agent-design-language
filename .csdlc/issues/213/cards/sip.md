# Structured Intent Prompt

Template: 1.0.0

Issue: 213

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Allow an existing unbound initialized or ready issue to repair STP acceptance criteria and SPP plan steps through typed, audited, atomic C-SDLC v2 operations.

## Required Outcome

The typed card owner supports exact initialized/ready acceptance and plan-step replacement, invalidates stale design approval, preserves lifecycle identity and audit truth, and proves the behavior with focused fail-closed regression tests.

## Scope

- Initialized/ready semantic operation authorization
- Design-review invalidation after acceptance or plan-step repair
- Focused Gate 2 regression proof

## Authority

- Only csdlc-edit apply owns semantic card mutation
- Only STP replace_acceptance_criteria and SPP replace_plan_steps gain initialized/ready authorization
- The existing store transaction, CAS, cross-card validator, renderer, and audit remain authoritative
- Binding, execution, review, publication, merge, and terminal authority are unchanged

## Assumptions

- none

## Operator Constraints

- Do not bind or mutate #205 before #213 is reviewed and merged ancestrally
- Never hand-edit rendered cards or values state
- Use the typed C-SDLC v2 lifecycle
- Open a ready PR and stop before merge
