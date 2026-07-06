# v0.91.7 Tools Sprint #4806 Review Remediation

Status: active_remediation_packet

Issue: #4961

Source sprint: #4806

## Purpose

This packet records the second-pass remediation for the repo-native workflow
stabilization sprint review. It exists because the first sprint review packet
was retained under the ignored `.adl/` sprint bundle, while release-tail review
needs a tracked, release-visible summary of the findings, fixes, residuals, and
validation.

## Findings And Dispositions

| Finding | Severity | Disposition |
| --- | --- | --- |
| Child SRP/SOR truth drift remained after #4806 closeout. Several closed child issues still had top-level `Card Status: ready` or `pr_open` scaffold facts despite merged PRs and closed issues. | P1 | Repaired local `.adl` card truth for #4737, #4738, #4787, #4788, #4713, #4836, and #4806. Terminal records now align with merged or `closed_no_pr` integration truth. |
| Sprint review evidence was local-only under `.adl/v0.91.7/sprints/...`, making release review depend on ignored records. | P1 | Added this tracked remediation packet as the release-visible review/remediation record. The `.adl` sprint packet remains the detailed local bundle. |
| Sprint-conductor helper scripts still contained raw `gh` calls for issue close and issue/PR state checks. | P2 | Replaced those paths with repo-native `adl-issue` / `adl-pr-validation` commands, with explicit environment overrides for tests and compatibility. |
| Sprint review synthesis said no follow-up issue was required even though #4950 remains open for watcher `closeout_needed` ambiguity. | P2 | Updated local sprint synthesis/review truth to keep #4950 visible as an open residual. This issue does not close or supersede #4950. |
| Owner-binary stale-primary fallback was operationally useful but not final "one repo binary" policy. | P2 | Tightened code/test wording to name the fallback as a temporary observable operational compromise, not final binary architecture proof. |

## Tracked Code Changes

- `adl/tools/skills/sprint-conductor/scripts/close_sprint_issue.py`
  now comments and closes sprint issues through repo-native issue commands
  instead of raw `gh issue close`.
- `adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py`
  now reads issue state and PR validation through repo-native commands instead
  of raw `gh issue view` / `gh pr view`.
- `adl/tools/test_sprint_conductor_helpers.sh` now provides repo-native fake
  issue and PR-validation binaries and fails if the checked paths invoke raw PR
  view fallback.
- `adl/tools/pr_delegate.sh` and
  `adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh` now describe
  the stale primary owner-binary path as a temporary observable last resort.

## Local Card Repairs

The following ignored local lifecycle records were repaired in the root `.adl`
bundle for release-tail truth alignment:

- #4737 SOR: completed/merged truth aligned with the already-approved SRP.
- #4738 SOR: completed/merged truth aligned; stale PR-publication follow-up
  replaced with merged closeout truth.
- #4787 SOR: completed/merged truth aligned.
- #4788 SOR: completed/merged truth aligned.
- #4713 SOR: completed/merged truth aligned.
- #4836 SOR: already terminal; checked for stale `pr_open` remnants.
- #4806 SOR and sprint synthesis: `closed_no_pr` umbrella truth preserved and
  #4950 kept visible as a residual.

These local records are not staged as tracked release artifacts. This packet is
the tracked release-visible summary of the repair.

## Validation

Focused validation for #4961:

- `python3 -m py_compile adl/tools/skills/sprint-conductor/scripts/close_sprint_issue.py adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py`
- `bash adl/tools/test_sprint_conductor_helpers.sh`
- `bash adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh`
- `bash adl/tools/test_pr_delegate_cargo_fallback_liveness.sh`
- `bash adl/tools/test_workflow_conductor_skill_contracts.sh`
- `git diff --check`

The sprint helper contract test includes both explicit-command override coverage
and a default repo-root resolution check from outside the repository directory.

## Residuals

- #4950 remains open for watcher `closeout_needed` classification ambiguity.
- This remediation does not implement the final one-binary distribution policy.
  It only keeps the temporary stale-primary fallback observable and explicitly
  non-final.
- This remediation does not claim broad Rust validation or milestone release
  readiness.
