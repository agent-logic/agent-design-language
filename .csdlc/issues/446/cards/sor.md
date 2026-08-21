# Structured Output Record

Template: 1.0.0

Issue: 446

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Wired six-resident Runtime tool authority and actual long-lived provider output through single-proposal extraction, UTS-to-ACC compilation, configured Freedom Gate evaluation, production Runtime observation dispatch, and checkpoint-bound redacted terminal receipts; production CLI regression proof now requires historical fixture-backed demo actuation to refuse before artifact publication while library-only fixture tests remain executable.

## Artifacts

- implementation-tree:48075df8480802ed6629069c57ce795e39281d09
- commit:e040eb35020fd4673422cc41841a44b5143345be
- evidence-commit:ac01f0181344e7571e706ed35366e4c619faecf6
- ci-repair-commit:2ba8feae04bef2f2151d0b094851d0b421390138
- .csdlc/evidence/446/live-gemma4-runtime-acc.log
- adl-runtime/src/resident_agent.rs
- adl/src/resident_tool_execution.rs
- adl/src/long_lived_agent.rs
- adl/src/governed_executor_parts/logic.rs
- adl/src/lib.rs

## Execution

- Added canonical digest-bound resident tool authority metadata and tamper denial in adl-runtime.
- Added Runtime-owned single-proposal extraction, including duplicate-proposal denial and redacted proposal identifiers.
- Routed actual long-lived Runtime provider output through UTS-to-ACC compilation and configured Freedom Gate policy before adapter dispatch.
- Added the production runtime.observe registry and bounded redacted Runtime snapshot adapter; fixture dispatch remains test-only and production fallback fails closed.
- Bound every terminal execution or denial receipt to resident identity, cycle, and exact checkpoint digest.
- Added deterministic full-tick and opt-in live Ollama gemma4:12b-mlx proof without changing generic provider configuration or expanding v0.92 scope.
- Corrected the causal workspace-coverage regression so production CLI tests assert fail-closed refusal of the historical fixture adapter, while cfg(test) library demo coverage still proves the bounded fixture behavior.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "resident_tool_execution",
      "--lib",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove allow, authority mismatch, compiler denial, gate denial, unsupported adapter denial, receipt redaction, and identical duplicate-proposal denial.",
    "outcome": "passed",
    "evidence_ref": "local exact implementation tree: 7 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "long_lived_agent::tests::tick_routes_provider_output_through_runtime_acc_and_adapter",
      "--lib",
      "--",
      "--exact",
      "--nocapture"
    ],
    "purpose": "Prove a full deterministic Runtime tick routes real provider StepOutput through ACC, Freedom Gate, production adapter, and checkpoint-bound receipt creation.",
    "outcome": "passed",
    "evidence_ref": "local exact implementation tree: 1 passed"
  },
  {
    "command": [
      "env",
      "ADL_TEST_LIVE_RESIDENT_MODEL=gemma4:12b-mlx",
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "long_lived_agent::tests::tick_routes_provider_output_through_runtime_acc_and_adapter",
      "--lib",
      "--",
      "--exact",
      "--nocapture"
    ],
    "purpose": "Prove the same full Runtime ACC path using a real local Ollama gemma4:12b-mlx agent response over HTTP.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/446/live-gemma4-runtime-acc.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "governed_executor",
      "--lib",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove governed executor behavior and test-only fixture isolation after injected adapter refactor.",
    "outcome": "passed",
    "evidence_ref": "local exact implementation tree: 25 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "resident_agent",
      "--lib",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove canonical resident authority construction, validation, and tamper denial.",
    "outcome": "passed",
    "evidence_ref": "local exact implementation tree: 5 passed"
  },
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib"
    ],
    "purpose": "Prove the Runtime library compiles across the integrated implementation.",
    "outcome": "passed",
    "evidence_ref": "local exact implementation tree"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove strict lint cleanliness for the integrated Runtime ACC path.",
    "outcome": "passed",
    "evidence_ref": "local exact implementation tree"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml",
      "--check"
    ],
    "purpose": "Prove Rust formatting hygiene.",
    "outcome": "passed",
    "evidence_ref": "local exact implementation tree"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove whitespace and conflict-marker hygiene.",
    "outcome": "passed",
    "evidence_ref": "local exact implementation tree"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "cli::runtime_v2_cmd::tests::trace_runtime_v2_governed_tools_flagship_demo",
      "--bin",
      "adl",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove production CLI paths refuse historical fixture adapter actuation before artifact publication while preserving argument/help behavior.",
    "outcome": "passed",
    "evidence_ref": "local CI repair: 2 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "runtime_v2_governed_tools_flagship_demo",
      "--lib",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove the bounded fixture-backed historical demo remains available only inside cfg(test) library validation.",
    "outcome": "passed",
    "evidence_ref": "local CI repair: 3 passed"
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
