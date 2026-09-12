---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "879-github-ingestion-review-prompt"
issue: 879
task_id: "issue-0879"
version: "v0.92.2"
title: "[v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet"
branch: "codex/879-github-ingestion"
generated_at: "2026-09-12T05:03:33.504300+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/879"
  - kind: "stp"
    ref: ".csdlc/issues/879/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/879/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/879/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/879/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/879/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/879/cards/stp.md"
  - ".csdlc/issues/879/cards/sip.md"
  - ".csdlc/issues/879/cards/vpp.md"
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
  - ".csdlc/issues/879/cards/stp.md"
  - ".csdlc/issues/879/cards/sip.md"
  - ".csdlc/issues/879/cards/vpp.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "pass"
notes: "Independent reviewer review_836 approved source at edc4d0a7df7b3c55c20f65d6a08c9a53affaa502 with no actionable production findings. R2 publication-card placeholders corrected in this record-only revision; final metadata acknowledgement is required before native publication. Source proof unchanged; CI pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/879/cards/stp.md
- .csdlc/issues/879/cards/sip.md
- .csdlc/issues/879/cards/vpp.md

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

- R1 cumulative metadata resource bound: fixed and regression passed. R2 P2: rendered SOR execution/PVF/verification placeholders and SRP result/source refs were not populated by summary aliases; corrected using native actual semantic fields and inline pairs. Original findings preserved in .csdlc/evidence/879/REVIEW.md.

### Dispositions

- R1 resolved by8MiB aggregate response cap and executed HTTP regression. R2 correction submitted for record-only acknowledgement; all six rendered cards scanned for literal placeholders and native validation required. Source approval remains exact-head edc4d0a7df.

### Recommended Outcome

- pass

## Notes

Independent reviewer review_836 approved source at edc4d0a7df7b3c55c20f65d6a08c9a53affaa502 with no actionable production findings. R2 publication-card placeholders corrected in this record-only revision; final metadata acknowledgement is required before native publication. Source proof unchanged; CI pending.
