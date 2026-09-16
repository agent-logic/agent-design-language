---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-codefriend-fitness-ci-review-prompt"
issue: 888
task_id: "issue-0888"
version: "0.92.2"
title: "[v0.92.2][CF-GOV-CI] Execute architecture fitness functions as a CI gate"
branch: "codex/888-v0922-codefriend-fitness-ci"
generated_at: "2026-09-12T00:05:18.245012+00:00"
card_status: "completed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/888"
  - kind: "stp"
    ref: ".csdlc/issues/888/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/888/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/888/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/888/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/888/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/888/cards/stp.md"
  - ".csdlc/issues/888/cards/sip.md"
  - ".csdlc/issues/888/cards/vpp.md"
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
  - ".csdlc/issues/888/cards/stp.md"
  - ".csdlc/issues/888/cards/sip.md"
  - ".csdlc/issues/888/cards/vpp.md"
review_results:
  findings_status: "resolved_no_open_findings"
  recommended_outcome: "approve_local_publication_hosted_ci_pending"
notes: "Evidence: .csdlc/evidence/888/review/final-source-review.json. Scope is final source plus local proof; publication enables the required actual hosted CI, which is not yet claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/888/cards/stp.md
- .csdlc/issues/888/cards/sip.md
- .csdlc/issues/888/cards/vpp.md

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

- Independent final source/local proof review PASS at b6ba3f77f9dab09aa95e840f731176efc0451cc0. R1 and R2 resolved; no open findings. All16sourcehashes and installedbinary hash verified, raw candidate/pass/fail/error reports checked for exact parity and0/0/1/2exits; optimized qualification independently rerun on allfour. Final record-only SHA attestation required before native publication. Hosted CI remains required before completion.

### Dispositions

- R1 resolved by bounded public governance facade policy with broader local.rs explicitly unassessed. R2 resolved by explicit Python checks, regression for optimization0/1/2, and independent optimized proof checks. No waiver.

### Recommended Outcome

- approve_local_publication_hosted_ci_pending

## Notes

Evidence: .csdlc/evidence/888/review/final-source-review.json. Scope is final source plus local proof; publication enables the required actual hosted CI, which is not yet claimed.
