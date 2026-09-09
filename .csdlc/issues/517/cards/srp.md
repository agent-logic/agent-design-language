---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "tail-01-quality-gate-review-prompt"
issue: 517
task_id: "issue-0517"
version: "v0.92.1"
title: "[v0.92.1][TAIL-01] Quality gate"
branch: "codex/517-tail-01-quality-gate"
generated_at: "2026-09-09T00:56:48.157331+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/517"
  - kind: "stp"
    ref: ".csdlc/issues/517/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/517/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/517/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/517/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/517/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/517/cards/stp.md"
  - ".csdlc/issues/517/cards/sip.md"
  - ".csdlc/issues/517/cards/vpp.md"
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
  - ".csdlc/issues/517/cards/stp.md"
  - ".csdlc/issues/517/cards/sip.md"
  - ".csdlc/issues/517/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Review result applies to this repair work product; publication requires an independent final committed-head recheck. Release remains BLOCKED."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/517/cards/stp.md
- .csdlc/issues/517/cards/sip.md
- .csdlc/issues/517/cards/vpp.md

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

- Independent reviewer review_748_repairs checked branch handling, complete Git review paths, current card truth, and version1.0.4 templates; no unresolved findings in the reviewed work product. Final immutable-head attestation is retained with publication evidence.

### Dispositions

- Original branch finding repaired with shared pre-intent validation and encoded readback; missing-path finding repaired with actual complete Git diff; stale cards repaired via versioned typed fields. Rename-source and stale-template review followups corrected and rechecked.

### Recommended Outcome

- pass

## Notes

Review result applies to this repair work product; publication requires an independent final committed-head recheck. Release remains BLOCKED.
