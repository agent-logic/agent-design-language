# Structured Intent Prompt

Template: 1.0.0

Issue: 513

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one independently owned Runtime v2 and Runtime v3 authority topology with executable migration and rollback proof while excluding Runtime v4.

## Required Outcome

One accepted Runtime v2/v3 source and authority topology, backed by executable source-denominator, reverse-reference, compatibility, rollback, and migration checks.

## Scope

- adl/src/runtime_v2/**
- adl-runtime/**
- adl-runtime-kernel/**
- docs/runtime/**
- docs/milestones/v0.92.1/evidence/runtime-decoupling/**
- .csdlc/prepared/issues/513/**
- .csdlc/issues/513/**

## Authority

- Runtime v2 authority remains under adl/src/runtime_v2/**.
- Runtime v3 authority remains under adl-runtime/** and adl-runtime-kernel/**.
- Shared docs and DEC-01 evidence record topology and proof only; they do not move runtime authority silently.
- Runtime v4 is excluded and any v4 requirement stops the issue for replanning.
- Issue #513 does not absorb #483 or sibling Sprint 1 work.

## Assumptions

- none

## Operator Constraints

- Use the canonical FastWork issue worktree.
- Do not touch tracked main.
- Do not touch #483.
- Use typed C-SDLC v2 routes only for lifecycle writes.
- Publish with Closes #513 and keep CI green before merge.
