# C-SDLC v2 Gate 9 Soak Matrix

| Scenario | Result | Primary evidence |
| --- | --- | --- |
| Docs-only lifecycle | Pass | generated `samples/docs-only/packet.json`; real Store lifecycle/restart test |
| Small Rust lifecycle | Pass | generated `samples/small-rust/packet.json`; real Store lifecycle/restart test |
| Validation failure and retry | Pass | Gate 4 fail→repair→retry test |
| Review finding and repair | Pass | Gate 5 unresolved-finding refusal and resolved-review tests |
| PR-check failure and recovery | Pass | Gate 7 failed/pending distinction and green-readiness tests |
| Merge and closeout | Pass | Gate 7 atomic/idempotent lifecycle test |
| Restart after initialized | Pass | Gate 9 persisted Store reopen/resume test |
| Restart after ready | Pass | Gate 9 persisted Store reopen/resume test |
| Restart after bound | Pass | Gate 9 persisted Store reopen/resume test |
| Restart after implemented | Pass | Gate 9 persisted Store reopen/resume test |
| Restart after reviewed | Pass | Gate 9 persisted Store reopen/resume test |
| Restart after published | Pass | Gate 9 persisted Store reopen/resume test |
| Interruption around atomic merge/closeout | Pass | exact terminal transaction, reopened Store, and idempotent retry test |
| Dirty-worktree refusal | Pass | Gate 7 prune/topology guard test |
| GitHub outage/ambiguous retry | Pass | Gate 6 create/no-op reconciliation plus publish observe-before-retry boundary |

Every row is represented in `SoakScenario`, a Strum-backed closed vocabulary.
`soak-evidence-input.json` contains the exact machine references consumed by
the decision evaluator. Missing, waiting, reference-free, or failed rows cannot
produce `proceed`.
