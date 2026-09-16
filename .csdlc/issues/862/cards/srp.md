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
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head review must trace complete production route migration, responsibility ownership, unchanged public/durable contracts and negative behavior, every acceptance and exclusion: 1. Every extracted local module has one named responsibility and no command-domain cycles. 2. All local production routes use the decomposition; moving code into a replacement god module does not qualify. 3. Public and serialized contracts, digests, error codes and paths remain compatible. 4. Local/native authority, mutation, terminal and negative-path regression proof passes at the reviewed revision. Remote command decomposition in this issue, lifecycle feature changes, schema redesign, weakened guards, or unrelated cleanup. Coordinate shared paths and installed-binary ownership with the SIM sprint before implementation. `CSDLC-REMOTE` separately completes the remote command owner, after this local extraction and CSDLC-MERGE/#849. It retains authority/credentials, GitHub issue/PR operations, publication/readback, mutation reconciliation, receipts and validation decomposition plus its own unchanged-contract proof. No remote work is discarded when #862 closes, and no child is created by this scope correction."
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

- Implementation review has not run; no implementation exists from this preparation.

### Dispositions

- No implementation finding is accepted, resolved or waived by preparation.

### Recommended Outcome

- block

## Notes

Independent exact-head review must trace complete production route migration, responsibility ownership, unchanged public/durable contracts and negative behavior, every acceptance and exclusion: 1. Every extracted local module has one named responsibility and no command-domain cycles. 2. All local production routes use the decomposition; moving code into a replacement god module does not qualify. 3. Public and serialized contracts, digests, error codes and paths remain compatible. 4. Local/native authority, mutation, terminal and negative-path regression proof passes at the reviewed revision. Remote command decomposition in this issue, lifecycle feature changes, schema redesign, weakened guards, or unrelated cleanup. Coordinate shared paths and installed-binary ownership with the SIM sprint before implementation. `CSDLC-REMOTE` separately completes the remote command owner, after this local extraction and CSDLC-MERGE/#849. It retains authority/credentials, GitHub issue/PR operations, publication/readback, mutation reconciliation, receipts and validation decomposition plus its own unchanged-contract proof. No remote work is discarded when #862 closes, and no child is created by this scope correction.
