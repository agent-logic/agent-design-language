# WP-09 Observatory Demos And Birthday-Visible Proof Review

Status: closeout_ready
Issue: #4636
WP: WP-09
Date: 2026-07-11

## Findings

No blocking findings remain for WP-09 closeout.

Pre-PR review finding disposition:

- P2: #4636 SOR recorded `pr_open` before a #4636 PR existed. Fixed by
  correcting the SOR integration state to `worktree_only` for pre-publication
  truth; `pr finish` owns updating PR state after publication.

Residual caveats are retained as non-claims rather than treated as defects:

- Unity Observatory proof is retained and operator-replayable with the
  operator-provisioned asset packs named by #4745; it is not clean-checkout
  self-contained imported-asset replay.
- Unity-MCP evidence is local editor proof tooling; it is not Unity player-build
  readiness and is not a runtime service claim.
- HTML Observatory proof consumes retained runtime/API, AWS heartbeat, and ACIP
  evidence; it does not claim full v0.92 activation readiness.

## Scope Summary

- Reviewed scope type: sprint.
- Umbrella issue: #4636 `[v0.91.7][WP-09] Observatory demos and birthday-visible proof`.
- Canonical WP-09 children: #4689, #4690, #4691.
- Related Unity proof wave consumed by WP-09: #4652, #4702, #4703, #4704, #4745.
- Related validation-enabler consumed by WP-09: #4990.
- Reviewed PRs and closeout state:
  - #4689 / PR #5152: merged and closeout validation passed.
  - #4691 / PR #5154: merged and closeout pruned the issue worktree.
  - #4702 / PR #5155: merged and closeout pruned the issue worktree.
  - #4690, #4652, #4703, #4704, #4745, and #4990 were previously closed with retained proof or enabling validation evidence consumed by this sprint.
- Changed surfaces reviewed:
  - `docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md`
  - `docs/milestones/v0.91.7/review/demo_matrix_4691/4691-birthday-visible-demo-matrix-proof.md`
  - `docs/milestones/v0.91.7/review/unity_observatory_4689/4689-unity-observatory-integrated-proof.md`
  - `docs/milestones/v0.91.7/review/unity_observatory_4702/4702-flagship-unity-observatory-parent-reconciliation.md`
  - retained Unity proof packets and images from #4652, #4703, #4704, and #4745.
- Skipped surfaces:
  - Fresh live Unity editor replay was not rerun in #4636; the umbrella consumes the retained #4652/#4703/#4704/#4689 proof chain.
  - Fresh runtime/API server loopback was not rerun in #4636; the matrix consumes the retained #4691/#4990 proof surfaces and the documented optional command.

## Lane Coverage

| Lane | Status | Evidence / reason |
| --- | --- | --- |
| gap_analysis | run | Compared WP-09 WBS/wave scope with open issue list, PR inventory, demo matrix, and retained proof packets. |
| code | evidence_reviewed | #4689 included validation-lane and finisher support code; PR #5152 merged with green required checks. No new code is introduced by #4636. |
| docs | run | Reviewed and updated demo matrix closeout truth plus retained WP-09 proof packets. |
| tests | run | Reviewed merged PR validation and ran focused docs/path checks for this umbrella packet. |
| evidence_and_closeout | run | Verified #4689/#4691/#4702 PRs merged and closeout passed; checked open PR inventory for no WP-09 tails. |
| synthesis | run | This packet synthesizes sprint proof, closeout truth, residual risk, and non-claims. |
| review_quality | run | Pre-PR subagent review found one P2 SOR integration-state issue; it was fixed before publication. |
| security | skipped | No new security-sensitive runtime or cloud path is introduced by this umbrella closeout. |
| architecture | partial | Checked claim boundaries for HTML/runtime evidence and Unity editor proof versus runtime/product readiness. |
| dependency | skipped | No dependency manifests changed in #4636. |
| release_evidence | partial | The packet is retained milestone evidence but does not approve v0.91.7 release readiness. |

## Lifecycle And Closeout Truth

- Umbrella #4636 is open at packet authoring time and should close only through the repo-native #4636 PR/closeout path.
- #4689, #4691, and #4702 are merged and closed through repo-native finish/merge paths.
- #4689 retained its issue worktree after closeout because a dirty stale worktree had local warm-cache residue; closeout validation passed after SRP/SOR truth repair.
- #4691 and #4702 worktrees were pruned by repo-native closeout.
- Open PR inventory after #4702 merge showed no WP-09 open PR tails.
- The remaining open WP-09 issue from repo-native issue listing is umbrella #4636.
- The sprint can consume retained evidence for birthday-visible review, with non-claims preserved for clean-checkout Unity asset replay, player builds, and v0.92 activation.

## Validation Summary

Reviewed child validation:

- #4689:
  - `bash adl/tools/test_v0917_unity_observatory_integrated_proof.sh`
  - `bash adl/tools/test_select_validation_lanes.sh`
  - `bash adl/tools/run_pr_fast_test_lane.sh --changed-files <changed-files>` with 195 focused tests passed
  - PR #5152 required GitHub checks passed before merge.
- #4691:
  - `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh`
  - `git diff --check`
  - validation manager docs-diff check
  - PR #5154 required GitHub checks passed before merge.
- #4702:
  - child proof presence checks for #4652/#4703/#4704/#4745
  - `git diff --check`
  - validation manager docs-diff check
  - PR #5155 required GitHub checks passed before merge.

Umbrella #4636 validation to run before publication:

```bash
git diff --check
bash adl/tools/validation_manager.sh --changed-files <changed-files> --json --run
bash adl/tools/validate_structured_prompt.sh --type sor --phase pre_run --input .adl/v0.91.7/tasks/issue-4636__v0-91-7-wp-09-observatory-demos-and-birthday-visible-proof/sor.md
```

## Residual Risk

- The retained Unity proof chain depends on operator-provisioned third-party Unity assets for full local visual replay.
- The sprint review did not rerun a fresh comprehensive code audit of #4689's finisher support change; it consumes merged PR review and green required checks.
- WP-09 does not close WP-15 demo convergence, WP-14 launch handoff, or v0.92 activation readiness.

## Follow-up Routing

- No new follow-up issue is required by this packet.
- Future Unity player build readiness, clean-checkout asset replay, or deeper runtime binding should be promoted as separate child issues instead of hidden under WP-09 closeout.
- Keep #4745 publication boundaries attached to any external-facing Unity Observatory communication.

## Non-Claims

- This packet does not approve release readiness.
- This packet does not claim Unity player-build readiness.
- This packet does not claim redistribution rights for third-party Unity assets.
- This packet does not claim clean-checkout replay of the full imported Unity flagship environment.
- This packet does not claim full runtime completion beyond the retained runtime/API evidence linked by the HTML Observatory proof.
