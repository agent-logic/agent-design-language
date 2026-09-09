---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "validation-integrity-review-prompt"
issue: 816
task_id: "issue-0816"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.11][quality] Repair redaction and hot-reload validation integrity"
branch: "codex/816-validation-integrity"
generated_at: "2026-09-09T20:50:00Z"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/816"
  - kind: "stp"
    ref: ".csdlc/issues/816/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/816/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/816/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/816/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/816/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/816/cards/stp.md"
  - ".csdlc/issues/816/cards/sip.md"
  - ".csdlc/issues/816/cards/vpp.md"
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
  - ".csdlc/issues/816/cards/stp.md"
  - ".csdlc/issues/816/cards/sip.md"
  - ".csdlc/issues/816/cards/vpp.md"
review_results:
  findings_status: "resolved_pending_rereview"
  recommended_outcome: "pending"
notes: "Fresh reviewer must verify production output, complete declared Runtime/UI/evidence coverage, exclusion classification, omitted-artifact rejection, and causal hot-reload tests at the new exact head."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/816/cards/stp.md
- .csdlc/issues/816/cards/sip.md
- .csdlc/issues/816/cards/vpp.md

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

- First review: P1 source scanning did not prove serialized Runtime output; P2 line grep accepted duplicate sensitive keys. First rereview: P1 structural sensitive-field checks did not cover every manifest publication. Post-CI exact-head review: P2 the self-selected manifest omitted a declared #512 evidence artifact containing a machine-local path.

### Dispositions

- Fixed by executing project_v1, rejecting duplicate JSON keys, structurally validating every included JSON, scanning non-JSON text, deriving manifest completeness from all tracked #512 evidence plus SRP/SOR declarations, recording reviewed non-publication exclusions, redacting the exposed path, and proving an omitted declared artifact fails.

### Recommended Outcome

- pending

## Notes

Fresh reviewer must verify production output, complete declared Runtime/UI/evidence coverage, exclusion classification, omitted-artifact rejection, and causal hot-reload tests at the new exact head.
