---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-derived-card-projections-review-prompt"
issue: 871
task_id: "issue-0871"
version: "0.92.2"
title: "[v0.92.2][SIM-05] Derived cards and precise evidence invalidation"
branch: "codex/871-v0922-derived-card-projections"
generated_at: "2026-09-11T23:59:41.750973+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/871"
  - kind: "stp"
    ref: ".csdlc/issues/871/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/871/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/871/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/871/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/871/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/871/cards/stp.md"
  - ".csdlc/issues/871/cards/sip.md"
  - ".csdlc/issues/871/cards/vpp.md"
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
  - ".csdlc/issues/871/cards/stp.md"
  - ".csdlc/issues/871/cards/sip.md"
  - ".csdlc/issues/871/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head implementation review required before publication. Review every acceptance item: 1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged. 2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner. 3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table. 4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation. 5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions. 6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout. 7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task. Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/871/cards/stp.md
- .csdlc/issues/871/cards/sip.md
- .csdlc/issues/871/cards/vpp.md

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

Independent exact-head implementation review required before publication. Review every acceptance item: 1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged. 2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner. 3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table. 4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation. 5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions. 6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout. 7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task. Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot.
