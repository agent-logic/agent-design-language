# v0.91.7 Prompt-Card Enum Typing Inventory

Issue: #4892
Sprint: #5035
Parent: #4651

## Purpose

This inventory fixes the finite Rust string-contract surface that the follow-on
Rust tooling simplification issues must use. It is intentionally a contract
inventory only. It does not implement enum conversions, change durable card
formats, or redesign prompt-card templates.

The source issue named
`.adl/docs/TBD/workflow_tooling/planning/V0917_PROMPT_CARD_ENUM_TYPING_PLAN.md`
as an input surface. That ignored local `.adl/docs` path is not present in this
checkout, so this tracked document is the reviewable replacement contract.
The other named source
`.adl/docs/TBD/rust_refactoring/ADL_CODE_SIMPLIFICATION_LIBRARY_OPPORTUNITIES.md`
is also absent from this checkout; this inventory therefore relies on the live
Rust, template, schema, and issue-card surfaces listed below.

## Evidence Surface

- `adl/src/csdlc_prompt_editor.rs`
- `adl/src/cli/tooling_cmd/structured_prompt.rs`
- `adl/src/cli/tooling_cmd/common.rs`
- `adl/src/cli/pr_cmd/finish_support.rs`
- `adl/src/cli/tooling_cmd/srp_sor_update.rs`
- `docs/templates/prompts/current.json`
- `docs/templates/prompts/1.0.3/*.md`
- `docs/templates/prompts/1.0.3/schemas/*.structure.json`
- `.adl/v0.91.7/tasks/issue-4892__v0-91-7-rust-enum-inventory-finite-rust-enum-string-contracts/*`

## Decision Legend

- `enum_now`: finite and stable enough for v0.91.7 internal enum-backed
  handling while preserving the durable string spellings.
- `string_extensible`: intentionally open-ended identifiers or descriptions;
  keep string-backed.
- `defer`: finite-looking but not ready for a stable enum until the named
  blocker is resolved.

## Inventory

