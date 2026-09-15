---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-production-memory-palace-retrieval-review-prompt"
issue: 889
task_id: "issue-0889"
version: "0.92.2"
title: "[v0.92.2][PLAT-MEMORY] Retrieve a compatible prior CodeFriend review through Memory Palace"
branch: "codex/889-v0922-production-memory-palace-retrieval"
generated_at: "2026-09-12T00:07:16.174214+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/889"
  - kind: "stp"
    ref: ".csdlc/issues/889/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/889/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/889/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/889/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/889/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/889/cards/stp.md"
  - ".csdlc/issues/889/cards/sip.md"
  - ".csdlc/issues/889/cards/vpp.md"
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
  - ".csdlc/issues/889/cards/stp.md"
  - ".csdlc/issues/889/cards/sip.md"
  - ".csdlc/issues/889/cards/vpp.md"
review_results:
  findings_status: "resolved_no_open_findings"
  recommended_outcome: "approve"
notes: "Independent exact-head review must trace actual caller and storage ownership, every acceptance and exclusion: 1. Execute an installed first-run/second-run scenario: admit a redacted run, store/index its bounded compatible references through the Runtime Memory Palace service, retrieve the baseline through the production boundary, then generate CF-MEMORY's deterministic delta. Instrument retained provenance sufficiently to prove the real caller/backend were used. 2. Missing/corrupt latest pointer, altered kernel packet/citation hashes, wrong continuity/run identity, incompatible schema/scope and stale/deleted baseline produce explicit denial/not-comparable states. No fallback to an unvalidated raw JSON file or a test-only in-memory backend can satisfy integration. 3. Enforce redaction before durable retention and retrieval/model use. Verify retention/deletion prevents later access through the comparison path, including stale cached/latest references. Sanitized identifiers and digests appear in logs; private data and credentials do not. 4. Bounded working-set selection and deterministic ordering use explicit observation time. Repeat compatible runs predictably; do not claim unlimited organizational memory or reconstruct deleted data from stale context. 5. Exercise actual `RuntimeMemoryPalaceService` storage and the installed CodeFriend consumer on local isolated fixtures; a packet constructor test or mocked adapter call is non-proving. Shared Runtime/kernel changes are limited to contracts needed by this consumer and retain their existing regression suites. Retain these exact specification obligations and record evidence for each; the concrete acceptance scenarios above define how they are proved. - Acceptance: production_caller, deterministic_retrieval, redaction_fail_closed, compatibility_explicit, production_second_review_retrieval, deterministic_compatible_baseline, redaction_and_deletion_enforced. - PVF obligations: production_path_test, retrieval_fixture, redaction_negative_suite, production_second_review_retrieval, deterministic_compatible_baseline, redaction_and_deletion_enforced, test_only_integration_rejected, unspecified_slice_rejected, incompatible_baseline_rejected. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: complete_speculative_memory_architecture. Stop on test_only_integration, private_data_leak, unbounded_memory_claim, required_proof_not_executed, partial_artifact_claimed_complete, comparison_uses_test_only_memory; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/889/cards/stp.md
- .csdlc/issues/889/cards/sip.md
- .csdlc/issues/889/cards/vpp.md

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

- Independent /root/repair_882_ci/review_989_integration PASS at b8d93cbe5993114aa3b8c5cb69e2f7dee322b618. No open findings. Reviewer reran7 consumer tests and verified installed binary plus durable generation SHA256 hashes. Privacy error echo and SHA256 contract findings fixed and independently verified.

### Dispositions

- No findings waived. Final record-only exact-head confirmation required before native review/publication.

### Recommended Outcome

- approve

## Notes

Independent exact-head review must trace actual caller and storage ownership, every acceptance and exclusion: 1. Execute an installed first-run/second-run scenario: admit a redacted run, store/index its bounded compatible references through the Runtime Memory Palace service, retrieve the baseline through the production boundary, then generate CF-MEMORY's deterministic delta. Instrument retained provenance sufficiently to prove the real caller/backend were used. 2. Missing/corrupt latest pointer, altered kernel packet/citation hashes, wrong continuity/run identity, incompatible schema/scope and stale/deleted baseline produce explicit denial/not-comparable states. No fallback to an unvalidated raw JSON file or a test-only in-memory backend can satisfy integration. 3. Enforce redaction before durable retention and retrieval/model use. Verify retention/deletion prevents later access through the comparison path, including stale cached/latest references. Sanitized identifiers and digests appear in logs; private data and credentials do not. 4. Bounded working-set selection and deterministic ordering use explicit observation time. Repeat compatible runs predictably; do not claim unlimited organizational memory or reconstruct deleted data from stale context. 5. Exercise actual `RuntimeMemoryPalaceService` storage and the installed CodeFriend consumer on local isolated fixtures; a packet constructor test or mocked adapter call is non-proving. Shared Runtime/kernel changes are limited to contracts needed by this consumer and retain their existing regression suites. Retain these exact specification obligations and record evidence for each; the concrete acceptance scenarios above define how they are proved. - Acceptance: production_caller, deterministic_retrieval, redaction_fail_closed, compatibility_explicit, production_second_review_retrieval, deterministic_compatible_baseline, redaction_and_deletion_enforced. - PVF obligations: production_path_test, retrieval_fixture, redaction_negative_suite, production_second_review_retrieval, deterministic_compatible_baseline, redaction_and_deletion_enforced, test_only_integration_rejected, unspecified_slice_rejected, incompatible_baseline_rejected. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: complete_speculative_memory_architecture. Stop on test_only_integration, private_data_leak, unbounded_memory_claim, required_proof_not_executed, partial_artifact_claimed_complete, comparison_uses_test_only_memory; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task.
