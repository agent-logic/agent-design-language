# Structured Intent Prompt

Template: 1.0.0

Issue: 686

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Give Runtime configuration one immutable generation receipt and atomic active reference shared by CSM, Guardian, kernel, status, and readiness.

## Required Outcome

Every Runtime launch and reload participant validates and reports one committed configuration generation and digest, with deterministic crash recovery and prior-generation restoration.

## Scope

- Runtime configuration-generation receipt and atomic active reference
- CSM Guardian kernel status and readiness generation propagation
- isolated failpoint and prior-generation recovery tests

## Authority

- No live Runtime rollout restart or mutation
- No binary installer or stable-route redesign
- No convergence-policy or provider behavior changes
- No tracked edits on main

## Assumptions

- none

## Operator Constraints

- Reuse merged #589 transaction primitives and #678 generation routing
- Use typed C-SDLC v2
- Bind a FastWork issue worktree
- Use deterministic isolated fixtures only
