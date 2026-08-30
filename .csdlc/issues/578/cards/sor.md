# Structured Output Record

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a general direct Z.ai GLM-5.3-Flash provider profile, preserved existing Z.ai GLM-5 endpoints, and wired the runtime adapter so reviewer trials can pass bounded GLM-5.3-Flash parameters into the hosted request path.

## Artifacts

- adl/src/provider/profiles.rs
- adl/src/provider/http_family.rs
- adl/src/provider/http_family/config.rs
- adl/src/provider_adapter.rs
- adl/src/provider_communication.rs
- adl/src/agent_comms.rs
- adl/src/agent_comms/dispatch/coding.inc
- adl/tests/provider_tests/profiles.rs
- adl/tests/provider_tests/http_family.rs
- docs/provider/inference-profiles.md
- docs/tooling/PROVIDER_SETUP.md
- docs/milestones/v0.92.1/evidence/provider/glm-5-3-flash/README.md
- .csdlc/prepared/issues/578/reviewer-selection-smoke.sh
- .csdlc/prepared/issues/578/glm-5-3-flash-reviewer-viability-smoke.sh
- .csdlc/prepared/issues/578/tooling-issue-bind-prepared-helper-materialization.md

## Execution

- Added `z_ai:glm-5.3-flash` as a first-class provider profile resolving to `hosted:adl-z-ai:glm-5.3-flash` and provider model id `glm-5.3-flash`.
- Scoped the new `https://api.z.ai/api/paas/v4/chat/completions` endpoint to GLM-5.3-Flash while preserving the established `https://open.bigmodel.cn/api/paas/v4/chat/completions` endpoint for existing `z_ai:glm-5` and `z_ai:glm-5-current` profiles.
- Materialized GLM-5.3-Flash defaults for `reasoning_effort=max`, `clear_thinking=false`, `temperature=1.0`, `top_p=0.95`, and bounded output tokens.
- Added runtime adapter request fields for reasoning effort, thinking cleanup, temperature, and top-p, with fail-closed validation for invalid GLM-5.3-Flash overrides before network dispatch.
- Added a credential-gated reviewer viability smoke script that uses `reasoning_effort=low`, `clear_thinking=true`, low token budget, and a 45-second timeout for fast reviewer trials.
- Documented the direct profile, source rationale, provider-variant boundaries, preserved legacy endpoint behavior, and the bind/prepared-helper materialization tooling issue.

## Validation

