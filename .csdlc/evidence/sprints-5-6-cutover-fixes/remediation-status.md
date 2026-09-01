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
- Captured cutover-readiness defect: #570/#571 live GitHub state is terminal
  and the shared Git terminal cache contains terminal envelopes, but the only
  registered FastWork worktrees for those issues are stale prep-state
  projections. Typed historical finish and cached-terminal validation both fail
  closed from those prep worktrees because the immutable terminal cache belongs
  to later canonical generations. See
  `.csdlc/evidence/sprints-5-6-cutover-fixes/terminal-cache-conflict-570-571.md`.
- Fixed remotely: PR #591 no longer contains the premature issue-closing
  keyword for #505. A narrow typed `pr_update` action was added to the Rust
  GitHub PR owner and used to change only the PR body linkage line to
  `Part-Of #505`; readback evidence is recorded at
  `.csdlc/evidence/591/pr-state-after-remove-closes.json`.
- Published for review: typed issue transport created tracking issue #596, and
  typed PR transport created PR #597 from
  `codex/sprints-5-6-cutover-fixes` to `main`. After exact-head review, PR
  #597 now uses non-closing `Part-Of` context for #596/#505/#534 because #596
  has not yet executed the typed bound/review/publication lifecycle.

Remaining remote-state blocker:

- No known premature remote-linkage blocker remains in PR #591. #505 still must
  not merge until review approval and explicit operator cutover authority.
- #570/#571 are live-remote terminal, but the stale registered prep worktrees
  need a typed diagnostic/reconciliation affordance before cutover so operators
  do not have to manually compare common terminal-cache generations.
- PR #597 is open and non-draft. Fresh typed PR-state readback after the
  linkage repair reported `linked_issue: null`; #596 remains open until a
  truthful typed lifecycle repair/adoption route exists.

Non-claims:

- This branch does not cut over authority to v3.
- This branch does not merge, finish, or clean #505.
- This branch does not use raw `gh` after the operator correction.
