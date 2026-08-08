# Structured Intent Prompt

Template: 1.0.0

Issue: 55

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Remove the small-runner serial tail by routing heavyweight hosted coverage aggregation through the established selected 16-core runner contract.

## Required Outcome

The heavyweight coverage aggregation job uses the selected 16-core runner and focused contracts prevent regression without changing stable status, producers, Spot, artifacts, Codecov, or coverage policy.

## Scope

- adl_coverage_hosted runner selection
- Focused CI runner-routing regression contracts
- Typed issue design, diagram, cards, validation, and review truth

## Authority

- Issue #55 owns only the heavyweight hosted coverage aggregation runner route
- The lightweight adl-coverage stable-status aggregator remains unchanged
- Coverage producer, artifact, Codecov, threshold, and test-selection semantics remain unchanged
- No AWS runner route is permitted

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Use only typed C-SDLC v2 lifecycle tools
- Use the existing ADL_HEAVY_RUNNER selector with 16-core fallback
- Preserve Spot as opt-in and do not use AWS
- Treat estimates as reviewable rather than hard implementation limits
