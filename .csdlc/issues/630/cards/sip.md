# Structured Intent Prompt

Template: 1.0.0

Issue: 630

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement and test v3 terminal truth, safe cleanup, and cutover decision routes under the one csdlc binary.

## Required Outcome

The v3 finish, clean, and cutover routes have simple fail-closed typed behavior, focused regression tests, real-issue canary coverage, and no v2 source changes.

## Scope

- csdlc-v3/src
- csdlc-v3/tests
- docs/csdlc-v3
- .csdlc/prepared/issues/630
- .csdlc/evidence/630

## Authority

- C-SDLC v2 remains live operational authority until #505 cutover
- v3 routes are construction evidence only
- No live GitHub mutation through v3
- No live issue finish through v3
- No operational worktree cleanup through v3 before cutover
- No v2 source changes

## Assumptions

- none

## Operator Constraints

- Do not merge #505
- Do not perform cutover
- Do not use raw gh
- Do not use /private/tmp
- Work only from a bound FastWork issue worktree
