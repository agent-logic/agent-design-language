# Design: Issue 417 implemented authored-design recovery epoch

## Problem and boundary

An implemented issue may clear a stale exact-head review with `recover_review`,
repair typed SPP/STP truth, and invalidate a stale authored-design approval with
`recover_design_review`. The current authored refresh guard accepts only an
immediate `recover_review` generation or a directly preceding authored refresh,
so the truthful repair sequence deadlocks before a new fresh design review.

This change only adjusts eligibility for
`refresh_authored_design_after_recovery` in `csdlc-v2/src/store.rs` and adds
focused public-operation regressions in `csdlc-v2/tests/gate5.rs`. It does not
alter #414, #268, #269, AWS, Runtime, review assignment, publication, finish, or
terminal authority.

## Recovery-epoch anchor and boundary

The anchor is the nearest audit event, searching newest to oldest, whose exact
operation is one of `assign_review`, `record_review`, or `recover_review`.
Eligibility requires that nearest event to be `recover_review`. The nearest
earlier event from the same three-operation set must be `record_review`, so an
assignment-only recovery cannot authorize repair. A later `assign_review`,
`record_review`, or `recover_review` supersedes the prior epoch because it
becomes the nearest anchor. Publication/readiness/terminal state also rejects
the operation independently.

Every event after the anchor and before the requested refresh must be in this
finite allowlist:

- plain audit event `recover_design_review`;
- `approve_design`;
- `replace_planning_collection` only for `affected_areas` or `non_goals`;
- `correct_stp_deliverables_after_recovery`;
- `correct_stp_dependencies_after_recovery`;
- `correct_stp_repo_inputs_after_recovery`;
- `correct_plan_summary_after_recovery`;
- `correct_plan_steps_after_recovery`;
- `correct_goal_after_recovery`;
- `correct_required_outcome_after_recovery`;
- `correct_review_prompts_after_recovery`;
- `replace_sor_follow_ups_after_recovery`;
- `correct_validation_summary_after_recovery`;
- `correct_validation_failure_policy_after_recovery`;
- `correct_sor_follow_ups_after_recovery`;
- `set_field` only for `task_boundary`, `plan_summary`, `failure_policy`, or
  `sor_summary`;
- `replace_plan_steps`;
- `replace_validation_lanes`;
- `refresh_authored_design_after_recovery`, enabling an iterative refresh in
  the same still-open epoch.

Malformed structured audit operations, any unlisted operation, or a collection
or field outside the listed sub-allowlist fails closed. A refresh-specific
predicate reuses the existing nearest-anchor and recorded-review rules but adds
only the three refresh-path events absent from the shared repair predicate:
`recover_design_review`, `correct_stp_deliverables_after_recovery`, and
`refresh_authored_design_after_recovery`. The shared
`implemented_pre_publication_review_recovery_is_clear` predicate and its other
repair/identity callers remain unchanged, preventing this fix from widening
their authorization after design recovery or authored refresh.

## Operation ordering and design authority

The required issue-#414-shaped sequence is:

1. a completed `record_review`;
2. `recover_review`, which establishes the epoch and clears review authority;
3. zero or more allowed typed planning/deliverable repairs;
4. `recover_design_review`, which changes the displayed design approval to
   pending without restoring downstream authority;
5. authored design/diagram content edits;
6. `refresh_authored_design_after_recovery`, which atomically retains the new
   files and updates SPP/VPP design bindings;
7. a later independent fresh design review and typed `approve_design`;
8. a later fresh exact-head implementation review assignment.

Immediate refresh after `recover_review` and an iterative refresh after a prior
pending refresh remain supported. The operation never restores downstream
authority.

## Provenance and proof

The refresh-specific predicate and audit change are exercised only through the
authored-refresh operation; focused negative proof also confirms the shared
implemented repair predicate remains closed after `recover_design_review`.
The refresh audit retains top-level `recovery_sequence` and
`recovery_generation` copied from the nearest qualifying `recover_review`
anchor, unchanged by intervening generations. Tests exercise the public
operations, assert the original anchor values, verify all downstream authority
fields remain empty, reject an unlisted intervening operation without mutation,
and retain immediate and iterative compatibility.

Eligibility otherwise fails closed on wrong phase, stale CAS, unsafe topology,
blank actor/reason, missing recorded-review provenance, unlisted events,
downstream authority, unsafe artifacts, or a no-op authored tuple.
