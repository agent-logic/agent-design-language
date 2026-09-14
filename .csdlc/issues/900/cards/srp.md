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
card_status: "under_review"
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
  findings_status: "addressed_pending_confirmation"
  recommended_outcome: "pending"
notes: "Local reproduction failed with a denied resident tool proposal. After repair, both exact Runtime integration tests pass, the package runtime example executes both accepted versions and all denial cases, cargo fmt --check passes, and git diff --check passes."
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

- The external P2 about repeated workloads is remediated by six role-specific views. Refreshed CI then exposed old empty-argument runtime.observe proposals in two Runtime integration fixtures. Independent review also found that the package example expected the pre-projection response and the hotload test did not assert execution. All identified callers and assertions are corrected locally.

### Dispositions

- Pending fresh exact-head confirmation and refreshed hosted CI. The production attempt-17 Runtime and evidence remain unchanged; this repair is limited to integration fixtures, the package example, and lifecycle truth.

### Recommended Outcome

- pending

## Notes

Local reproduction failed with a denied resident tool proposal. After repair, both exact Runtime integration tests pass, the package runtime example executes both accepted versions and all denial cases, cargo fmt --check passes, and git diff --check passes.
