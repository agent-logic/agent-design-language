# Structured Intent Prompt

Template: 1.0.0

Issue: 689

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the current-generation Rust CSM route the sole documented permanent Runtime control path and prevent legacy shell commands from claiming authority.

## Required Outcome

Operators receive accurate canonical start, status, stop, reload, ownership, and identity guidance, while legacy CSMctl Runtime verbs refuse with a concise migration message.

## Scope

- START_CSM_RUNBOOK canonical command and path correction
- legacy CSMctl Runtime verb refusal
- Observatory-only legacy command preservation
- small deterministic routing and docs guards

## Authority

- No changes to canonical Rust ownership semantics
- No live Runtime or launchd mutation
- No cloud provider agent model edge or Observatory UI changes
- No tracked edits on main

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2
- Bind a FastWork issue worktree
- Keep the live Runtime running
- Prefer a direct refusal over delegation complexity
