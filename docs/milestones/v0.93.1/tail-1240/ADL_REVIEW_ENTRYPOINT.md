# ADL repository external-review entrypoint

Status: prepared for a bounded **ADL repository review**, substantive public ADL review completed with CHANGES_REQUIRED and not a Sprint 4 release-acceptance result. The latest operator request narrows this preparation to the ADL repository. Unrelated product release work does not prevent preparing or reviewing public ADL source.

## Exact revision and freeze

**Original reviewed ADL product source: `f75f0d3f9c4df85743638b956f2c2a80f3424f38`. Final repaired source: `ac2b596915538f3ed9ce418aec5aa7b917c18fd4`.** Product source revisions are separate from the commit containing this packet. Preserve original coverage and consume the repair delta and [final synthesis inputs](FINAL_SYNTHESIS_INPUTS.md); do not restart the whole review against a moving branch.

The freeze includes #1234 documentation merge `11e62214592612bf48ced201e80a06732dd72f27` (reviewed head `f3943dac7171ba026bd21f31938c0c552b1e18d7`) and #1284 cleanup / PR #1287 (reviewed head `1839d0536790bbd8653776a9cb30820658a05776`). Local Git ancestry checks confirm both the documentation merge and reviewed cleanup head are ancestors of the frozen commit. The coordinator's authenticated native receipt confirms PR #1287 merged at the frozen commit and #1284 closed; all eight checks passed in run `38840558566`. #1234 is merged, closed and natively finished. These are retained owner results, not new tests run by this packet.

The initial #1287 CI failure on a stale UTS test remains historical evidence; the reviewed correction above and successful exact-head checks supersede it. The earlier inspected baseline `2f26936b9c0ec582471a3f8fbebcce31b471512e` is orientation history only. Public ADL source-review preparation can now advance against the freeze without waiting for unrelated website release work.

The #1238 internal report at `1c7246e340740ec23de8642b4bf749db3eabd115` remains an interim NOT_PROVEN release assessment. Final Sprint 4 TAIL-05 synthesis still requires accepted #1236/#1238 inputs and the resolved findings register; freezing public ADL does not satisfy those separate release gates. After the recorded access preflight, the coordinator assigned substantive public ADL review against the frozen source. Its report is now retained below; four findings require correction.

## Reader order

1. Read `README.md`, `REVIEW.md`, `docs/REPOSITORIES.md`, `CONTRIBUTING.md` and `AGENTS.md` at the selected revision. The public repository owns language/contracts and installed-product routing; it does not own private product implementations.
2. Read `adl/README.md`, `adl/Cargo.toml`, `adl/src/main.rs` and `adl/src/lib.rs`, then trace actual public dispatch/selection/refusal implementations and corresponding tests. There is no root Cargo workspace: every command uses its explicit package manifest.
3. Read each public package's README, manifest, source and focused tests: `adl-schema/`, `adl-uts/`, `adl-legacy-contracts/`. For `adl-v2/`, start with its workspace `Cargo.toml`, then member READMEs/manifests/source under `crates/adl-language/` and `crates/adl-compiler/`; the workspace root has no README or `src/lib.rs`. Compare the language specification and schemas in `adl-spec/` to parser/model/validation behavior and representative examples. Check retained compatibility boundaries rather than treating old-version paths as automatically defective.
4. Inspect `tools/public_adl/` validation/coverage policy and selected public CI workflows; use `tools/source_retirement/README.md` and #1223 retained correspondence evidence to assess the split. Read current user/operator guidance and source-linked commands; historical milestone/demo records retain their dates and original claims.

## Included and excluded surfaces

