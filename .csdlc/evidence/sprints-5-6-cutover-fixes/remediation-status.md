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
- Fixed locally: the v3 local command emits a retained render manifest for all
  six lifecycle cards using active-registry template refs and
  repo-local render-manifest digests; #505 trial evidence is retained at
  `.csdlc/evidence/505/v3-local-trial.json`.
- Fixed locally: the v3 remote command is now exposed through the single
  `csdlc remote` binary shape as a pre-cutover bridge verifier. It rejects
  caller-forged refs, refuses non-JSON/schema-less evidence, fingerprints the
  referenced repo-local evidence contents, and still reports
  `operational_authority: false` / `trusted_authority: false` until #505
  explicitly switches authority. Retained trial evidence is at
  `.csdlc/evidence/505/v3-remote-bridge-trial.json`.
- Fixed locally: cleanup identity now verifies both sides of Git's worktree
  registration pointer (`<worktree>/.git` and
  `<repo>/.git/worktrees/<name>/gitdir`) before accepting a cleanup identity
  digest.
- Fixed locally: `csdlc-finish --diagnose-cached-issue` now performs a
  non-mutating terminal-cache diagnostic and classifies stale local projections
  distinctly from matching, missing, or conflicting terminal authority. Real
  #570/#571 stale prep worktrees now report
  `stale_projection_terminal_exists`; evidence is retained at
  `.csdlc/evidence/sprints-5-6-cutover-fixes/terminal-cache-diagnostic-570.json`
  and
  `.csdlc/evidence/sprints-5-6-cutover-fixes/terminal-cache-diagnostic-571.json`.
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
- Repaired remotely on 2026-09-01: #596 had been closed by remote PR merge
  while its canonical C-SDLC truth remained `phase: ready` and its SOR remained
  pre-execution/not-published/not-merged. Typed v2 `csdlc-github-issue`
  reopened #596 with readback `state: open`; see
  `.csdlc/evidence/sprints-5-6-cutover-fixes/issue-596-reopen-readback-20260901.json`.
- Repaired remotely on 2026-09-01: the sprint synthesis classified #501,
  #502, #503, #504, and Sprint 5 umbrella #533 as failed/partial despite their
  remote closed state. Typed v2 `csdlc-github-issue` reopened all five; see
  `.csdlc/evidence/sprints-5-6-cutover-fixes/reopened-failed-issues-20260901.json`.
- Retained locally on 2026-09-01: Sprint 6 live umbrella membership is v5 and
  includes #570; Sprint 5 remains membership v4 and has been reopened after the
  sprint-review failure. See
  `docs/milestones/v0.92.1/evidence/wp-01/sprint-umbrella-membership-v5-retained-readback.json`.
- Added records-hygiene evidence:
  `.csdlc/evidence/sprints-5-6-cutover-fixes/records-hygiene-sprints-5-6-20260901.yaml`.
- Added Gemini-assisted review evidence:
  `.csdlc/evidence/sprints-5-6-cutover-fixes/gemini-remediation-review/receipt.json`
  and
  `.csdlc/evidence/sprints-5-6-cutover-fixes/gemini-remediation-review/review.md`.
  Gemini confirmed the remote/cleanup/reopen slices looked repaired and flagged
  CI, storage, projection-repair, and import-digest concerns. Current branch
  evidence resolves those concerns as follows: CI path policy selects
  `csdlc_v3_standalone_required=true`; `csdlc-v3/src/storage/mod.rs` and
  `csdlc-v3/tests/transactions.rs` cover interrupted-intent recovery, failed
  durable state replacement, and post-commit projection repair; and
  `csdlc-v3/src/application/mod.rs` plus `csdlc-v3/tests/foundation.rs`
  recompute and reject issue/card digest drift.
- Fixed locally after Gemini review: the v3 `remote` CLI no longer only accepts
  opaque repo-local evidence refs; it parses typed PVF, accepted-review,
  publication-intent, PR-readback, issue-readback, and cleanup-inspection JSON,
  then derives the remote delivery result from those typed observations while
  keeping `operational_authority=false` before #505 cutover.
- Historical note: typed issue transport created tracking issue #596, and typed
  PR transport created PR #597 from `codex/sprints-5-6-cutover-fixes` to
  `main`. PR #597 was later merged, but that merge did not itself terminalize
  #596 because #596 never executed the typed bound/review/publication lifecycle.

Remaining remote-state blocker:

- No known premature remote-linkage blocker remains in PR #591. #505 still must
  not merge until review approval and explicit operator cutover authority.
- #570/#571 are live-remote terminal, and stale registered prep worktrees now
  have a typed, non-mutating diagnostic instead of requiring manual comparison
  of common terminal-cache generations.
- #501/#502/#503/#504/#533/#596 remain open until their issue-local typed
  lifecycle truth, exact-head review, publication/readback, and sprint-level
  result evidence are reconciled successfully.

Non-claims:

- This branch does not cut over authority to v3.
- This branch does not merge, finish, or clean #505.
- This branch does not use raw `gh` after the operator correction.
