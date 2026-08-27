# Structured Intent Prompt

Template: 1.0.0

Issue: 558

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Stabilize the governed learner replication test so the adl-runtime coverage profile is no longer blocked by instrumentation-sensitive timing.

## Required Outcome

The existing four-node learner replication proof passes deterministically without changing governed learner transport product semantics.

## Scope

- adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
- .csdlc/issues/558
- .csdlc/prepared/issues/558
- .csdlc/evidence/558

## Authority

- Typed C-SDLC v2 controls issue lifecycle, publication, and finish.
- Runtime learner authorization, membership, append routing, and product transport semantics are not changed by this issue.
- The issue exists as a shared gate for #499 and #514 only.

## Assumptions

- none

## Operator Constraints

- Do not touch #483, Sprint 2, or Sprint 3.
- Do not edit tracked main.
- Use a new FastWork issue worktree.
- Obtain independent OpenAI Responses API exact-head review before publication/finish.
