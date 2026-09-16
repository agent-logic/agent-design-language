---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-remote-command-decomposition-review-prompt"
issue: 907
task_id: "issue-0907"
version: "0.92.2"
title: "[v0.92.2][CSDLC-REMOTE] Decompose the remote C-SDLC command owner"
branch: "codex/907-v0922-remote-command-decomposition"
generated_at: "2026-09-16T00:27:06.103043+00:00"
card_status: "reviewed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/907"
  - kind: "stp"
    ref: ".csdlc/issues/907/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/907/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/907/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/907/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/907/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/907/cards/stp.md"
  - ".csdlc/issues/907/cards/sip.md"
  - ".csdlc/issues/907/cards/vpp.md"
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
  - ".csdlc/issues/907/cards/stp.md"
  - ".csdlc/issues/907/cards/sip.md"
  - ".csdlc/issues/907/cards/vpp.md"
review_results:
  findings_status: "resolved_no_open_findings"
  recommended_outcome: "pass"
notes: "Independent exact-head review must trace complete production route migration, responsibility ownership, unchanged public/durable contracts and negative behavior, every acceptance and exclusion: 1. Extract the complete bounded remote responsibilities into cohesive production modules, with every existing route still using them. No replacement god module, parallel implementation, command-domain cycle or partially migrated route qualifies. 2. Preserve public CLI, schemas/serialized bytes, fields/defaults, error/status codes, artifact paths, authority and review checks, operation/intent/receipt digests, idempotency and fail-closed behavior. Preserve #849 closing/part-of linkage merge guard and current remote uncertainty recovery exactly. 3. Execute focused existing `csdlc-v3/tests/remote_publication_commands.rs`, operational CLI and remote merge-case suites as applicable, with golden serialized contracts and negative authority/review/base/head/linkage/corruption cases. Controlled fake remote transport must prove each affected mutation/readback and crash/reconciliation path; no live writes required by these tests. 4. Record before/after recursive source and responsibility inventory, route-to-owner mapping and dependency checks. Confirm thin dispatch and absence of cycles independently; facade line counts alone prove neither simplification nor preserved behavior. 5. Focused proof and required CI pass on the independently reviewed candidate; update bounded internal ownership/recovery docs. Build isolated candidate binaries, never replace another session's stable operational writer as a side effect of tests. PVF: deterministic local C-SDLC contract/integration, controlled fake authenticated remote transport, bounded CPU/disk, required milestone support gate. No lifecycle feature/schema redesign, weakened guard, local-owner scope, broad cleanup or live remote mutation. Stop for overlapping ownership, unstable baseline, public/serialized drift, missing route proof or replacement god module. acceptance: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `thin_entrypoints`, `no_command_domain_cycles`, `public_contract_compatible`, `decomposition_inventory_complete`. pvf: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `public_contract_drift_rejected`, `replacement_god_module_rejected`, `domain_cycle_rejected`, `native_v3_regression`, `serialized_contract_goldens`, `module_dependency_check`, `before_after_inventory`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `behavior_change`, `replacement_god_module`, `weakened_guard`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `new_lifecycle_features`, `schema_redesign`, `unrelated_cleanup`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. acceptance: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `thin_entrypoints`, `no_command_domain_cycles`, `public_contract_compatible`, `decomposition_inventory_complete`. pvf: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `public_contract_drift_rejected`, `replacement_god_module_rejected`, `domain_cycle_rejected`, `native_v3_regression`, `serialized_contract_goldens`, `module_dependency_check`, `before_after_inventory`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `behavior_change`, `replacement_god_module`, `weakened_guard`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `new_lifecycle_features`, `schema_redesign`, `unrelated_cleanup`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/907/cards/stp.md
- .csdlc/issues/907/cards/sip.md
- .csdlc/issues/907/cards/vpp.md

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

- Initial exact-head review found two P2s: alternate import spellings could bypass the dependency guard, and the SOR retained contradictory preparation fields. Both were fixed. Final independent review at e05b516d32596cfb250e00bbf4984ea129c9289f found no actionable findings.

### Dispositions

- No findings waived. The dependency guard now rejects grouped, absolute, root-alias, ancestor-alias, and direct sibling-alias spellings with negative fixtures. The SOR was normalized through native typed edits. Structural 3/3, remote 65/65, strict Clippy, formatting, and diff checks passed.

### Recommended Outcome

- pass

## Notes

Independent exact-head review must trace complete production route migration, responsibility ownership, unchanged public/durable contracts and negative behavior, every acceptance and exclusion: 1. Extract the complete bounded remote responsibilities into cohesive production modules, with every existing route still using them. No replacement god module, parallel implementation, command-domain cycle or partially migrated route qualifies. 2. Preserve public CLI, schemas/serialized bytes, fields/defaults, error/status codes, artifact paths, authority and review checks, operation/intent/receipt digests, idempotency and fail-closed behavior. Preserve #849 closing/part-of linkage merge guard and current remote uncertainty recovery exactly. 3. Execute focused existing `csdlc-v3/tests/remote_publication_commands.rs`, operational CLI and remote merge-case suites as applicable, with golden serialized contracts and negative authority/review/base/head/linkage/corruption cases. Controlled fake remote transport must prove each affected mutation/readback and crash/reconciliation path; no live writes required by these tests. 4. Record before/after recursive source and responsibility inventory, route-to-owner mapping and dependency checks. Confirm thin dispatch and absence of cycles independently; facade line counts alone prove neither simplification nor preserved behavior. 5. Focused proof and required CI pass on the independently reviewed candidate; update bounded internal ownership/recovery docs. Build isolated candidate binaries, never replace another session's stable operational writer as a side effect of tests. PVF: deterministic local C-SDLC contract/integration, controlled fake authenticated remote transport, bounded CPU/disk, required milestone support gate. No lifecycle feature/schema redesign, weakened guard, local-owner scope, broad cleanup or live remote mutation. Stop for overlapping ownership, unstable baseline, public/serialized drift, missing route proof or replacement god module. acceptance: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `thin_entrypoints`, `no_command_domain_cycles`, `public_contract_compatible`, `decomposition_inventory_complete`. pvf: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `public_contract_drift_rejected`, `replacement_god_module_rejected`, `domain_cycle_rejected`, `native_v3_regression`, `serialized_contract_goldens`, `module_dependency_check`, `before_after_inventory`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `behavior_change`, `replacement_god_module`, `weakened_guard`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `new_lifecycle_features`, `schema_redesign`, `unrelated_cleanup`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. acceptance: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `thin_entrypoints`, `no_command_domain_cycles`, `public_contract_compatible`, `decomposition_inventory_complete`. pvf: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `public_contract_drift_rejected`, `replacement_god_module_rejected`, `domain_cycle_rejected`, `native_v3_regression`, `serialized_contract_goldens`, `module_dependency_check`, `before_after_inventory`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `behavior_change`, `replacement_god_module`, `weakened_guard`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `new_lifecycle_features`, `schema_redesign`, `unrelated_cleanup`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.
