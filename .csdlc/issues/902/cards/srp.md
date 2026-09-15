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
card_status: "ready"
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
  findings_status: "no_actionable_findings_all_six_resolved"
  recommended_outcome: "approve"
notes: "Independent reviewer /root/review_902_exact_head verified the original three bypasses reject, 13 focused methods and 28 named negatives pass, the real retained input passes exactly 5/5, canonical run-log hashes match, and all six review findings are resolved. The reviewer then inspected native generation 13 digest adb4ee269f34e49c47d5e3e0032566e043ec7b90dab6a97732e10fe28bda3a55 over exact tracked head 2bc26ac0a6ed5a13a358309b8f5a12e8bd798ca2 and reported no actionable findings."
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

- Earlier exact-head review findings were fixed before PR #983. The subsequent PR review at 185a36f reported three P2 admission gaps: timeout evidence was not consumed, underlying #899/#852 execution evidence could be absent, and execution profiles were not bound. Remediation review then found three adjacent P2 truth gaps: retained review-evidence path confinement, canonical run-log hash retention, and stale SRP/SOR counts. The implementation and proof defects are fixed at 2bc26ac0a6ed5a13a358309b8f5a12e8bd798ca2; the native generation-13 card update resolved the remaining lifecycle-truth finding; final rereview found no actionable findings.

### Dispositions

- All PR-review findings were actionable and fixed. DRT-C-ac-2 now consumes digest-bound timeout request/result and 150 ms deadline observations; #899 and #852 rows require their underlying execution archives; all five row profiles are exact and cross-linked to producer evidence. Review evidence paths are confined to protected_root, retained run logs match validation.json, and the SOR records 13 methods and 28 named negatives. No finding was waived or deferred.

### Recommended Outcome

- approve

## Notes

Independent reviewer /root/review_902_exact_head verified the original three bypasses reject, 13 focused methods and 28 named negatives pass, the real retained input passes exactly 5/5, canonical run-log hashes match, and all six review findings are resolved. The reviewer then inspected native generation 13 digest adb4ee269f34e49c47d5e3e0032566e043ec7b90dab6a97732e10fe28bda3a55 over exact tracked head 2bc26ac0a6ed5a13a358309b8f5a12e8bd798ca2 and reported no actionable findings.
