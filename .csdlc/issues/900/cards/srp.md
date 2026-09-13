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
notes: "Independent review verified the signed local host identities, local_ollama provider binding and substitution denial, all 12 provider result/status/effect hashes, exact producer and binary identities, attempt-11 public hash chain, seven deny-before-effect negatives, three Python harness suites, the six-case plan validator, six Rust continuity tests, formatting, Clippy, JSON parsing and diff hygiene."
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

- The initial review found four actionable evidence and lifecycle-truth gaps. All were repaired. Renewed independent exact-head review at 2d119db18595e136448aab1e69571e916eb77471 reported no actionable findings after the current origin/main merge.

### Dispositions

- Pass for native draft publication. The seven negative scenarios deny before inappropriate effects and preserve admission, pointers and generations. The retained run remains bound to producer e9fe1adf7b768dfd5fef234446d7db6cdfbaf37a; CI and merge remain pending.

### Recommended Outcome

- pass

## Notes

Independent review verified the signed local host identities, local_ollama provider binding and substitution denial, all 12 provider result/status/effect hashes, exact producer and binary identities, attempt-11 public hash chain, seven deny-before-effect negatives, three Python harness suites, the six-case plan validator, six Rust continuity tests, formatting, Clippy, JSON parsing and diff hygiene.
