# Structured Intent Prompt

Template: 1.0.0

Issue: 693

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make model-backed A2A initiation a reliable first-class Runtime action rather than an exact-JSON roleplay convention.

## Required Outcome

Production conversation ingress selects a typed bounded A2A action from ordinary model-style provider output, the Runtime validates and dispatches it through existing authority, the recipient executes, and correlated activity is observable.

## Scope

- Runtime provider conversation prompt and output boundary
- governed A2A initiation bridge
- isolated live-style end-to-end acceptance
- focused compatibility tests

## Authority

- Runtime constructs and validates initiation authority
- Preserve Layer8 admission eligibility replay cancellation and correlation semantics
- No live Runtime mutation
- No cloud or paid provider execution
- No tracked edits on main

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2
- Bind beneath /Volumes/FastWork/adl-worktrees
- Test until production-ingress behavior works reliably
- Do not overlap #686 or #689
