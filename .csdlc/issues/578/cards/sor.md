# Structured Output Record

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a general direct Z.ai GLM-5.3-Flash provider profile, preserved existing Z.ai GLM-5 endpoints, corrected runtime-safe defaults to low effort with clear thinking, and proved the direct profile through live reviewer-style calls including high/max budget characterization.

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
- .csdlc/prepared/issues/578/design.md
- .csdlc/prepared/issues/578/reviewer-selection-smoke.sh
- .csdlc/prepared/issues/578/glm-5-3-flash-reviewer-viability-smoke.sh
- .csdlc/prepared/issues/578/tooling-issue-bind-prepared-helper-materialization.md

## Execution

- Added `z_ai:glm-5.3-flash` as a first-class provider profile resolving to `hosted:adl-z-ai:glm-5.3-flash` and provider model id `glm-5.3-flash`.
- Scoped the new `https://api.z.ai/api/paas/v4/chat/completions` endpoint to GLM-5.3-Flash while preserving the established `https://open.bigmodel.cn/api/paas/v4/chat/completions` endpoint for existing `z_ai:glm-5` and `z_ai:glm-5-current` profiles.
- Materialized GLM-5.3-Flash defaults for `reasoning_effort=low`, `clear_thinking=true`, `temperature=1.0`, `top_p=0.95`, and bounded output tokens; high and max remain explicit runtime overrides.
- Kept provider-facing `reasoning_effort` validation strict to documented values `low`, `high`, and `max`; a human-facing medium tier must be an ADL preset mapped internally to `high` with larger output budget and timeout, not a raw Z.ai parameter.
- Added runtime adapter request fields for reasoning effort, thinking cleanup, temperature, and top-p, with fail-closed validation for invalid GLM-5.3-Flash overrides before network dispatch.
- Added a credential-gated reviewer viability smoke script and live proof using the approved local Z.ai key source without printing or serializing the key.
- Documented the direct profile, source rationale, provider-variant boundaries, preserved legacy endpoint behavior, Ox Alpha identity, live open-PR reviewer smoke, and high/max output-budget behavior.

## Validation

[
  {
    "command": [
      "cargo fmt --manifest-path adl/Cargo.toml --check"
    ],
    "purpose": "Verify Rust formatting for touched provider/profile/adapter code.",
    "outcome": "passed",
    "evidence_ref": "local command output, #578 worktree"
  },
  {
    "command": [
      "cargo test --manifest-path adl/Cargo.toml --test provider_tests"
    ],
    "purpose": "Prove provider profile expansion, request materialization, strict invalid override rejection, and existing provider coverage after default correction.",
    "outcome": "passed",
    "evidence_ref": "66 passed, 1 ignored"
  },
  {
    "command": [
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> .csdlc/prepared/issues/578/glm-5-3-flash-reviewer-viability-smoke.sh"
    ],
    "purpose": "Prove direct Z.ai GLM-5.3-Flash reviewer-viability hosted request with approved credential source and runtime-safe parameters.",
    "outcome": "passed",
    "evidence_ref": ".adl/provider-smoke/glm-5-3-flash/result.json status=passed duration_ms=1714"
  },
  {
    "command": [
      "raw curl probe to https://api.z.ai/api/paas/v4/chat/completions for reasoning_effort low/high/max with clear_thinking=true"
    ],
    "purpose": "Confirm Z.ai accepts documented low/high/max effort values and returns visible content; verify medium is not needed as a provider value.",
    "outcome": "passed",
    "evidence_ref": "low/high/max returned HTTP 200 and content OK; high/max included reasoning_content"
  },
  {
    "command": [
      "env ZAI_API_KEY=<redacted> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/open-pr-reviews/research/pr-582-high-boosted-request.json --out .adl/provider-smoke/open-pr-reviews/research/pr-582-high-boosted-result.json --log .adl/provider-smoke/open-pr-reviews/research/pr-582-high-boosted-run.jsonl",
      "env ZAI_API_KEY=<redacted> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/open-pr-reviews/research/pr-582-max-boosted-request.json --out .adl/provider-smoke/open-pr-reviews/research/pr-582-max-boosted-result.json --log .adl/provider-smoke/open-pr-reviews/research/pr-582-max-boosted-run.jsonl"
    ],
    "purpose": "Prove previous high/max empty-output failures were output-budget starvation rather than provider callability failure.",
    "outcome": "passed",
    "evidence_ref": "high 8192 tokens/180s passed at 36240ms; max 16384 tokens/240s passed at 151326ms"
  },
  {
    "command": [
      "git diff --check"
    ],
    "purpose": "Verify whitespace hygiene for the current issue worktree delta.",
    "outcome": "passed",
    "evidence_ref": "local command output, #578 worktree"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
