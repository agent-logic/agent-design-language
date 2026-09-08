# Native PR reconciliation repair under #523

The operator authorized repairing publication blockers under #523. Typed `github-pr` created PR #743 from reviewed commit `d842069ac98d2259dd5cd6c2dd9512d96c41b452`, then returned `github_mutation_reconciliation_pending`. A repeated invocation refused to replay the mutation and reported `github_mutation_reconciliation_unavailable`. Authenticated read-only observation confirmed exactly one PR, targeting `main`, with the expected commit and draft state.

The cause is a native adapter mismatch: the PR-create reconciliation owner emits `pull-requests-by-head`, but the real read-only adapter does not recognize that operation. Fake-adapter tests did not exercise the real translation. The fix adds a bounded authenticated read-only lookup by exact head; the existing marker, repository, branch, base, commit, title, body and draft checks still determine reconciliation. The existing durable create intent must reconcile without another PR write.

The focused regression is a deterministic local CPU adapter/contract test with no network or paid resources. It verifies URL construction and rejects unsafe inputs. The live proof is authenticated native reconciliation of the already-created PR; no caller-forged receipt or raw GitHub write is acceptable. Independent review and exact-head publication apply again after the repair commit.


The same path also incorrectly copied the PR number into the reconciliation issue field. The repair retains the requested issue for PR operations and preserves assigned-number behavior for issue creation.

## Executed proof

The repaired native binary replayed the existing durable intent through authenticated read-only reconciliation and returned success for issue #523 / PR #743, with `idempotent_replay: true`. No additional create request was sent. `native-pr-create-reconciliation.json` retains that immutable original-head result. Full native suite: 185 tests passed; Clippy with warnings denied and formatting passed. Independent reviewer `review_523` found no actionable findings and independently ran the two real-adapter construction tests plus restart/identity regressions. The final repair commit requires refreshed review and PR head observation; the original-head receipt is retained as history.
