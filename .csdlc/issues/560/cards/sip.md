# Structured Intent Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Stabilize the three runtime_v2 unified-runtime-kernel tests that time out only under workspace ci-coverage instrumentation.

## Required Outcome

Workspace coverage no longer fails on the three exact runtime_v2 unified-runtime-kernel 120s timeout cases without changing Runtime v2 semantics.

## Scope

- adl/.config/nextest.toml
- .csdlc/issues/560
- .csdlc/prepared/issues/560
- .csdlc/evidence/560

## Authority

- Typed C-SDLC v2 controls issue lifecycle, publication, and finish.
- Runtime v2 event correlation, event order drift, summary drift, participant drift, and product semantics are not changed by this issue.
- The issue exists as a shared workspace coverage gate repair for #514 and related Sprint 1 coverage work.

## Assumptions

- none

## Operator Constraints

- Do not touch #483, Sprint 2, or Sprint 3.
- Do not edit tracked main.
- Use a new FastWork issue worktree.
- Obtain independent OpenAI Responses API exact-head review before publication/finish.
