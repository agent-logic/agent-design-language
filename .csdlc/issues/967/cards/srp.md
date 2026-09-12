---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "issue-967-deterministic-hosted-a2a-review-prompt"
issue: 967
task_id: "issue-0967"
version: "v0.92.2"
title: "[v0.92.2][RT-PROVIDER][corrective] Make hosted A2A initiation deterministic across provider output formats"
branch: "codex/967-deterministic-hosted-a2a"
generated_at: "2026-09-12"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/967"
  - kind: "stp"
    ref: ".csdlc/issues/967/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/967/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/967/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/967/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/967/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/967/cards/stp.md"
  - ".csdlc/issues/967/cards/sip.md"
  - ".csdlc/issues/967/cards/vpp.md"
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
  - ".csdlc/issues/967/cards/stp.md"
  - ".csdlc/issues/967/cards/sip.md"
  - ".csdlc/issues/967/cards/vpp.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "request_changes"
notes: "Preserve failed hosted-live-03 report SHA256 651f9a00fb5b96e2a6b0539d1f9321c4546631a16b78e9c723fe823c0d95d439: OpenAI five successful calls, Anthropic two successful calls, Vertex zero. Raw provider output was not retained; exact response shape is unknown. Inherited corrective working-tree proof: Runtime production dispatch 1/1, OpenAPI contracts 11/11, zero-paid five-provider matrix 5/5 and hosted-topology matrix 3/3. These are local working-tree results, not #967 committed-head, CI or paid hosted acceptance. Commit corrective source under #967, rebuild exact-head binaries, retain and review exact proof, pass current CI, and obtain fresh authorization before one bounded hosted acceptance run. No hosted retry or merge is claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/967/cards/stp.md
- .csdlc/issues/967/cards/sip.md
- .csdlc/issues/967/cards/vpp.md

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

- Hosted live03 demonstrated that successful provider text transport did not establish an A2A exchange. A working-tree review also found an OpenAPI/Runtime name and size mismatch; that mismatch was corrected. Exact #967 committed-head review remains pending.

### Dispositions

- Typed requested_agent_action repair and OpenAPI correction have working-tree review/proof; no final #967 review, hosted acceptance or CI disposition is asserted.

### Recommended Outcome

- request_changes

## Notes

Preserve failed hosted-live-03 report SHA256 651f9a00fb5b96e2a6b0539d1f9321c4546631a16b78e9c723fe823c0d95d439: OpenAI five successful calls, Anthropic two successful calls, Vertex zero. Raw provider output was not retained; exact response shape is unknown. Inherited corrective working-tree proof: Runtime production dispatch 1/1, OpenAPI contracts 11/11, zero-paid five-provider matrix 5/5 and hosted-topology matrix 3/3. These are local working-tree results, not #967 committed-head, CI or paid hosted acceptance. Commit corrective source under #967, rebuild exact-head binaries, retain and review exact proof, pass current CI, and obtain fresh authorization before one bounded hosted acceptance run. No hosted retry or merge is claimed.
