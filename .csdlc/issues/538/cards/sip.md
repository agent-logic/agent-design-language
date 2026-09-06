# Structured Intent Prompt

Template: 1.0.0

Issue: 538

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prepare and coordinate the exact eleven-child v0.92.1 release tail from admission through release ceremony.

## Required Outcome

Canonical planning, live issue membership, typed child readiness, sequential dependency gates, watcher policy, and first-child handoff agree without claiming unresolved dependencies are satisfied.

## Scope

- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- docs/milestones/v0.92.1/evidence/integration/sprint-10/**
- .csdlc/issues/516 through .csdlc/issues/526 readiness truth
- .csdlc/prepared/issues/516 through .csdlc/prepared/issues/526
- .csdlc/issues/538/**
- .csdlc/prepared/issues/538/**

## Authority

- Live issue #538 membership version 7 is the sprint roster authority
- Each child issue remains the authority for its own result and acceptance criteria
- Typed C-SDLC v2 remains lifecycle authority until explicit #505 cutover
- A predecessor reviewed green merge is required before its successor executes

## Assumptions

- none

## Operator Constraints

- Do not implement child work inside #538
- Do not start #516 while any declared admission prerequisite is open
- Do not start later children before their immediate predecessor has merged green
- Do not treat typed finish or worktree cleanup as a successor execution dependency
- Preserve stale or concurrently owned worktrees untouched
