# Structured Intent Prompt

Template: 1.0.0

Issue: 544

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prevent native C-SDLC issue initialization from writing bootstrap state into the repository primary checkout.

## Required Outcome

csdlc-issue create fails closed when invoked from the Git topology primary checkout and continues to work from isolated non-primary staging checkouts.

## Scope

- csdlc-v2/src/lifecycle.rs
- focused C-SDLC lifecycle tests
- operator documentation for primary-checkout bootstrap topology
- issue-local lifecycle and evidence surfaces

## Authority

- typed C-SDLC v2 binaries remain lifecycle authority
- root main is inspection-only and must not receive tracked issue state
- csdlc-bind remains the only canonical execution worktree binding route

## Assumptions

- none

## Operator Constraints

- Do not write tracked files on the primary main checkout
- Use the isolated staging checkout for initialization
- Use the canonical FastWork issue worktree for implementation after typed bind
- Do not use raw GitHub writes
- Do not migrate or delete existing issue records or other sessions' worktrees
- Do not merge, finish, or clean up without separate operator authority
