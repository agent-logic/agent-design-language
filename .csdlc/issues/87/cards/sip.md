# Structured Intent Prompt

Template: 1.0.0

Issue: 87

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Repair the ACIP minor-version compatibility predicate so Sprint 4 strict Clippy lanes pass without weakening negotiation.

## Required Outcome

ACIP accepts exactly the supported major and every valid inclusive minor range containing the local minor, rejects malformed and unsupported ranges, and passes both named strict Clippy targets.

## Scope

- adl-runtime/src/acip.rs negotiation predicate
- Focused ACIP positive and negative unit coverage
- Issue-local C-SDLC lifecycle evidence

## Authority

- Issue 87 owns only the shared ACIP predicate and focused colocated tests
- Sprint children 5866, 5871, and 5872 retain exclusive ownership of their implementation and integration-test modules
- Publication closes only agent-logic/agent-design-language#87

## Assumptions

- none

## Operator Constraints

- Never write tracked issue changes on main
- Use typed C-SDLC v2 lifecycle tools
- Do not use AWS or gh
- Do not merge without explicit current authorization
