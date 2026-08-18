# Structured Intent Prompt

Template: 1.0.0

Issue: 415

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make immutable builder preflight failures individually attributable and durably retained before another #268 paid attempt.

## Required Outcome

#415 is merged and terminally finished with exact retained builder preflight diagnostics, preserving cleanup and blocking all paid retries until completion.

## Scope

- Builder image preflight diagnostics and focused regression tests.
- Minimum remote-runner wiring needed to retain early-failure diagnostics.

## Authority

- No AWS paid launch or provider mutation.
- No #268 or #269 lifecycle mutation.
- Preserve immutable builder image, source revision, and exact-owner cleanup policy.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 only.
- Implement in a bound FastWork worktree.
- Obtain fresh independent design and exact-head implementation reviews.
- Publish and finish terminal only when all gates are green.
