# #369 Bound False Design-Review Recovery

## Boundary

Add one typed owner operation to `csdlc-edit` and the store for bound or implemented records whose current design approval is known false. It owns only the minimum request/schema, store transition, CLI dispatch, and focused gate regression paths. It does not touch Runtime product code or #275 product paths.

## Request

`recover-design-review` requires: issue, exact expected phase (`bound` or `implemented`), expected generation and digest, exact previous approved reviewer and revision, false reviewer identity, actor, reason, and disposition. The false reviewer must equal the previous reviewer. Empty values fail closed.

## Transition

Under the issue lock, recover interrupted state, load and verify canonical cards and record, compare exact phase/generation/digest and approved reviewer/revision, and reject review assignment/result, publication, readiness, migration, terminal state, or topology drift. Set only current `design_review` to pending. Increment generation, update card identities and projections atomically, and append an explicit `recover_design_review` audit event containing previous approval, false reviewer, and disposition. Preserve every prior audit entry and branch/worktree topology.

Repeated correction fails because current design review is no longer approved. No replacement reviewer or approval is accepted by this operation.

## Proof

The exact focused denominator is these four literal `gate2` cases:

1. `bound_design_review_recovery_clears_false_approval`
2. `implemented_design_review_recovery_clears_false_approval`
3. `design_review_recovery_rejects_invalid_authority_and_repeat`
4. `design_review_recovery_matches_issue_275_shape`

`.csdlc/prepared/issues/369/run_exact_focused_matrix.py` fails closed unless
`cargo test --test gate2 -- --list` exposes exactly those four names matching
the recovery contract and each exact-filter invocation reports one run, one
pass, zero failures, and zero ignored tests. The cases prove bound and
implemented success; stale generation/digest; wrong phase;
reviewer/revision mismatch; empty reason/disposition; repeated correction;
review/publication/terminal authority rejection; topology preservation;
append-only audit; and exact #275 recovery shape. Existing initialized
decomposition recovery remains unchanged.

`.csdlc/prepared/issues/369/validate_exact_scope.py` evaluates the immutable
base-to-HEAD diff together with staged, unstaged, and untracked paths. It
allows only the five declared tooling paths plus exact #369 lifecycle,
prepared, evidence, and `.csdlc/locks/369.lock` surfaces, and runs committed
and worktree diff hygiene checks. Any other path fails closed.
