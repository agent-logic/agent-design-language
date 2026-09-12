---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-merge-linkage-admission-review-prompt"
issue: 849
task_id: "issue-0849"
version: "0.92.2"
title: "[v0.92.2][C-SDLC v3][P2] Preserve publication linkage during native PR merge"
branch: "codex/849-v0922-merge-linkage-admission"
generated_at: "2026-09-12T00:22:05.416044+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/849"
  - kind: "stp"
    ref: ".csdlc/issues/849/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/849/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/849/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/849/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/849/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/849/cards/stp.md"
  - ".csdlc/issues/849/cards/sip.md"
  - ".csdlc/issues/849/cards/vpp.md"
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
  - ".csdlc/issues/849/cards/stp.md"
  - ".csdlc/issues/849/cards/sip.md"
  - ".csdlc/issues/849/cards/vpp.md"
review_results:
  findings_status: "no_actionable_findings"
  recommended_outcome: "accept"
notes: "Reviewer independently reran 16 merge cases and one qualified numeric adapter test; inspected exact review linkage digest, qualified issue/mode, initial/fresh/postmerge relation and state, complete-page fail-closed checks, durable intent/target guard and replay. Evidence .csdlc/evidence/849/VALIDATION.md. No live merge proof, no CI pass claim."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/849/cards/stp.md
- .csdlc/issues/849/cards/sip.md
- .csdlc/issues/849/cards/vpp.md

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

- Independent sprint8_909 reviewed implementation 0264f321ac0e19fb9ae957b4f05b6dfb2f7c63b2: no actionable source/test findings. Final card and any integration delta require renewal. Full-suite release-preflight baseline failure remains a separate blocker, not waived.

### Dispositions

- No implementation findings to repair. Preserve separate release inventory failure and #948 pending owner delta.

### Recommended Outcome

- accept

## Notes

Reviewer independently reran 16 merge cases and one qualified numeric adapter test; inspected exact review linkage digest, qualified issue/mode, initial/fresh/postmerge relation and state, complete-page fail-closed checks, durable intent/target guard and replay. Evidence .csdlc/evidence/849/VALIDATION.md. No live merge proof, no CI pass claim.
