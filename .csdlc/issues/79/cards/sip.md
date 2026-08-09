# Structured Intent Prompt

Template: 1.0.0

Issue: 79

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Remove the circular pre-bind admission deadlock blocking Sprint #5862 children #5866, #5871, and #5872 without weakening false-readiness rejection.

## Required Outcome

An initialized issue may bind only when absent future Rust source and test paths are exact owned deliverables selected by fail-closed lanes with meaningful deferrals and an issue-owned temporary path harness; all undeclared, unroutable, non-proving, or post-bind missing targets still fail.

## Scope

- C-SDLC v2 readiness validation for issue-owned Rust targets
- Focused Gate 2 admission and false-readiness regression fixtures
- Sprint #5862 child shapes for #5866, #5871, and #5872

## Authority

- Issue #79 owns only C-SDLC v2 admission logic and focused proof
- Distributed Guardian product implementation remains owned by children #5866, #5871, #5872, and integration child #5878
- The temporary harness exception cannot register production modules or waive post-bind proof

## Assumptions

- none

## Operator Constraints

- Never write tracked issue changes on main
- Use only typed C-SDLC v2 lifecycle tools
- Work only on codex/79-bind-safe-deferred-targets in the required FastWork worktree
- Obtain exact-head independent subagent review before publication
- Publish one ready PR and do not merge
