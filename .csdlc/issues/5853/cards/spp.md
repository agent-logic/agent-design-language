# Structured Planning Prompt

Template: 1.0.0

Issue: 5853

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Use the seven retained 16-core measurements to select the test-only command structure, route adl-rust-tests to the 16-core runner, remove the dispatch-only experiment harness, and require the implementation PR as the production canary.

## Plan

Revision 14

## Steps

[
  {
    "id": "S1",
    "action": "Verify migration, budget, runner-group, selected-repository, bounded maximum concurrency ten, and rollback gates",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Retain one cold, three warm, and three test-only 16-core measurements and recompute the bounded decision",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Route adl-rust-tests and heavy Rust validation producers to the selected runner, preserve lightweight orchestration on standard runners, validate, and require a green direct-test production canary",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Heavy Rust validation producers use the selected 16-core runner pool capped at ten; lightweight orchestration remains on standard GitHub-hosted runners and the standard runner remains the immediate fallback
- Exact-head validation and proof quality are independent of runner class
- Required-check names and branch protection remain stable
- No tracked work occurs on main
- No sample or error is silently discarded
- Issue-selected validation runs before merge; ordinary main pushes do not repeat paid validation, and scheduled or manual full validation remains explicit

## Risks

- Queue latency erases execution gains
- Cache asymmetry produces a false speedup
- Paid runner access or secrets widen beyond the selected repository
- A canary changes required-check identity or proof semantics
- Cost exceeds the owner-approved cap
- Migration or CI instability confounds the experiment

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5853/design.md

Digest: 366401d1c5744adc5ed02ba96780df2f4f754d83f7c264916054be203a723965

## Diagram

.csdlc/prepared/issues/5853/diagram.mmd

Digest: 17d5a853df6d07718c8495e9076f622572dea08eba574fccccf450a39131a966

## Stop Conditions

- WP-02 or WP-02A entry evidence is incomplete
- Budget, alerts, selected-repository access, or rollback cannot be verified
- The comparison inputs cannot be held constant
- Proof or artifact parity fails
- Untrusted code can reach privileged runner context
- Protected-path collision

## Handoff

Proceed only after doctor readiness.
