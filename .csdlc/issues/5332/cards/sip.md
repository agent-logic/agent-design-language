# Structured Intent Prompt

Template: 1.0.0

Issue: 5332

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare the Unity Observatory ILPP GetDomainName loop follow-up for later execution while preserving occupied #4739 and #4741 worktrees.

## Required Outcome

A typed issue-local preparation packet exists for #5332 and truthfully records that source implementation is blocked until Unity sidecar ownership is reconciled.

## Scope

- Issue-local C-SDLC v2 preparation record for #5332
- Unity ILPP GetDomainName loop problem statement and future validation boundary
- Preservation blocker for occupied #4739 and #4741 sidecar worktrees

## Authority

- Preparation only in this session
- No implementation, PR, review, broad tests, raw gh, AWS, or root-main writes
- Do not modify, stage, commit, reset, or replace #4739 or #4741 worktrees
- Future source edits require fresh ownership and dirty-worktree reconciliation

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- Work only in /Volumes/FastWork/adl-wp-5332 on codex/5332-v0918-unity-ilpp-getdomainname-loop
- No implementation, PR, review, broad tests, raw gh, AWS, root-main writes, or #4739/#4741 mutation
- Commit/push only safe new preparation
