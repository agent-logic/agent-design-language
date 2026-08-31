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
- Fixed remotely: PR #591 no longer contains the premature issue-closing
  keyword for #505. A narrow typed `pr_update` action was added to the Rust
  GitHub PR owner and used to change only the PR body linkage line to
  `Part-Of #505`; readback evidence is recorded at
  `.csdlc/evidence/591/pr-state-after-remove-closes.json`.
- Published for review: typed issue transport created tracking issue #596, and
  typed PR transport created PR #597 from
  `codex/sprints-5-6-cutover-fixes` to `main`. PR #597 visibly closes #596 and
  only references #505/#534 as non-closing `Part-Of` context.

Remaining remote-state blocker:

- No known premature remote-linkage blocker remains in PR #591. #505 still must
  not merge until review approval and explicit operator cutover authority.
- PR #597 is open and non-draft. The retained typed readback captured before
  subsequent evidence commits reports linked issue #596, review `pending`,
  merge state `blocked`, and classification `waiting`; fresh typed PR-state
  readback is required before review, merge, finish, or closeout claims.

Non-claims:

- This branch does not cut over authority to v3.
- This branch does not merge, finish, or clean #505.
- This branch does not use raw `gh` after the operator correction.
