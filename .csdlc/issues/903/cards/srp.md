---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-mlx-metal-provider-review-prompt"
issue: 903
task_id: "issue-0903"
version: "0.92.2"
title: "[v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter"
branch: "codex/903-v0922-mlx-metal-provider"
generated_at: "2026-09-12T00:18:14.454312+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/903"
  - kind: "stp"
    ref: ".csdlc/issues/903/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/903/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/903/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/903/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/903/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/903/cards/stp.md"
  - ".csdlc/issues/903/cards/sip.md"
  - ".csdlc/issues/903/cards/vpp.md"
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
  - ".csdlc/issues/903/cards/stp.md"
  - ".csdlc/issues/903/cards/sip.md"
  - ".csdlc/issues/903/cards/vpp.md"
review_results:
  findings_status: "changes_requested"
  recommended_outcome: "block"
notes: "Reviewer sprint8_908 bounded implementation/evidence review; source unchanged since prior accepted corrected source. Full issue acceptance is not established: required CI and successful matched real-review comparison remain incomplete. Do not treat failed model reviews or direct endpoint timing as production-workflow proof."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/903/cards/stp.md
- .csdlc/issues/903/cards/sip.md
- .csdlc/issues/903/cards/vpp.md

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

- Independent exact-head review at838f06669a7bf6a23d47256db2128a6818f410f4 found no new source defects. Seven source hashes match the earlier corrected implementation review. Hardware public/private measurement hashes match. P2: stale SRP preparation-only claims require correction in this update.

### Dispositions

- Prior explicit-model, capability and seed defects were repaired before3374315d3. Root separately reviewed and executed reviewer-authored integration fixture. SOR stale placeholders corrected and independently rechecked. SRP stale preparation claims corrected by this native edit; updated metadata needs final rereview.

### Recommended Outcome

- block

## Notes

Reviewer sprint8_908 bounded implementation/evidence review; source unchanged since prior accepted corrected source. Full issue acceptance is not established: required CI and successful matched real-review comparison remain incomplete. Do not treat failed model reviews or direct endpoint timing as production-workflow proof.
