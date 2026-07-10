---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0-91-7-wp-11-loops-implement-loop-runtime-in-full-review-prompt"
issue: 4695
task_id: "issue-4695"
version: "v0.91.7"
title: "[v0.91.7][WP-11][loops] Implement loop runtime in full"
branch: "codex/4695-v0-91-7-wp-11-loops-implement-loop-runtime-in-full"
generated_at: "2026-07-10T02:57:02Z"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/4695"
  - kind: "stp"
    ref: ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/sip.md"
  - kind: "spp"
    ref: ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/spp.md"
  - kind: "vpp"
    ref: ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/vpp.md"
  - kind: "sor"
    ref: ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/stp.md"
  - ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/sip.md"
  - ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/vpp.md"
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
  - ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/stp.md"
  - ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/sip.md"
  - ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Pre-PR review was attempted through repo-native code-review tooling. Fixture review returned no findings but is non-proving; live Ollama review attempts returned unavailable/skipped results, including HTTP 404 before commit and generate transport failures after commit, so PR publication remains blocked until a proving review or explicit operator waiver exists."
---

Canonical Template Source: `docs/templates/prompts/1.0.3/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review was attempted after implementation and local validation; the current review gate blocks PR publication because no proving reviewer result is available.

## Scope Basis

- .adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/stp.md
- .adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/sip.md
- .adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/vpp.md

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

Machine-readable review result recorded in frontmatter:

```yaml
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
```

### Findings

- Repo-native fixture review at `.adl/reviews/issue-4695-pre-pr-review/review_result.json` reported no findings, but its disposition was `non_proving`.
- Repo-native live Ollama review at `.adl/reviews/issue-4695-pre-pr-review-live/review_result.json` reported no findings because review execution was skipped after `Ollama returned HTTP 404 Not Found`.
- Repo-native live Ollama reviews at `.adl/reviews/issue-4695-pre-pr-review-live-gemma12b-committed/review_result.json`, `.adl/reviews/issue-4695-pre-pr-review-live-phi4mini-committed/review_result.json`, and `.adl/reviews/issue-4695-pre-pr-review-live-phi4mini-focused/review_result.json` reported no findings, but all were skipped because the local Ollama generate request failed.
- No actionable code findings are currently recorded.

### Dispositions

- Fixture review result: no code changes required, but not sufficient for PR publication.
- Live review result: unavailable; no code disposition possible from the skipped Ollama attempts.
- Publication disposition: blocked until a proving pre-PR review result or explicit operator waiver is available.

### Recommended Outcome

- BLOCK PR publication under the current gate result.

## Notes

Review tooling evidence:
- `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review --backend fixture --visibility read-only-repo --issue 4695 --include-working-tree --fixture-case clean`
- `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live --backend ollama --visibility read-only-repo --issue 4695 --include-working-tree --allow-live-ollama --timeout-secs 90`
- `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-gemma12b-committed --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model gemma4:12b-mlx --timeout-secs 180`
- `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-phi4mini-committed --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model phi4-mini:latest --timeout-secs 180`
- `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-phi4mini-focused --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model phi4-mini:latest --timeout-secs 180 --file adl/src/runtime_v2/loop_runtime.rs --file adl/src/runtime_v2/tests/loop_runtime.rs --file adl/src/cli/runtime_v2_cmd/commands.rs --file adl/src/cli/runtime_v2_cmd/tests.rs --file adl/src/cli/runtime_v2_cmd/helpers.rs --file adl/src/runtime_v2/mod.rs --file adl/src/runtime_v2/tests.rs --file adl/src/cli/usage.rs`

The fixture packet is shape-valid and clean but non-proving. The live reviewer is unavailable in this environment. This SRP does not claim independent review approval.
