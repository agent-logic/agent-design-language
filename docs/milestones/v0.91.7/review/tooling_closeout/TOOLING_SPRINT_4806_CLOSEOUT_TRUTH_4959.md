# #4806 Tooling Sprint Closeout Truth Repair (#4959)

Status: ready for review
Date: 2026-07-06
Issue: #4959
Source sprint: #4806

## Summary

This packet resolves the release-review gap for the #4806 repo-native workflow stabilization sprint. The sprint delivered real tooling improvements, but the earlier sprint review packet lived under ignored `.adl/` state and overstated local closeout cleanliness.

For release review, this tracked packet is the durable evidence surface. The ignored `.adl` sprint packets remain useful local workflow records, but they are not the publication proof for #4806.

## Findings Addressed

| Finding | Disposition |
| --- | --- |
| Stale child card truth existed in local `.adl` records, specifically #4737 SRP and #4836 SOR. | Local ignored records were normalized during #4959. This tracked packet records that those local records are not release evidence. |
| Sprint review packet was local-only under ignored `.adl/`. | This tracked packet is now the release-review evidence surface for the closeout-truth repair. |
| Sprint review said no follow-up was required while watcher ambiguity remained. | Routed to #4950; #4950 is now closed with retained settled-state proof. |
| Remaining raw `gh` usage existed in sprint-conductor helper scripts. | Routed to #4960; #4960 is now closed. |
| Owner-binary stale primary last-resort is an operational compromise. | Routed to existing #4907; #4907 is now closed. |

## Local `.adl` State Boundary

The repository `.gitignore` excludes `.adl/`, so sprint review packets and local lifecycle cards under `.adl/` are not tracked release artifacts by default.

During #4959, the local primary-checkout records named by review were normalized narrowly:

- `#4737` SRP: changed from pre-review `not_run` truth to completed local review truth with no actionable findings.
- `#4836` SOR: changed stale `pr_open` / `worktree_only` scaffold remnants to merged local closeout truth.

Those local repairs prevent future local review confusion, but the release-review proof is this tracked packet.

## Worktree / Residue Check

The #4806 child worktree check found two retained clean child worktrees:

- `.worktrees/adl-wp-4737`
- `.worktrees/adl-wp-4738`

Both were clean retained worktrees, not dirty residue. They were pruned during #4959 so closeout truth no longer depends on retained stale child worktrees.

## Follow-up Routing

| Follow-up | Status | Owner |
| --- | --- | --- |
| Distinguish validated closeout from closeout-needed watch state. | Closed; retained proof recorded in `docs/milestones/v0.91.7/review/V0917_WP03_CLOSEOUT_SETTLED_STATE_PROOF_4950.md` | #4950 |
| Remove remaining raw `gh` usage from sprint-conductor helpers. | Closed | #4960 |
| Converge owner-binary fallback onto the final repo-binary story. | Closed | #4907 |

## Historical `pr.sh watch 4806 --json` Truth

During #4959, `pr.sh watch 4806 --json` still reported:

```json
{
  "classification": "closeout_needed",
  "shepherd_state": "closeout_required",
  "reason": "issue_closed_completed"
}
```

That output is retained as historical #4959 evidence only. The watcher
ambiguity was later routed through #4950, which is now closed with settled-state
proof retained in
`docs/milestones/v0.91.7/review/V0917_WP03_CLOSEOUT_SETTLED_STATE_PROOF_4950.md`.

## Release Review Non-Claims

- This packet did not itself remove sprint-conductor `gh` usage; #4960 owned
  and closed that follow-up.
- This packet did not itself fix watch-state ambiguity; #4950 owned and closed
  that follow-up with retained proof.
- This packet did not itself converge owner-binary fallback; #4907 owned and
  closed that route.
- This packet does not force-track ignored `.adl` local workflow state.

## Validation Plan For #4959

- Validate repaired local #4737 SRP and #4836 SOR with focused structured-prompt checks where the local validators accept their terminal phase.
- Run `git diff --check` for tracked packet hygiene.
- Retain the historical #4950-routed closeout-needed output in the #4959 SOR
  only as original #4959 evidence; current watch-state truth is owned by the
  later #4950 settled-state proof.
