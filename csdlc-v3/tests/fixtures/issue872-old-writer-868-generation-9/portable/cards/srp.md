---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-installed-command-contract-review-prompt"
issue: 868
task_id: "issue-0868"
version: "0.92.2"
title: "[v0.92.2][SIM-02] One current installed command contract"
branch: "codex/868-v0922-installed-command-contract"
generated_at: "2026-09-12T05:26:43.787099+00:00"
card_status: "completed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/868"
  - kind: "stp"
    ref: ".csdlc/issues/868/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/868/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/868/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/868/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/868/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/868/cards/stp.md"
  - ".csdlc/issues/868/cards/sip.md"
  - ".csdlc/issues/868/cards/vpp.md"
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
  - ".csdlc/issues/868/cards/stp.md"
  - ".csdlc/issues/868/cards/sip.md"
  - ".csdlc/issues/868/cards/vpp.md"
review_results:
  findings_status: "resolved"
  recommended_outcome: "approve"
notes: "Reviewer review_868_exact_source independent of contributors verified full diff against5fa19bda,26committedsourcehashes,candidate,27descriptors,25journeyattempts,29schemaoutputs,270tests and strictClippy/fmt/diff. Local proof only; CI/merge/terminal pending. Merge effectunknown retained truthfully."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/868/cards/stp.md
- .csdlc/issues/868/cards/sip.md
- .csdlc/issues/868/cards/vpp.md

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

- Independent exact-head review at 6425ba9bbce4cc1f46789c2e3ac019adf86238b8 approves source and local installed proof with no remaining actionable findings. Four prior P2 findings resolved; evidence .csdlc/evidence/868/execution/independent-review-6425.json.

### Dispositions

- Release findings normalized; sprint statuses/nested findings corrected; remote replay effects sourced from ephemeral owner evidence and actual inventories; five manual EFFECTS paragraphs corrected. No waivers.

### Recommended Outcome

- approve

## Notes

Reviewer review_868_exact_source independent of contributors verified full diff against5fa19bda,26committedsourcehashes,candidate,27descriptors,25journeyattempts,29schemaoutputs,270tests and strictClippy/fmt/diff. Local proof only; CI/merge/terminal pending. Merge effectunknown retained truthfully.
