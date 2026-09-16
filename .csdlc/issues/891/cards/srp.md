---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-operator-review-shell-review-prompt"
issue: 891
task_id: "issue-0891"
version: "0.92.2"
title: "[v0.92.2][CF-SHELL] Operate a real repository review through the installed CodeFriend shell"
branch: "codex/891-v0922-operator-review-shell"
generated_at: "2026-09-12T00:09:56.378553+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/891"
  - kind: "stp"
    ref: ".csdlc/issues/891/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/891/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/891/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/891/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/891/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/891/cards/stp.md"
  - ".csdlc/issues/891/cards/sip.md"
  - ".csdlc/issues/891/cards/vpp.md"
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
  - ".csdlc/issues/891/cards/stp.md"
  - ".csdlc/issues/891/cards/sip.md"
  - ".csdlc/issues/891/cards/vpp.md"
review_results:
  findings_status: "findings_remediated_pending_fresh_review"
  recommended_outcome: "review_required"
notes: "Fresh review is required against the remediated immutable head after this edit and commit. Prior PASS is not claimed. Residual external-provider/macOS/Linux observations remain separate publication/CI gates and are not replaced by deterministic local loopback tests."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/891/cards/stp.md
- .csdlc/issues/891/cards/sip.md
- .csdlc/issues/891/cards/vpp.md

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

- First independent exact-head review of d78c5ba0b9 by fresh no-context subagent /root/review_891_exact_head_r1 returned FAIL with two P2 findings: (1) retry after cancellation could not succeed because retry left stale cancel-request.json in place; (2) cancellation during the final lane could still settle as complete because the runner checked cancellation only before lane start.

### Dispositions

- Both P2 findings were accepted and remediated in the implementation session. Retry now archives the stale root cancel request to the prior attempt before starting a new attempt, preserving evidence without poisoning the retry. The runner now checks for a cancel request after lane execution as well, so a final-lane cancel records failed/cancelled truth rather than fabricated completion. Focused regressions were added for retry-after-cancel and final-lane-cancel behavior.

### Recommended Outcome

- review_required

## Notes

Fresh review is required against the remediated immutable head after this edit and commit. Prior PASS is not claimed. Residual external-provider/macOS/Linux observations remain separate publication/CI gates and are not replaced by deterministic local loopback tests.
