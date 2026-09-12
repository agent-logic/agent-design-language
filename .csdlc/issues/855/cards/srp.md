---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "issue-855-provider-neutral-dynamic-agent-lifecycle-review-prompt"
issue: 855
task_id: "issue-0855"
version: "v0.92.2"
title: "[v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle"
branch: "codex/855-provider-neutral-lifecycle"
generated_at: "2026-09-12"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/855"
  - kind: "stp"
    ref: ".csdlc/issues/855/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/855/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/855/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/855/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/855/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/855/cards/stp.md"
  - ".csdlc/issues/855/cards/sip.md"
  - ".csdlc/issues/855/cards/vpp.md"
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
  - ".csdlc/issues/855/cards/stp.md"
  - ".csdlc/issues/855/cards/sip.md"
  - ".csdlc/issues/855/cards/vpp.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "pass"
notes: "Independent source review complete through d96f90ecc7bcea702605f73814d9fce02486df28; final card and proof document changes awaiting root acknowledgment. Actual hosted demo authorized butpending gcloud preflight. Preserve required CI and hosted boundaries."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/855/cards/stp.md
- .csdlc/issues/855/cards/sip.md
- .csdlc/issues/855/cards/vpp.md

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

- Source findings at 3734: local CLI deadline/cap, asynchronous health publication race, and premature action success accounting. All corrected and independently reviewed at 41e5. Follow-on Runtime mock delay bound independently reviewed at d96f90; no actionable source findings remain. Full hosted and remoteCI proof remains pending.

### Dispositions

- review_804 independently approved corrected Rust 41e5 and narrow mock delta d96f90. Root approved 61095 harnesscleanup-timer delta and exactsource finding dispositions. PASS applies to truthful draft publication only; no full acceptance/merge readiness claim.

### Recommended Outcome

- pass

## Notes

Independent source review complete through d96f90ecc7bcea702605f73814d9fce02486df28; final card and proof document changes awaiting root acknowledgment. Actual hosted demo authorized butpending gcloud preflight. Preserve required CI and hosted boundaries.
