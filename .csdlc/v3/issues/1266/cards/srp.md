---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "<slug>-review-prompt"
issue: 1266
task_id: "issue-1266"
version: "1.0.5"
title: "[v0.93.1][C-SDLC v3][defect] Reconcile recordless merged closeout in external repositories"
branch: "codex/1266-recordless-merged-closeout-recovery"
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1266"
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
review_scope: "Review the complete issue #1266 branch diff, authenticated consumer-package observations, terminal-replay implementation, retained receipt identity checks, complete v0.93.1 denominator, and fail-closed cases against the live issue acceptance criteria."
review_instructions: "Findings first. Distinguish source correctness from live operational acceptance. Reject any package, activation, terminal, or publication claim that is not authenticated by the retained native evidence."
review_evidence_requirements: "Require exact-head source inspection, focused installed terminal tests, strict lint and diff hygiene, the five issue-local evidence ledgers, live consumer authority readbacks, native validation, and exact receipt byte-stability checks."
reviewer_constraints: "The reviewer must be independent of the implementer, must not modify files or GitHub state, and must not approve incomplete live acceptance merely because the source tests pass."
refusal_policy: "Refuse on wrong repository, wrong issue or PR identity, stale review head, missing package or activation proof, receipt drift, incomplete denominator coverage, ambiguous remote effect, or lifecycle projection drift."
follow_up_routing: "Return actionable source defects to this branch. Route missing cross-repository package endorsements or unsupported host upgrade authority to explicit typed successor work without weakening issue #1266."
non_claims:
  - "This prompt does not claim review has already run."
  - "This prompt does not guarantee review quality by itself."
policy_refs:
  - "<stp_card>"
  - "<sip_card>"
  - "<vpp_card>"
review_results: {"status":"not_run"}
notes: "<review_notes>"
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

<reviewer>

### Findings

- <findings>

### Outcome

<outcome>

## Notes

<review_notes>