[
  {
    "command": [
      "cargo fmt --manifest-path adl/Cargo.toml --check"
    ],
    "purpose": "Verify Rust formatting for the touched provider and adapter code.",
    "outcome": "passed",
    "evidence_ref": "local command output, #578 worktree"
  },
  {
    "command": [
      "cargo test --manifest-path adl/Cargo.toml provider_communication::tests::request_validation_rejects_empty_and_zero_policy_fields"
    ],
    "purpose": "Prove provider request validation rejects invalid GLM-5.3-Flash token, reasoning, temperature, and top-p overrides.",
    "outcome": "passed",
    "evidence_ref": "local command output, #578 worktree"
  },
  {
    "command": [
      "cargo test --manifest-path adl/Cargo.toml provider_adapter::tests::zai_glm_5_3_flash_adapter_request_uses_documented_defaults_and_fast_overrides"
    ],
    "purpose": "Prove adapter materialization emits documented GLM-5.3-Flash defaults, accepts fast reviewer overrides, and keeps legacy GLM-5 on the established endpoint.",
    "outcome": "passed",
    "evidence_ref": "local command output, #578 worktree"
  },
  {
    "command": [
      "cargo test --manifest-path adl/Cargo.toml --test provider_tests z_ai_glm_5_3_flash",
      "cargo test --manifest-path adl/Cargo.toml --test provider_tests expand_provider_profiles_accepts_zai_glm5_profile"
    ],
    "purpose": "Prove the new profile expands for reviewer selection and existing Z.ai GLM-5 profile expansion preserves its endpoint.",
    "outcome": "passed",
    "evidence_ref": "local command output, #578 worktree"
  },
  {
    "command": [
      ".csdlc/prepared/issues/578/glm-5-3-flash-reviewer-viability-smoke.sh"
    ],
    "purpose": "Exercise the reviewer-viability smoke harness; live provider dispatch remains credential-gated.",
    "outcome": "deferred",
    "evidence_ref": "skipped with `ZAI_API_KEY not set`; no live Z.ai request was performed"
  },
  {
    "command": [
      "git diff --check origin/main...HEAD",
      "git diff --check"
    ],
    "purpose": "Verify whitespace hygiene for the exact PR diff and uncommitted lifecycle delta.",
    "outcome": "passed",
    "evidence_ref": "local command output, #578 worktree"
  },
  {
    "command": [
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> .csdlc/prepared/issues/578/glm-5-3-flash-reviewer-viability-smoke.sh"
    ],
    "purpose": "Prove the direct Z.ai GLM-5.3-Flash profile can complete a reviewer-viability hosted request with the approved credential source, bounded reviewer-fast parameters, one attempt, and the 45-second timeout.",
    "outcome": "passed",
    "evidence_ref": ".adl/provider-smoke/glm-5-3-flash/result.json final_status=ok http_status=200 duration_ms=3748 output_text=GLM reviewer smoke ok; provider-run.jsonl run_finish status=ok"
  },
  {
    "command": [
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/glm-5-3-flash/reviewer-proof-request.json --out .adl/provider-smoke/glm-5-3-flash/reviewer-proof-result.json --log .adl/provider-smoke/glm-5-3-flash/reviewer-proof-run.jsonl"
    ],
    "purpose": "Prove Ox Alpha / GLM-5.3-Flash can complete a bounded reviewer-style verdict task through the direct Z.ai profile with the approved credential source and reviewer-fast parameters.",
    "outcome": "passed",
    "evidence_ref": ".adl/provider-smoke/glm-5-3-flash/reviewer-proof-result.json final_status=ok http_status=200 duration_ms=2405 output_text='VERDICT: PASS; FINDINGS: none; LIMITATIONS: Single-run smoke with max_attempts=1 and 64-token output confirms connectivity only, not sustained reliability or reasoning quality of the glm-5.3-flash endpoint.'; provider-run.jsonl run_finish status=ok"
  },
  {
    "command": [
      "gh pr list --repo agent-logic/agent-design-language --state open --limit 50 --json number,title,headRefName,headRefOid,baseRefName,isDraft,mergeable,url",
      "gh pr diff <pr> --repo agent-logic/agent-design-language --patch",
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/open-pr-reviews/pr-<pr>-request.json --out .adl/provider-smoke/open-pr-reviews/pr-<pr>-result.json --log .adl/provider-smoke/open-pr-reviews/pr-<pr>-run.jsonl"
    ],
    "purpose": "Probe GLM-5.3-Flash as a local-only PR triage reviewer across all currently open ADL PRs using exact PR heads, PR metadata, changed-file lists, and bounded diff excerpts, with no GitHub review comments posted.",
    "outcome": "passed",
    "evidence_ref": "Open PR set: #588 e9816192741ae1b35f678c5b363fb5567605fcf5, #586 822c81e9d0ad15e479960de542d419e64c80e1f9, #585 8e0c2c217c42c8719fac55de07a4fe498900fac7, #584 a5474e6d4cecf90b989c36e5845acd91dbfa3ead, #582 497271aceb9bc17fed7469fb48681d658a8af14e. All five provider calls final_status=ok within 45s: #588 9667ms, #586 7922ms, #585 8664ms, #584 12906ms, #582 8993ms. Model returned NEEDS_HUMAN_REVIEW for all five because diffs were truncated; this proves fast triage behavior and honest limitation handling, not exact-head approval."
  },
  {
    "command": [
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/open-pr-reviews/pr-582-max-request.json --out .adl/provider-smoke/open-pr-reviews/pr-582-max-result.json --log .adl/provider-smoke/open-pr-reviews/pr-582-max-run.jsonl",
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/open-pr-reviews/pr-582-max-180s-request.json --out .adl/provider-smoke/open-pr-reviews/pr-582-max-180s-result.json --log .adl/provider-smoke/open-pr-reviews/pr-582-max-180s-run.jsonl",
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/open-pr-reviews/pr-582-high-request.json --out .adl/provider-smoke/open-pr-reviews/pr-582-high-result.json --log .adl/provider-smoke/open-pr-reviews/pr-582-high-run.jsonl"
    ],
    "purpose": "Characterize whether higher GLM-5.3-Flash reasoning_effort settings are operationally usable for PR-review prompts after low-effort review triage succeeded.",
    "outcome": "passed",
    "evidence_ref": "Operational characterization passed: high/max effort failures were reproduced and bounded. reasoning_effort=max with 45s timeout returned final_status=failed provider_empty_text_output at 37960ms; reasoning_effort=max with 180s timeout returned final_status=failed provider_empty_text_output at 33979ms; reasoning_effort=high with 90s timeout returned final_status=failed provider_empty_text_output at 37169ms. Keep reviewer-trial default at low; treat high/max as not viable for this PR-review prompt shape until separately repaired."
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
