# Issue #523 planning repair handoff

Status: docs-only planning corrections. Native tooling implementation and its known review findings are extracted into #749; branch-name reconciliation remains tracked in #746. This document does not claim merge, #524/#525 completion or release approval.

## Authorized scope and current steering

Repair the six v0.92.2 planning findings as #523; let #523/#524/#525 planning proceed alongside current closeout, retaining their separate outputs and exact-revision review. Include existing #717/#718/#720 and exclude other backlog. The operator explicitly corrected that #718 is not backlog and needs urgent delivery. This overrides the initial local classification and the original session-goal wording; #718 is included in every final denominator.

## Findings disposition

| Prior finding | Repair |
|---|---|
| F01 remediation freshness | TAIL-06 refreshes changed artifacts, affected proof and internal/external review; TAIL-10 checks final candidate/manifest binding |
| F02 shared contract | CF-EVIDENCE owns finding/run semantics and consumer fixtures before parallel implementations |
| F03 ignored authority | Tracked adopted design contracts carry the selected requirements; source digests retain historical provenance |
| F04 integration barrier | Product integration waits for actual product prerequisites; other admitted support converges at TAIL-01 |
| F05 review independence | Isolated pre-synthesis inputs, committed results and retained disagreement are explicit acceptance/proof |
| F06 successor drift | Runtime v4 stays v0.93; customer-scale CodeFriend is prioritized immediately after qualified Beta 1 |

Existing issue mapping: RT-A2A=#718 (urgent, independently runnable after its own readiness), RT-ORIENT=#717, OBS-LIVE=#720. No replacement issues created. Historical #620/#439 are not reopened. The 33-row denominator includes three existing bindings; WP-01 separately resolves its conductor and 29 prospective new children.

## Independent review

Reviewer: independent Codex subagent `review_523`, read-only review of tracked diff, new contracts, source/issue evidence and validator. Initial review found three P2 inconsistencies: v0.92.1 catalog dependency drift, new-wave predecessor closure ambiguity, and PLAT-MEMORY's missing prose dependency. All were corrected. Final recheck reported no actionable findings and confirmed 33 work packages, six negative fixtures and diff whitespace validation. This is #523 pre-publication review, not the #525 independent planning-review deliverable. Content identities are retained in reviewed-content-manifest.json.

## Validation

`python3 docs/milestones/v0.92.2/validate_planning.py --self-test` passed. Checks cover YAML/specification denominators, existing identities/urgency, dependencies, release-tail order, product/support convergence, shared-contract/review freshness obligations, links and preservation of current-release gates. Six deliberately invalid planning variants are rejected. `git diff --check` passed. These are docs/local-CPU deterministic issue gates, not Runtime or release proof. Python 3 plus Ruby standard YAML are the declared validator dependencies.

## Tooling extraction

The operator restored PR #743 to docs-only scope. Native card validation, PR reconciliation/readiness, worktree routing and their tests are removed from its final diff. Issue [#749](https://github.com/agent-logic/agent-design-language/issues/749) owns the preserved implementation and the incomplete bind-to-worktree state handoff; [#746](https://github.com/agent-logic/agent-design-language/issues/746) owns valid-branch reconciliation.

Original implementation and evidence remain available at commit `e92ecc96e2bdb484278a067eac8166ffe4b76acd`, preserved by `codex/749-native-tooling-extraction`. The earlier 191-test pass did not prove native state handoff: the test manually relocated records. Neither that pass nor the earlier review is acceptance of the extracted tooling.

## Next owners

#523 publishes only these planning corrections and the focused planning validator after independent review. #524 owns the successor closeout-plan result; #525 reviews the settled planning revision independently. Their preparation can overlap current closeout, but current ceremony still requires both #522 remediation and #525 final planning review. #718 implementation belongs to its own urgent execution lane, not this documentation change.

## Subsequent operator addition

The operator added the [C-SDLC simplification plan](../../cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md) as the first coherent sprint alongside Runtime. The current denominator is 41 rows and the planning validator has nine negative fixtures; earlier 33-row review and validation records above remain historical evidence. See [simplification-addition.md](simplification-addition.md).