| Included | Boundary |
| --- | --- |
| `adl/src/`, `adl/Cargo.toml`, `adl/Cargo.lock`, public tests | CLI behavior, portable workflow presentation, dispatch, selectors, refusal behavior, path/credential handling and dependency boundary. |
| `adl-schema/`, `adl-uts/`, `adl-v2/`, `adl-legacy-contracts/`, `adl-spec/` | Public language, schemas, contracts, compatibility semantics, tests and examples. |
| `tools/public_adl/`, `tools/source_retirement/`, relevant `.github/workflows/` | Public validation, coverage denominators, CI routing and evidence of independent ownership. Read relevant tooling; do not execute broad lanes by default. |
| Root guidance, `adl/README.md`, `docs/REPOSITORIES.md`, current `docs/` and relevant `demos/` | Accuracy of public instructions, scope, links, version/installation claims and historical labeling. Prioritize current entrypoints and changed documentation; record unexamined historical material. |
| Split interfaces | ADL-facing producer/consumer contracts and retained selected proof only. Trace an external implementation only if needed to resolve a concrete ADL-boundary finding, documenting that extension. |

Excluded: full Runtime, CodeFriend, C-SDLC, infrastructure, enterprise and website implementation reviews; generated targets/caches, installed binaries, operational credentials and private customer traces; a seven-repository rebuild; public deployment and milestone closure. A remaining on-disk historical owner directory is not automatically an active product implementation: inspect tracked package/dispatch boundaries and cleanup evidence. The separate thirteen-repository documentation wave is supporting claim evidence, not thirteen new repository reviews.

## Commands and retained evidence

Read-only candidate setup is `git rev-parse HEAD`, `git status --short --branch`, and `git diff --name-status <accepted-baseline> <selected-head>` using recorded exact revisions. Review source before deciding whether any new execution is warranted. Inspect existing CI receipts and #1223 `FINAL_VALIDATION.json` / `FINAL_SOURCE_CORRESPONDENCE.json` under `docs/milestones/v0.93.1/repository-decomposition/rd07-1223/`. Preserve their exact source identities, five-package proof, installed consumer counts and final 182 unchanged inputs plus three separately validated guard changes; these receipts do not qualify arbitrary later revisions.

If a concrete ADL change needs new proof, the repository's public CLI entrypoint is `bash tools/public_adl/run_cli_ci.sh`; other public packages use their own manifests and focused lanes. This document does not request running that lane. Reuse accepted evidence for unchanged surfaces and report whether proof was inspected, reused or newly executed. Never equate green public CI with installed private-owner or release qualification.

## Known findings and readiness boundaries

The prior independent diagnostic review found IR-1240-01, the public-input count imprecision; corrected at ADL `bd93dab710d99c076a7030b6908af4fd9e301a34`. IR-1238-04's stale review-facing documentation is fixed by selection of the corrected #1234 pages. Check that the selected ADL packet carries those corrections, without repeating their reviews.

The unresolved Sprint 4 register also contains product artifact/platform correspondence, final quality/edge reconciliation, private review access and the CodeFriend/website export contract. Those remain real release gates but are **not automatically blockers for public ADL code review**. The website repair is relevant here only if its evidence is necessary to assess an ADL-owned dispatch or documented interface claim. Do not delay a language/schema/CLI review solely for website release readiness, and do not use an ADL review PASS to close those product findings.

ADL dispatch readiness requires: a selected immutable ADL commit, the verified cleanup inclusion above, the concrete scope/diff and retained evidence index, and an independent reviewer able to read those inputs. The final Sprint 4 external synthesis retains its separate all-findings-resolved gate; this scoped review supplies ADL coverage to that later synthesis rather than claiming to complete it.

## Reviewer and access

Use a fresh independent reviewer outside the ADL change-authoring and #1238 internal-review team, with no inherited implementation conversation. Record actual identity/affiliation and limitations; describe an independent agent honestly, not as an independent third-party organization. No paid provider call is authorized by this packet. The separate assigned reviewer is conducting public ADL review; this packet does not claim its result.

Public ADL source and public evidence require no new private access grant. Supply existing authorized local evidence only when a concrete interface question needs it; omit credentials, private source unrelated to the question, customer content and operational receipts. The assigned reviewer must identify inaccessible inputs and their effect on coverage rather than requesting blanket access. Verify actual access at assignment. Return findings first with severity, exact file/line, consequence and bounded fix, followed by examined/unexamined paths, source revision, reused/new proof and a verdict limited to ADL review scope.

