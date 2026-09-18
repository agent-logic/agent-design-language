---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-installed-local-review-agent-review-prompt"
issue: 1058
task_id: "issue-1058"
version: "v0.92.2"
title: "[v0.92.2][CF-AGENT] Run website-controlled reviews through an installed local agent"
branch: "codex/1058-v0922-installed-local-review-agent"
generated_at: "2026-09-16T20:16:52.878587+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1058"
  - kind: "stp"
    ref: ".csdlc/issues/1058/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1058/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/1058/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/1058/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/1058/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/1058/cards/stp.md"
  - ".csdlc/issues/1058/cards/sip.md"
  - ".csdlc/issues/1058/cards/vpp.md"
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
  - ".csdlc/issues/1058/cards/stp.md"
  - ".csdlc/issues/1058/cards/sip.md"
  - ".csdlc/issues/1058/cards/vpp.md"
review_results:
  findings_status: "resolved"
  recommended_outcome: "draft_publication_only"
notes: "Exact-head source review passed33ad3d7abd146e4845344fed3b0473a03441bb87. Combined58tests, separate1journal-lock regression,640 repeated agent tests and focused clippy passed. Fresh native57integration proof passed. Isolated macOS binaries installed and website native-verifier compatibility probe passed. Real OAuth/provider/platform journeys and current CI remain separate acceptance gates. Parent1063 native proof/review/publication passed b34a32da6d. Final generated-record head requires review refresh."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/1058/cards/stp.md
- .csdlc/issues/1058/cards/sip.md
- .csdlc/issues/1058/cards/vpp.md

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

- Four user P2s at670b7b74bb: acknowledged operations lost on disconnect; candidate/model attribution discarded; aggregate report can exceed transport cap; unpair strands retained payloads. Followup review: directory durability, transient controls, cached output timeout, partial-evidence bootstrap dispatch.

### Dispositions

- All four user P2 findings and subsequent review findings repaired. Independent exact-head source review passed33ad3d7abd146e4845344fed3b0473a03441bb87; journal lock lifetime repair reviewed and validated. No live acceptance or merge-ready claim.

### Recommended Outcome

- draft_publication_only

## Notes

Exact-head source review passed33ad3d7abd146e4845344fed3b0473a03441bb87. Combined58tests, separate1journal-lock regression,640 repeated agent tests and focused clippy passed. Fresh native57integration proof passed. Isolated macOS binaries installed and website native-verifier compatibility probe passed. Real OAuth/provider/platform journeys and current CI remain separate acceptance gates. Parent1063 native proof/review/publication passed b34a32da6d. Final generated-record head requires review refresh.
