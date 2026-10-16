# ADL repository external-review entrypoint

Status: prepared for a bounded **ADL repository review**, not dispatched and not a Sprint 4 release-acceptance result. The latest operator request narrows this preparation to the ADL repository. Unrelated product release work does not prevent preparing or reviewing public ADL source.

## Exact revision and freeze

**Final review candidate: NOT YET FROZEN.** Select the full immutable ADL commit after the required merges; do not dispatch against a moving branch. Inspected public baseline `2f26936b9c0ec582471a3f8fbebcce31b471512e` is orientation evidence, not the final candidate.

The eventual freeze must include #1284 cleanup / PR #1287. Its initial CI failed on a stale UTS test; the reviewed correction is `1839d0536790bbd8653776a9cb30820658a05776`, with final proof/publication owned by the cleanup lane. The #1234 documentation owner reports local final preparation `f3943dac7171ba026bd21f31938c0c552b1e18d7` awaiting review. These are dated preparation inputs, not final selected commits. Preserve them as history rather than continually replacing this entrypoint with transient heads. The #1236 owner supplies the final source/evidence manifest once merges settle.

The #1238 internal report at `1c7246e340740ec23de8642b4bf749db3eabd115` is explicitly NOT_PROVEN. Final TAIL-05 acceptance still requires accepted #1236/#1238 inputs. If only packet documentation changes after selection, retain the product source revision separately from the packet commit.

## Reader order

1. Read `README.md`, `REVIEW.md`, `docs/REPOSITORIES.md`, `CONTRIBUTING.md` and `AGENTS.md` at the selected revision. The public repository owns language/contracts and installed-product routing; it does not own private product implementations.
2. Read `adl/README.md`, `adl/Cargo.toml`, `adl/src/main.rs` and `adl/src/lib.rs`, then trace actual public dispatch/selection/refusal implementations and corresponding tests. There is no root Cargo workspace: every command uses its explicit package manifest.
3. Read each public package's README, manifest, source and focused tests: `adl-schema/`, `adl-uts/`, `adl-v2/`, `adl-legacy-contracts/`. Compare the language specification and schemas in `adl-spec/` to parser/model/validation behavior and representative examples. Check retained compatibility boundaries rather than treating old-version paths as automatically defective.
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

ADL dispatch readiness requires: a selected immutable ADL commit, the cleanup inclusion decision, the concrete scope/diff and retained evidence index, and an independent reviewer able to read those inputs. The final Sprint 4 external synthesis retains its separate all-findings-resolved gate; this scoped review supplies ADL coverage to that later synthesis rather than claiming to complete it.

## Reviewer and access

Use a fresh independent reviewer outside the ADL change-authoring and #1238 internal-review team, with no inherited implementation conversation. Record actual identity/affiliation and limitations; describe an independent agent honestly, not as an independent third-party organization. No paid provider call or reviewer dispatch is performed by this preparation.

Public ADL source and public evidence require no new private access grant. Supply existing authorized local evidence only when a concrete interface question needs it; omit credentials, private source unrelated to the question, customer content and operational receipts. The assigned reviewer must identify inaccessible inputs and their effect on coverage rather than requesting blanket access. Verify actual access at assignment. Return findings first with severity, exact file/line, consequence and bounded fix, followed by examined/unexamined paths, source revision, reused/new proof and a verdict limited to ADL review scope.
