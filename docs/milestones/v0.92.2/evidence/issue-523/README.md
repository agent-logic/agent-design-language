# Issue #523 planning repair handoff

Status: planning corrections and the operator-authorized native validator repair are implemented. Native validation of the real #523 six-card bundle passes. Publication and exact-head review receipts are retained through the native lifecycle route; this document does not claim merge, #524/#525 completion or release approval.

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

## Native tooling defect and repair

The native v3 issue initialization, typed edits and bind succeeded. Native `validate` rejects all six cards with `card_structure_invalid`, while its lifecycle digest check passes. At the baseline, `csdlc-v3/src/commands/local/mod.rs:2559` resolves an active `docs/templates/prompts/1.0.3/<card>.md` schema through two parent directories to the nonexistent `docs/templates/prompts/schemas/<card>.structure.json`, ignoring the registry's actual versioned schema path. The failure was captured separately under the primary checkout's ignored `.adl/reviews/issue-523/` packet. No alias schema, hand-edited state, old-generation lifecycle fallback or raw GitHub write was used to bypass it.

The native bind was invoked after the validation failure and succeeded; it does not establish card validation or publication readiness. This handoff does not accept the failed validator as a pass. The operator subsequently authorized the bounded tooling repair within #523. The validator now reads the registry-declared schema and checks actual structure instead of requiring every scaffold vocabulary line. The repaired native binary validates all six real #523 cards: `lifecycle_digest_valid` and `six_card_validation_passed`. See [native-validator-repair.md](native-validator-repair.md) for the preserved baseline defect, repair contract and proof classification.

## Publication follow-through

Typed publication exposed and repaired a missing real-adapter branch lookup and a PR/issue identity mismatch. Authenticated native reconciliation recovered existing PR #743 without repeating creation. See [native-publication-repair.md](native-publication-repair.md); that repair passed 185 tests. The subsequent [ready-command repair](native-ready-repair.md) uses the documented GitHub mutation with authenticated identity checks; the final native suite passes 190 tests. The original validator and review evidence remains historical and intact.

## Next owners

#523 publishes these planning corrections together with the authorized validator repair after independent review. #524 owns the successor closeout-plan result; #525 reviews the settled planning revision independently. Their preparation can overlap current closeout, but current ceremony still requires both #522 remediation and #525 final planning review. #718 implementation belongs to its own urgent execution lane, not this documentation change.

## Subsequent operator addition

The operator added the [C-SDLC simplification plan](../../cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md) as the first coherent sprint alongside Runtime. The current denominator is 41 rows and the planning validator has nine negative fixtures; earlier 33-row review and validation records above remain historical evidence. See [simplification-addition.md](simplification-addition.md).
