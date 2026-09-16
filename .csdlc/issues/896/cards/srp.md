---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-codefriend-markdown-renderer-review-prompt"
issue: 896
task_id: "issue-0896"
version: "0.92.2"
title: "[v0.92.2][CF-RENDER-MD] Render an approved review as Markdown"
branch: "codex/896-v0922-codefriend-markdown-renderer"
generated_at: "2026-09-12T00:10:01.454600+00:00"
card_status: "draft"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/896"
  - kind: "stp"
    ref: ".csdlc/issues/896/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/896/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/896/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/896/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/896/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/896/cards/stp.md"
  - ".csdlc/issues/896/cards/sip.md"
  - ".csdlc/issues/896/cards/vpp.md"
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
  - ".csdlc/issues/896/cards/stp.md"
  - ".csdlc/issues/896/cards/sip.md"
  - ".csdlc/issues/896/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head implementation review required before publication. Review every acceptance item: 1. Consume actual approved publication inputs plus completed synthesis, remediation and test-plan outputs; produce a complete Markdown report and bound manifest through the installed renderer. Include source scope/revision, findings and citations, attribution/severity/uncertainty, disagreements, actionable plans, failures/omissions and approval/renderer identity. 2. Claims and finding identities match the governed semantic input exactly. Execute canonical claim-set parity and snapshot checks, including empty findings, long text, partial/not-comparable evidence and withheld publication. A template filled by hand is not proof. 3. Refuse missing/stale approval, changed renderer/target/artifact identity and missing provenance. Recheck redaction at render/export, reject leaked secrets and sanitize unsafe links/embedded content. Write only to the explicitly selected local target; no external publication is implied. 4. Open/read the actual emitted report and manifest, verify usable citations and complete content, and retain actual output hashes. HTML/PDF parity later compares against this canonical semantic report; Markdown does not claim those renderers work."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/896/cards/stp.md
- .csdlc/issues/896/cards/sip.md
- .csdlc/issues/896/cards/vpp.md

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

- No implementation findings have been accepted or waived.

### Recommended Outcome

- block

## Notes

Independent exact-head implementation review required before publication. Review every acceptance item: 1. Consume actual approved publication inputs plus completed synthesis, remediation and test-plan outputs; produce a complete Markdown report and bound manifest through the installed renderer. Include source scope/revision, findings and citations, attribution/severity/uncertainty, disagreements, actionable plans, failures/omissions and approval/renderer identity. 2. Claims and finding identities match the governed semantic input exactly. Execute canonical claim-set parity and snapshot checks, including empty findings, long text, partial/not-comparable evidence and withheld publication. A template filled by hand is not proof. 3. Refuse missing/stale approval, changed renderer/target/artifact identity and missing provenance. Recheck redaction at render/export, reject leaked secrets and sanitize unsafe links/embedded content. Write only to the explicitly selected local target; no external publication is implied. 4. Open/read the actual emitted report and manifest, verify usable citations and complete content, and retain actual output hashes. HTML/PDF parity later compares against this canonical semantic report; Markdown does not claim those renderers work.
