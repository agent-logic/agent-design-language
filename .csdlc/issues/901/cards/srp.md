---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-provider-recovery-qualification-review-prompt"
issue: 901
task_id: "issue-0901"
version: "0.92.2"
title: "[v0.92.2][QUAL-PROVIDER] Execute real provider failure and recovery qualification"
branch: "codex/901-v0922-provider-recovery-qualification"
generated_at: "2026-09-12T00:15:14.473149+00:00"
card_status: "reviewed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/901"
  - kind: "stp"
    ref: ".csdlc/issues/901/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/901/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/901/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/901/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/901/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/901/cards/stp.md"
  - ".csdlc/issues/901/cards/sip.md"
  - ".csdlc/issues/901/cards/vpp.md"
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
  - ".csdlc/issues/901/cards/stp.md"
  - ".csdlc/issues/901/cards/sip.md"
  - ".csdlc/issues/901/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Independent review verified direct transport-loss handling, four distinct task-owned provider PIDs and requests, real timeout and SIGTERM interruption, fresh-PID recovery, raw result equality, separately hashed execution-observation binding, coordinated-tamper rejection, 19 validator tests, 51 Rust provider-adapter tests, redaction/path hygiene, #851 exclusion, and coherent SPP/VPP/SRP/SOR truth."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/901/cards/stp.md
- .csdlc/issues/901/cards/sip.md
- .csdlc/issues/901/cards/vpp.md

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

- Final exact-head review at 06ad3771f14a12c399f03a404fb587d9de33b153 reported no actionable findings. Earlier passes found five actionable evidence/lifecycle issues: synthetic proxy 502, summary-to-raw evidence binding, stale SOR truth, stale VPP run state, and stale operative SPP state. All were fixed before the final review.

### Dispositions

- Pass for native draft publication. Live-run-09 is bound to exact source c6651bb59fc117f10abc9e613d0042f567759ee8 and retained privately; the portable report is tracked. Hosted CI, merge, and terminal closeout remain pending.

### Recommended Outcome

- pass

## Notes

Independent review verified direct transport-loss handling, four distinct task-owned provider PIDs and requests, real timeout and SIGTERM interruption, fresh-PID recovery, raw result equality, separately hashed execution-observation binding, coordinated-tamper rejection, 19 validator tests, 51 Rust provider-adapter tests, redaction/path hygiene, #851 exclusion, and coherent SPP/VPP/SRP/SOR truth.
