---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-provider-recovery-qualification-review-prompt"
issue: 901
task_id: "issue-0901"
version: "0.92.2"
title: "[v0.92.2][QUAL-PROVIDER] Execute real provider failure and recovery qualification"
branch: "codex/901-v0922-provider-recovery-qualification"
generated_at: "2026-09-12T00:15:14.473149+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/901"
  - kind: "stp"
    ref: ".csdlc/issues/901/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/901/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/901/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/901/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/901/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/901/cards/stp.md"
  - ".csdlc/issues/901/cards/sip.md"
  - ".csdlc/issues/901/cards/vpp.md"
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
  - ".csdlc/issues/901/cards/stp.md"
  - ".csdlc/issues/901/cards/sip.md"
  - ".csdlc/issues/901/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head review must trace actual production provider/resident invocation, observable effects, complete scenario denominator and evidence boundaries, every acceptance and exclusion: 1. RT-PROVIDER/#855 must deliver its registered-provider lifecycle before this qualification. Select a scoped owned test subprocess/session through that production route, observe actual start, kill/loss, deadline timeout and cancellation/interruption, and retain process/request identity, timestamps, exit state, Runtime outcome and correlation. 2. Restore service through a healthy configured production provider and execute successful subsequent work. Prove bounded retry/recovery, no duplicate accepted work and no stale provider identity reused. A ready flag without a real healthy result is insufficient. 3. Confirm real failure triggers reached provider invocation. Negative fixtures reject supplied failure flags, static reference-trace metadata, absent execution logs, wrong process/request identity and timeout classifications without elapsed/deadline evidence. 4. Exercise loss, timeout and interruption separately; do not infer one from another. Preserve statuses, cancellation, truncation and redaction at adapter boundaries. Where WSS failure-event evidence is consumed, require the reviewed #852 output rather than reimplementing or claiming it here; the exact graph remains RT-PROVIDER as this task's execution prerequisite, with full cross-producer joining in QUAL-EVIDENCE. 5. Use only task-owned processes and the explicitly approved environment; never kill shared resident/provider services. Independently review exact source/run/profile and all scenario evidence. Local mock transport proof is labeled separately and cannot replace required production provider execution. - Acceptance: actual_process_loss, actual_timeout_interrupt, healthy_recovery, actual_execution_receipts, independent_review. - PVF: actual_process_loss, actual_timeout_interrupt, healthy_recovery, caller_failure_flags_not_proof, reference_trace_not_execution, provider_failure_recovery. Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/901/cards/stp.md
- .csdlc/issues/901/cards/sip.md
- .csdlc/issues/901/cards/vpp.md

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

Independent exact-head review must trace actual production provider/resident invocation, observable effects, complete scenario denominator and evidence boundaries, every acceptance and exclusion: 1. RT-PROVIDER/#855 must deliver its registered-provider lifecycle before this qualification. Select a scoped owned test subprocess/session through that production route, observe actual start, kill/loss, deadline timeout and cancellation/interruption, and retain process/request identity, timestamps, exit state, Runtime outcome and correlation. 2. Restore service through a healthy configured production provider and execute successful subsequent work. Prove bounded retry/recovery, no duplicate accepted work and no stale provider identity reused. A ready flag without a real healthy result is insufficient. 3. Confirm real failure triggers reached provider invocation. Negative fixtures reject supplied failure flags, static reference-trace metadata, absent execution logs, wrong process/request identity and timeout classifications without elapsed/deadline evidence. 4. Exercise loss, timeout and interruption separately; do not infer one from another. Preserve statuses, cancellation, truncation and redaction at adapter boundaries. Where WSS failure-event evidence is consumed, require the reviewed #852 output rather than reimplementing or claiming it here; the exact graph remains RT-PROVIDER as this task's execution prerequisite, with full cross-producer joining in QUAL-EVIDENCE. 5. Use only task-owned processes and the explicitly approved environment; never kill shared resident/provider services. Independently review exact source/run/profile and all scenario evidence. Local mock transport proof is labeled separately and cannot replace required production provider execution. - Acceptance: actual_process_loss, actual_timeout_interrupt, healthy_recovery, actual_execution_receipts, independent_review. - PVF: actual_process_loss, actual_timeout_interrupt, healthy_recovery, caller_failure_flags_not_proof, reference_trace_not_execution, provider_failure_recovery. Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.
