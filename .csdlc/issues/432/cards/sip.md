# Structured Intent Prompt

Template: 1.0.0

Issue: 432

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make .adl strictly local-only while preserving worktree-placement enforcement outside .adl.

## Required Outcome

Zero tracked .adl paths, zero active repository dependencies on .adl authority, relocated worktree policy, and deterministic regression guards.

## Scope

- The exact 27 tracked .adl paths
- All active tracked references that require .adl authority
- Worktree policy relocation and focused guards

## Authority

- .csdlc remains lifecycle authority
- config/worktree-policy.json becomes worktree placement authority
- Historical evidence may mention .adl but cannot act as executable authority

## Assumptions

- none

## Operator Constraints

- Never delete unrelated operator-local .adl content
- Do not publish logs, credentials, provider output, or private state
- Use the new agent-logic repository and typed v2 lifecycle
