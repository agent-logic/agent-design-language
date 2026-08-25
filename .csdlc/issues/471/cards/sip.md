# Structured Intent Prompt

Template: 1.0.0

Issue: 471

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Resolve all ten confirmed Runtime v3 kernel architecture findings with enforced contracts and executable proof.

## Required Outcome

Runtime v3 rejects invalid wiring before spawn, enforces determinism and backpressure, starts and stops by dependency layer, supervises failures with bounded policy, and exposes truthful aggregate health.

## Scope

- adl-runtime-kernel component contracts and context
- kernel channel construction and metrics
- topology lifecycle and staged shutdown
- supervision policy and capability degradation
- Runtime health projection
- focused kernel tests and bounded architecture documentation

## Authority

- Issue #471 owns only Runtime v3 kernel contract and proof surfaces
- WP-27 issue #315 remains untouched and unmerged
- No cloud, provider, distributed consensus, or Runtime v4 authority
- Existing valid Runtime assemblies remain supported through an explicit contract migration

## Assumptions

- none

## Operator Constraints

- Fix every confirmed finding now
- Prefer the simplest truthful kernel-owned mechanism
- Use typed C-SDLC v2 in a bound FastWork worktree
- No unresolved test failures
- Obtain independent exact-head review before publication
