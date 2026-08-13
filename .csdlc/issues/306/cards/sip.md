# Structured Intent Prompt

Template: 1.0.0

Issue: 306

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make csdlc-publish and csdlc-finish agree on a single exact-clean publication tail contract.

## Required Outcome

Publishing a reviewed head cannot leave required local publication metadata after the pushed head in a way that forces another publication cycle or blocks exact-clean finish.

## Scope

- csdlc-v2 publication owner
- csdlc-v2 finish readiness interaction
- focused publication/finish regression tests
- issue-local lifecycle and evidence

## Authority

- No active issue worktree is used as a fixture
- Exact-head review and exact-clean finish remain authoritative
- Publication metadata may be safe only when the contract explicitly proves it
- No broad lifecycle redesign

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2
- Bind beneath /Volumes/FastWork/adl-worktrees
- Do not touch #258, #295, #298, #301, #5913 active worktrees, root staging, locks, or lifecycle
- Fresh exact-head review
- Stop before merge
