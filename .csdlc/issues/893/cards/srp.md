---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-remediation-planner-review-prompt"
issue: 893
task_id: "issue-0893"
version: "0.92.2"
title: "[v0.92.2][CF-REMEDIATE] Generate a bounded remediation plan from review findings"
branch: "codex/893-v0922-remediation-planner"
generated_at: "2026-09-12T00:10:02.687479+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/893"
  - kind: "stp"
    ref: ".csdlc/issues/893/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/893/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/893/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/893/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/893/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/893/cards/stp.md"
  - ".csdlc/issues/893/cards/sip.md"
  - ".csdlc/issues/893/cards/vpp.md"
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
  - ".csdlc/issues/893/cards/stp.md"
  - ".csdlc/issues/893/cards/sip.md"
  - ".csdlc/issues/893/cards/vpp.md"
review_results:
  findings_status: "findings_remediated_pending_fresh_review"
  recommended_outcome: "review_required"
notes: "Prior review PASS/review truth is superseded by this remediation. Fresh independent exact-head review is required against the new immutable commit before publication or merge."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/893/cards/stp.md
- .csdlc/issues/893/cards/sip.md
- .csdlc/issues/893/cards/vpp.md

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

- Latest exact-head review at 856843ce37d622063a3df05a19231c86fdfabdf6 found only lifecycle-card truth findings: VPP still recorded stale `codefriend_remediate` 4/4 proof after the fifth regression was added, and SOR values contradicted themselves by recording 5/5 elsewhere while still claiming 4/4 and no absolute path leakage despite Git-common CARGO_TARGET_DIR commands.

### Dispositions

- Findings accepted and remediated through native card edit. VPP and SOR now consistently record `codefriend_remediate` 5/5 for the current regression suite. SOR no longer claims zero absolute path leakage; it explicitly bounds the host-local Git-common CARGO_TARGET_DIR to validation-cache use and distinguishes it from generated product artifacts.

### Recommended Outcome

- review_required

## Notes

Prior review PASS/review truth is superseded by this remediation. Fresh independent exact-head review is required against the new immutable commit before publication or merge.
