# Structured Task Prompt

Template: 1.0.0

Issue: 648

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Rebase or replay the local #622 repair packet onto a #648 issue-bound FastWork worktree and publish a corrective PR against current main.

## Deliverables

- Run-scoped provider reload handle wiring for production CSM execution
- Identity-aware compatibility global reload guard
- Overlapping two-workflow and shutdown-order regression tests
- Direct global guard ownership regression test
- Fresh exact-head review and corrective PR readiness evidence

## Acceptance

1. AC-1: Production CSM adl_workflow execution passes an explicit run-scoped ProviderReloadHandle into execution.
2. AC-2: Sequential, deterministic concurrent, retry, and called-workflow paths propagate the scoped handle before any compatibility global fallback.
3. AC-3: Dropping an older compatibility global guard cannot clear a newer global registration.
4. AC-4: Two overlapping workflows with the same provider id and distinct sidecars keep separate provider snapshots.
5. AC-5: Shutting down one workflow's provider reload owner does not clear or alter the other workflow's active reload handle.
6. AC-6: Focused production and safety validation lanes pass with nonzero tests and no live provider credentials.
7. AC-7: Formatting and clippy validation pass locally before review.
8. AC-8: Fresh exact-head independent review passes for the corrective PR head.
9. AC-9: Corrective PR is open against current main with issue-closing linkage for #648 and required CI/checks green before merge-ready.

## Dependencies

- #622 closed by PR #646 at stale head; retained as provenance and not semantic completion
- PR #646 merge commit 290d5e9b9798bd90770b3c6b4b39f5075ada24e8
- Planning #4 owns live Runtime/#640 cutover and live config

## Inputs

- agent-logic/agent-design-language#648
- agent-logic/agent-design-language#622
- agent-logic/agent-design-language#646
- adl/src/provider/reload.rs
- adl/src/execute/mod.rs
- adl/src/execute/runner.rs
- adl/src/execute/tests.rs
- adl/src/long_lived_agent.rs

## Non Goals

- Live Runtime/#640 cutover, restart, stop, replacement, or config mutation
- Credential-backed provider inference or paid provider proof
- Provider architecture redesign beyond scoped reload ownership
- Rewriting unrelated #622 lifecycle history
- Merging without explicit operator authorization
