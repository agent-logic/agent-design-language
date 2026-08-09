# Structured Intent Prompt

Template: 1.0.0

Issue: 5854

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate real demonstrations, consumer proofs, governance handoff, proof coverage, and complete launch media.

## Required Outcome

A reviewable sprint coordination lane can route #5835, #5836, #5838, #5839, #5840, #5844, #5845 through their own typed lifecycles without scope collision or false completion.

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
- Never use /private/tmp
- Use repo-native GitHub tools only
- The umbrella coordinates child sessions and never implements child code
- Every child session reads AGENTS.md, binds its own worktree, and creates its own goal before implementation
