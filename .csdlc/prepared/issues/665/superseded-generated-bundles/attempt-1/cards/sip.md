# Structured Intent Prompt

Template: 1.0.0

Issue: 665

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide a fail-closed typed C-SDLC v2 recovery operation that adopts a verified issue-owned emergency branch/worktree into lifecycle authority without weakening implementation, review, or publication gates.

## Required Outcome

An operator can submit a typed adoption request for an existing issue branch/worktree; the bind owner verifies exact repository, issue, branch, worktree, HEAD, base ancestry, checkout ownership, and collision state; records immutable adoption evidence; and advances only to the truthful bound phase.

## Scope

- csdlc-v2/src/bind.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/model.rs
- csdlc-v2/src/bin/csdlc-bind.rs
- csdlc-v2/tests/**
- docs/tooling/**
- .csdlc/prepared/issues/665/**
- .csdlc/issues/665/**

## Authority

- Issue authority is agent-logic/agent-design-language#665
- Typed C-SDLC v2 remains the lifecycle authority
- The operation may adopt only verified issue-owned emergency branch/worktree topology
- Implementation finalization, exact-head review, publication, merge, and closeout remain separate required gates
- Issue #660 product changes and live AWS mutation are out of scope

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Do not reset, force checkout, rebase, merge, overwrite, or copy product work through main
- Do not use raw gh as a lifecycle write workaround
- Do not weaken publication or exact-head review guards
- No live cloud mutation is required
