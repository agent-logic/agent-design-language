---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "pr-ready-reconciliation-review-prompt"
issue: 824
task_id: "issue-0824"
version: "v0.92.1"
title: "[v0.92.1][tooling] Reconcile native pull-request ready mutations"
branch: "codex/824-pr-ready-reconciliation"
generated_at: "2026-09-10T00:00:00Z"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/824"
  - kind: "stp"
    ref: ".csdlc/issues/824/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/824/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/824/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/824/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/824/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/824/cards/stp.md"
  - ".csdlc/issues/824/cards/sip.md"
  - ".csdlc/issues/824/cards/vpp.md"
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
  - ".csdlc/issues/824/cards/stp.md"
  - ".csdlc/issues/824/cards/sip.md"
  - ".csdlc/issues/824/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Independent exact-head rereview by subagent:/root/fix_814_runtime returned PASS with no actionable P0-P3 findings at 84f72df2a7205f76962b722cdababfcb0642ea2c."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/824/cards/stp.md
- .csdlc/issues/824/cards/sip.md
- .csdlc/issues/824/cards/vpp.md

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

- Three historical P2 findings were identified across two review rounds: deterministic pre-dispatch failure could consume the one recovery; SOR retained placeholders and contradictory local/hosted truth; repeated recovery could leave a private GraphQL input after the consumed-recovery check failed.

### Dispositions

- All three findings are resolved. Deterministic preparation precedes recovery consumption; missing credentials preserve the retry; already-consumed recovery is checked before private input creation; every post-creation error removes the input; repeated recovery proves exact input absence; SOR is fully populated and separates local proof from hosted CI.

### Recommended Outcome

- pass

## Notes

Independent exact-head rereview by subagent:/root/fix_814_runtime returned PASS with no actionable P0-P3 findings at 84f72df2a7205f76962b722cdababfcb0642ea2c.
