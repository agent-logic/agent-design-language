---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-six-resident-qualification-review-prompt"
issue: 900
task_id: "issue-0900"
version: "0.92.2"
title: "[v0.92.2][QUAL-RESIDENT] Execute resident workload and signed restore qualification"
branch: "codex/900-v0922-six-resident-qualification"
generated_at: "2026-09-12T00:15:07.665153+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/900"
  - kind: "stp"
    ref: ".csdlc/issues/900/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/900/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/900/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/900/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/900/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/900/cards/stp.md"
  - ".csdlc/issues/900/cards/sip.md"
  - ".csdlc/issues/900/cards/vpp.md"
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
  - ".csdlc/issues/900/cards/stp.md"
  - ".csdlc/issues/900/cards/sip.md"
  - ".csdlc/issues/900/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head review must trace actual production provider/resident invocation, observable effects, complete scenario denominator and evidence boundaries, every acceptance and exclusion: 1. Resolve the reviewed #852 failure-event repair (QUAL-RUNTIME) and RT-PROVIDER/#855 first. Execute six actual distinct roles from the bounded plan: shepherd_controller, planner, tool_executor, runtime_observer, recovery_custodian and reviewer_escalation. Bind role/tool authority/model/configuration identities and exact producer revision; placeholder artifact/configuration hashes are rejected before execution. 2. Retain each resident's tick, ACC/UTS operation, request/correlation identity, generated/provider result where applicable and observable workload effect. Measure exact admitted population and prove six distinct workloads. Caller labels or six receipt rows alone do not demonstrate six executions. 3. Exercise the production signed dehydration boundary, validate the retained population/lineage, restore through production verification and execute resumed work. Completed cases must not replay; only the exact pending case resumes. Match pre/post identity, signature, workload effects and population counts. 4. Execute tamper, omission and substitution negatives on isolated copies: changed signature/payload, removed resident, swapped lineage/provider/config or stale snapshot must be rejected before restored work. Capture actual denial and absence of inappropriate effects. 5. Distinguish deterministic local mock-provider regression from live model qualification. Complete the approved qualification profile's real production provider work; a mock trace alone cannot close the operational claim. Pin model/artifact/configuration, environment and resource limits before calls. 6. Independently review exact candidate, run artifacts, full scenario denominator, signatures and effect evidence. Sanitize public references while preserving auditable authorized access to sensitive proof; privately retained historical Run72 detail is not silently substituted for this current run. - Acceptance: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, actual_execution_receipts, independent_review. - PVF: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, tamper_rejected, omission_rejected, substitution_rejected, mock_vs_live_explicit, resident_execution. Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/900/cards/stp.md
- .csdlc/issues/900/cards/sip.md
- .csdlc/issues/900/cards/vpp.md

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

Independent exact-head review must trace actual production provider/resident invocation, observable effects, complete scenario denominator and evidence boundaries, every acceptance and exclusion: 1. Resolve the reviewed #852 failure-event repair (QUAL-RUNTIME) and RT-PROVIDER/#855 first. Execute six actual distinct roles from the bounded plan: shepherd_controller, planner, tool_executor, runtime_observer, recovery_custodian and reviewer_escalation. Bind role/tool authority/model/configuration identities and exact producer revision; placeholder artifact/configuration hashes are rejected before execution. 2. Retain each resident's tick, ACC/UTS operation, request/correlation identity, generated/provider result where applicable and observable workload effect. Measure exact admitted population and prove six distinct workloads. Caller labels or six receipt rows alone do not demonstrate six executions. 3. Exercise the production signed dehydration boundary, validate the retained population/lineage, restore through production verification and execute resumed work. Completed cases must not replay; only the exact pending case resumes. Match pre/post identity, signature, workload effects and population counts. 4. Execute tamper, omission and substitution negatives on isolated copies: changed signature/payload, removed resident, swapped lineage/provider/config or stale snapshot must be rejected before restored work. Capture actual denial and absence of inappropriate effects. 5. Distinguish deterministic local mock-provider regression from live model qualification. Complete the approved qualification profile's real production provider work; a mock trace alone cannot close the operational claim. Pin model/artifact/configuration, environment and resource limits before calls. 6. Independently review exact candidate, run artifacts, full scenario denominator, signatures and effect evidence. Sanitize public references while preserving auditable authorized access to sensitive proof; privately retained historical Run72 detail is not silently substituted for this current run. - Acceptance: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, actual_execution_receipts, independent_review. - PVF: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, tamper_rejected, omission_rejected, substitution_rejected, mock_vs_live_explicit, resident_execution. Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.
