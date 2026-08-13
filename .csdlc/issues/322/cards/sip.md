# Structured Intent Prompt

Template: 1.0.0

Issue: 322

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Repair the advertised-but-stubbed adl-review read-only review compatibility routing needed by Sprint 6 CodeFriend/tooling closeout.

## Required Outcome

Advertised adl-review read-only review commands either execute through current supported code or the help surface is truthfully narrowed, with focused tests proving no removed v1 lifecycle multiplexer is used.

## Scope

- adl-review compatibility binary dispatch
- read-only repository review contract verification
- CodeFriend/CodeBuddy deterministic smoke route
- focused compatibility tests

## Authority

- Typed C-SDLC v2 remains lifecycle authority
- No sunset v1 lifecycle wrappers are revived
- No provider credentials or hosted model calls are executed
- No active WP-18C issue scope is touched

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Keep primary main checkout clean except unavoidable typed bootstrap state
- Stop before publication/merge unless separately authorized
