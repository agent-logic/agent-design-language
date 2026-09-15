---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-runtime-criterion-evidence-review-prompt"
issue: 902
task_id: "issue-0902"
version: "0.92.2"
title: "[v0.92.2][QUAL-EVIDENCE] Validate criterion-bound Runtime qualification evidence"
branch: "codex/902-v0922-runtime-criterion-evidence"
generated_at: "2026-09-12T00:14:10.636008+00:00"
card_status: "draft"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/902"
  - kind: "stp"
    ref: ".csdlc/issues/902/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/902/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/902/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/902/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/902/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/902/cards/stp.md"
  - ".csdlc/issues/902/cards/sip.md"
  - ".csdlc/issues/902/cards/vpp.md"
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
  - ".csdlc/issues/902/cards/stp.md"
  - ".csdlc/issues/902/cards/sip.md"
  - ".csdlc/issues/902/cards/vpp.md"
review_results:
  findings_status: "review_pending"
  recommended_outcome: "block"
notes: "Independent exact-head implementation review required before publication. Review every acceptance item: 1. Admit exactly `RUST-01-ac-4`, `DRT-B-ac-2`, `DRT-B-ac-3`, `DRT-C-ac-2` and `DRT-C-ac-3`. Bind each exact criterion text/digest to its canonical source revision, producer revision, execution profile/log/artifact digests, required scenario set and independently reviewed result. Preserve explicit many-to-one producer mappings without permitting cross-criterion evidence substitution. 2. Execute the consumer against actual completed QUAL-INVENTORY two-revision measurements, QUAL-RESIDENT population/workload/signed-restore proofs, QUAL-PROVIDER failure/recovery proofs and #852 dispatch/WSS/correlation/redaction proofs. Missing producer execution leaves the relevant row not-proven and blocks aggregate completion; authoring a mapping is insufficient. 3. Preserve original 19 findings separately from the five criterion rows. Preserve the five cloud-control gaps and two execution-proof gaps as distinct historical categories. Reconcile references for historical consumers #522 and #833 without reopening/closing them or asserting all their original findings newly proved by five rows. 4. Negative fixtures individually remove/alter criterion text/digest, source/producer revision, scenario, execution log, signature/provenance and independent review; substitute another criterion's evidence, self-authored approval, stale/partial evidence or synthetic metadata. Every invalid case must fail admission with actionable reasons, not merely fail JSON parsing. 5. Verify producer evidence and actual outcomes rather than trusting a success boolean. Independent review binds exact candidate and artifacts; absence or unresolved findings cannot become accepted. Report per-row pass/fail/not-proven plus complete/excluded/missing denominators and residual risks. 6. Expose this validator as the real consumer used by the current qualification evidence workflow, and retain a positive run on complete real inputs plus all negatives. Public sanitized manifests may reference protected artifacts under authorized verification, but inaccessible proof cannot be represented as independently verified. No release approval is conferred. Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly. Implementation is now locally complete; review the exact committed candidate, validator semantics, 14 negative cases, sanitized packet and protected-evidence allowlist before publication."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/902/cards/stp.md
- .csdlc/issues/902/cards/sip.md
- .csdlc/issues/902/cards/vpp.md

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

- Independent implementation review pending.

### Dispositions

- No implementation findings recorded yet.

### Recommended Outcome

- block

## Notes

Independent exact-head implementation review required before publication. Review every acceptance item: 1. Admit exactly `RUST-01-ac-4`, `DRT-B-ac-2`, `DRT-B-ac-3`, `DRT-C-ac-2` and `DRT-C-ac-3`. Bind each exact criterion text/digest to its canonical source revision, producer revision, execution profile/log/artifact digests, required scenario set and independently reviewed result. Preserve explicit many-to-one producer mappings without permitting cross-criterion evidence substitution. 2. Execute the consumer against actual completed QUAL-INVENTORY two-revision measurements, QUAL-RESIDENT population/workload/signed-restore proofs, QUAL-PROVIDER failure/recovery proofs and #852 dispatch/WSS/correlation/redaction proofs. Missing producer execution leaves the relevant row not-proven and blocks aggregate completion; authoring a mapping is insufficient. 3. Preserve original 19 findings separately from the five criterion rows. Preserve the five cloud-control gaps and two execution-proof gaps as distinct historical categories. Reconcile references for historical consumers #522 and #833 without reopening/closing them or asserting all their original findings newly proved by five rows. 4. Negative fixtures individually remove/alter criterion text/digest, source/producer revision, scenario, execution log, signature/provenance and independent review; substitute another criterion's evidence, self-authored approval, stale/partial evidence or synthetic metadata. Every invalid case must fail admission with actionable reasons, not merely fail JSON parsing. 5. Verify producer evidence and actual outcomes rather than trusting a success boolean. Independent review binds exact candidate and artifacts; absence or unresolved findings cannot become accepted. Report per-row pass/fail/not-proven plus complete/excluded/missing denominators and residual risks. 6. Expose this validator as the real consumer used by the current qualification evidence workflow, and retain a positive run on complete real inputs plus all negatives. Public sanitized manifests may reference protected artifacts under authorized verification, but inaccessible proof cannot be represented as independently verified. No release approval is conferred. Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly. Implementation is now locally complete; review the exact committed candidate, validator semantics, 14 negative cases, sanitized packet and protected-evidence allowlist before publication.
