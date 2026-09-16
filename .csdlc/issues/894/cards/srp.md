---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-test-planner-review-prompt"
issue: 894
task_id: "issue-0894"
version: "0.92.2"
title: "[v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings"
branch: "codex/894-v0922-test-planner"
generated_at: "2026-09-12T00:10:09.925246+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/894"
  - kind: "stp"
    ref: ".csdlc/issues/894/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/894/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/894/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/894/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/894/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/894/cards/stp.md"
  - ".csdlc/issues/894/cards/sip.md"
  - ".csdlc/issues/894/cards/vpp.md"
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
  - ".csdlc/issues/894/cards/stp.md"
  - ".csdlc/issues/894/cards/sip.md"
  - ".csdlc/issues/894/cards/vpp.md"
review_results:
  findings_status: "fresh_exact_head_pass_no_actionable_findings"
  recommended_outcome: "pass"
notes: "Fresh exact-head PASS at 45f25ee536a227550ecff9c4909d739123888879. Focused post-merge proof passed: codefriend_testplan 8/8 and codefriend_ux 7/7. Publication remains pending metadata commit, one final exact-head metadata review, push, and renewed standard CI."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/894/cards/stp.md
- .csdlc/issues/894/cards/sip.md
- .csdlc/issues/894/cards/vpp.md

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

- Fresh no-context exact-head review of 7b6245f93c482c0f0622c4b07850fa800c9f2845 by fresh-session:04af6f7c-8784-4703-b326-921b1a5b2c24 returned FAIL. P1: generation accepted a standalone synthesis without validating the complete accepted #892 bundle. P1: the plan reader accepted tampered or incompletely mapped plans because it did not validate the manifest and retained synthesis together. P1: the accepted DNS finding still mapped to a generic root integration test rather than the crate's runnable unit-test/vector surface. P2: the source-immutability test asserted immutability without measuring the inspected repository tree.

### Dispositions

- The earlier four findings remain accepted and remediated. Replacement no-context reviewer fresh-session:d2802ed4-e88c-48f5-9b10-9af6c64012e9 reviewed exact revision 45f25ee536a227550ecff9c4909d739123888879 after the current-main merge and returned PASS with no findings. The review verified #894 test-plan routing, #895 publication routing preservation, root and dot-prefixed repository paths, complete accepted synthesis-bundle admission, create-only output, and tamper rejection.

### Recommended Outcome

- pass

## Notes

Fresh exact-head PASS at 45f25ee536a227550ecff9c4909d739123888879. Focused post-merge proof passed: codefriend_testplan 8/8 and codefriend_ux 7/7. Publication remains pending metadata commit, one final exact-head metadata review, push, and renewed standard CI.
