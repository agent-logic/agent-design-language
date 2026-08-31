# Sprint 5/6 remediation status

Source review: `/Volumes/FastWork/adl-reviews/csdlc-sprints-5-6-20260830/SYNTHESIS.md`

Branch: `codex/sprints-5-6-cutover-fixes`

Status as of this branch:

- Fixed locally: required standalone `csdlc-v3` CI selection and workflow lane.
- Fixed locally: single v3 `csdlc` binary shape for foundation/local subcommands.
- Fixed locally: v2 importer verifies issue-record digest plus card values,
  rendered Markdown, and AST digests.
- Fixed locally: durable reopen, failed durable state replacement, and
  projection-repair failure semantics have focused tests.
- Fixed locally: remote review evidence, finish linkage, self-review
  whitespace, part-of delivery, and cleanup path/removal distinctions have
  focused tests.
- Fixed locally: v3 README/package-state ledger and locked validation command
  guidance are refreshed.
- Fixed locally: CI/path-policy validation scratch is repo-contained for the
  exercised test path.
- Added canary evidence: real issue #592 was created/read through typed v2,
  bootstrapped locally, validated/doctor-checked, and compared against v3 local
  preparation. The canary records v3/v2 readiness disagreement as a defect.
- Added canary evidence: OBS-A/#511 and OBS-B/#512 were prepared through typed
  v2 plus the single v3 local command. #511 is execution-ready for the next
  issue step; #512 is structurally healthy but execution-blocked on #511/#536.

Remaining remote-state blocker:

- PR #591 still begins with `Closes #505` at typed `csdlc-github-pr` readback.
  The installed typed PR owner exposes `pr_state` but no PR body-edit action.
  `csdlc-publish` may be able to republish a body through the #505 lifecycle,
  but #505's execution worktree is currently dirty and must not be republished
  from this remediation branch or through raw `gh`. Until this is corrected in
  the #505 worktree with typed lifecycle authority, PR #591 must not merge.

Non-claims:

- This branch does not cut over authority to v3.
- This branch does not merge, finish, or clean #505.
- This branch does not use raw `gh` after the operator correction.
