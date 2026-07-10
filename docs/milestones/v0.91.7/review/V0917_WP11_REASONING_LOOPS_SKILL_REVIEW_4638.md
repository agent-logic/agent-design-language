# WP-11 Reasoning Graph, Loops, Skill, AEE/ObsMem, and Godel Review

Status: review_findings_present
Issue: #4638
WP: WP-11
Date: 2026-07-10

## Findings

| ID | Severity | Status | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- | --- |
| WP11-F-001 | P1 | addressed_for_publication | The WP-11 umbrella was not closeout-ready even though the main child/follow-on PRs had merged. Issue `#4638` remains open, but the issue-local SRP/SOR have now been updated from bootstrap/pre-review truth to review-findings-present closeout truth and both structured prompt validators pass. | Original evidence: `bash adl/tools/pr.sh doctor 4638 --slug v0-91-7-wp-11-implement-reasoning-graphs-loops-and-adl-skill-v1-in-full --version v0.91.7 --allow-open-pr-wave --json` returned `doctor_status: BLOCK`, `pr_finish_readiness: blocked`, SRP `pre_review`, and SOR `scaffold`. Repair evidence: `bash adl/tools/validate_structured_prompt.sh --type srp --phase pre_run --input .adl/v0.91.7/tasks/issue-4638__v0-91-7-wp-11-implement-reasoning-graphs-loops-and-adl-skill-v1-in-full/srp.md` passed; matching SOR validator command passed. | Before closing `#4638`, keep SRP/SOR truth current through finish/closeout so umbrella state records the merged child work, review findings, validation, remaining non-claims, and closeout decision. |
| WP11-F-002 | P1 | addressed_by_supersession | Local lifecycle-card truth for several closed WP-11 child/follow-on issues is stale in this worktree. Their GitHub PR bodies contain merged implementation/validation truth, but local `.adl` SOR/SRP cards for `#4695`, `#4696`, `#4697`, `#4912`, and `#5096` still read as pre-run/not-run scaffolds; `#4694` still says the PR/merge path is not complete. The retained umbrella evidence packet now explicitly supersedes those stale local child-card statements for WP-11 release consumption, without claiming the ignored local cards were globally repaired. | Local task cards under `.adl/v0.91.7/tasks/issue-4694__...`, `issue-4695__...`, `issue-4696__...`, `issue-4697__...`, `issue-4912__...`, and `issue-5096__...`; merged PRs `#5091`, `#5104`, `#5099`, `#5101`, `#5106`, `#5127`; supersession evidence in `docs/milestones/v0.91.7/review/V0917_WP11_RUNTIME_V2_COGNITIVE_CONTROL_EVIDENCE_4638.md`. | Consume merged PR evidence and the retained packet's child-card supersession section for release-facing WP-11 truth. Do not use stale local child cards as current closeout proof. |
| WP11-F-003 | P2 | addressed_for_publication | WP-11 had implementation proof, but not one retained umbrella proof packet that tied all child slices into a single Runtime v2 cognitive-control claim. A retained umbrella evidence packet has now been added, with non-claims that prevent it from becoming a release approval or broad no-regression claim. | Original evidence: PR `#5091` reasoning graph proof path `.adl/local-artifacts/issue-4694/reasoning-graph.json`; PR `#5104` loop-runtime proof path `artifacts/v0917/issue-4695-loop-runtime/loop-runtime.json`; PR `#5101`, `#5106`, `#5127`, and `#5138` local ignored proof roots. Repair evidence: `docs/milestones/v0.91.7/review/V0917_WP11_RUNTIME_V2_COGNITIVE_CONTROL_EVIDENCE_4638.md`. | Keep WP-11 release claims scoped to merged code, green CI, issue-local proof commands, and the retained evidence packet's explicit non-claims. |
| WP11-F-004 | P2 | open | GHB/Runtime v2 follow-on validation contains bounded broad-lane caveats that must stay visible. `#5096` records focused proof pass but partial broad lane due unrelated CSM full-suite smoke flakes; `#5136` records focused pass and exact rerun pass after a broad-lane harness artifact. | PR `#5127` validation results; PR `#5138` validation results. | Do not convert focused GHB/Runtime v2 proof into a broad no-regression claim. Consume the caveats as residual validation risk or route them to the relevant tooling/runtime follow-up. |
| WP11-F-005 | P3 | fixed | Repo-native doctor reported stale session claims for `#4638` at review time. The stale #4638 claims have been released, and the closeout-truth repair claim was also released after this packet update. | Original evidence: `pr.sh doctor 4638` returned `preflight_block_kind: session_stale_claim_manual_inspection`; fresh claim `csdlc-issue-4638-20260710t233614z` was created with `adl-session claim` and released before publication staging. Repair evidence: `adl session status --json` on 2026-07-10 showed `csdlc-issue-4638-20260710t174523z` and `csdlc-issue-4638-20260710t233614z` released; `adl session release --claim-id csdlc-issue-4638-20260710t234124z --json` released the repair claim at 2026-07-10T23:48:59Z. | Keep future #4638 claims short-lived and release them at handoff/finish so workflow state remains unambiguous. |

