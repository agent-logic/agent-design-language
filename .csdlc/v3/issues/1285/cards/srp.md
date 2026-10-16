---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "<slug>-review-prompt"
issue: 1285
task_id: "issue-1285"
version: "1.0.5"
title: "[Podcast] Move The Cognitive Stack into its dedicated repository"
branch: "codex/1285-podcast-dedicated-repository"
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1285"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
  - kind: "vpp"
    ref: "<vpp_card>"
  - kind: "sor"
    ref: "<sor_card>"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - "<stp_card>"
  - "<sip_card>"
  - "<vpp_card>"
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
review_scope: "ADL #1285 migration against source02cf787; destination13f217d;182 removals and521 imported source/provenance records."
review_instructions: "Check mapping completeness, exact preserved bytes/modes, active consumers, narrow guard transition and negative tests. Inspect the final fixes for unrelated issue test rejection and generated registry drift."
review_evidence_requirements: "Fresh remote clone hash/mode membership proof; source/destination exact revisions; focused offline tests; explicit full1223 guard baseline failure."
reviewer_constraints: "Independent read-only reviewer; no provider/deployment calls, source changes or broad builds. Historical evidence remains immutable."
refusal_policy: "Block on unpreserved assets, credentials, broken active consumers or weakened unrelated protections; do not claim old guard green."
follow_up_routing: "Fix bounded migration findings in1285. Planning12 owns pre-existing1223 document drift under1284. Legacy runtime portability and four-speaker audio remain follow-ups."
non_claims:
  - "This prompt does not claim review has already run."
  - "This prompt does not guarantee review quality by itself."
policy_refs:
  - "<stp_card>"
  - "<sip_card>"
  - "<vpp_card>"
review_results: {"status":"pass"}
notes: "Independent review verified all 521 imported hashes and tracked membership, feed preservation and final metadata. No remaining actionable migration findings. The full #1223 guard has unchanged baseline drift and is not claimed green."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- <stp_card>
- <sip_card>
- <vpp_card>

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

When finalizing review, submit exactly these typed result fields. `outcome` must match `review_results.status`:

```yaml
review_results:
  status: "pass | block | needs_followup"
reviewer: "independent reviewer identity"
findings:
  - "substantive finding; use an empty list when there are no findings"
outcome: "pass | block | needs_followup"
```

### Reviewer

worker-27/podcast_migration_review

### Findings

- ["P1 unrelated future issue records rejected by historical test: fixed by scoped copied podcast evidence.","P2 generated tool registry drift: regenerated and transition pin updated."]

### Outcome

pass

## Notes

Independent review verified all 521 imported hashes and tracked membership, feed preservation and final metadata. No remaining actionable migration findings. The full #1223 guard has unchanged baseline drift and is not claimed green.
