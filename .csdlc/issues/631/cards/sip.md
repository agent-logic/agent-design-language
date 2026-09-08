# Structured Intent Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement and test v3 proof, shadow, soak, and install routes under the one csdlc binary.

## Required Outcome

The v3 proof, shadow, soak, and install routes have typed fail-closed behavior, focused regression tests, and no v2 source changes.

## Scope

- csdlc-v3/src
- csdlc-v3/tests
- docs/csdlc-v3
- .csdlc/prepared/issues/631
- .csdlc/evidence/631

## Authority

- C-SDLC v2 remains live operational authority until #505 cutover
- v3 routes are construction evidence only
- No hidden v2 fallback in v3 proof
- No selector or cutover mutation before #505
- No provider or long-running soak side effects before cutover
- No v2 source changes

## Assumptions

- none

## Operator Constraints

- Do not merge #505
- Do not perform cutover
- Do not use raw gh
- Do not use /private/tmp
- Work only from a bound FastWork issue worktree