## Scope Summary

- Reviewed scope type: `sprint` / WP issue wave.
- Umbrella issue: `#4638` `[v0.91.7][WP-11] Implement reasoning graphs loops and adl.skill.v1 in full`.
- Canonical child issues from `docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml`: `#4694`, `#4695`, `#4696`, `#4697`.
- Additional WP-11 follow-ons found by label and PR evidence: `#4912`, `#5096`, `#5136`.
- Reviewed PRs:
  - `#5091` / `#4694` reasoning graph runtime.
  - `#5104` / `#4695` loop runtime.
  - `#5099` / `#4696` `adl.skill.v1`.
  - `#5101` / `#4697` AEE/ObsMem/PVF trace handoff.
  - `#5106` / `#4912` Godel snapshot/diff protocol.
  - `#5127` / `#5096` GHB recursive self-improvement loop.
  - `#5138` / `#5136` GHB as Runtime v2 agent runtime.
- Skipped surfaces: full line-by-line re-review of every changed Rust path was not rerun in this packet; this review consumes merged PR evidence, local card truth, retained docs, and GitHub/CI state.

## Lane Coverage

| Lane | Status | Evidence / reason |
| --- | --- | --- |
| gap_analysis | run | Compared WP-11 WBS/wave scope against issue/PR/card state. |
| code | evidence_reviewed | Code changed substantially across Runtime v2, CLI, skill, ObsMem, and Godel surfaces; this packet reviewed PR validation and known review dispositions rather than re-running a full source audit. |
| docs | run | Checked existing register and review packet absence; this file is the first WP-11 review packet. |
| tests | run | Reviewed PR validation summaries and CI status for all listed PRs. |
| evidence_and_closeout | run | Checked umbrella issue state, child issue state, PR state, local cards, retained/local proof boundaries, and doctor output. |
| synthesis | run | Findings in this packet synthesize the above evidence. |
| review_quality | run | Bounded pre-publication subagent review completed on 2026-07-10; actionable doc findings were dispositioned before staging. |
| security | skipped | No dedicated security lane was run; redaction/path risks are noted only where PR evidence names them. |
| architecture | partial | Reviewed architectural scope boundaries for reasoning graph -> loop -> skill/AEE/Godel integration from PR evidence. |
| dependency | skipped | No dependency review was run for this packet. |
| release_evidence | partial | Release-facing non-claims and retained-proof gaps are recorded; no release evidence package was assembled. |

## Lifecycle And Closeout Truth

