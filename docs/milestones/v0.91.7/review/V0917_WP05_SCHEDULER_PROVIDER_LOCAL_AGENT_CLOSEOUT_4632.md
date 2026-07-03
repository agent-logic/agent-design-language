# v0.91.7 WP-05 Scheduler Provider Local-Agent Closeout Review (#4632)

Status: review_ready_for_publication

Issue: #4632

Child issues: #4671, #4672, #4673, #4674, #4675

## Findings

- No open product/runtime finding blocks WP-05 closeout. The scheduler/provider/local-agent slices are implemented as code, covered by retained proof packets, and integrated through merged PRs.
- Tooling residual: `pr.sh closeout` validated child SORs but left uneven closeout-card truth in root ignored records. #4672 and #4673 still said `Closeout state: not_started`, `PR state: not_open`, and `Watcher disposition: not_started`; #4674 and #4675 lacked explicit closeout/PR/watcher lines. Those ignored root closeout records were repaired with `sor-editor` semantics and revalidated where stale lines were changed. This is recorded as workflow tooling friction, not as a WP-05 product blocker.
- Tooling residual: the umbrella `pr.sh run 4632` binding initially stopped on an unrelated open WP-06 PR in the explicit WP queue. The run was resumed with the repo-native `--allow-open-pr-wave` flag because this packet only closes WP-05 truth and does not touch WP-06 work.
- Tooling residual: several child SORs record unknown elapsed/token metrics because the operator requested a WP-05 sprint goal instead of independent issue goals for each child. Unknown metrics are retained as unknown and are not treated as zero.

## Scope

WP-05 owns the v0.91.7 cognitive scheduler and provider/local-agent execution bridge needed before v0.92. This closeout packet reviews the completed child wave:

| Issue | PR | Surface | Result |
| --- | --- | --- | --- |
| #4671 | #4809 | Cognitive scheduler v1 | Merged; closeout SOR says `completed_with_pr_closeout`. |
| #4672 | #4812 | Provider profile selection | Merged; closeout SOR repaired to `completed_with_pr_closeout`. |
| #4673 | #4823 | Model suitability selection proof | Merged; closeout SOR repaired to `completed_with_pr_closeout`. |
| #4674 | #4827 | Cheapest validated outcome policy | Merged; closeout SOR normalized to `completed_with_pr_closeout`. |
| #4675 | #4834 | Local-agent delegation readiness | Merged; closeout SOR normalized to `completed_with_pr_closeout`. |

## Implemented Product Surface

WP-05 now has an integrated scheduler/provider decision path with these proved slices:

- deterministic cognitive scheduler v1 with lane selection, dependency status, confidence, manual override state, deterministic rank keys, and retained CLI plan output;
- provider profile selection with schema-gated provider route inputs and task-scoped provider-route decisions;
- model suitability selection with retained v0.91.6 evidence consumption and advisory-only model role suitability;
- cheapest validated outcome policy that selects the lowest-cost retained-evidence candidate only after role suitability and validation evidence pass;
- local-agent delegation readiness that composes provider route, model suitability, and cheapest validated outcome inputs while enforcing advisory-only, shadow-mode local delegation boundaries.

The resulting scheduler surface can produce machine-readable plan artifacts for the WP-05 decision chain. It does not silently grant live model authority, GitHub authority, merge authority, closeout authority, or autonomous repo mutation.

## Evidence Reviewed

Primary proof packets:

- `docs/milestones/v0.91.7/review/scheduler/COGNITIVE_SCHEDULER_V1_4671.md`
- `docs/milestones/v0.91.7/review/provider/PROVIDER_PROFILE_SELECTION_4672.md`
- `docs/milestones/v0.91.7/review/provider/MODEL_SUITABILITY_SELECTION_4673.md`
- `docs/milestones/v0.91.7/review/provider/CHEAPEST_VALIDATED_OUTCOME_POLICY_4674.md`
- `docs/milestones/v0.91.7/review/provider/LOCAL_AGENT_DELEGATION_READINESS_4675.md`

Retained machine-readable artifacts:

- `docs/milestones/v0.91.7/review/scheduler/artifacts/cognitive_scheduler_v1_plan_4671.json`
- `docs/milestones/v0.91.7/review/provider/artifacts/provider_profile_selection_input_4672.json`
- `docs/milestones/v0.91.7/review/provider/artifacts/provider_profile_selection_plan_4672.json`
- `docs/milestones/v0.91.7/review/provider/artifacts/model_suitability_plan_4673.json`
- `docs/milestones/v0.91.7/review/provider/artifacts/srp_sor_facts_4673.yaml`
- `docs/milestones/v0.91.7/review/provider/artifacts/cheapest_validated_cost_table_4674.json`
- `docs/milestones/v0.91.7/review/provider/artifacts/cheapest_validated_outcome_plan_4674.json`
- `docs/milestones/v0.91.7/review/provider/artifacts/local_agent_delegation_readiness_plan_4675.json`

Lifecycle evidence reviewed:

- root child SOR bundles under `.adl/v0.91.7/tasks/issue-4671__*` through `.adl/v0.91.7/tasks/issue-4675__*`;
- repo-native shepherd observations for #4632 and child issues;
- `pr.sh closeout` validation output for #4671 through #4675.

## Validation Summary

Child issues recorded these proving lanes:

- #4671: scheduler unit tests, CLI scheduler tests, CLI proof artifact regeneration, and JSON ordering assertions.
- #4672: focused provider scheduler tests, provider-route CLI proof artifact generation, JSON assertions, pre-PR review fixes, and retained proof artifacts.
- #4673: formatting, model suitability scheduler tests, CLI proof generation, JSON contract assertions, negative-case tests, and retained evidence refs.
- #4674: cheapest validated outcome focused tests, scheduler regression tests, CLI proof generation, JSON policy assertions, and retained cost table.
- #4675: local-agent delegation focused tests, scheduler regression tests, scheduler economics finish-lane filter, CLI proof artifact, JSON assertions, pre-PR review fixes, and retained proof artifacts.

Umbrella closeout validation:

- child closeout rerun for #4671-#4675 with `ADL_PR_CLOSEOUT_BIN=... pr.sh closeout <issue>`;
- #4672 SOR final validation: `bash adl/tools/validate_structured_prompt.sh --type sor --phase final --input .adl/v0.91.7/tasks/issue-4672__v0-91-7-wp-05-providers-implement-provider-profile-selection/sor.md`;
- #4673 SOR final validation: `bash adl/tools/validate_structured_prompt.sh --type sor --phase final --input .adl/v0.91.7/tasks/issue-4673__v0-91-7-wp-05-models-implement-model-suitability-selection-proof/sor.md`;
- #4674 SOR final validation: `bash adl/tools/validate_structured_prompt.sh --type sor --phase final --input .adl/v0.91.7/tasks/issue-4674__v0-91-7-wp-05-policy-implement-cheapest-validated-outcome-policy/sor.md`;
- #4675 SOR final validation: `bash adl/tools/validate_structured_prompt.sh --type sor --phase final --input .adl/v0.91.7/tasks/issue-4675__v0-91-7-wp-05-delegation-implement-local-agent-delegation-readiness/sor.md`.

## Closeout Truth

- #4632 remains open until this umbrella closeout PR merges and the issue is closed.
- The child implementation issues are closed and their PRs are merged.
- Child worktrees for #4671-#4675 are absent or pruned.
- No tracked WP-05 implementation artifact is intentionally left only in a worktree.
- Ignored root `.adl` closeout records remain the canonical local card bundle for child closeout truth; #4672-#4675 required narrow SOR truth repair after closeout validation exposed stale or missing closeout fields.

## Residual Risks

- WP-05 proves deterministic scheduler/provider/local-agent planning. It does not prove live hosted-provider invocation, live local Ollama quality, runtime agent execution, or autonomous multi-agent operation.
- Local-agent delegation readiness is intentionally `shadow_only`; granting broader authority requires later explicit runtime/governance work.
- Cost evidence is retained and bounded, not live price discovery.
- Build/finish friction remains visible: narrow issue work still encountered broad Rust/AWS dependency compilation during finish lanes. That belongs to the validation/build tooling track, not to WP-05 product behavior.

## Non-Claims

- This packet does not claim WP-06 validation manager work is complete.
- This packet does not claim runtime Soak #2 is complete.
- This packet does not claim OpenTelemetry/runtime logging is complete.
- This packet does not claim local models can merge, close, publish, mutate files, or operate without operator authority.
- This packet does not claim exact cost optimization beyond retained proof-table selection.

## Recommended Disposition

WP-05 is ready to close after this umbrella packet is published, reviewed, merged, and #4632 closeout truth is normalized.
