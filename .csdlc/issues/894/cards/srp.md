---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-test-planner-review-prompt"
issue: 894
task_id: "issue-0894"
version: "0.92.2"
title: "[v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings"
branch: "codex/894-v0922-test-planner"
generated_at: "2026-09-12T00:10:09.925246+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/894"
  - kind: "stp"
    ref: ".csdlc/issues/894/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/894/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/894/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/894/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/894/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/894/cards/stp.md"
  - ".csdlc/issues/894/cards/sip.md"
  - ".csdlc/issues/894/cards/vpp.md"
in_scope_surfaces:
  - "tracked changes for this issue branch"
evidence_policy:
  - "Use repository evidence, targeted validation output, and linked issue-bundle artifacts only."
validation_inputs:
  - "Issue-local proofs recorded in the VPP and SOR."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
reviewer_constraints:
  - "Do not widen issue scope."
  - "Do not merge, publish, or close the issue."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
  - "Refuse approving behavior outside the recorded issue scope."
follow_up_routing:
  - "Route actionable defects back to the issue branch before PR publication."
non_claims:
  - "This prompt does not claim review has already run."
  - "This prompt does not guarantee review quality by itself."
policy_refs:
  - ".csdlc/issues/894/cards/stp.md"
  - ".csdlc/issues/894/cards/sip.md"
  - ".csdlc/issues/894/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head review must trace actual generator/reader or publication admission and source immutability, every acceptance and exclusion: 1. Consume actual CF-SYNTHESIS output and generate a readable test plan mapping every selected finding to behavior under test, source evidence, concrete proposed test location, fixture/input, expected failure before a fix, expected result after it and suitable assertion. Distinguish proposed paths from existing files. 2. Explain why each case detects the reported behavior; reject tests that merely mirror implementation or validate an unrelated schema. Record missing test infrastructure, nondeterminism and unsupported claims explicitly. Include validation lane, role and resource requirements suitable for the repository under review. 3. Run production generation and reader on predecessor output plus known findings; independently check useful behavior/fixture/assertion mapping. Reject missing evidence, incompatible runs, vague test placeholders and unmapped selected findings. 4. Prove generation changes no inspected source and creates no issue or test file in that repository. The product supplies a complete actionable plan; test implementation remains human-controlled. A canned packet or plan-only implementation is insufficient. acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/894/cards/stp.md
- .csdlc/issues/894/cards/sip.md
- .csdlc/issues/894/cards/vpp.md

## In-Scope Surfaces

- tracked changes for this issue branch

## Evidence Rules

- Use repository evidence, targeted validation output, and linked issue-bundle artifacts only.

## Validation Inputs

- Issue-local proofs recorded in the VPP and SOR.

## Allowed Dispositions

- PASS
- BLOCK
- NEEDS_FOLLOWUP

## Reviewer Constraints

- Do not widen issue scope.
- Do not merge, publish, or close the issue.

## Refusal Policy

- Refuse claims that are unsupported by repository evidence.
- Refuse approving behavior outside the recorded issue scope.

## Follow-up Routing

- Route actionable defects back to the issue branch before PR publication.

## Non-Claims

- This prompt does not claim review has already run.
- This prompt does not guarantee review quality by itself.

## Review Results

When finalizing review, record the machine-readable review result in frontmatter:

```yaml
review_results:
  findings_status: "no_findings | findings_present | review_unavailable | review_timeout | review_cancelled | review_failed"
  recommended_outcome: "pass | block | needs_followup"
```

### Findings

- Implementation review has not run; no implementation exists from this preparation.

### Dispositions

- No implementation finding is accepted, resolved or waived by preparation.

### Recommended Outcome

- block

## Notes

Independent exact-head review must trace actual generator/reader or publication admission and source immutability, every acceptance and exclusion: 1. Consume actual CF-SYNTHESIS output and generate a readable test plan mapping every selected finding to behavior under test, source evidence, concrete proposed test location, fixture/input, expected failure before a fix, expected result after it and suitable assertion. Distinguish proposed paths from existing files. 2. Explain why each case detects the reported behavior; reject tests that merely mirror implementation or validate an unrelated schema. Record missing test infrastructure, nondeterminism and unsupported claims explicitly. Include validation lane, role and resource requirements suitable for the repository under review. 3. Run production generation and reader on predecessor output plus known findings; independently check useful behavior/fixture/assertion mapping. Reject missing evidence, incompatible runs, vague test placeholders and unmapped selected findings. 4. Prove generation changes no inspected source and creates no issue or test file in that repository. The product supplies a complete actionable plan; test implementation remains human-controlled. A canned packet or plan-only implementation is insufficient. acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.