- Umbrella `#4638` is open.
- Canonical child issues `#4694`, `#4695`, `#4696`, and `#4697` are closed.
- Additional WP-11 follow-ons `#4912`, `#5096`, and `#5136` are closed.
- PRs `#5091`, `#5104`, `#5099`, `#5101`, `#5106`, `#5127`, and `#5138` are merged with required GitHub checks green in the reviewed GitHub state.
- Local umbrella SRP/SOR truth has been moved out of bootstrap/pre-review state: SRP records findings present with `needs_followup`, SOR is `IN_PROGRESS` and records review-findings-present closeout truth. Both structured prompt validators pass.
- Local child card truth is stale for several closed children; the retained Runtime v2 cognitive-control evidence packet explicitly supersedes those stale statements for WP-11 release consumption while preserving the non-claim that the ignored local cards were not globally repaired.
- Earlier doctor state for `#4638` was blocked by stale session claims and SOR/finish readiness, not by child PR failure. The stale #4638 claims have since been released; a later doctor invocation hung after its initial GitHub state read and was interrupted, so this packet relies on structured prompt validation plus session-ledger evidence for the repair state.

## Validation Summary

The reviewed PRs record focused validation for the main WP-11 behavior:

- `#5091`: reasoning graph Rust/CLI tests, JSON proof generation, PR-fast test lane, and diff hygiene passed.
- `#5104`: loop runtime tests, deterministic replay/negative cases, CLI proof generation, coverage-impact tests, PR-fast coverage, and diff hygiene passed.
- `#5099`: `adl.skill.v1` contract/runtime dispatch tests, live code-review gate, PR-fast lane, and policy proof passed.
- `#5101`: AEE/ObsMem/PVF handoff tests, retained-evidence slow-feature proof, CLI materialization, coverage-impact checks, fmt, clippy, and diff hygiene passed.
- `#5106`: Godel snapshot/diff tests, recovery/low-disk regressions, standalone CSM parser/proof path, and diff hygiene passed.
- `#5127`: focused GHB loop/CLI proof passed; broad lane remained partial due unrelated CSM smoke flakes.
- `#5138`: Runtime v2 Godel agent runtime focused tests, GHB guard tests, CLI proof, and exact rerun of broad-lane artifact passed; broad lane itself recorded an unrelated harness artifact.

## Subagent Review Disposition

- Bounded pre-publication subagent review completed on 2026-07-10.
- P1 staging finding: the register referenced this packet while it was still untracked; disposition is to stage this packet and the register together.
- P3 provenance finding: the register header still named prior update `#5143`; disposition is to set `Current update: #4638` and add a WP-11 current-summary bullet.
- No other actionable packet or register findings were reported.

## Residual Risk

- WP-11 cannot be consumed as closeout-clean until umbrella `#4638` is finished/closed.
- Local `.adl` child cards should not be used as release proof; the retained Runtime v2 cognitive-control evidence packet provides a narrow supersession path for WP-11 release consumption.
- Several proof packets are reproducible local ignored artifacts, not retained tracked milestone evidence.
- Hosted-provider paths in GHB/Godel Runtime v2 are resolved or classified but not invoked live unless explicitly stated by the issue proof.
- This packet is a sprint review synthesis, not a fresh comprehensive code audit of every changed Rust line.

## Follow-up Routing

- Must keep closed-loop before closing `#4638`: WP11-F-001 is addressed for publication but requires final finish/closeout truth. WP11-F-005 is fixed for the claims reviewed in this packet.
- WP11-F-002 is addressed by explicit retained-packet supersession. WP11-F-003 is addressed for publication by the retained umbrella evidence packet, but its non-claims must remain intact.
- Keep as residual validation caveat unless separately routed: WP11-F-004.
- No new follow-up issue was created by this review packet.

## Non-Claims

- This packet does not approve merge or close `#4638`.
- This packet does not claim v0.91.7 release readiness.
- This packet does not claim live hosted-provider invocation for GHB/Godel agent runtime.
- This packet does not claim the local `.adl` child cards are current where they contradict merged PR evidence.
- This packet does not claim broad no-regression coverage beyond the validation surfaces recorded by the individual PRs.
