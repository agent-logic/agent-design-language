---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-compatible-review-comparison-review-prompt"
issue: 885
task_id: "issue-0885"
version: "0.92.2"
title: "[v0.92.2][CF-MEMORY] Stable second-run comparison and longitudinal review memory"
branch: "codex/885-v0922-compatible-review-comparison"
generated_at: "2026-09-12T00:07:14.938170+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/885"
  - kind: "stp"
    ref: ".csdlc/issues/885/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/885/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/885/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/885/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/885/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/885/cards/stp.md"
  - ".csdlc/issues/885/cards/sip.md"
  - ".csdlc/issues/885/cards/vpp.md"
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
  - ".csdlc/issues/885/cards/stp.md"
  - ".csdlc/issues/885/cards/sip.md"
  - ".csdlc/issues/885/cards/vpp.md"
review_results:
  findings_status: "interim_pass_final_review_pending"
  recommended_outcome: "block"
notes: "Independent exact-head review must trace actual caller and storage ownership, every acceptance and exclusion: 1. Execute two actual compatible admitted run artifacts through the installed comparison command. Known fixtures cover added/resolved/changed/unchanged findings, moved locations with stable identity and prose changes that do not create false identities. Emit match reasons and before/after references. 2. Deterministic identical inputs produce stable delta ordering and identities. Scope, schema, rule/lane versions and completion state determine comparability; unsupported compatibility yields not-comparable with reasons. 3. Missing baseline, deleted artifact, identity collision, altered provenance, incompatible version and partial/narrower current coverage must not silently produce resolution or success. An absent finding in an incomplete run cannot be declared resolved. 4. Demonstrate the real product comparison consumer opens its bounded prior run from the admission store, emits persisted delta artifacts and respects deletion/redaction. A matching library called only from tests is insufficient. 5. Keep backend retrieval separate from semantic matching; document the input/output contract PLAT-MEMORY will consume and avoid unbounded organizational memory claims. Retain these exact specification obligations and record evidence for each; the concrete acceptance scenarios above define how they are proved. - Acceptance: stable_matching, added_resolved_changed_classification, schema_policy, missing_baseline_explicit. - PVF obligations: two_run_fixture, version_mismatch, deleted_artifact, deterministic_delta. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unbounded organizational_memory. Stop on silent_incompatible_compare, identity_collision; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/885/cards/stp.md
- .csdlc/issues/885/cards/sip.md
- .csdlc/issues/885/cards/vpp.md

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

- Interim source review by /root/sprint3_preparation_review PASS; final committed-head and proof review pending. Adapter trust contract documented; no findings waived.

### Dispositions

- No implementation finding is accepted, resolved or waived by preparation.

### Recommended Outcome

- block

## Notes

Independent exact-head review must trace actual caller and storage ownership, every acceptance and exclusion: 1. Execute two actual compatible admitted run artifacts through the installed comparison command. Known fixtures cover added/resolved/changed/unchanged findings, moved locations with stable identity and prose changes that do not create false identities. Emit match reasons and before/after references. 2. Deterministic identical inputs produce stable delta ordering and identities. Scope, schema, rule/lane versions and completion state determine comparability; unsupported compatibility yields not-comparable with reasons. 3. Missing baseline, deleted artifact, identity collision, altered provenance, incompatible version and partial/narrower current coverage must not silently produce resolution or success. An absent finding in an incomplete run cannot be declared resolved. 4. Demonstrate the real product comparison consumer opens its bounded prior run from the admission store, emits persisted delta artifacts and respects deletion/redaction. A matching library called only from tests is insufficient. 5. Keep backend retrieval separate from semantic matching; document the input/output contract PLAT-MEMORY will consume and avoid unbounded organizational memory claims. Retain these exact specification obligations and record evidence for each; the concrete acceptance scenarios above define how they are proved. - Acceptance: stable_matching, added_resolved_changed_classification, schema_policy, missing_baseline_explicit. - PVF obligations: two_run_fixture, version_mismatch, deleted_artifact, deterministic_delta. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unbounded organizational_memory. Stop on silent_incompatible_compare, identity_collision; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task.
