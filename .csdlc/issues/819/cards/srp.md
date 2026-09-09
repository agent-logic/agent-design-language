---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "csdlc-v3-retained-proof-review-prompt"
issue: 819
task_id: "issue-0819"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.08b][quality] Close C-SDLC v3 retained proof gaps"
branch: "codex/819-csdlc-v3-retained-proof"
generated_at: "2026-09-09T22:20:00Z"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/819"
  - kind: "stp"
    ref: ".csdlc/issues/819/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/819/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/819/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/819/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/819/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/819/cards/stp.md"
  - ".csdlc/issues/819/cards/sip.md"
  - ".csdlc/issues/819/cards/vpp.md"
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
  - ".csdlc/issues/819/cards/stp.md"
  - ".csdlc/issues/819/cards/sip.md"
  - ".csdlc/issues/819/cards/vpp.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "block"
notes: "First findings-first review: /root/fix_814_runtime at exact clean b91f486e019640be63400dd17da5dff31f926a1d. Safe validator replays passed but were correctly classified as insufficient against the semantic and authority defects. Current repaired head must receive a fresh independent verdict."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/819/cards/stp.md
- .csdlc/issues/819/cards/sip.md
- .csdlc/issues/819/cards/vpp.md

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

- At b91f486e019640be63400dd17da5dff31f926a1d, independent reviewer /root/fix_814_runtime found: P1 broad root-cause test mappings promoted non-proving criteria; P1 generic PR #591 language fabricated criterion-specific amendment approval; P2 empty candidate_evidence passed vacuously; P3 the retained test log had a trailing blank line.

### Dispositions

- All four findings were repaired: execution credit is limited to 51 previously source-supported rows joined to complete candidate execution; 101 non-proving rows are criterion-specific proposals pending operator review with release_ready=false; candidate evidence must be nonempty and exactly match declared source paths; log output is normalized to one trailing newline. Fresh exact-head rereview remains required.

### Recommended Outcome

- block

## Notes

First findings-first review: /root/fix_814_runtime at exact clean b91f486e019640be63400dd17da5dff31f926a1d. Safe validator replays passed but were correctly classified as insufficient against the semantic and authority defects. Current repaired head must receive a fresh independent verdict.
