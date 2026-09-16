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
notes: "The remediation changed production path authority and tests after the failed review. A different canonical fresh-session reviewer must inspect the final current-base immutable commit. No #894/#895 implementation scope is included."
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

- Canonical exact-head review of e090b75bf3922c642e09e749136e5e62b06f766f by fresh-session:1f19bfd4-51a6-4860-bbdf-a9da737c302d returned FAIL with one P1: ingestion admits valid repository paths containing spaces and other ASCII graphic characters, while remediation used a narrower grammar, silently discarded exact evidence paths, and could infer truncated semantic-anchor fragments such as docs/My and File.md instead of docs/My File.md. The generation-13 SOR therefore overstated complete evidence-path binding.

### Dispositions

- Finding accepted and remediated. Remediation now uses the shared ingestion validate_path contract, requires every synthesized evidence ID to resolve to an exact admitted evidence path, propagates any missing or invalid binding as an error, and no longer infers repository paths from semantic-anchor text. A completed canonical ReviewRecord fixture with docs/My File.md proves exact preservation and documentation ownership; the focused suite now passes 9/9.

### Recommended Outcome

- review_required

## Notes

The remediation changed production path authority and tests after the failed review. A different canonical fresh-session reviewer must inspect the final current-base immutable commit. No #894/#895 implementation scope is included.
