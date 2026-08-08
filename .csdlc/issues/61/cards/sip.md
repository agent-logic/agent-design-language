# Structured Intent Prompt

Template: 1.0.0

Issue: 61

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make bind topology classification interpret retained relative worktree paths against canonical repository topology and skip unrelated historical records before artifact verification.

## Required Outcome

A #5791-shaped unrelated dot-worktree record with absent local artifacts no longer blocks a distinct bind, while genuine issue, branch, and canonical-worktree conflicts still fail closed with typed contextual diagnostics.

## Scope

- csdlc-v2/src/lifecycle.rs bind topology relevance classification
- csdlc-v2/tests/gate2.rs focused real-binary regression
- csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md only if operator-facing behavior changes

## Authority

- Git branch and registered canonical worktree topology remain lifecycle ownership authority
- Retained historical issue records remain immutable evidence
- Full card and authored-artifact verification remains mandatory for records relevant by issue, branch, or canonical worktree

## Assumptions

- none

## Operator Constraints

- Use only typed C-SDLC v2 lifecycle commands
- Never write tracked issue work on main or use /private/tmp
- Do not use AWS or broad repository test suites
- Keep implementation bounded to issue #61 and close the PR with Closes #61
