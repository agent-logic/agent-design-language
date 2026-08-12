# Structured Intent Prompt

Template: 1.0.0

Issue: 248

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Remove scheduler-dependent terminal classification when oversized file output and a process deadline compete.

## Required Outcome

Oversized output observably present at deadline arbitration deterministically yields output_limit; ordinary hangs yield timeout; both paths leave no artifacts.

## Scope

- Runtime process-backend deadline arbitration
- file-output limit classification
- process-backend cleanup and parity proof

## Authority

- ProcessBackend remains the sole terminal classifier and process-tree owner.
- The server-owned post-termination output state decides precedence.
- No conversation or authority surface is changed.

## Assumptions

- none

## Operator Constraints

- Keep #244 and PR #247 unchanged.
- Keep #112 authority paths unchanged.
- Use typed C-SDLC v2 lifecycle owners and an issue-bound FastWork worktree.
- Run repeated focused pressure and required Runtime CI only.
