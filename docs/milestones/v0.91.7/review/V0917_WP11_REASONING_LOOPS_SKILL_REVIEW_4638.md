# WP-11 Reasoning Graph, Loops, Skill, AEE/ObsMem, and Godel Review

Status: review_findings_present
Issue: #4638
WP: WP-11
Date: 2026-07-10

## Findings

| ID | Severity | Status | Finding | Evidence | Required disposition |
| --- | --- | --- | --- | --- | --- |
| WP11-F-001 | P1 | open | The WP-11 umbrella is not closeout-ready even though the main child/follow-on PRs have merged. Issue `#4638` remains open, the issue-local SOR is still the bootstrap `NOT_STARTED` scaffold, and the SRP has no final review results. | `bash adl/tools/pr.sh doctor 4638 --slug v0-91-7-wp-11-implement-reasoning-graphs-loops-and-adl-skill-v1-in-full --version v0.91.7 --allow-open-pr-wave --json` returned `doctor_status: BLOCK`, `pr_finish_readiness: blocked`, SRP `pre_review`, and SOR `scaffold`; local cards under `.adl/v0.91.7/tasks/issue-4638__v0-91-7-wp-11-implement-reasoning-graphs-loops-and-adl-skill-v1-in-full/`. | Before closing `#4638`, normalize SRP/SOR truth through the editor/finish path so umbrella state records the merged child work, review findings, validation, remaining non-claims, and closeout decision. |
| WP11-F-002 | P1 | open | Local lifecycle-card truth for several closed WP-11 child/follow-on issues is stale in this worktree. Their GitHub PR bodies contain merged implementation/validation truth, but local `.adl` SOR/SRP cards for `#4695`, `#4696`, `#4697`, `#4912`, and `#5096` still read as pre-run/not-run scaffolds; `#4694` still says the PR/merge path is not complete. | Local task cards under `.adl/v0.91.7/tasks/issue-4694__...`, `issue-4695__...`, `issue-4696__...`, `issue-4697__...`, `issue-4912__...`, and `issue-5096__...`; merged PRs `#5091`, `#5104`, `#5099`, `#5101`, `#5106`, `#5127`. | Treat PR bodies and GitHub state as current implementation truth for this review, but do not use the local child cards as closeout proof until they are normalized or explicitly superseded. |
| WP11-F-003 | P2 | open | WP-11 currently has implementation proof, but not one retained umbrella proof packet that ties all child slices into a single release-consumable Runtime v2 cognitive-control claim. Several proof artifacts named by PRs are local ignored outputs under `.adl/local-artifacts/` or `artifacts/...`, not tracked retained milestone packets. | PR `#5091` reasoning graph proof path `.adl/local-artifacts/issue-4694/reasoning-graph.json`; PR `#5104` loop-runtime proof path `artifacts/v0917/issue-4695-loop-runtime/loop-runtime.json`; PR `#5101`, `#5106`, `#5127`, and `#5138` local ignored proof roots. | Keep WP-11 release claims scoped to merged code, green CI, and issue-local proof commands. Add or route an umbrella retained evidence packet before claiming integrated WP-11 release readiness. |
| WP11-F-004 | P2 | open | GHB/Runtime v2 follow-on validation contains bounded broad-lane caveats that must stay visible. `#5096` records focused proof pass but partial broad lane due unrelated CSM full-suite smoke flakes; `#5136` records focused pass and exact rerun pass after a broad-lane harness artifact. | PR `#5127` validation results; PR `#5138` validation results. | Do not convert focused GHB/Runtime v2 proof into a broad no-regression claim. Consume the caveats as residual validation risk or route them to the relevant tooling/runtime follow-up. |
| WP11-F-005 | P3 | open | Repo-native doctor also reports stale session claims for `#4638`. A fresh review-session claim was created for this packet and released after drafting, but the stale historical claims still block doctor preflight until manually inspected or released through the approved workflow. | `pr.sh doctor 4638` returned `preflight_block_kind: session_stale_claim_manual_inspection`; fresh claim `csdlc-issue-4638-20260710t233614z` was created with `adl-session claim` and released before publication staging. | Resolve stale session claims before publication/finish so workflow state is not ambiguous. |

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
- Local umbrella SRP/SOR truth is not final: SRP is ready/pre-review and SOR is scaffold.
- Local child card truth is stale for several closed children; PR bodies are stronger evidence than the local cards at review time.
- Doctor state for `#4638` is `BLOCK` due stale session claims and SOR/finish readiness, not due child PR failure.

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

- WP-11 cannot be consumed as closeout-clean until umbrella SRP/SOR and stale session claims are reconciled.
- Local `.adl` child cards should not be used as release proof until they are normalized or explicitly superseded by PR/closeout evidence.
- Several proof packets are reproducible local ignored artifacts, not retained tracked milestone evidence.
- Hosted-provider paths in GHB/Godel Runtime v2 are resolved or classified but not invoked live unless explicitly stated by the issue proof.
- This packet is a sprint review synthesis, not a fresh comprehensive code audit of every changed Rust line.

## Follow-up Routing

- Must fix before closing `#4638`: WP11-F-001 and WP11-F-005.
- Should fix or explicitly supersede before release consumption: WP11-F-002 and WP11-F-003.
- Keep as residual validation caveat unless separately routed: WP11-F-004.
- No new follow-up issue was created by this review packet.

## Non-Claims

- This packet does not approve merge or close `#4638`.
- This packet does not claim v0.91.7 release readiness.
- This packet does not claim live hosted-provider invocation for GHB/Godel agent runtime.
- This packet does not claim the local `.adl` child cards are current where they contradict merged PR evidence.
- This packet does not claim broad no-regression coverage beyond the validation surfaces recorded by the individual PRs.
