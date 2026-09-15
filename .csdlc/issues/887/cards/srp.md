---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-codefriend-local-fitness-review-prompt"
issue: 887
task_id: "issue-0887"
version: "0.92.2"
title: "[v0.92.2][CF-GOV] Execute local architecture fitness functions"
branch: "not bound yet; proposed codex/887-v0922-codefriend-local-fitness"
generated_at: "2026-09-12T00:05:16.416878+00:00"
card_status: "draft"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/887"
  - kind: "stp"
    ref: ".csdlc/issues/887/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/887/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/887/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/887/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/887/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/887/cards/stp.md"
  - ".csdlc/issues/887/cards/sip.md"
  - ".csdlc/issues/887/cards/vpp.md"
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
  - ".csdlc/issues/887/cards/stp.md"
  - ".csdlc/issues/887/cards/sip.md"
  - ".csdlc/issues/887/cards/vpp.md"
review_results:
  findings_status: "interim_findings_resolved_final_review_pending"
  recommended_outcome: "await_committed_exact_head_review"
notes: "Independent exact-head implementation review required before publication. Review every acceptance item: 1. Invoke `local_fitness_runner` through the installed command on a policy and known passing/violating repository fixtures. Verify actual predicate execution, deterministic result, policy/evidence identity, violating locations and distinct pass/fail/error states. 2. Missing/malformed policy, unsupported rule, incomplete required evidence and runner fault do not become pass. Human judgments such as architecture quality remain explicitly unassessed; no opaque model score becomes a release predicate. 3. Demonstrate policy configuration is visible in the declared manifest/runner input, never hidden in test code, shard logic or environment-specific conditionals. Identical inputs repeat with stable output. 4. Persist a result artifact and define the exit/artifact contract consumed by CF-GOV-CI. Exercise this contract locally without claiming actual CI integration. 5. Scope/evidence constraints and redaction remain intact. Do not execute arbitrary repository-provided scripts or permit policy files to grant mutation authority. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, general_ci_orchestration. Stop on policy_hidden_in_tests, nondeterministic_gate, required_proof_not_executed, partial_artifact_claimed_complete; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/887/cards/stp.md
- .csdlc/issues/887/cards/sip.md
- .csdlc/issues/887/cards/vpp.md

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

- Interim review found raw identifier spelling could bypass prefix match; fixed with IdentExt::unraw and three regression cases. Subsequent interim library/CLI review found no remaining actionable finding.

### Dispositions

- Raw identifier finding fixed and tested. Final committed exact-head review pending.

### Recommended Outcome

- await_committed_exact_head_review

## Notes

Independent exact-head implementation review required before publication. Review every acceptance item: 1. Invoke `local_fitness_runner` through the installed command on a policy and known passing/violating repository fixtures. Verify actual predicate execution, deterministic result, policy/evidence identity, violating locations and distinct pass/fail/error states. 2. Missing/malformed policy, unsupported rule, incomplete required evidence and runner fault do not become pass. Human judgments such as architecture quality remain explicitly unassessed; no opaque model score becomes a release predicate. 3. Demonstrate policy configuration is visible in the declared manifest/runner input, never hidden in test code, shard logic or environment-specific conditionals. Identical inputs repeat with stable output. 4. Persist a result artifact and define the exit/artifact contract consumed by CF-GOV-CI. Exercise this contract locally without claiming actual CI integration. 5. Scope/evidence constraints and redaction remain intact. Do not execute arbitrary repository-provided scripts or permit policy files to grant mutation authority. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, general_ci_orchestration. Stop on policy_hidden_in_tests, nondeterministic_gate, required_proof_not_executed, partial_artifact_claimed_complete; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task.