| Contract | Current durable spellings | Current sources | Decision | Follow-on owner | Notes |
|---|---|---|---|---|---|
| `PromptCardKind` | `sip`, `stp`, `spp`, `vpp`, `srp`, `sor` | `PromptCardKind::parse_key`, template registry lifecycle | `enum_now` | #4893 | Already implemented as a Rust enum. Reuse as the pattern for parse/display tests and template registry coverage. |
| `CardStatus` | `draft`, `ready`, `reviewed`, `approved`, `completed`, `blocked`, `superseded` | `CARD_STATUS_VALUES`, `ALLOWED_CARD_STATUS`, card front matter and `Card Status:` markdown field | `enum_now` | #4893, #4894, #4895 | Durable across editor and structured validator. `completed` must keep SRP/SOR terminal guards rather than becoming a blind allowed value. |
| `StpStatus` | `draft`, `active`, `complete` | STP structured validator front matter `status` check | `enum_now` | #4893, #4895 | This is not the same contract as `CardStatus`; do not collapse it into shared lifecycle status. |
| `StpAction` | `create`, `edit`, `close`, `split`, `supersede` | STP structured validator front matter `action` check | `enum_now` | #4893, #4895 | Stable finite issue-intent action set. |
| `IssueOutcomeType` | `code`, `docs`, `tests`, `demo`, `combination` | SIP structured validator `Execution.Required outcome type` | `enum_now` | #4893, #4895 | Stable finite values in prompt-card execution metadata. |
| `SppStatus` | `draft`, `ready`, `reviewed`, `approved` | SPP structured validator front matter `status`; editor currently uses broader `CARD_STATUS_VALUES` | `enum_now` | #4893, #4894, #4895 | Follow-on work should narrow the editor select values to the validator set. |
| `ActivationState` | Canonical: `draft`, `ready`, `reviewed`, `approved`; legacy aliases: `design_time_ready`, `ready_for_execution` -> `ready`; `ready_for_execution_binding`, `active` -> `approved` | SPP front matter `activation_state`, editor select, legacy import normalizer | `enum_now` | #4893, #4894 | Current editor reuses `CARD_STATUS_VALUES`; follow-on work should split this into a dedicated enum and preserve the listed import aliases only. |
| `CodexPlanStatus` | `pending`, `in_progress`, `completed` | SPP structured validator `codex_plan.status` check | `enum_now` | #4893, #4895 | Stable finite per-step plan state. |
| `EstimateConfidence` | `low`, `medium`, `high`, `unknown` | SPP structured validator and editor field | `enum_now` | #4893, #4894, #4895 | Shared by estimate and actual metrics confidence where applicable. |
| `EstimateDataSource` | `manual_entry`, `derived_sprint_state`, `unknown` | SPP structured validator and editor field | `enum_now` | #4893, #4894, #4895 | Preserve source-ref coupling rules outside the enum. |
| `VppStatus` | `draft`, `ready`, `reviewed`, `approved` | VPP structured validator and editor field | `enum_now` | #4893, #4894, #4895 | Stable finite validation-planning lifecycle status. |
| `ValidationSizeSplit` | `small_only`, `large_only`, `mixed`, `not_applicable`, `unknown` | VPP editor select field | `enum_now` | #4893, #4894 | Structured validator currently requires presence but not value membership; #4895 should add validator coverage if the enum lands. |
| `SrpStatus` | `draft`, `ready`, `approved` | SRP structured validator front matter `status` check | `enum_now` | #4893, #4895 | Different from `CardStatus`; keep separate. |
| `ReviewFindingsStatus` | `not_run`, `findings_present`, `no_findings` | SRP editor field; SRP completed-card validator requires `no_findings` or `findings_present` for final results | `enum_now` | #4893, #4894, #4895 | `not_run` is valid for pre-review prompt state, not completed review truth. |
| `ReviewRecommendedOutcome` | `not_run`, `pass`, `block`, `needs_followup` | SRP editor field; SRP completed-card validator requires `pass`, `block`, or `needs_followup` for final results | `enum_now` | #4893, #4894, #4895 | Keep final-state guard separate from enum membership. |
| `AllowedReviewDisposition` | `PASS`, `BLOCK`, `NEEDS_FOLLOWUP` | SRP structured validator `allowed_dispositions` check | `enum_now` | #4893, #4895 | Uppercase public review disposition contract. |
| `SorExecutionStatus` | `NOT_STARTED`, `IN_PROGRESS`, `DONE`, `FAILED` | `ALLOWED_OUTPUT_STATUS`, SOR structured validator, review-surface tooling | `enum_now` | #4893, #4895 | Do not merge with lowercase card lifecycle status. |
| `CompletionState` | `completed`, `completed_with_follow_on`, `blocked`, `failed`, `deferred`, `cancelled`, `unknown` | SOR editor field | `enum_now` | #4893, #4894 | Add structured validator coverage only if SOR values rendering depends on this field after #4893. |
| `VarianceRequired` | `not_applicable`, `no`, `yes` | SOR editor field and SOR values validation | `enum_now` | #4893, #4894, #4895 | Preserve cross-field validation with metric variance rules. |
| `VarianceCompleted` | `not_applicable`, `no`, `yes` | SOR editor field and SOR values validation | `enum_now` | #4893, #4894, #4895 | Preserve rule: cannot be `yes` unless variance is required. |
| `VarianceCategory` | `not_applicable`, `validation_misclassification`, `pr_wait`, `merge_conflict`, `tool_failure`, `unclear_scope`, `model_drift`, `human_wait`, `external_api_latency`, `overestimated_scope` | SOR editor field and SOR values validation | `enum_now` | #4893, #4894 | Stable sprint metrics taxonomy. |
| `BudgetSource` | `issue_goal_budget`, `sprint_rollup`, `manual_entry`, `not_applicable`, `unknown` | SOR editor field | `enum_now` | #4893, #4894 | Add structured validator coverage if this becomes required outside values rendering. |
| `ActualMetricsDataSource` | `codex_goal_tool`, `manual_entry`, `derived_sprint_state`, `unknown` | SOR editor field and SOR values validation | `enum_now` | #4893, #4894, #4895 | Preserve source-ref coupling rules outside enum membership. |
| `IntegrationState` | Structured validator: `worktree_only`, `pr_open`, `merged`, `closed_no_pr`; editor also offers `failed`, `blocked`; finish aliases include `worktree`, `open_pr`, `open-pr`, `pr-ready`, `pr_ready`, `closed-no-pr`, `no-pr`, `no_pr`, `merged-pr`, `merged_pr` | SOR structured validator, SOR editor field, finish alias normalizer, doctor closeout checks | `enum_now` with mismatch repair | #4893, #4894, #4895 | Canonical durable set for rendered SOR should be the structured validator set. Treat `failed` and `blocked` as completion/result concepts, not integration states, unless #4893 deliberately widens validator and closeout semantics. |
| `VerificationScope` | Structured validator: `worktree`, `pr_branch`, `main_repo`; editor also offers `ci`, `not_run`; finish aliases include `main`, `main repo`, `main-repo`, `repo`, `pr`, `pr branch`, `pr-branch`, `branch`, `worktree-only`, `worktree_only` | SOR structured validator, SOR editor field, finish alias normalizer | `enum_now` with mismatch repair | #4893, #4894, #4895 | Canonical durable set for rendered SOR should be the structured validator set. Treat `ci` and `not_run` as validation/result metadata unless validator semantics are deliberately widened. |
| `IntegrationResult` | `PASS`, `FAIL`, plus editor-only `NOT_RUN`, `BLOCKED` | SOR structured validator allows `PASS`, `FAIL`; editor offers wider set | `defer` | #4893, #4894, #4895 | Needs a decision before enum implementation: either keep terminal integration result binary and move `NOT_RUN`/`BLOCKED` elsewhere, or widen validator semantics. |
| `ValidationResult` | `PASS`, `FAIL`, `NOT_RUN`, `BLOCKED` | SOR editor field | `enum_now` | #4893, #4894 | Stable finite validation-command result. Structured validator currently checks validation truth text rather than this field directly. |
| `VerificationStatus` fields | Repeated values include `PASS`, `FAIL`, `PARTIAL`, `NOT_RUN`, `BLOCKED` across validation, determinism, security/privacy, and artifacts verification fields | SOR editor verification fields and bootstrap/sample values | `enum_now` | #4893, #4894 | Candidate for one shared `ProofStatus` enum if #4893 confirms every field shares semantics. Preserve `PARTIAL` because current SOR defaults use it for limited security/privacy review truth. |
| `VerificationSchemaChangesApproved` | `yes`, `no`, `not_applicable` | SOR editor field and bootstrap defaults | `enum_now` | #4893, #4894 | Stable finite approval state. |
| `DemoRequired` | `true`, `false` | STP/SIP structured validation | `enum_now` as bool, not string enum | #4893, #4895 | Keep as boolean in Rust surfaces where possible. |
| `Prompt template set`, `planned_pvf_lane`, `validation_family`, `lane_registry_template_set`, `sprint_goal_ref`, issue refs, lane names, command strings, paths, titles, labels, slugs, summaries, notes | Open-ended strings | Templates, values renderer, cards, VPP/SPP fields | `string_extensible` | all follow-ons | Explicitly do not convert to closed enums. Lane names and validation families are policy/config identifiers, not lifecycle states. |

