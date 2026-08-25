# Structured Intent Prompt

Template: 1.0.0

Issue: 317

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce the exact non-mutating v0.92 terminal issue universe and acyclic release-tail action plan that enables #318 after #317's reviewed green merge.

## Required Outcome

A reviewed documentation packet with exact row ownership, merge-based dependency gates, asynchronous finish and cleanup routing, and no release mutation.

## Scope

- canonical v0.92 terminal issue universe
- merge-based release-tail dependency DAG
- typed finish and cleanup asynchronous routing
- focused completeness, ancestry, cycle, and negative validation

## Authority

- #316 PR #472 reviewed green merge and ancestry are the predecessor authority
- typed finish and worktree cleanup are asynchronous and non-gating
- #318 and #319 execution, merge, tag, release, closure, and v0.93 activation are outside scope
- legacy #5850 artifacts are provenance only

## Assumptions

- PR #472 merge commit 5002b387b79f2d8dbf41a8c1a99e5a03bcb5c5d5 is ancestral to current main
- canonical v0.92 planning and release-tail documents define the issue denominator

## Operator Constraints

- Bind beneath /Volumes/FastWork/adl-worktrees
- Do not write tracked changes on main
- Do not use /private/tmp
- Do not serialize on closeout or cleanup
- Do not merge without explicit operator authority
