# Structured Intent Prompt

Template: 1.0.0

Issue: 17

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make doctor fail closed when typed issue plans cannot execute from their declared repository, ownership, validator, or test-denominator contract.

## Required Outcome

Doctor emits deterministic repair findings for all four false-ready conditions demonstrated by issue 5795.

## Scope

- csdlc-v2 repository identity diagnosis
- csdlc-v2 owned Rust module routing validation
- csdlc-v2 validator target and issue-specific denominator validation
- focused gate2 regression fixtures

## Authority

- C-SDLC v2 doctor and execution-readiness validation only
- No Runtime or issue 5795 product implementation
- No network access required for diagnosis

## Assumptions

- none

## Operator Constraints

- Use the new agent-logic repository
- Keep active work on FastWork
- Run only focused proving validation
