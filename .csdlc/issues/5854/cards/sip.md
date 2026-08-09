# Structured Intent Prompt

Template: 1.0.0

Issue: 5854

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate real demonstrations, consumer proofs, governance handoff, proof coverage, and complete launch media.

## Required Outcome

A reviewed Sprint 5 coordination packet routes prepared #5835, #5836, #5838, #5839, and #5840 without starting them, preserves WP-24 product and GitHub completion while typed closeout remains asynchronous, and records WP-24A as an independent out-of-band stream that cannot gate sprint readiness, execution, review, or closeout.

## Scope

- .csdlc/issues/5854
- .csdlc/prepared/issues/5854
- .csdlc/evidence/5854

## Authority

- Sprint coordination records only; child issues own implementation, evidence, review, publication, and closeout.
- Lifecycle authority is the issue-bound Git branch and worktree recorded by typed C-SDLC v2.
- Historical claim, lease, heartbeat, and protected-path language is compatibility evidence only and must not control execution.
- The operator retains external deployment and publication authority.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Never write tracked changes on main
- Use repo-native GitHub tools only
- The umbrella coordinates child sessions and never implements child code
- Every child session reads AGENTS.md, binds its own worktree, and creates its own goal before implementation
- Use FastWork for child worktrees and substantial generated artifacts.
- Do not execute child deliverables from the umbrella worktree.
- Do not run optional proof jobs or publish private media without explicit authorization.
