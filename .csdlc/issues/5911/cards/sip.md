# Structured Intent Prompt

Template: 1.0.0

Issue: 5911

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Recover local disk capacity safely and make FastWork the mandatory parent for future ADL worktrees.

## Required Outcome

Produce a verified FastWork transcript archive without deletion and enforce /Volumes/FastWork/adl-worktrees in typed binding.

## Scope

- local Codex transcript storage diagnosis and verified FastWork archive
- typed C-SDLC v2 worktree-path enforcement
- focused policy documentation and validation

## Authority

- no destructive cleanup without separate operator approval
- typed v2 lifecycle binaries own binding
- Git topology remains issue ownership authority

## Assumptions

- none

## Operator Constraints

- never inspect or use /private/tmp
- all future ADL worktrees live under /Volumes/FastWork/adl-worktrees
- do not prune or relocate existing worktrees
- do not expose transcript content or secrets
