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
| Sprint review said no follow-up was required while watcher ambiguity remained. | Watcher ambiguity is explicitly routed to #4950. |
| Remaining raw `gh` usage existed in sprint-conductor helper scripts. | Routed to #4960, not fixed in this issue. |
| Owner-binary stale primary last-resort is an operational compromise. | Routed to existing #4907, not fixed in this issue. |

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
| Distinguish validated closeout from closeout-needed watch state. | Open | #4950 |
| Remove remaining raw `gh` usage from sprint-conductor helpers. | Open | #4960 |
| Converge owner-binary fallback onto the final repo-binary story. | Open/existing route | #4907 |

## Current `pr.sh watch 4806 --json` Truth

`pr.sh watch 4806 --json` still reports:

```json
{
  "classification": "closeout_needed",
  "shepherd_state": "closeout_required",
  "reason": "issue_closed_completed"
}
```

This is expected until #4950 lands. It is no longer treated as an untracked #4806 sprint finding; it is an explicitly routed tooling bug.

## Release Review Non-Claims

- This packet does not claim all sprint-conductor `gh` usage is removed; #4960 owns that.
- This packet does not claim watch-state ambiguity is fixed; #4950 owns that.
- This packet does not claim the stale primary owner-binary fallback is the final architecture; #4907 owns that.
- This packet does not force-track ignored `.adl` local workflow state.

## Validation Plan For #4959

- Validate repaired local #4737 SRP and #4836 SOR with focused structured-prompt checks where the local validators accept their terminal phase.
- Run `git diff --check` for tracked packet hygiene.
- Run `pr.sh watch 4806 --json` and retain the expected #4950-routed closeout-needed output in the #4959 SOR.
