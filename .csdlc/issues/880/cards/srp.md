---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "880-ci-ingestion-review-prompt"
issue: 880
task_id: "issue-0880"
version: "v0.92.2"
title: "[v0.92.2][CF-ADAPTER-CI] Ingest CI repository inputs into a repository packet"
branch: "codex/880-ci-ingestion"
generated_at: "2026-09-12T05:07:36.211905+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/880"
  - kind: "stp"
    ref: ".csdlc/issues/880/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/880/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/880/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/880/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/880/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/880/cards/stp.md"
  - ".csdlc/issues/880/cards/sip.md"
  - ".csdlc/issues/880/cards/vpp.md"
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
  - ".csdlc/issues/880/cards/stp.md"
  - ".csdlc/issues/880/cards/sip.md"
  - ".csdlc/issues/880/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "needs_followup"
notes: "Implemented CI acquisition via the accepted local immutable capture and production packet reader. Separate bounded allowlisted receipt preserves packet parity and explicit revision authority. Ten CLI scenarios pass locally; required installed hosted smoke and upload are wired into adl-ci. Final focused checks, independent review and hosted CI remain pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/880/cards/stp.md
- .csdlc/issues/880/cards/sip.md
- .csdlc/issues/880/cards/vpp.md

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

- Independent implementation review has not run.

### Dispositions

- Await exact candidate review after focused validation.

### Recommended Outcome

- needs_followup

## Notes

Implemented CI acquisition via the accepted local immutable capture and production packet reader. Separate bounded allowlisted receipt preserves packet parity and explicit revision authority. Ten CLI scenarios pass locally; required installed hosted smoke and upload are wired into adl-ci. Final focused checks, independent review and hosted CI remain pending.