## Original public ADL access preflight snapshot

Fresh reviewer `OpenAI Codex /root/adl_external_reviewer` recorded ACCESS_PREFLIGHT_ONLY for packet `b282e79796e7016bd0d163bf7b1f4a816f63c0c4` and source `f75f0d3f9c4df85743638b956f2c2a80f3424f38`: all 34 selected packet, source and retained-proof paths were readable through local Git objects. The reviewer had no participation in change authoring, findings disposition or #1238 internal review and received no implementation conversation. It consulted the memory registry for orientation only; no conclusions were drawn. This establishes independent agent role, not third-party organizational independence. The retained coordinator receipt is named `adl-external-reviewer-access.json`.

At that original snapshot, this was access verification only, with no substantive review, correctness judgment, tests, downloads, hash recomputation, provider calls or release acceptance. The workspace/member path clarification above corrects navigation, not a product finding. Full inventories and final accepted #1236/#1238 inputs were not examined. At that snapshot, private final-release evidence access remained unverified and was a separate final Sprint 4 synthesis gate; it is not required for the public ADL inputs already checked.

The subsequent split-interface access extension verified all 17 selected retained records and nine exact-revision contracts/ledgers. No checked-input access gap remains; historical archive availability/current installation was not retested. Final accepted candidate, internal report and disposition heads still need readback when finalized. These access observations are not substantive findings or release acceptance.

## Completed bounded ADL review

[Independent report](ADL_INDEPENDENT_REVIEW_F75F0D3.md) and [coverage ledger](ADL_REVIEW_COVERAGE.json) preserve the reviewer-authored results for frozen source `f75f0d3`: **CHANGES_REQUIRED, two P2 and two P3 findings**. The report is copied without substantive changes. Its 591-path inventory is not a count of reviewed files. One isolated std-only prompt probe ran; broad tests, builds, CI, network/provider calls and installed execution did not. Private split-interface preflight remains access evidence, not private implementation review. The result is independent-agent coverage, not third-party organizational review or full Sprint 4 acceptance.

#1241 owns the four original finding IDs and dispositions. Repair owners must provide exact commits and focused regression/review evidence; the independent reviewer should assess only the affected repair delta and retained coverage before advancing the product freeze. Do not discard this adverse report or repeat the whole repository campaign.

## Independent repair delta assessment

The original reviewer returned [PASS for all four bounded repair deltas](ADL_REPAIR_DELTA_REVIEW.md), with no new material finding. Exact heads: prompt `ba221958e90a85de74e01600dc33d7efa5d7cfef`; compiler `5e41adf2c63c073df56781ee2b5deba5254fc6d0`; TTL/specification `1a979c83df1a862a42024a4f80e257a99aae89b3`. The reviewer reused prior proof and unchanged coverage, with no additional execution. Original frozen-source CHANGES_REQUIRED report and coverage remain unmodified.

Prompt #1288 / PR #1290 has merged as `15485055270e7c1376638010e520b29734abb8ef`, with all eight CI checks passed and native finish/cleanup complete. Compiler CI and TTL/specification publication were pending at this assessment. The eventual combined ADL source must include all three reviewed repairs; this individual-head PASS does not claim integrated-candidate or Sprint 4 acceptance. Only the affected integration delta needs verification after the remaining merges.

Final correction integration: compiler PR #1292 merged `a0b4c38d373fdbf53d1866cb9b8d84a2f9507260`; TTL/spec PR #1293 merged `ac2b596915538f3ed9ce418aec5aa7b917c18fd4` at strengthened test head `38761b0805ceff3d2811a2a4e30088b83460c899`. The final merge tree matches the integrated eight-check CI run `38846853808` exactly. All four external repairs are delivered; final packet synthesis remains a separate result. Earlier pending CI/publication statements describe the repair report's original observation time.
