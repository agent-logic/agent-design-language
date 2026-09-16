---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-process-parser-simplification-review-prompt"
issue: 906
task_id: "issue-0906"
version: "0.92.2"
title: "[v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor"
branch: "codex/906-v0922-process-parser-simplification"
generated_at: "2026-09-12T00:22:05.416044+00:00"
card_status: "completed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/906"
  - kind: "stp"
    ref: ".csdlc/issues/906/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/906/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/906/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/906/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/906/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/906/cards/stp.md"
  - ".csdlc/issues/906/cards/sip.md"
  - ".csdlc/issues/906/cards/vpp.md"
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
  - ".csdlc/issues/906/cards/stp.md"
  - ".csdlc/issues/906/cards/sip.md"
  - ".csdlc/issues/906/cards/vpp.md"
review_results:
  findings_status: "resolved_no_open_findings"
  recommended_outcome: "pass"
notes: "Reviewed implementation and evidence at 9a1578aca9bf52d269369b3745ea2cac4b64b8a8 by /root/review_906. Local proof: 12 focused parser binary-path executions, 11 standalone adl-process tests, 16 installed CLI tests, strict clippy, formatting, diff hygiene, native six-card validation, inventory reproduction, and merge-tree conflict check. Hosted required CI remains a publication gate."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/906/cards/stp.md
- .csdlc/issues/906/cards/sip.md
- .csdlc/issues/906/cards/vpp.md

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

- Independent review by /root/review_906 found two P2 documentation-truth defects and no code-behavior defect. The first required exact source hashes plus before/after production-line, owned-function, and lexical control-site accounting. The second required removal of stale pre-implementation SOR claims. Both were fixed. Final exact-head re-review at 9a1578aca9bf52d269369b3745ea2cac4b64b8a8 reported no actionable findings.

### Dispositions

- All actionable findings were fixed without waiver. The reviewer reproduced every hash and count, independently passed native generation-7 six-card and digest validation, verified a conflict-free merge against current origin/main, and confirmed the historical dirty worktree remains unchanged at 1ea9140 with patch SHA-256 aef64c67a22d74ff5dc4962d5c6f25ab09a06ca64bb1763c48984d73558fbf84.

### Recommended Outcome

- pass

## Notes

Reviewed implementation and evidence at 9a1578aca9bf52d269369b3745ea2cac4b64b8a8 by /root/review_906. Local proof: 12 focused parser binary-path executions, 11 standalone adl-process tests, 16 installed CLI tests, strict clippy, formatting, diff hygiene, native six-card validation, inventory reproduction, and merge-tree conflict check. Hosted required CI remains a publication gate.