## Implementation Requirements For #4893

1. Introduce shared enum-backed internal types only for rows marked
   `enum_now`.
2. Preserve durable string spellings exactly for rendered cards, imported
   values, and public validator diagnostics.
3. Keep parser/display tests for every enum spelling and alias listed above.
4. Keep semantic guards outside enum membership, especially:
   - SRP `card_status=completed` requires final review results or a final
     policy exception.
   - SOR `card_status=completed` requires terminal integration truth.
   - SOR variance fields must still obey metric variance coupling.
   - Source-ref fields must still be required for non-unknown metric sources.
5. Do not convert open-ended identifiers such as PVF lane names, queue names,
   validation family names, paths, commands, labels, titles, or issue refs.

## Validation Tests Required By Follow-On Issues

- `#4893`: unit tests for parse/display/allowed-values for every enum marked
  `enum_now`; regression tests for legacy aliases listed for `ActivationState`,
  `IntegrationState`, and `VerificationScope`.
- `#4894`: prompt editor model tests proving select fields use the dedicated
  enum values rather than borrowed unrelated arrays; values validation tests
  for invalid enum spellings.
- `#4895`: structured prompt validation tests proving validator values match
  the shared enum surface; negative tests for mismatched editor-only values
  such as SOR `integration_state=blocked` and
  `verification_scope=not_run` unless those are deliberately accepted.

## Follow-On Linkage

- #4893 consumes the `enum_now`, alias, and `string_extensible` decisions.
- #4894 consumes the editor/value-renderer mismatch notes.
- #4895 consumes the structured validator mismatch notes.
- #4896 through #4899 should not depend on prompt-card enum implementation
  except through the stabilized shared contract rows above.
