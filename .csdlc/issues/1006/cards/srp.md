---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-native-coordination-completion-review-prompt"
issue: 1006
task_id: "issue-1006"
version: "0.92.2"
title: "[v0.92.2][C-SDLC] Support native completion closure for coordination issues"
branch: "codex/1006-v0922-native-coordination-completion"
generated_at: "2026-09-16T02:45:19.992791+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1006"
  - kind: "stp"
    ref: ".csdlc/issues/1006/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1006/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/1006/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/1006/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/1006/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/1006/cards/stp.md"
  - ".csdlc/issues/1006/cards/sip.md"
  - ".csdlc/issues/1006/cards/vpp.md"
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
  - ".csdlc/issues/1006/cards/stp.md"
  - ".csdlc/issues/1006/cards/sip.md"
  - ".csdlc/issues/1006/cards/vpp.md"
review_results:
  findings_status: "resolved"
  recommended_outcome: "Approve updated publication after exact-head review; new CI pending."
notes: "Reviewer is independent of implementer /root/palace_authority. Explicit coordination-only contract, operator authority, authenticated child state/merged linkage, admin preservation, installed ordinary path, semantic retry controls and retained proof reviewed. No remote publication or merge performed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/1006/cards/stp.md
- .csdlc/issues/1006/cards/sip.md
- .csdlc/issues/1006/cards/vpp.md

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

- External review P2 at6498d655: valid child Closes #887 plus Part of #505 wrongly rejected with github_merge_linkage_ineligible. Independently reproduced installed beforefix. Added completed-child-only parent-context handling and exact closing/evidence negative regressions.

### Dispositions

- P2 fixed; /root/repair_882_ci working-diff review PASS with no findings. Exact updated-head receipt and fresh CI remain required. Earlier independent review and original proof preserved as historical baseline.

### Recommended Outcome

- Approve updated publication after exact-head review; new CI pending.

## Notes

Reviewer is independent of implementer /root/palace_authority. Explicit coordination-only contract, operator authority, authenticated child state/merged linkage, admin preservation, installed ordinary path, semantic retry controls and retained proof reviewed. No remote publication or merge performed.
