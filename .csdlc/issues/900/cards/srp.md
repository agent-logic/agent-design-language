---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-six-resident-qualification-review-prompt"
issue: 900
task_id: "issue-0900"
version: "0.92.2"
title: "[v0.92.2][QUAL-RESIDENT] Execute resident workload and signed restore qualification"
branch: "codex/900-v0922-six-resident-qualification"
generated_at: "2026-09-12T00:15:07.665153+00:00"
card_status: "reviewed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/900"
  - kind: "stp"
    ref: ".csdlc/issues/900/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/900/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/900/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/900/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/900/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/900/cards/stp.md"
  - ".csdlc/issues/900/cards/sip.md"
  - ".csdlc/issues/900/cards/vpp.md"
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
  - ".csdlc/issues/900/cards/stp.md"
  - ".csdlc/issues/900/cards/sip.md"
  - ".csdlc/issues/900/cards/vpp.md"
review_results:
  findings_status: "addressed"
  recommended_outcome: "pass"
notes: "Independent review verified 12 executed workloads, six unique views, six unique pre effects, six unique post effects, exact attempt-17 public/private hashes, seven deny-before-effect negatives, two Python harness suites, eight focused Rust tests and local-only scope."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/900/cards/stp.md
- .csdlc/issues/900/cards/sip.md
- .csdlc/issues/900/cards/vpp.md

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

- The external P2 found that six residents repeated one empty runtime.observe proposal. The remediation enforces six role-specific views and argument/effect bindings. Independent review at 4e9d2663a0091027e52c768d7b7bb3cf2f0296c1 verified the implementation and attempt-17 evidence; its only finding was stale lifecycle text, corrected by this native edit.

### Dispositions

- Pass after this lifecycle correction and exact-head confirmation. The distinct-workload P2 is addressed; CI, merge and closeout remain pending.

### Recommended Outcome

- pass

## Notes

Independent review verified 12 executed workloads, six unique views, six unique pre effects, six unique post effects, exact attempt-17 public/private hashes, seven deny-before-effect negatives, two Python harness suites, eight focused Rust tests and local-only scope.
