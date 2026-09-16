---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-publication-approval-review-prompt"
issue: 895
task_id: "issue-0895"
version: "0.92.2"
title: "[v0.92.2][CF-UX] Enforce exact-artifact publication approval"
branch: "codex/895-v0922-publication-approval"
generated_at: "2026-09-12T00:10:15.986006+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/895"
  - kind: "stp"
    ref: ".csdlc/issues/895/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/895/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/895/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/895/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/895/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/895/cards/stp.md"
  - ".csdlc/issues/895/cards/sip.md"
  - ".csdlc/issues/895/cards/vpp.md"
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
  - ".csdlc/issues/895/cards/stp.md"
  - ".csdlc/issues/895/cards/sip.md"
  - ".csdlc/issues/895/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head review must trace actual generator/reader or publication admission and source immutability, every acceptance and exclusion: 1. Through the installed product inspect exact run, finding set, artifact manifest, rendering contract/version, claims/nonclaims and destination identity; record explicit approval, withholding or invalidation with provenance. An approval flag unbound to identities is insufficient. 2. Require the exact fresh approval at the production publication admission boundary, and exercise an approved local controlled target plus denied/withheld attempts. This issue delivers approval enforcement and manifest admission; successor renderers deliver their own complete output. Do not claim a renderer exists from a fixture. 3. Mutate scope, finding set, rendered artifact/renderer version or target after approval and prove publication is denied until a new matching decision. Reject absent/stale approval, partial run misrepresented as complete, missing provenance, tampered manifest and redaction failure. Repository instructions never authorize approval. 4. Allow the operator to inspect withheld state and recover by making a new explicit decision. No default approval, automatic remote publishing or public hosting is introduced. Preserve the shared CF-EVIDENCE schema and its conformance vectors rather than a private UI contract. acceptance: `approval_binding`, `withheld_state`, `manifest_complete`, `human_approval_required`, `withheld_state_supported`. pvf: `approval_binding`, `withheld_state`, `manifest_complete`, `changed_artifact_invalidates`, `unapproved_publish_denied`, `approval_negative_test`, `manifest_validation`. stop_conditions: `automatic_publication`, `required_proof_not_executed`, `partial_artifact_claimed_complete`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. acceptance: `approval_binding`, `withheld_state`, `manifest_complete`, `human_approval_required`, `withheld_state_supported`. pvf: `approval_binding`, `withheld_state`, `manifest_complete`, `changed_artifact_invalidates`, `unapproved_publish_denied`, `approval_negative_test`, `manifest_validation`. stop_conditions: `automatic_publication`, `required_proof_not_executed`, `partial_artifact_claimed_complete`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/895/cards/stp.md
- .csdlc/issues/895/cards/sip.md
- .csdlc/issues/895/cards/vpp.md

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

Independent exact-head review must trace actual generator/reader or publication admission and source immutability, every acceptance and exclusion: 1. Through the installed product inspect exact run, finding set, artifact manifest, rendering contract/version, claims/nonclaims and destination identity; record explicit approval, withholding or invalidation with provenance. An approval flag unbound to identities is insufficient. 2. Require the exact fresh approval at the production publication admission boundary, and exercise an approved local controlled target plus denied/withheld attempts. This issue delivers approval enforcement and manifest admission; successor renderers deliver their own complete output. Do not claim a renderer exists from a fixture. 3. Mutate scope, finding set, rendered artifact/renderer version or target after approval and prove publication is denied until a new matching decision. Reject absent/stale approval, partial run misrepresented as complete, missing provenance, tampered manifest and redaction failure. Repository instructions never authorize approval. 4. Allow the operator to inspect withheld state and recover by making a new explicit decision. No default approval, automatic remote publishing or public hosting is introduced. Preserve the shared CF-EVIDENCE schema and its conformance vectors rather than a private UI contract. acceptance: `approval_binding`, `withheld_state`, `manifest_complete`, `human_approval_required`, `withheld_state_supported`. pvf: `approval_binding`, `withheld_state`, `manifest_complete`, `changed_artifact_invalidates`, `unapproved_publish_denied`, `approval_negative_test`, `manifest_validation`. stop_conditions: `automatic_publication`, `required_proof_not_executed`, `partial_artifact_claimed_complete`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. acceptance: `approval_binding`, `withheld_state`, `manifest_complete`, `human_approval_required`, `withheld_state_supported`. pvf: `approval_binding`, `withheld_state`, `manifest_complete`, `changed_artifact_invalidates`, `unapproved_publish_denied`, `approval_negative_test`, `manifest_validation`. stop_conditions: `automatic_publication`, `required_proof_not_executed`, `partial_artifact_claimed_complete`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.
