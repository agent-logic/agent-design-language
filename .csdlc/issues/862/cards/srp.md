---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-local-command-decomposition-review-prompt"
issue: 862
task_id: "issue-0862"
version: "0.92.2"
title: "[v0.92.2][C-SDLC v3][refactor] Decompose the local command owner"
branch: "codex/862-v0922-local-command-decomposition"
generated_at: "2026-09-16T00:27:06.103043+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/862"
  - kind: "stp"
    ref: ".csdlc/issues/862/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/862/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/862/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/862/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/862/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/862/cards/stp.md"
  - ".csdlc/issues/862/cards/sip.md"
  - ".csdlc/issues/862/cards/vpp.md"
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
  - ".csdlc/issues/862/cards/stp.md"
  - ".csdlc/issues/862/cards/sip.md"
  - ".csdlc/issues/862/cards/vpp.md"
review_results:
  findings_status: "no_actionable_findings"
  recommended_outcome: "Ready for native review and publication; hosted CI, merge and terminal closeout remain pending."
notes: "Independent reviewer reran the structural suite 3/3 at b0bcbe6cec and found no public or serialized contract drift, lost routes, cycles or behavioral changes. The full focused result is 152/152 with strict clippy, formatting, diff checks and native card validation green. The unchanged owner-lane 1.0.3-versus-1.0.5 mismatch remains separately disclosed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/862/cards/stp.md
- .csdlc/issues/862/cards/sip.md
- .csdlc/issues/862/cards/vpp.md

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

- Final exact-head independent re-review at b0bcbe6cec found no actionable findings. The three earlier P2 findings are resolved.

### Dispositions

- Resolved: alternate, absolute, grouped and aliased sibling imports fail closed while canonical imports remain rank checked. Resolved: filesystem, results and failpoints are cohesive leaf owners separate from lifecycle storage. Resolved: lifecycle cards consistently record implementation and review state.

### Recommended Outcome

- Ready for native review and publication; hosted CI, merge and terminal closeout remain pending.

## Notes

Independent reviewer reran the structural suite 3/3 at b0bcbe6cec and found no public or serialized contract drift, lost routes, cycles or behavioral changes. The full focused result is 152/152 with strict clippy, formatting, diff checks and native card validation green. The unchanged owner-lane 1.0.3-versus-1.0.5 mismatch remains separately disclosed.
