# Structured Task Prompt

Template: 1.0.0

Issue: 544

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Initialize, design-review, bind, implement, validate, independently review, and publish issue #544 without merging or finishing.

## Deliverables

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/primary_checkout_bootstrap_guard.rs
- docs/onboarding.md
- csdlc-v2/README.md
- .csdlc/prepared/issues/544/design.md
- .csdlc/prepared/issues/544/diagram.mmd

## Acceptance

1. AC-1: csdlc-issue create detects the Git topology primary checkout rather than relying on branch-name heuristics.
2. AC-2: initialization from the primary checkout fails before creating design, diagram, issue, prepared, or lock surfaces.
3. AC-3: initialization from a non-primary checkout sharing the same Git common directory remains supported.
4. AC-4: existing initialized-record reconciliation in an isolated checkout remains idempotent.
5. AC-5: focused tests prove primary rejection, zero residue after rejection, isolated-checkout success, fail-closed ambiguous topology, and unchanged bind policy.
6. AC-6: operator documentation says the primary checkout is inspection-only and bootstrap uses an isolated staging checkout.

## Dependencies

- GitHub issue #544 created through typed csdlc-github-issue
- existing C-SDLC v2 lifecycle and bind topology code
- FastWork worktree policy

## Inputs

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/code_repository_migration.rs
- csdlc-v2/README.md
- docs/onboarding.md
- .adl/worktree-policy.json

## Non Goals

- migration of existing issue records
- cleanup of other sessions' worktrees
- GitHub issue semantic changes
- raw GitHub writes
- merge, finish, or cleanup
