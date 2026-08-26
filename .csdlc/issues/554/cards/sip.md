# Structured Intent Prompt

Template: 1.0.0

Issue: 554

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Restore required workspace coverage baseline blocking #514 PR #549 without weakening coverage or hiding failures.

## Required Outcome

A reviewed and merged #554 repair PR makes the shared coverage gate green so #549 can be rerun truthfully.

## Scope

- docs/milestones/v0.92/README.md
- adl/src/runtime_v2/**
- adl/tests/memory_palace_tests.rs

## Authority

- Do not touch #483.
- Do not change #514 provider-profile behavior.
- Do not weaken required coverage, skip failing tests, or hide failures.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 routes.
- Use a bound FastWork issue worktree.
- Merge only after review and green required checks.
