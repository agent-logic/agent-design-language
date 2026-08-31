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
  },
  {
    "command": [
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-low-request.json --out .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-low-result.json --log .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-low-run.jsonl",
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-high-request.json --out .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-high-result.json --log .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-high-run.jsonl",
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/glm-5-3-flash-quality/current-endpoint-repair-high-request.json --out .adl/provider-smoke/glm-5-3-flash-quality/current-endpoint-repair-high-result.json --log .adl/provider-smoke/glm-5-3-flash-quality/current-endpoint-repair-high-run.jsonl"
    ],
    "purpose": "Test GLM-5.3-Flash reviewer quality without truncated packets by comparing focused, complete endpoint-regression packets against the known #582 review failure and the repaired current endpoint split.",
    "outcome": "passed",
    "evidence_ref": "Focused complete-packet reviewer quality characterization passed: old rejected endpoint candidate low effort returned FAIL in 8752ms and correctly identified both z_ai:glm-5 and z_ai:glm-5-current rerouted from open.bigmodel.cn to api.z.ai; old rejected endpoint candidate high effort returned FAIL in 7668ms with the same migration-scope finding; current repaired endpoint candidate high effort returned PASS in 10240ms and did not repeat the stale endpoint finding. Conclusion: GLM-5.3-Flash is viable for focused complete-packet first-pass review, but truncated/noisy PR packets remain unsafe for exact-head approval."
  },
  {
    "command": [
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-low-request.json --out .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-low-result.json --log .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-low-run.jsonl",
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-high-request.json --out .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-high-result.json --log .adl/provider-smoke/glm-5-3-flash-quality/old-endpoint-regression-high-run.jsonl",
      "env ZAI_API_KEY=<redacted from /Users/daniel/keys/z.ai.ADL-default.key> cargo run --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- --request .adl/provider-smoke/glm-5-3-flash-quality/current-endpoint-repair-high-request.json --out .adl/provider-smoke/glm-5-3-flash-quality/current-endpoint-repair-high-result.json --log .adl/provider-smoke/glm-5-3-flash-quality/current-endpoint-repair-high-run.jsonl"
    ],
    "purpose": "Test GLM-5.3-Flash reviewer quality without truncated packets by comparing focused endpoint-regression packets against the rejected #582 head 724515224c36e55bed59e16c08e99d559271fa7d and the repaired worktree head c9759517d30b9ea14175be6d6fdaf9d019183593.",
    "outcome": "passed",
    "evidence_ref": "Packet-bound reviewer quality characterization passed: rejected #582 head 724515224c36e55bed59e16c08e99d559271fa7d low-effort request sha256 22276c77f89ec5e5de0e3a627a5ff18ded42f8e3dc27f24802a2d036220d9c31 returned FAIL in 8752ms and correctly identified both z_ai:glm-5 and z_ai:glm-5-current rerouted from open.bigmodel.cn to api.z.ai; rejected #582 head 724515224c36e55bed59e16c08e99d559271fa7d high-effort request sha256 23cfe04eab78a62bb8da103b5e560f3ed2c13f76d16ddfb60e38392d594ae76e returned FAIL in 7668ms with the same migration-scope finding; repaired worktree head c9759517d30b9ea14175be6d6fdaf9d019183593 high-effort request sha256 daec9511fedded4f188de98ac5b5231b48b12f3d09eb6851e270fe52e635cec5 returned PASS in 10240ms and did not repeat the stale endpoint finding. Result digests: old-low a30a311fa1582411737328e613d76d0b47af10025ea6b24dee137decf62014a9, old-high 4c434c2f818474cad2649b11785c14f366c493b05386101a0ab0fecee496f09e, current-high 6b46ccc3a1506074da7499d4b81639a9bdbe04b29d36c2b571a49c8b0e2f210d. Run-log digests: old-low 687d895b9801c7b6515d3fdbe94970a412d27fb99e777fd6b06c8d361a3329ce, old-high c65831cb869f77b77dd00739d34ce475734b129743b3fcb9de9b552a36786120, current-high 5cd08d46e77271703c4fab4673a5d9424087bbf6859d731906f14abf4a325107. Conclusion: GLM-5.3-Flash is viable for focused packet-bound first-pass review, but truncated/noisy PR packets remain unsafe for exact-head approval."
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
