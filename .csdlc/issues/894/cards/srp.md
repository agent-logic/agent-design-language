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
  findings_status: "findings_remediated_pending_fresh_review"
  recommended_outcome: "review_required"
notes: "Prior review evidence is retained as FAIL and is not converted into approval. Current product proof is green, current origin/main is ancestral, sibling scope has been removed, and publication remains held until a distinct fresh no-context reviewer returns PASS on the final immutable commit."
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

- Exact-head review of e8a16c55155a3a4666cae735dcc8330b8d960133 returned FAIL with three P1 findings: generated cases were generic/canned rather than concrete behavior-fixture-assertion mappings; required accepted #892 predecessor-output proof was missing; and the exact diff absorbed #893 scope through the development stack.

### Dispositions

- All findings are accepted and remediated. The generator derives concrete behavior, fixture, pre-fix failure, post-fix assertion, and detection rationale from admitted finding context, with a specific DNS parser same-parser/RDATA buffer-reuse regression plan. Focused proof consumes the retained #892 predecessor synthesis and verifies dot-directory/root-file path preservation, omission handling, invalid-plan rejection, installed CLI readback, and no source mutation. After #893 merged, four unmerged #893 corrective commits and their lifecycle residue were removed forward-only; the exact diff against current origin/main contains only #894-owned paths. A distinct fresh exact-head reviewer must now verify the immutable candidate.

### Recommended Outcome

- review_required

## Notes

Prior review evidence is retained as FAIL and is not converted into approval. Current product proof is green, current origin/main is ancestral, sibling scope has been removed, and publication remains held until a distinct fresh no-context reviewer returns PASS on the final immutable commit.
