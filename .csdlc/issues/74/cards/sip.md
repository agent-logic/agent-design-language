# Structured Intent Prompt

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prove bind ignores unrelated legacy claim-bearing projections while preserving strict collision detection.

## Required Outcome

A real csdlc-bind canary succeeds with an unrelated stale claim-bearing record, but relevant malformed records and true collisions still fail closed.

## Scope

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs

## Authority

- Git branch and canonical worktree remain ownership authority
- Only records relevant by issue, branch, or canonical path receive full verification
- Unrelated historical records remain immutable

## Assumptions

- none

## Operator Constraints

- No AWS
- No unrelated worktree mutation
- Prefer regression-only completion if current main already behaves correctly
- Run one focused Rust target
