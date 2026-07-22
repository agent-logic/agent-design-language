# Structured Intent Prompt

Template: 1.0.0

Issue: 5007

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare typed C-SDLC v2 state for later execution of the Memory Palace ADR acceptance follow-on.

## Required Outcome

A later execution session can bind this issue from the issue worktree and either accept the ADR with complete proof or record an operator-approved blocker.

## Scope

- Create minimal typed v2 issue state for #5007.
- Preserve the ADR acceptance dependency on complete #4760 proof.
- Keep this session preparation-only.

## Authority

- Typed C-SDLC v2 artifacts are the preparation authority.
- Legacy .adl task bundle is source context only.
- Receipts are audit-only and non-blocking for this preparation pass.

## Assumptions

- none

## Operator Constraints

- No implementation.
- No PR, prep review, broad tests, raw gh, AWS, or root-main writes.
- Use one issue-bound worktree under /Volumes/FastWork from origin/main.
