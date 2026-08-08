# Structured Intent Prompt

Template: 1.0.0

Issue: 18

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make split C-SDLC schema and machine-readable output terminate cleanly when a downstream stdout reader closes early.

## Required Outcome

The shared split-binary output path treats EPIPE as normal termination without panic text while preserving JSON stdout and real-error behavior.

## Scope

- Shared C-SDLC v2 machine-readable stdout writer
- Split GitHub issue and PR binaries
- Focused broken-pipe process regression coverage
- Machine-output contract documentation

## Authority

- Issue #18 owns only machine-output termination behavior and its focused proof
- GitHub request schemas, action semantics, and credential resolution remain unchanged
- Non-broken-pipe output failures remain fail-closed

## Assumptions

- none

## Operator Constraints

- Never write tracked issue changes on main
- Use only typed C-SDLC v2 lifecycle tools
- Do not use AWS or remote builders
- Do not merge without explicit operator authorization
